<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { vault } from "$lib/stores/vault";
  import * as Card from "$lib/components/ui/card";
  import { Skeleton } from "$lib/components/ui/skeleton";
  import { toast } from "svelte-sonner";

  interface DashboardSummary {
    budget_income: number;
    budget_expenses: number;
    budget_subscriptions: number;
    budget_remaining: number;
    expense_count_this_month: number;
    subscriptions_count: number;
    savings_plans_count: number;
    savings_plans_monthly: number;
    portfolio_positions: number;
    portfolio_invested: number;
  }

  let summary: DashboardSummary | null = $state(null);
  let loading = $state(false);
  let unlocked = $state(false);

  onMount(() => {
    return vault.subscribe((s) => {
      if (s.status === "unlocked" && !unlocked) {
        unlocked = true;
        loadSummary();
      } else if (s.status !== "unlocked") {
        unlocked = false;
        summary = null;
      }
    });
  });

  async function loadSummary() {
    loading = true;
    try {
      summary = await invoke<DashboardSummary>("get_dashboard_summary");
    } catch {
      toast.error("Fehler beim Laden des Dashboards");
    } finally {
      loading = false;
    }
  }

  function fmt(value: number, decimals = 2): string {
    return value.toLocaleString("de-DE", {
      minimumFractionDigits: decimals,
      maximumFractionDigits: decimals,
    });
  }

  const now = new Date();
  const monthName = now.toLocaleDateString("de-DE", { month: "long", year: "numeric" });

  let budgetUsedPct = $derived(
    summary && summary.budget_income > 0
      ? Math.min(100, ((summary.budget_expenses + summary.budget_subscriptions) / summary.budget_income) * 100)
      : 0
  );
</script>

