<script lang="ts">
  import "../app.css";
  import favicon from "$lib/assets/favicon.svg";
  import { onMount } from "svelte";
  import AppSidebar from "$lib/components/navbar/app-sidebar.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { effectiveTheme, initThemeFromStorage } from "$lib/stores/theme";
  import { page } from "$app/stores";
  import { vault } from "$lib/stores/vault";
  import VaultGate from "$lib/components/vault/vault-gate.svelte";

  let { children } = $props();

  let showSidebar = $derived(
    $vault.status === "unlocked" &&
      !$page.url.pathname.startsWith("/signin") &&
      !$page.url.pathname.startsWith("/signup")
  );

  onMount(() => {
    vault.init();
    initThemeFromStorage();

    const unsub = effectiveTheme.subscribe((t) => {
      if (t === "dark") document.documentElement.classList.add("dark");
      else document.documentElement.classList.remove("dark");
      document.documentElement.setAttribute(
        "data-theme",
        t === "dark" ? "dark" : "light"
      );
    });

    return unsub;
  });
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

{#if $vault.status === "checking"}
  <div class="flex h-screen items-center justify-center bg-background">
    <span class="text-muted-foreground text-sm">Loading...</span>
  </div>
{:else if $vault.status === "uninitialized" || $vault.status === "locked"}
  <VaultGate />
{:else if showSidebar}
  <Sidebar.Provider>
    <AppSidebar />
    <Sidebar.Inset>
      <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
        {@render children?.()}
      </div>
    </Sidebar.Inset>
  </Sidebar.Provider>
{:else}
  {@render children?.()}
{/if}
