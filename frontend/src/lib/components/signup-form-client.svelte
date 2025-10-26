<script lang="ts">
  import * as Card from "$lib/components/ui/card/index.js";
  import {
    FieldGroup,
    Field,
    FieldLabel,
    FieldDescription,
  } from "$lib/components/ui/field/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Alert, AlertDescription } from "$lib/components/ui/alert/index.js";
  import { cn } from "$lib/utils.js";
  import type { HTMLAttributes } from "svelte/elements";
  import { goto } from "$app/navigation";
  import { AuthService } from "$lib/services/auth";

  let { class: className, ...restProps }: HTMLAttributes<HTMLDivElement> =
    $props();

  const id = crypto.randomUUID();

  let username = $state("");
  let password = $state("");
  let confirmPassword = $state("");
  let isLoading = $state(false);
  let errorMessage = $state("");

  // Password validation
  let passwordError = $derived.by(() => {
    if (!password) return "";
    if (password.length < 8) return "Password must be at least 8 characters";
    if (!/(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/.test(password)) {
      return "Password must contain uppercase, lowercase, and number";
    }
    return "";
  });

  let confirmError = $derived.by(() => {
    if (!confirmPassword) return "";
    if (password !== confirmPassword) return "Passwords do not match";
    return "";
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();

    if (!username.trim()) {
      errorMessage = "Username is required";
      return;
    }

    if (passwordError || confirmError) {
      errorMessage = "Please fix the form errors";
      return;
    }

    isLoading = true;
    errorMessage = "";

    try {
      const response = await AuthService.register(username, password);

      // Set cookie via API
      await fetch("/api/set-auth-cookie", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ token: response.token }),
      });

      // Redirect to home
      goto("/");
    } catch (error) {
      errorMessage =
        error instanceof Error ? error.message : "Registration failed";
      isLoading = false;
    }
  }
</script>

<div class={cn("flex flex-col gap-6", className)} {...restProps}>
  <Card.Root class="overflow-hidden p-0">
    <Card.Content class="grid p-0 md:grid-cols-2">
      <form onsubmit={handleSubmit} class="p-6 md:p-8">
        <FieldGroup>
          <div class="flex flex-col items-center gap-2 text-center">
            <h1 class="text-2xl font-bold">Create account</h1>
            <p class="text-muted-foreground text-balance">
              Sign up for your FinanceVault account
            </p>
          </div>

          {#if errorMessage}
            <Alert variant="destructive">
              <AlertDescription>{errorMessage}</AlertDescription>
            </Alert>
          {/if}

          <Field>
            <FieldLabel for="username-{id}">Username</FieldLabel>
            <Input
              id="username-{id}"
              type="text"
              placeholder="Choose a username"
              bind:value={username}
              required
              disabled={isLoading}
            />
          </Field>

          <Field>
            <FieldLabel for="password-{id}">Password</FieldLabel>
            <Input
              id="password-{id}"
              type="password"
              bind:value={password}
              required
              disabled={isLoading}
            />
            {#if passwordError}
              <p class="text-sm text-destructive mt-1">{passwordError}</p>
            {/if}
          </Field>

          <Field>
            <FieldLabel for="confirmPassword-{id}">Confirm Password</FieldLabel>
            <Input
              id="confirmPassword-{id}"
              type="password"
              bind:value={confirmPassword}
              required
              disabled={isLoading}
            />
            {#if confirmError}
              <p class="text-sm text-destructive mt-1">{confirmError}</p>
            {/if}
          </Field>

          <!-- Password Requirements -->
          <div class="text-xs text-muted-foreground space-y-1">
            <p class="font-medium">Password requirements:</p>
            <ul class="space-y-1 pl-4">
              <li class:text-green-600={password.length >= 8}>
                • At least 8 characters
              </li>
              <li class:text-green-600={/(?=.*[a-z])/.test(password)}>
                • One lowercase letter
              </li>
              <li class:text-green-600={/(?=.*[A-Z])/.test(password)}>
                • One uppercase letter
              </li>
              <li class:text-green-600={/(?=.*\d)/.test(password)}>
                • One number
              </li>
            </ul>
          </div>

          <Field>
            <Button
              type="submit"
              class="w-full"
              disabled={isLoading || !!passwordError || !!confirmError}
            >
              {#if isLoading}
                <div
                  class="animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2"
                ></div>
              {/if}
              {isLoading ? "Creating account..." : "Sign up"}
            </Button>
          </Field>

          <FieldDescription class="text-center">
            Already have an account? <a href="/signin" class="underline"
              >Sign in</a
            >
          </FieldDescription>
        </FieldGroup>
      </form>
      <div class="bg-muted relative hidden md:block">
        <img
          src="/logos/FinanceVault.png"
          alt="FinanceVault Logo"
          class="absolute inset-0 h-full w-full object-cover dark:brightness-[0.8]"
        />
      </div>
    </Card.Content>
  </Card.Root>
</div>
