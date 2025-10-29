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
  let particlesContainer: HTMLDivElement;

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

  const features = [
    {
      icon: "🔒",
      title: "Sicherheit",
      description: "Bank-grade Verschlüsselung",
    },
    {
      icon: "📊",
      title: "Analytics",
      description: "Detaillierte Finanzanalysen",
    },
    {
      icon: "⚡",
      title: "Schnell",
      description: "Blitzschnelle Performance",
    },
  ];
</script>

<div class={cn("min-h-screen flex", className)} {...restProps}>
  <!-- Left Side - Login Form -->
  <div class="flex-1 flex items-center justify-center p-8 bg-background">
    <div class="w-full max-w-md space-y-8">
      <div class="text-center">
        <h1 class="text-3xl font-bold tracking-tight">Willkommen zurück</h1>
        <p class="text-muted-foreground mt-2">
          Melden Sie sich in Ihrem FinanceVault Konto an
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
              >Benutzername</FieldLabel
            >
            <Input
              id="username-{id}"
              type="text"
              placeholder="Geben Sie Ihren Benutzernamen ein"
              bind:value={username}
              required
              disabled={isLoading}
              class="mt-1"
            />
          </Field>

          <Field>
            <div class="flex items-center justify-between">
              <FieldLabel for="password-{id}" class="text-sm font-medium"
                >Passwort</FieldLabel
              >
              <a
                href="##"
                class="text-sm text-primary hover:text-primary/80 underline-offset-2 hover:underline"
              >
                Passwort vergessen?
              </a>
            </div>
            <Input
              id="password-{id}"
              type="password"
              placeholder="Geben Sie Ihr Passwort ein"
              bind:value={password}
              required
              disabled={isLoading}
              class="mt-1"
            />
          </Field>

          <Button type="submit" class="w-full h-11" disabled={isLoading}>
            {#if isLoading}
              <div
                class="animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2"
              ></div>
            {/if}
            {isLoading ? "Anmeldung läuft..." : "Anmelden"}
          </Button>
        </FieldGroup>

        <div class="text-center text-sm text-muted-foreground">
          Haben Sie noch kein Konto?
          <a
            href="/signup"
            class="text-primary hover:text-primary/80 underline underline-offset-2"
          >
            Registrieren
          </a>
        </div>
      </form>
    </div>
  </div>

  <!-- Right Side - Animated Background with Features -->
  <div class="flex-1 relative hidden lg:flex flex-col overflow-hidden">
    <!-- Animated Background Container -->
    <div
      class="absolute inset-0 rounded-l-3xl overflow-hidden bg-gradient-to-br from-[#000814] via-[#001d3d] to-[#003566]"
    >
      <!-- Gradient Orbs -->
      <div class="gradient-orb orb1"></div>
      <div class="gradient-orb orb2"></div>
      <div class="gradient-orb orb3"></div>

      <!-- Particles Container -->
      <div class="particles-container" bind:this={particlesContainer}></div>

      <!-- Blur Overlay -->
      <div class="blur-overlay"></div>
    </div>

    <!-- Feature Cards -->
    <div class="absolute top-8 left-8 right-8 z-20">
      <div class="grid grid-cols-3 gap-4">
        {#each features as feature}
          <Card.Root
            class="backdrop-blur-md bg-white/10 border-white/20 text-white"
          >
            <Card.Content class="p-4">
              <div class="text-2xl mb-2">{feature.icon}</div>
              <h3 class="font-semibold text-sm mb-1">{feature.title}</h3>
              <p class="text-xs text-white/80">{feature.description}</p>
            </Card.Content>
          </Card.Root>
        {/each}
      </div>
    </div>

    <!-- Logo Overlay -->
    <div class="absolute inset-0 flex items-center justify-center z-10">
      <div class="text-center text-white">
        <h2 class="text-4xl font-bold mb-4 text-shadow-glow">FinanceVault</h2>
        <p class="text-xl opacity-90">Ihre sichere Finanzlösung</p>
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

  .orb1 {
    width: 600px;
    height: 600px;
    background: radial-gradient(
      circle,
      rgba(0, 119, 182, 0.8) 0%,
      rgba(0, 53, 102, 0.3) 70%
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
      rgba(0, 180, 216, 0.6) 0%,
      rgba(0, 119, 182, 0.2) 70%
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
      rgba(72, 202, 228, 0.5) 0%,
      rgba(0, 119, 182, 0.2) 70%
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
    background: rgba(72, 202, 228, 0.8);
    border-radius: 50%;
    animation: particleFloat 15s infinite ease-in-out;
  }

  .animated-line {
    position: absolute;
    height: 2px;
    background: linear-gradient(
      90deg,
      transparent,
      rgba(72, 202, 228, 0.6),
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
    text-shadow: 0 0 30px rgba(72, 202, 228, 0.5);
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