<div class="h-screen w-full p-6 overflow-auto">
  {#if !unlocked}
    <div class="flex h-full items-center justify-center">
      <span class="text-muted-foreground text-sm">Vault gesperrt</span>
    </div>
  {:else if loading}
    <div class="grid gap-4 grid-cols-2 lg:grid-cols-4">
      {#each { length: 8 } as _}
        <Skeleton class="h-28 rounded-xl" />
      {/each}
    </div>
  {:else if summary}
    <div class="space-y-6">
      <div>
        <h1 class="text-2xl font-semibold tracking-tight">Dashboard</h1>
        <p class="text-muted-foreground text-sm">{monthName}</p>
      </div>

      <div class="grid gap-4 grid-cols-2 lg:grid-cols-4">
        <Card.Root>
          <Card.Header class="pb-2">
            <Card.Description>Einkommen</Card.Description>
            <Card.Title class="text-2xl">{fmt(summary.budget_income)} €</Card.Title>
          </Card.Header>
          <Card.Content>
            <p class="text-muted-foreground text-xs">Budget diesen Monat</p>
          </Card.Content>
        </Card.Root>

        <Card.Root class={summary.budget_remaining < 0 ? "border-destructive" : ""}>
          <Card.Header class="pb-2">
            <Card.Description>Verbleibend</Card.Description>
            <Card.Title class="text-2xl {summary.budget_remaining < 0 ? 'text-destructive' : 'text-green-600'}">
              {fmt(summary.budget_remaining)} €
            </Card.Title>
          </Card.Header>
          <Card.Content>
            <div class="h-1.5 rounded-full bg-muted overflow-hidden">
              <div
                class="h-full rounded-full transition-all {budgetUsedPct >= 100 ? 'bg-destructive' : budgetUsedPct >= 80 ? 'bg-yellow-500' : 'bg-green-500'}"
                style="width: {budgetUsedPct}%"
              ></div>
            </div>
            <p class="text-muted-foreground text-xs mt-1">{fmt(budgetUsedPct, 1)}% des Budgets genutzt</p>
          </Card.Content>
        </Card.Root>

        <Card.Root>
          <Card.Header class="pb-2">
            <Card.Description>Ausgaben</Card.Description>
            <Card.Title class="text-2xl">{fmt(summary.budget_expenses)} €</Card.Title>
          </Card.Header>
          <Card.Content>
            <p class="text-muted-foreground text-xs">{summary.expense_count_this_month} Buchungen diesen Monat</p>
          </Card.Content>
        </Card.Root>

        <Card.Root>
          <Card.Header class="pb-2">
            <Card.Description>Abonnements</Card.Description>
            <Card.Title class="text-2xl">{fmt(summary.budget_subscriptions)} €</Card.Title>
          </Card.Header>
          <Card.Content>
            <p class="text-muted-foreground text-xs">{summary.subscriptions_count} aktive Abos / Monat</p>
          </Card.Content>
        </Card.Root>
      </div>

      <div class="grid gap-4 grid-cols-1 lg:grid-cols-3">
        <Card.Root>
          <Card.Header class="pb-2">
            <Card.Description>Sparpläne</Card.Description>
            <Card.Title class="text-2xl">{fmt(summary.savings_plans_monthly)} €</Card.Title>
          </Card.Header>
          <Card.Content>
            <p class="text-muted-foreground text-xs">{summary.savings_plans_count} Pläne · monatliches Äquivalent</p>
          </Card.Content>
        </Card.Root>

        <Card.Root>
          <Card.Header class="pb-2">
            <Card.Description>Portfolio — Positionen</Card.Description>
            <Card.Title class="text-2xl">{summary.portfolio_positions}</Card.Title>
          </Card.Header>
          <Card.Content>
            <p class="text-muted-foreground text-xs">Aktive Depotpositionen</p>
          </Card.Content>
        </Card.Root>

        <Card.Root>
          <Card.Header class="pb-2">
            <Card.Description>Portfolio — Investiert</Card.Description>
            <Card.Title class="text-2xl">{fmt(summary.portfolio_invested)} €</Card.Title>
          </Card.Header>
          <Card.Content>
            <p class="text-muted-foreground text-xs">Einstandswert gesamt</p>
          </Card.Content>
        </Card.Root>
      </div>

      <div class="grid gap-4 grid-cols-1 lg:grid-cols-2">
        <Card.Root>
          <Card.Header>
            <Card.Title class="text-base">Monatsübersicht</Card.Title>
          </Card.Header>
          <Card.Content class="space-y-3">
            {#each [
              { label: "Einkommen", value: summary.budget_income, color: "bg-green-500" },
              { label: "Ausgaben", value: summary.budget_expenses, color: "bg-blue-500" },
              { label: "Abonnements", value: summary.budget_subscriptions, color: "bg-orange-500" },
              { label: "Sparpläne", value: summary.savings_plans_monthly, color: "bg-purple-500" },
            ] as row}
              {@const pct = summary.budget_income > 0 ? Math.min(100, (row.value / summary.budget_income) * 100) : 0}
              <div>
                <div class="flex justify-between text-sm mb-1">
                  <span class="text-muted-foreground">{row.label}</span>
                  <span class="font-medium">{fmt(row.value)} €</span>
                </div>
                <div class="h-1.5 rounded-full bg-muted overflow-hidden">
                  <div class="h-full rounded-full {row.color}" style="width: {pct}%"></div>
                </div>
              </div>
            {/each}
          </Card.Content>
        </Card.Root>

        <Card.Root>
          <Card.Header>
            <Card.Title class="text-base">Monatliche Belastung</Card.Title>
          </Card.Header>
          <Card.Content>
            <div class="space-y-2 text-sm">
              <div class="flex justify-between py-1 border-b">
                <span class="text-muted-foreground">Feste Ausgaben (Abos)</span>
                <span>{fmt(summary.budget_subscriptions)} €</span>
              </div>
              <div class="flex justify-between py-1 border-b">
                <span class="text-muted-foreground">Variable Ausgaben</span>
                <span>{fmt(summary.budget_expenses)} €</span>
              </div>
              <div class="flex justify-between py-1 border-b">
                <span class="text-muted-foreground">Investitionen (Sparpläne)</span>
                <span>{fmt(summary.savings_plans_monthly)} €</span>
              </div>
              <div class="flex justify-between py-1 font-semibold">
                <span>Gesamt</span>
                <span>{fmt(summary.budget_expenses + summary.budget_subscriptions + summary.savings_plans_monthly)} €</span>
              </div>
            </div>
          </Card.Content>
        </Card.Root>
      </div>
    </div>
  {/if}
</div>
