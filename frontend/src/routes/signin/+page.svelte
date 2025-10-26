<script lang="ts">
  import { goto } from "$app/navigation";
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Button } from "$lib/components/ui/button";
  import { Alert, AlertDescription } from "$lib/components/ui/alert";
  import { AuthService } from "$lib/services/auth";
  import { authStore } from "$lib/stores/auth";
  import { AlertTriangle, Eye, EyeOff, Lock, User } from "@lucide/svelte";
  import { onMount } from "svelte";

  let username = $state("");
  let password = $state("");
  let showPassword = $state(false);
  let isLoading = $state(false);
  let error = $state("");

  // Form validation
  let usernameError = $state("");
  let passwordError = $state("");

  function validateForm() {
    let isValid = true;

    // Reset errors
    usernameError = "";
    passwordError = "";

    // Validate username
    if (!username.trim()) {
      usernameError = "Username is required";
      isValid = false;
    } else if (username.length < 3) {
      usernameError = "Username must be at least 3 characters";
      isValid = false;
    }

    // Validate password
    if (!password) {
      passwordError = "Password is required";
      isValid = false;
    } else if (password.length < 6) {
      passwordError = "Password must be at least 6 characters";
      isValid = false;
    }

    return isValid;
  }

  async function handleSubmit() {
    if (!validateForm()) return;

    isLoading = true;
    error = "";

    try {
      const response = await AuthService.login(username, password);

      // Store auth data
      authStore.login(
        { id: response.user_id, username: response.username },
        response.token
      );

      // Redirect to dashboard or home
      goto("/");
    } catch (err) {
      error = err instanceof Error ? err.message : "Login failed";
    } finally {
      isLoading = false;
    }
  }

  function togglePasswordVisibility() {
    showPassword = !showPassword;
  }

  onMount(() => {
    authStore.initialize();
  });
</script>

<svelte:head>
  <title>Sign In - FinanceVault</title>
  <meta name="description" content="Sign in to your FinanceVault account" />
</svelte:head>

<div
  class="min-h-screen flex items-center justify-center bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-950 dark:to-slate-900 p-4"
>
  <div class="w-full max-w-md space-y-8">
    <!-- Logo/Branding -->
    <div class="text-center">
      <div
        class="mx-auto h-12 w-12 bg-gradient-to-r from-blue-600 to-purple-600 rounded-xl flex items-center justify-center mb-4"
      >
        <Lock class="h-6 w-6 text-white" />
      </div>
      <h1
        class="text-3xl font-bold tracking-tight text-gray-900 dark:text-gray-100"
      >
        FinanceVault
      </h1>
      <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
        Secure your financial future
      </p>
    </div>

    <!-- Sign In Form -->
    <Card>
      <CardHeader class="space-y-1 text-center">
        <CardTitle class="text-2xl">Welcome back</CardTitle>
        <CardDescription>
          Enter your credentials to access your account
        </CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        {#if error}
          <Alert variant="destructive">
            <AlertTriangle class="h-4 w-4" />
            <AlertDescription>{error}</AlertDescription>
          </Alert>
        {/if}

        <form onsubmit={handleSubmit} class="space-y-4">
          <!-- Username Field -->
          <div class="space-y-2">
            <Label for="username">Username</Label>
            <div class="relative">
              <User
                class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400"
              />
              <Input
                id="username"
                type="text"
                placeholder="Enter your username"
                bind:value={username}
                error={!!usernameError}
                class="pl-10"
                disabled={isLoading}
                required
              />
            </div>
            {#if usernameError}
              <p class="text-sm text-red-600 dark:text-red-400">
                {usernameError}
              </p>
            {/if}
          </div>

          <!-- Password Field -->
          <div class="space-y-2">
            <Label for="password">Password</Label>
            <div class="relative">
              <Lock
                class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400"
              />
              <Input
                id="password"
                type={showPassword ? "text" : "password"}
                placeholder="Enter your password"
                bind:value={password}
                error={!!passwordError}
                class="pl-10 pr-10"
                disabled={isLoading}
                required
              />
              <button
                type="button"
                onclick={togglePasswordVisibility}
                class="absolute right-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400 hover:text-gray-600 transition-colors"
                disabled={isLoading}
              >
                {#if showPassword}
                  <EyeOff class="h-4 w-4" />
                {:else}
                  <Eye class="h-4 w-4" />
                {/if}
              </button>
            </div>
            {#if passwordError}
              <p class="text-sm text-red-600 dark:text-red-400">
                {passwordError}
              </p>
            {/if}
          </div>

          <!-- Submit Button -->
          <Button type="submit" class="w-full" disabled={isLoading}>
            {#if isLoading}
              <div
                class="animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2"
              ></div>
            {/if}
            {isLoading ? "Signing In..." : "Sign In"}
          </Button>
        </form>

        <!-- Divider -->
        <div class="relative">
          <div class="absolute inset-0 flex items-center">
            <span class="w-full border-t border-gray-300 dark:border-gray-700"
            ></span>
          </div>
          <div class="relative flex justify-center text-xs uppercase">
            <span
              class="bg-white dark:bg-gray-950 px-2 text-gray-500 dark:text-gray-400"
            >
              Don't have an account?
            </span>
          </div>
        </div>

        <!-- Sign Up Link -->
        <div class="text-center">
          <Button variant="link" href="/signup" class="text-sm">
            Create an account
          </Button>
        </div>
      </CardContent>
    </Card>

    <!-- Footer -->
    <div class="text-center text-xs text-gray-500 dark:text-gray-400">
      <p>© 2024 FinanceVault. All rights reserved.</p>
    </div>
  </div>
</div>
