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
  import { authStore } from "$lib/stores/auth";

  let { class: className, ...restProps }: HTMLAttributes<HTMLDivElement> =
    $props();

  const id = crypto.randomUUID();

  let username = $state("");
  let password = $state("");
  let confirmPassword = $state("");
  let isLoading = $state(false);
  let errorMessage = $state("");
  let particlesContainer: HTMLDivElement;

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

  // Initialize particles and lines when component mounts
  $effect(() => {
    if (particlesContainer) {
      initializeAnimations();
    }
  });

  function initializeAnimations() {
    // Clear existing particles
    particlesContainer.innerHTML = "";

    // Create particles
    for (let i = 0; i < 30; i++) {
      const particle = document.createElement("div");
      particle.className = "particle";

      const startX = Math.random() * 100;
      const startY = Math.random() * 100;
      const endX = (Math.random() - 0.5) * 400;
      const endY = (Math.random() - 0.5) * 400;

      particle.style.left = startX + "%";
      particle.style.top = startY + "%";
      particle.style.setProperty("--tx", endX + "px");
      particle.style.setProperty("--ty", endY + "px");
      particle.style.animationDelay = Math.random() * 15 + "s";
      particle.style.animationDuration = 10 + Math.random() * 10 + "s";

      particlesContainer.appendChild(particle);
    }

    // Create lines
    for (let i = 0; i < 5; i++) {
      const line = document.createElement("div");
      line.className = "animated-line";

      const width = 100 + Math.random() * 200;
      const startY = Math.random() * 100;
      const angle = (Math.random() - 0.5) * 30;

      line.style.width = width + "px";
      line.style.top = startY + "%";
      line.style.left = "-200px";
      line.style.setProperty("--angle", angle + "deg");
      line.style.animationDelay = Math.random() * 8 + "s";
      line.style.animationDuration = 6 + Math.random() * 4 + "s";

      particlesContainer.appendChild(line);
    }
  }

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

      // Update auth store
      authStore.login(
        { id: response.user_id, username: response.username },
        response.token
      );

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

