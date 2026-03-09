<script lang="ts">
  import { onMount } from "svelte";
  import { BudgetService, type BudgetSummary } from "$lib/services/budget";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { toast } from "svelte-sonner";
  import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
  import TrendingDownIcon from "@lucide/svelte/icons/trending-down";
  import WalletIcon from "@lucide/svelte/icons/wallet";
  import ReceiptIcon from "@lucide/svelte/icons/receipt";
  import RepeatIcon from "@lucide/svelte/icons/repeat";
  import PiggyBankIcon from "@lucide/svelte/icons/piggy-bank";
  import ChevronLeftIcon from "@lucide/svelte/icons/chevron-left";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import CheckIcon from "@lucide/svelte/icons/check";

  const now = new Date();
  let selectedMonth = $state(
    `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, "0")}`
  );
  let income = $state(0);
  let summary: BudgetSummary | null = $state(null);
  let loading = $state(true);
  let saving = $state(false);
  let editingIncome = $state(false);

  onMount(loadSummary);

  async function loadSummary() {
    loading = true;
    try {
      summary = await BudgetService.getSummary(selectedMonth);
      income = summary.income;
    } catch {
      summary = { month: selectedMonth, income: 0, total_expenses: 0, total_subscriptions_monthly: 0, remaining: 0 };
      income = 0;
    } finally {
      loading = false;
    }
  }

  async function saveIncome() {
    saving = true;
    try {
      await BudgetService.upsert(selectedMonth, income);
      await loadSummary();
      editingIncome = false;
      toast.success("Einkommen gespeichert");
    } catch {
      toast.error("Fehler beim Speichern");
    } finally {
      saving = false;
    }
  }

  function fmt(value: number) {
    return new Intl.NumberFormat("de-DE", { style: "currency", currency: "EUR" }).format(value);
  }

  function monthLabel(ym: string) {
    const [y, m] = ym.split("-");
    return new Date(Number(y), Number(m) - 1).toLocaleDateString("de-DE", { month: "long", year: "numeric" });
  }

  function shiftMonth(delta: number) {
    const [y, m] = selectedMonth.split("-").map(Number);
    const d = new Date(y, m - 1 + delta);
    selectedMonth = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}`;
    loadSummary();
  }

  let spentPct = $derived((() => {
    const s = summary as BudgetSummary | null;
    if (!s || s.income <= 0) return 0;
    return Math.min(((s.total_expenses + s.total_subscriptions_monthly) / s.income) * 100, 100);
  })());

  let statusColor = $derived(
    spentPct >= 100 ? "text-destructive" :
    spentPct >= 80  ? "text-orange-500" :
    "text-green-500"
  );
</script>

<div class="p-6 space-y-6 max-w-3xl">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-semibold tracking-tight">Budget</h1>
      <p class="text-sm text-muted-foreground">Einkommen vs. Ausgaben im Überblick</p>
    </div>
    <div class="flex items-center gap-1">
      <Button variant="outline" size="icon" onclick={() => shiftMonth(-1)}>
        <ChevronLeftIcon class="size-4" />
      </Button>
      <span class="w-36 text-center text-sm font-medium">{monthLabel(selectedMonth)}</span>
      <Button variant="outline" size="icon" onclick={() => shiftMonth(1)}>
        <ChevronRightIcon class="size-4" />
      </Button>
    </div>
  </div>

  <!-- Income Card -->
  <Card.Root>
    <Card.Header class="flex flex-row items-center justify-between pb-2">
      <div class="flex items-center gap-2">
        <WalletIcon class="size-4 text-muted-foreground" />
        <Card.Title class="text-sm font-medium">Monatliches Einkommen</Card.Title>
      </div>
      {#if !editingIncome}
        <Button variant="ghost" size="icon" onclick={() => (editingIncome = true)}>
          <PencilIcon class="size-3.5" />
        </Button>
      {/if}
    </Card.Header>
    <Card.Content>
      {#if editingIncome}
        <div class="flex gap-2">
          <Input
            type="number"
            step="0.01"
            bind:value={income}
            class="max-w-xs"
            autofocus
            onkeydown={(e) => e.key === "Enter" && saveIncome()}
          />
          <Button size="sm" onclick={saveIncome} disabled={saving}>
            <CheckIcon class="size-4 mr-1" />
            {saving ? "..." : "Speichern"}
          </Button>
          <Button size="sm" variant="ghost" onclick={() => { editingIncome = false; income = summary?.income ?? 0; }}>
            Abbrechen
          </Button>
        </div>
      {:else}
        <p class="text-3xl font-bold">{fmt(summary?.income ?? 0)}</p>
        <p class="text-xs text-muted-foreground mt-1">Netto pro Monat</p>
      {/if}
    </Card.Content>
  </Card.Root>

  {#if loading}
    <div class="flex items-center justify-center h-40">
      <span class="text-muted-foreground text-sm">Laden...</span>
    </div>
  {:else if summary}
    <!-- Stats Grid -->
    <div class="grid grid-cols-2 gap-4 sm:grid-cols-4">
      <Card.Root>
        <Card.Header class="pb-1 pt-4 px-4">
          <div class="flex items-center gap-1.5">
            <ReceiptIcon class="size-3.5 text-muted-foreground" />
            <Card.Title class="text-xs font-medium text-muted-foreground">Ausgaben</Card.Title>
          </div>
        </Card.Header>
        <Card.Content class="px-4 pb-4">
          <p class="text-lg font-semibold">{fmt(summary.total_expenses)}</p>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Header class="pb-1 pt-4 px-4">
          <div class="flex items-center gap-1.5">
            <RepeatIcon class="size-3.5 text-muted-foreground" />
            <Card.Title class="text-xs font-medium text-muted-foreground">Abonnements</Card.Title>
          </div>
        </Card.Header>
        <Card.Content class="px-4 pb-4">
          <p class="text-lg font-semibold">{fmt(summary.total_subscriptions_monthly)}</p>
          <p class="text-xs text-muted-foreground">monatl. Anteil</p>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Header class="pb-1 pt-4 px-4">
          <div class="flex items-center gap-1.5">
            <PiggyBankIcon class="size-3.5 text-muted-foreground" />
            <Card.Title class="text-xs font-medium text-muted-foreground">Verbleibend</Card.Title>
          </div>
        </Card.Header>
        <Card.Content class="px-4 pb-4">
          <p class="text-lg font-semibold {statusColor}">{fmt(summary.remaining)}</p>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Header class="pb-1 pt-4 px-4">
          <div class="flex items-center gap-1.5">
            {#if summary.remaining >= 0}
              <TrendingUpIcon class="size-3.5 text-green-500" />
            {:else}
              <TrendingDownIcon class="size-3.5 text-destructive" />
            {/if}
            <Card.Title class="text-xs font-medium text-muted-foreground">Status</Card.Title>
          </div>
        </Card.Header>
        <Card.Content class="px-4 pb-4">
          <Badge variant={summary.remaining >= 0 ? "outline" : "destructive"} class="text-xs">
            {summary.remaining >= 0 ? "Im Budget" : "Überbudget"}
          </Badge>
          <p class="text-xs text-muted-foreground mt-1">{spentPct.toFixed(0)}% verbraucht</p>
        </Card.Content>
      </Card.Root>
    </div>

    <!-- Progress breakdown -->
    <Card.Root>
      <Card.Header>
        <Card.Title class="text-sm font-medium">Monatsübersicht</Card.Title>
        <Card.Description>Anteil am Einkommen von {fmt(summary.income)}</Card.Description>
      </Card.Header>
      <Card.Content class="space-y-4">
        {#each [
          { label: "Ausgaben", value: summary.total_expenses, color: "bg-primary" },
          { label: "Abonnements", value: summary.total_subscriptions_monthly, color: "bg-orange-500" },
        ] as item}
          {@const pct = summary.income > 0 ? Math.min((item.value / summary.income) * 100, 100) : 0}
          <div class="space-y-1.5">
            <div class="flex justify-between text-sm">
              <span class="text-muted-foreground">{item.label}</span>
              <span class="font-medium">{fmt(item.value)} <span class="text-muted-foreground font-normal">({pct.toFixed(1)}%)</span></span>
            </div>
            <div class="h-2 w-full rounded-full bg-muted overflow-hidden">
              <div class="h-full rounded-full transition-all duration-500 {item.color}" style="width: {pct}%"></div>
            </div>
          </div>
        {/each}

        <Separator />

        <div class="flex justify-between text-sm font-medium">
          <span>Gesamt verbraucht</span>
          <span class="{statusColor}">{fmt(summary.total_expenses + summary.total_subscriptions_monthly)} ({spentPct.toFixed(1)}%)</span>
        </div>
        <div class="h-2.5 w-full rounded-full bg-muted overflow-hidden">
          <div
            class="h-full rounded-full transition-all duration-500 {spentPct >= 100 ? 'bg-destructive' : spentPct >= 80 ? 'bg-orange-500' : 'bg-green-500'}"
            style="width: {spentPct}%"
          ></div>
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
