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
  import { onMount } from "svelte";

  let username = $state("");
  let password = $state("");
  let confirmPassword = $state("");
  let showPassword = $state(false);
  let showConfirmPassword = $state(false);
  let isLoading = $state(false);
  let error = $state("");
  let success = $state("");

  // Form validation
  let usernameError = $state("");
  let passwordError = $state("");
  let confirmPasswordError = $state("");

  function validateForm() {
    let isValid = true;

    // Reset errors
    usernameError = "";
    passwordError = "";
    confirmPasswordError = "";

    // Validate username
    if (!username.trim()) {
      usernameError = "Username is required";
      isValid = false;
    } else if (username.length < 3) {
      usernameError = "Username must be at least 3 characters";
      isValid = false;
    } else if (!/^[a-zA-Z0-9_]+$/.test(username)) {
      usernameError =
        "Username can only contain letters, numbers, and underscores";
      isValid = false;
    }

    // Validate password
    if (!password) {
      passwordError = "Password is required";
      isValid = false;
    } else if (password.length < 8) {
      passwordError = "Password must be at least 8 characters";
      isValid = false;
    } else if (!/(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/.test(password)) {
      passwordError =
        "Password must contain at least one uppercase letter, one lowercase letter, and one number";
      isValid = false;
    }

    // Validate confirm password
    if (!confirmPassword) {
      confirmPasswordError = "Please confirm your password";
      isValid = false;
    } else if (password !== confirmPassword) {
      confirmPasswordError = "Passwords do not match";
      isValid = false;
    }

    return isValid;
  }

  async function handleSubmit() {
    if (!validateForm()) return;

    isLoading = true;
    error = "";
    success = "";

    try {
      const response = await AuthService.register(username, password);

      // Store auth data
      authStore.login(
        { id: response.user_id, username: response.username },
        response.token
      );

      success = "Account created successfully! Redirecting...";

      // Wait a moment to show success message, then redirect
      setTimeout(() => {
        goto("/");
      }, 1500);
    } catch (err) {
      error = err instanceof Error ? err.message : "Registration failed";
    } finally {
      isLoading = false;
    }
  }

  function togglePasswordVisibility() {
    showPassword = !showPassword;
  }

  function toggleConfirmPasswordVisibility() {
    showConfirmPassword = !showConfirmPassword;
  }

  onMount(() => {
    authStore.initialize();
  });
</script>

<svelte:head>
  <title>Sign Up - FinanceVault</title>
  <meta name="description" content="Create your FinanceVault account" />
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
        <!-- Lock Icon placeholder -->
        <svg
          class="h-6 w-6 text-white"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
          />
        </svg>
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

    <!-- Sign Up Form -->
    <Card>
      <CardHeader class="space-y-1 text-center">
        <CardTitle class="text-2xl">Create account</CardTitle>
        <CardDescription>
          Enter your information to create a new account
        </CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        {#if error}
          <Alert variant="destructive">
            <!-- Alert Triangle Icon placeholder -->
            <svg
              class="h-4 w-4"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path
                d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"
              />
              <path d="M12 9v4" />
              <path d="m12 17 .01 0" />
            </svg>
            <AlertDescription>{error}</AlertDescription>
          </Alert>
        {/if}

        {#if success}
          <Alert
            class="border-green-200 bg-green-50 text-green-800 dark:border-green-800 dark:bg-green-950 dark:text-green-200"
          >
            <!-- Check Circle Icon placeholder -->
            <svg
              class="h-4 w-4"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
              <path d="m9 11 3 3L22 4" />
            </svg>
            <AlertDescription>{success}</AlertDescription>
          </Alert>
        {/if}

        <form onsubmit={handleSubmit} class="space-y-4">
          <!-- Username Field -->
          <div class="space-y-2">
            <Label for="username">Username</Label>
            <div class="relative">
              <!-- User Icon placeholder -->
              <svg
                class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                />
              </svg>
              <Input
                id="username"
                type="text"
                placeholder="Choose a username"
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
              <!-- Lock Icon placeholder -->
              <svg
                class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                />
              </svg>
              <Input
                id="password"
                type={showPassword ? "text" : "password"}
                placeholder="Create a password"
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
                  <!-- Eye Off Icon placeholder -->
                  <svg
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  >
                    <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24" />
                    <path
                      d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"
                    />
                    <path
                      d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"
                    />
                    <line x1="2" x2="22" y1="2" y2="22" />
                  </svg>
                {:else}
                  <!-- Eye Icon placeholder -->
                  <svg
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  >
                    <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" />
                    <circle cx="12" cy="12" r="3" />
                  </svg>
                {/if}
              </button>
            </div>
            {#if passwordError}
              <p class="text-sm text-red-600 dark:text-red-400">
                {passwordError}
              </p>
            {/if}
          </div>

          <!-- Confirm Password Field -->
          <div class="space-y-2">
            <Label for="confirmPassword">Confirm Password</Label>
            <div class="relative">
              <!-- Lock Icon placeholder -->
              <svg
                class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                />
              </svg>
              <Input
                id="confirmPassword"
                type={showConfirmPassword ? "text" : "password"}
                placeholder="Confirm your password"
                bind:value={confirmPassword}
                error={!!confirmPasswordError}
                class="pl-10 pr-10"
                disabled={isLoading}
                required
              />
              <button
                type="button"
                onclick={toggleConfirmPasswordVisibility}
                class="absolute right-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-400 hover:text-gray-600 transition-colors"
                disabled={isLoading}
              >
                {#if showConfirmPassword}
                  <!-- Eye Off Icon placeholder -->
                  <svg
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  >
                    <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24" />
                    <path
                      d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"
                    />
                    <path
                      d="M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"
                    />
                    <line x1="2" x2="22" y1="2" y2="22" />
                  </svg>
                {:else}
                  <!-- Eye Icon placeholder -->
                  <svg
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  >
                    <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z" />
                    <circle cx="12" cy="12" r="3" />
                  </svg>
                {/if}
              </button>
            </div>
            {#if confirmPasswordError}
              <p class="text-sm text-red-600 dark:text-red-400">
                {confirmPasswordError}
              </p>
            {/if}
          </div>

          <!-- Password Requirements -->
          <div class="text-xs text-gray-600 dark:text-gray-400 space-y-1">
            <p class="font-medium">Password requirements:</p>
            <ul class="space-y-1">
              <li class="flex items-center gap-2">
                <span
                  class={password.length >= 8
                    ? "text-green-600"
                    : "text-gray-400"}>•</span
                >
                At least 8 characters
              </li>
              <li class="flex items-center gap-2">
                <span
                  class={/(?=.*[a-z])/.test(password)
                    ? "text-green-600"
                    : "text-gray-400"}>•</span
                >
                One lowercase letter
              </li>
              <li class="flex items-center gap-2">
                <span
                  class={/(?=.*[A-Z])/.test(password)
                    ? "text-green-600"
                    : "text-gray-400"}>•</span
                >
                One uppercase letter
              </li>
              <li class="flex items-center gap-2">
                <span
                  class={/(?=.*\d)/.test(password)
                    ? "text-green-600"
                    : "text-gray-400"}>•</span
                >
                One number
              </li>
            </ul>
          </div>

          <!-- Submit Button -->
          <Button type="submit" class="w-full" disabled={isLoading}>
            {#if isLoading}
              <div
                class="animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2"
              ></div>
            {/if}
            {isLoading ? "Creating Account..." : "Create Account"}
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
              Already have an account?
            </span>
          </div>
        </div>

        <!-- Sign In Link -->
        <div class="text-center">
          <Button variant="link" href="/signin" class="text-sm">
            Sign in to your account
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