<div class={cn("min-h-screen flex", className)} {...restProps}>
  <!-- Left Side - Signup Form -->
  <div class="flex-1 flex items-center justify-center p-8 bg-background">
    <div class="w-full max-w-md space-y-8">
      <div class="text-center">
        <h1 class="text-3xl font-bold tracking-tight">Create your account</h1>
        <p class="text-muted-foreground mt-2">
          Sign up for your FinanceVault account
        </p>
      </div>

      <form onsubmit={handleSubmit} class="space-y-6">
        {#if errorMessage}
          <Alert variant="destructive">
            <AlertDescription>{errorMessage}</AlertDescription>
          </Alert>
        {/if}

        <FieldGroup class="space-y-4">
          <Field>
            <FieldLabel for="username-{id}" class="text-sm font-medium"
              >Username</FieldLabel
            >
            <Input
              id="username-{id}"
              type="text"
              placeholder="Choose a username"
              bind:value={username}
              required
              disabled={isLoading}
              class="mt-1"
            />
          </Field>

          <Field>
            <FieldLabel for="password-{id}" class="text-sm font-medium"
              >Password</FieldLabel
            >
            <Input
              id="password-{id}"
              type="password"
              bind:value={password}
              required
              disabled={isLoading}
              class="mt-1"
            />
            {#if passwordError}
              <p class="text-sm text-destructive mt-1">{passwordError}</p>
            {/if}
          </Field>

          <Field>
            <FieldLabel for="confirmPassword-{id}" class="text-sm font-medium"
              >Confirm Password</FieldLabel
            >
            <Input
              id="confirmPassword-{id}"
              type="password"
              bind:value={confirmPassword}
              required
              disabled={isLoading}
              class="mt-1"
            />
            {#if confirmError}
              <p class="text-sm text-destructive mt-1">{confirmError}</p>
            {/if}
          </Field>

          <Button
            type="submit"
            class="w-full h-11"
            disabled={isLoading || !!passwordError || !!confirmError}
          >
            {#if isLoading}
              <div
                class="animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2"
              ></div>
            {/if}
            {isLoading ? "Creating account..." : "Sign up"}
          </Button>

          <div class="text-center text-sm text-muted-foreground">
            Already have an account?
            <a
              href="/signin"
              class="text-primary hover:text-primary/80 underline underline-offset-2"
              >Sign in</a
            >
          </div>
        </FieldGroup>
      </form>
    </div>
  </div>

  <!-- Right Side - Animated Background with Features -->
  <div class="flex-1 relative hidden lg:flex flex-col overflow-hidden">
    <!-- Animated Background Container -->
    <div class="absolute inset-0 rounded-l-3xl overflow-hidden animated-bg">
      <!-- Gradient Orbs -->
      <div class="gradient-orb orb1"></div>
      <div class="gradient-orb orb2"></div>
      <div class="gradient-orb orb3"></div>

      <!-- Particles Container -->
      <div class="particles-container" bind:this={particlesContainer}></div>

      <!-- Blur Overlay -->
      <div class="blur-overlay"></div>
    </div>

    <!-- Logo Overlay -->
    <div class="absolute inset-0 flex items-center justify-center z-10">
      <div class="text-center text-white">
        <img
          src="/logos/FinanceVault.png"
          alt="FinanceVault Logo"
          class="mx-auto mb-4 w-32 h-32 md:w-40 h-40 lg:w-[240px] h-[240px]"
        />

        <h2 class="text-4xl font-bold mb-4 text-shadow-glow">FinanceVault</h2>
      </div>
    </div>
  </div>
</div>

<style>
  /* Gradient Orbs */
  .gradient-orb {
    position: absolute;
    border-radius: 50%;
    filter: blur(80px);
    opacity: 0.6;
    animation: float 20s infinite ease-in-out;
  }

  .animated-bg {
    background: linear-gradient(
      135deg,
      var(--color-primary),
      var(--color-chart-2),
      var(--color-chart-1)
    );
  }

  .orb1 {
    width: 600px;
    height: 600px;
    background: radial-gradient(
      circle,
      var(--color-chart-1) 0%,
      var(--color-chart-2) 70%
    );
    top: -200px;
    left: -200px;
    animation-delay: 0s;
  }

  .orb2 {
    width: 500px;
    height: 500px;
    background: radial-gradient(
      circle,
      var(--color-chart-2) 0%,
      var(--color-primary) 70%
    );
    bottom: -150px;
    right: -150px;
    animation-delay: -7s;
  }

  .orb3 {
    width: 400px;
    height: 400px;
    background: radial-gradient(
      circle,
      var(--color-chart-3) 0%,
      var(--color-chart-2) 70%
    );
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    animation-delay: -14s;
  }

  .particles-container {
    position: absolute;
    width: 100%;
    height: 100%;
  }

  .particle {
    position: absolute;
    width: 3px;
    height: 3px;
    background: var(--color-chart-3);
    border-radius: 50%;
    animation: particleFloat 15s infinite ease-in-out;
  }

  .animated-line {
    position: absolute;
    height: 2px;
    background: linear-gradient(
      90deg,
      transparent,
      var(--color-chart-3),
      transparent
    );
    animation: lineMove 8s infinite ease-in-out;
  }

  .blur-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    backdrop-filter: blur(100px);
    -webkit-backdrop-filter: blur(100px);
    z-index: 1;
  }

  .text-shadow-glow {
    text-shadow: 0 0 30px var(--color-chart-3);
  }

  @keyframes float {
    0%,
    100% {
      transform: translate(0, 0) scale(1);
    }
    25% {
      transform: translate(100px, 100px) scale(1.1);
    }
    50% {
      transform: translate(200px, -50px) scale(0.9);
    }
    75% {
      transform: translate(-50px, 150px) scale(1.05);
    }
  }

  @keyframes particleFloat {
    0%,
    100% {
      transform: translate(0, 0);
      opacity: 0;
    }
    10% {
      opacity: 1;
    }
    90% {
      opacity: 1;
    }
    100% {
      transform: translate(var(--tx), var(--ty));
      opacity: 0;
    }
  }

  @keyframes lineMove {
    0% {
      transform: translateX(-100%) translateY(0) rotate(var(--angle));
      opacity: 0;
    }
    10% {
      opacity: 0.6;
    }
    90% {
      opacity: 0.6;
    }
    100% {
      transform: translateX(200%) translateY(100px) rotate(var(--angle));
      opacity: 0;
    }
  }
</style>
