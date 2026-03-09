<script lang="ts">
  import { onMount } from "svelte";
  import { BudgetService, type BudgetSummary } from "$lib/services/budget";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { toast } from "svelte-sonner";

  const now = new Date();
  let selectedMonth = $state(
    `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, "0")}`
  );
  let income = $state(0);
  let summary: BudgetSummary | null = $state(null);
  let loading = $state(true);
  let saving = $state(false);

  onMount(async () => {
    await loadSummary();
  });

  async function loadSummary() {
    loading = true;
    try {
      summary = await BudgetService.getSummary(selectedMonth);
      income = summary.income;
    } catch {
      summary = null;
    } finally {
      loading = false;
    }
  }

  async function saveIncome() {
    saving = true;
    try {
      await BudgetService.upsert(selectedMonth, income);
      await loadSummary();
      toast.success("Einkommen gespeichert");
    } catch {
      toast.error("Fehler beim Speichern");
    } finally {
      saving = false;
    }
  }

  function formatCurrency(value: number) {
    return new Intl.NumberFormat("de-DE", {
      style: "currency",
      currency: "EUR",
    }).format(value);
  }

  function monthLabel(ym: string) {
    const [y, m] = ym.split("-");
    return new Date(Number(y), Number(m) - 1).toLocaleDateString("de-DE", {
      month: "long",
      year: "numeric",
    });
  }

  function prevMonth() {
    const [y, m] = selectedMonth.split("-").map(Number);
    const d = new Date(y, m - 2);
    selectedMonth = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}`;
    loadSummary();
  }

  function nextMonth() {
    const [y, m] = selectedMonth.split("-").map(Number);
    const d = new Date(y, m);
    selectedMonth = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}`;
    loadSummary();
  }
</script>

<div class="p-6 max-w-2xl mx-auto space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">Budget</h1>
    <div class="flex items-center gap-2">
      <Button variant="outline" size="sm" onclick={prevMonth}>&#8249;</Button>
      <span class="text-sm font-medium w-36 text-center">{monthLabel(selectedMonth)}</span>
      <Button variant="outline" size="sm" onclick={nextMonth}>&#8250;</Button>
    </div>
  </div>

  <div class="rounded-lg border border-input p-4 space-y-3">
    <h2 class="text-sm font-medium text-muted-foreground">Monatliches Einkommen</h2>
    <div class="flex gap-2">
      <Input
        type="number"
        step="0.01"
        bind:value={income}
        placeholder="0.00"
        class="max-w-xs"
      />
      <Button onclick={saveIncome} disabled={saving}>
        {saving ? "Speichern..." : "Speichern"}
      </Button>
    </div>
  </div>

  {#if loading}
    <div class="flex items-center justify-center h-32">
      <span class="text-muted-foreground text-sm">Laden...</span>
    </div>
  {:else if summary}
    <div class="grid grid-cols-2 gap-4">
      <div class="rounded-lg border border-input p-4 space-y-1">
        <p class="text-xs text-muted-foreground">Einkommen</p>
        <p class="text-xl font-semibold text-green-600 dark:text-green-400">
          {formatCurrency(summary.income)}
        </p>
      </div>

      <div class="rounded-lg border border-input p-4 space-y-1">
        <p class="text-xs text-muted-foreground">Ausgaben (diesen Monat)</p>
        <p class="text-xl font-semibold text-red-500">
          {formatCurrency(summary.total_expenses)}
        </p>
      </div>

      <div class="rounded-lg border border-input p-4 space-y-1">
        <p class="text-xs text-muted-foreground">Abonnements (monatlich)</p>
        <p class="text-xl font-semibold text-orange-500">
          {formatCurrency(summary.total_subscriptions_monthly)}
        </p>
        <p class="text-xs text-muted-foreground">Jahresabos auf Monat umgerechnet</p>
      </div>

      <div
        class="rounded-lg border p-4 space-y-1
          {summary.remaining >= 0
          ? 'border-green-500/30 bg-green-500/5'
          : 'border-red-500/30 bg-red-500/5'}"
      >
        <p class="text-xs text-muted-foreground">Verbleibend</p>
        <p
          class="text-xl font-semibold
            {summary.remaining >= 0
            ? 'text-green-600 dark:text-green-400'
            : 'text-red-500'}"
        >
          {formatCurrency(summary.remaining)}
        </p>
      </div>
    </div>

    <div class="rounded-lg border border-input p-4 space-y-3">
      <h2 class="text-sm font-medium">Monatsübersicht</h2>
      <div class="space-y-2">
        {#each [
          { label: "Einkommen", value: summary.income, color: "bg-green-500" },
          { label: "Ausgaben", value: summary.total_expenses, color: "bg-red-500" },
          { label: "Abonnements", value: summary.total_subscriptions_monthly, color: "bg-orange-500" },
        ] as item}
          {@const pct = summary.income > 0 ? Math.min((item.value / summary.income) * 100, 100) : 0}
          <div class="space-y-1">
            <div class="flex justify-between text-xs">
              <span class="text-muted-foreground">{item.label}</span>
              <span>{formatCurrency(item.value)}</span>
            </div>
            <div class="h-2 w-full rounded-full bg-muted overflow-hidden">
              <div
                class="h-full rounded-full transition-all {item.color}"
                style="width: {pct}%"
              ></div>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>
