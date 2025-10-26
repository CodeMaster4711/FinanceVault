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
  let isLoading = $state(false);
  let errorMessage = $state("");

  async function handleSubmit(event: Event) {
    event.preventDefault();

    if (!username.trim() || !password) {
      errorMessage = "Username and password are required";
      return;
    }

    isLoading = true;
    errorMessage = "";

    try {
      const response = await AuthService.login(username, password);

      // Set cookie via API
      await fetch("/api/set-auth-cookie", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ token: response.token }),
      });

      // Redirect to home
      goto("/");
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : "Login failed";
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
            <h1 class="text-2xl font-bold">Welcome back</h1>
            <p class="text-muted-foreground text-balance">
              Login to your FinanceVault account
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
              placeholder="Enter your username"
              bind:value={username}
              required
              disabled={isLoading}
            />
          </Field>
          <Field>
            <div class="flex items-center">
              <FieldLabel for="password-{id}">Password</FieldLabel>
              <a
                href="##"
                class="ml-auto text-sm underline-offset-2 hover:underline"
              >
                Forgot your password?
              </a>
            </div>
            <Input
              id="password-{id}"
              type="password"
              bind:value={password}
              required
              disabled={isLoading}
            />
          </Field>
          <Field>
            <Button type="submit" class="w-full" disabled={isLoading}>
              {#if isLoading}
                <div
                  class="animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2"
                ></div>
              {/if}
              {isLoading ? "Logging in..." : "Login"}
            </Button>
          </Field>
          <FieldDescription class="text-center">
            Don't have an account? <a href="/signup" class="underline"
              >Sign up</a
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
