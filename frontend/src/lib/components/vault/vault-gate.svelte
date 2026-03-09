<script lang="ts">
  import { vault } from "$lib/stores/vault";
  import { invoke } from "@tauri-apps/api/core";

  type Mode = "unlock" | "setup" | "setup-confirm" | "totp-setup";

  let mode = $derived<Mode>(
    $vault.status === "uninitialized" ? "setup" : "unlock"
  );

  let passphrase = $state("");
  let passphraseConfirm = $state("");
  let totpCode = $state("");
  let totpSecret = $state("");
  let totpQr = $state("");
  let totpEnabled = $state(false);
  let loading = $state(false);
  let localError = $state<string | null>(null);

  async function handleSetup() {
    localError = null;
    if (passphrase.length < 12) {
      localError = "Passphrase must be at least 12 characters.";
      return;
    }
    if (passphrase !== passphraseConfirm) {
      localError = "Passphrases do not match.";
      return;
    }
    loading = true;
    const ok = await vault.setup(passphrase);
    loading = false;
    if (ok) {
      passphrase = "";
      passphraseConfirm = "";
    }
  }

  async function handleUnlock() {
    localError = null;
    loading = true;
    const hasTotpEnabled = await invoke<boolean>("totp_is_enabled").catch(
      () => false
    );
    totpEnabled = hasTotpEnabled;

    if (hasTotpEnabled && !totpCode) {
      loading = false;
      localError = "Please enter your 2FA code.";
      return;
    }

    const ok = await vault.unlock(passphrase, totpCode || undefined);
    loading = false;
    if (!ok) {
      passphrase = "";
      totpCode = "";
    }
  }

  async function handleTotpSetup() {
    localError = null;
    loading = true;
    try {
      const secret = await invoke<string>("totp_generate_secret");
      const qr = await invoke<string>("totp_get_qr_base64", {
        secretBase32: secret,
        account: "FinanceVault",
      });
      totpSecret = secret;
      totpQr = qr;
    } catch (e) {
      localError = e as string;
    }
    loading = false;
  }

  async function handleTotpEnable() {
    localError = null;
    loading = true;
    try {
      await invoke("totp_enable", {
        secretBase32: totpSecret,
        code: totpCode,
      });
      totpCode = "";
      totpSecret = "";
      totpQr = "";
    } catch (e) {
      localError = e as string;
    }
    loading = false;
  }
</script>

<div
  class="flex h-screen items-center justify-center bg-background text-foreground"
>
  <div class="w-full max-w-sm space-y-6 px-6">
    <div class="space-y-1 text-center">
      <h1 class="text-2xl font-semibold tracking-tight">FinanceVault</h1>
      <p class="text-muted-foreground text-sm">
        {#if $vault.status === "uninitialized"}
          Create your encrypted vault
        {:else if totpSecret}
          Scan the QR code with your authenticator app
        {:else}
          Enter your passphrase to unlock
        {/if}
      </p>
    </div>

    {#if $vault.status === "uninitialized" && !totpSecret}
      <form onsubmit={(e) => { e.preventDefault(); handleSetup(); }} class="space-y-4">
        <div class="space-y-2">
          <label for="passphrase" class="text-sm font-medium">Passphrase</label>
          <input
            id="passphrase"
            type="password"
            bind:value={passphrase}
            placeholder="Min. 12 characters"
            class="w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
            required
          />
        </div>
        <div class="space-y-2">
          <label for="passphrase-confirm" class="text-sm font-medium"
            >Confirm Passphrase</label
          >
          <input
            id="passphrase-confirm"
            type="password"
            bind:value={passphraseConfirm}
            placeholder="Repeat passphrase"
            class="w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
            required
          />
        </div>
        {#if localError || $vault.error}
          <p class="text-destructive text-sm">{localError ?? $vault.error}</p>
        {/if}
        <button
          type="submit"
          disabled={loading}
          class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
        >
          {loading ? "Creating vault..." : "Create Vault"}
        </button>
      </form>
    {:else if totpSecret}
      <div class="space-y-4">
        <div class="flex justify-center">
          <img
            src={`data:image/png;base64,${totpQr}`}
            alt="TOTP QR Code"
            class="rounded-md"
            width="200"
            height="200"
          />
        </div>
        <p class="text-muted-foreground break-all text-center font-mono text-xs">
          {totpSecret}
        </p>
        <div class="space-y-2">
          <label for="totp-verify" class="text-sm font-medium"
            >Verify 6-digit code</label
          >
          <input
            id="totp-verify"
            type="text"
            inputmode="numeric"
            maxlength="6"
            bind:value={totpCode}
            placeholder="000000"
            class="w-full rounded-md border border-input bg-transparent px-3 py-2 text-center font-mono text-sm tracking-widest focus:outline-none focus:ring-2 focus:ring-ring"
          />
        </div>
        {#if localError}
          <p class="text-destructive text-sm">{localError}</p>
        {/if}
        <button
          onclick={handleTotpEnable}
          disabled={loading || totpCode.length !== 6}
          class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
        >
          {loading ? "Enabling 2FA..." : "Enable 2FA"}
        </button>
        <button
          onclick={() => { totpSecret = ""; totpQr = ""; totpCode = ""; }}
          class="w-full rounded-md border border-input px-4 py-2 text-sm hover:bg-accent"
        >
          Skip for now
        </button>
      </div>
    {:else}
      <form onsubmit={(e) => { e.preventDefault(); handleUnlock(); }} class="space-y-4">
        <div class="space-y-2">
          <label for="unlock-passphrase" class="text-sm font-medium"
            >Passphrase</label
          >
          <input
            id="unlock-passphrase"
            type="password"
            bind:value={passphrase}
            placeholder="Your vault passphrase"
            class="w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
            required
            autofocus
          />
        </div>
        {#if totpEnabled}
          <div class="space-y-2">
            <label for="totp-code" class="text-sm font-medium">2FA Code</label>
            <input
              id="totp-code"
              type="text"
              inputmode="numeric"
              maxlength="6"
              bind:value={totpCode}
              placeholder="000000"
              class="w-full rounded-md border border-input bg-transparent px-3 py-2 text-center font-mono text-sm tracking-widest focus:outline-none focus:ring-2 focus:ring-ring"
            />
          </div>
        {/if}
        {#if localError || $vault.error}
          <p class="text-destructive text-sm">{localError ?? $vault.error}</p>
        {/if}
        <button
          type="submit"
          disabled={loading}
          class="w-full rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
        >
          {loading ? "Unlocking..." : "Unlock Vault"}
        </button>
        {#if !totpEnabled}
          <button
            type="button"
            onclick={handleTotpSetup}
            disabled={loading}
            class="w-full rounded-md border border-input px-4 py-2 text-sm hover:bg-accent disabled:opacity-50"
          >
            Setup 2FA
          </button>
        {/if}
      </form>
    {/if}
  </div>
</div>
