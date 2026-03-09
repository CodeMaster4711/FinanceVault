<script lang="ts">
  import { onMount } from "svelte";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { toast } from "svelte-sonner";
  import { YahooService, type Quote } from "$lib/services/yahoo";
  import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
  import TrendingDownIcon from "@lucide/svelte/icons/trending-down";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import PencilIcon from "@lucide/svelte/icons/pencil";

  interface Position {
    id: string;
    ticker: string;
    name: string;
    assetType: "stock" | "etf" | "crypto" | "other";
    quantity: number;
    avgBuyPrice: number;
    currency: string;
    country: string;
  }

  const STORAGE_KEY = "fv_portfolio";

  let positions: Position[] = $state([]);
  let quotes: Record<string, Quote> = $state({});
  let loadingQuotes = $state(false);
  let showAddDialog = $state(false);
  let showEditDialog = $state(false);
  let editingPosition: Position | null = $state(null);

  let formData = $state({
    ticker: "",
    name: "",
    assetType: "stock" as Position["assetType"],
    quantity: 0,
    avgBuyPrice: 0,
    currency: "EUR",
    country: "",
  });

  const assetTypes = [
    { value: "stock", label: "Aktie" },
    { value: "etf", label: "ETF" },
    { value: "crypto", label: "Krypto" },
    { value: "other", label: "Sonstiges" },
  ];

  onMount(() => {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved) {
      try { positions = JSON.parse(saved); } catch { positions = []; }
    }
    if (positions.length > 0) refreshQuotes();
  });

  function saveToDisk() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(positions));
  }

  async function refreshQuotes() {
    if (positions.length === 0) return;
    loadingQuotes = true;
    try {
      const tickers = positions.map((p) => p.ticker);
      const data = await YahooService.fetchQuotes(tickers);
      const map: Record<string, Quote> = {};
      data.forEach((q) => { map[q.ticker] = q; });
      quotes = map;
    } catch {
      toast.error("Kurse konnten nicht geladen werden");
    } finally {
      loadingQuotes = false;
    }
  }

  function openAdd() {
    formData = { ticker: "", name: "", assetType: "stock", quantity: 0, avgBuyPrice: 0, currency: "EUR", country: "" };
    showAddDialog = true;
  }

  function addPosition() {
    if (!formData.ticker || !formData.quantity || !formData.avgBuyPrice) {
      toast.error("Ticker, Anzahl und Kaufpreis sind Pflichtfelder");
      return;
    }
    const pos: Position = { id: crypto.randomUUID(), ...formData, ticker: formData.ticker.toUpperCase() };
    positions = [...positions, pos];
    saveToDisk();
    showAddDialog = false;
    refreshQuotes();
    toast.success("Position hinzugefügt");
  }

  function openEdit(pos: Position) {
    editingPosition = pos;
    formData = { ticker: pos.ticker, name: pos.name, assetType: pos.assetType, quantity: pos.quantity, avgBuyPrice: pos.avgBuyPrice, currency: pos.currency, country: pos.country };
    showEditDialog = true;
  }

  function saveEdit() {
    if (!editingPosition) return;
    positions = positions.map((p) =>
      p.id === editingPosition!.id ? { ...p, ...formData, ticker: formData.ticker.toUpperCase() } : p
    );
    saveToDisk();
    showEditDialog = false;
    refreshQuotes();
    toast.success("Position aktualisiert");
  }

  function deletePosition(id: string) {
    if (!confirm("Position wirklich löschen?")) return;
    positions = positions.filter((p) => p.id !== id);
    saveToDisk();
    toast.success("Position gelöscht");
  }

  function fmt(value: number, currency = "EUR") {
    return new Intl.NumberFormat("de-DE", { style: "currency", currency }).format(value);
  }

  function fmtPct(value: number) {
    return `${value >= 0 ? "+" : ""}${value.toFixed(2)}%`;
  }

  let totalInvested = $derived(positions.reduce((s, p) => s + p.quantity * p.avgBuyPrice, 0));

  let totalCurrent = $derived(
    positions.reduce((s, p) => {
      const q = quotes[p.ticker];
      return s + p.quantity * (q ? q.price : p.avgBuyPrice);
    }, 0)
  );

  let totalGainPct = $derived(totalInvested > 0 ? ((totalCurrent - totalInvested) / totalInvested) * 100 : 0);

  let countryAllocation = $derived(() => {
    const map: Record<string, number> = {};
    positions.forEach((p) => {
      const q = quotes[p.ticker];
      const val = p.quantity * (q ? q.price : p.avgBuyPrice);
      const c = p.country || "Unbekannt";
      map[c] = (map[c] || 0) + val;
    });
    return Object.entries(map)
      .map(([country, value]) => ({ country, value, pct: totalCurrent > 0 ? (value / totalCurrent) * 100 : 0 }))
      .sort((a, b) => b.value - a.value);
  });

  let typeAllocation = $derived(() => {
    const map: Record<string, number> = {};
    positions.forEach((p) => {
      const q = quotes[p.ticker];
      const val = p.quantity * (q ? q.price : p.avgBuyPrice);
      map[p.assetType] = (map[p.assetType] || 0) + val;
    });
    return Object.entries(map)
      .map(([type, value]) => ({ type, value, pct: totalCurrent > 0 ? (value / totalCurrent) * 100 : 0 }))
      .sort((a, b) => b.value - a.value);
  });
</script>

<div class="p-6 space-y-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-semibold tracking-tight">Portfolio</h1>
      <p class="text-sm text-muted-foreground">Kurse via Yahoo Finance (read-only, keine Daten werden hochgeladen)</p>
    </div>
    <div class="flex gap-2">
      <Button variant="outline" size="sm" onclick={refreshQuotes} disabled={loadingQuotes}>
        <RefreshCwIcon class="size-4 mr-1.5 {loadingQuotes ? 'animate-spin' : ''}" />
        Kurse aktualisieren
      </Button>
      <Button size="sm" onclick={openAdd}>
        <PlusIcon class="size-4 mr-1.5" />
        Position
      </Button>
    </div>
  </div>

  {#if positions.length === 0}
    <Card.Root>
      <Card.Content class="flex flex-col items-center justify-center h-48 gap-3">
        <p class="text-muted-foreground text-sm">Noch keine Positionen vorhanden.</p>
        <Button size="sm" onclick={openAdd}>
          <PlusIcon class="size-4 mr-1.5" />
          Erste Position hinzufügen
        </Button>
      </Card.Content>
    </Card.Root>
  {:else}
    <!-- Summary -->
    <div class="grid grid-cols-2 gap-4 sm:grid-cols-4">
      <Card.Root>
        <Card.Header class="pb-1 pt-4 px-4">
          <Card.Title class="text-xs font-medium text-muted-foreground">Investiert</Card.Title>
        </Card.Header>
        <Card.Content class="px-4 pb-4">
          <p class="text-lg font-semibold">{fmt(totalInvested)}</p>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Header class="pb-1 pt-4 px-4">
          <Card.Title class="text-xs font-medium text-muted-foreground">Aktueller Wert</Card.Title>
        </Card.Header>
        <Card.Content class="px-4 pb-4">
          <p class="text-lg font-semibold">{fmt(totalCurrent)}</p>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Header class="pb-1 pt-4 px-4">
          <Card.Title class="text-xs font-medium text-muted-foreground">Gewinn/Verlust</Card.Title>
        </Card.Header>
        <Card.Content class="px-4 pb-4">
          <p class="text-lg font-semibold {totalCurrent >= totalInvested ? 'text-green-500' : 'text-destructive'}">
            {fmt(totalCurrent - totalInvested)}
          </p>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Header class="pb-1 pt-4 px-4">
          <div class="flex items-center gap-1">
            {#if totalGainPct >= 0}
              <TrendingUpIcon class="size-3.5 text-green-500" />
            {:else}
              <TrendingDownIcon class="size-3.5 text-destructive" />
            {/if}
            <Card.Title class="text-xs font-medium text-muted-foreground">Performance</Card.Title>
          </div>
        </Card.Header>
        <Card.Content class="px-4 pb-4">
          <p class="text-lg font-semibold {totalGainPct >= 0 ? 'text-green-500' : 'text-destructive'}">
            {fmtPct(totalGainPct)}
          </p>
        </Card.Content>
      </Card.Root>
    </div>

    <!-- Positions table -->
    <Card.Root>
      <Card.Header>
        <Card.Title class="text-sm font-medium">Positionen</Card.Title>
      </Card.Header>
      <Card.Content class="p-0">
        <div class="rounded-b-lg overflow-hidden">
          <table class="w-full text-sm">
            <thead class="border-b bg-muted/40">
              <tr>
                {#each ["Ticker", "Name", "Typ", "Anzahl", "Ø Kaufpreis", "Aktuell", "Wert", "G/V", "Tagesänderung", ""] as h}
                  <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground">{h}</th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each positions as pos}
                {@const q = quotes[pos.ticker]}
                {@const currentPrice = q ? q.price : pos.avgBuyPrice}
                {@const invested = pos.quantity * pos.avgBuyPrice}
                {@const current = pos.quantity * currentPrice}
                {@const gain = current - invested}
                {@const gainPct = invested > 0 ? (gain / invested) * 100 : 0}
                <tr class="border-b last:border-0 hover:bg-muted/30 transition-colors">
                  <td class="px-4 py-3 font-mono font-medium">{pos.ticker}</td>
                  <td class="px-4 py-3 text-muted-foreground max-w-[160px] truncate">{q?.name || pos.name}</td>
                  <td class="px-4 py-3">
                    <Badge variant="secondary" class="text-xs">{assetTypes.find(t => t.value === pos.assetType)?.label}</Badge>
                  </td>
                  <td class="px-4 py-3">{pos.quantity}</td>
                  <td class="px-4 py-3">{fmt(pos.avgBuyPrice, pos.currency)}</td>
                  <td class="px-4 py-3">{q ? fmt(q.price, q.currency) : "—"}</td>
                  <td class="px-4 py-3 font-medium">{fmt(current, pos.currency)}</td>
                  <td class="px-4 py-3 {gain >= 0 ? 'text-green-500' : 'text-destructive'}">
                    {fmt(gain, pos.currency)} ({fmtPct(gainPct)})
                  </td>
                  <td class="px-4 py-3 {(q?.change_pct ?? 0) >= 0 ? 'text-green-500' : 'text-destructive'}">
                    {q ? fmtPct(q.change_pct) : "—"}
                  </td>
                  <td class="px-4 py-3">
                    <div class="flex gap-1">
                      <Button variant="ghost" size="icon" class="size-7" onclick={() => openEdit(pos)}>
                        <PencilIcon class="size-3.5" />
                      </Button>
                      <Button variant="ghost" size="icon" class="size-7 text-destructive hover:text-destructive" onclick={() => deletePosition(pos.id)}>
                        <TrashIcon class="size-3.5" />
                      </Button>
                    </div>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </Card.Content>
    </Card.Root>

    <!-- Allocation -->
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
      <Card.Root>
        <Card.Header>
          <Card.Title class="text-sm font-medium">Länderallokation</Card.Title>
        </Card.Header>
        <Card.Content class="space-y-3">
          {#each countryAllocation() as item}
            <div class="space-y-1">
              <div class="flex justify-between text-sm">
                <span>{item.country}</span>
                <span class="text-muted-foreground">{item.pct.toFixed(1)}%</span>
              </div>
              <div class="h-1.5 w-full rounded-full bg-muted overflow-hidden">
                <div class="h-full rounded-full bg-primary transition-all" style="width: {item.pct}%"></div>
              </div>
            </div>
          {/each}
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Header>
          <Card.Title class="text-sm font-medium">Asset-Klassen</Card.Title>
        </Card.Header>
        <Card.Content class="space-y-3">
          {#each typeAllocation() as item}
            {@const colors = { stock: "bg-blue-500", etf: "bg-green-500", crypto: "bg-orange-500", other: "bg-muted-foreground" }}
            <div class="space-y-1">
              <div class="flex justify-between text-sm">
                <span>{assetTypes.find(t => t.value === item.type)?.label ?? item.type}</span>
                <span class="text-muted-foreground">{item.pct.toFixed(1)}%</span>
              </div>
              <div class="h-1.5 w-full rounded-full bg-muted overflow-hidden">
                <div class="h-full rounded-full transition-all {colors[item.type as keyof typeof colors] ?? 'bg-primary'}" style="width: {item.pct}%"></div>
              </div>
            </div>
          {/each}
        </Card.Content>
      </Card.Root>
    </div>
  {/if}
</div>

<!-- Add Dialog -->
<Dialog.Root bind:open={showAddDialog}>
  <Dialog.Content class="max-w-md">
    <Dialog.Header>
      <Dialog.Title>Position hinzufügen</Dialog.Title>
      <Dialog.Description>Ticker-Symbol und Kaufdaten eingeben.</Dialog.Description>
    </Dialog.Header>
    <div class="grid gap-4 py-2">
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5">
          <Label>Ticker</Label>
          <Input bind:value={formData.ticker} placeholder="z.B. AAPL" class="uppercase" />
        </div>
        <div class="space-y-1.5">
          <Label>Typ</Label>
          <Select.Root type="single" bind:value={formData.assetType}>
            <Select.Trigger>{assetTypes.find(t => t.value === formData.assetType)?.label}</Select.Trigger>
            <Select.Content>
              {#each assetTypes as t}<Select.Item value={t.value}>{t.label}</Select.Item>{/each}
            </Select.Content>
          </Select.Root>
        </div>
      </div>
      <div class="space-y-1.5">
        <Label>Name (optional)</Label>
        <Input bind:value={formData.name} placeholder="z.B. Apple Inc." />
      </div>
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5">
          <Label>Anzahl</Label>
          <Input type="number" step="0.0001" bind:value={formData.quantity} placeholder="0" />
        </div>
        <div class="space-y-1.5">
          <Label>Ø Kaufpreis</Label>
          <Input type="number" step="0.01" bind:value={formData.avgBuyPrice} placeholder="0.00" />
        </div>
      </div>
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5">
          <Label>Währung</Label>
          <Input bind:value={formData.currency} placeholder="EUR" />
        </div>
        <div class="space-y-1.5">
          <Label>Land (optional)</Label>
          <Input bind:value={formData.country} placeholder="z.B. USA" />
        </div>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showAddDialog = false)}>Abbrechen</Button>
      <Button onclick={addPosition}>Hinzufügen</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Edit Dialog -->
<Dialog.Root bind:open={showEditDialog}>
  <Dialog.Content class="max-w-md">
    <Dialog.Header>
      <Dialog.Title>Position bearbeiten</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-2">
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5">
          <Label>Ticker</Label>
          <Input bind:value={formData.ticker} class="uppercase" />
        </div>
        <div class="space-y-1.5">
          <Label>Typ</Label>
          <Select.Root type="single" bind:value={formData.assetType}>
            <Select.Trigger>{assetTypes.find(t => t.value === formData.assetType)?.label}</Select.Trigger>
            <Select.Content>
              {#each assetTypes as t}<Select.Item value={t.value}>{t.label}</Select.Item>{/each}
            </Select.Content>
          </Select.Root>
        </div>
      </div>
      <div class="space-y-1.5">
        <Label>Name</Label>
        <Input bind:value={formData.name} />
      </div>
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5">
          <Label>Anzahl</Label>
          <Input type="number" step="0.0001" bind:value={formData.quantity} />
        </div>
        <div class="space-y-1.5">
          <Label>Ø Kaufpreis</Label>
          <Input type="number" step="0.01" bind:value={formData.avgBuyPrice} />
        </div>
      </div>
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5">
          <Label>Währung</Label>
          <Input bind:value={formData.currency} />
        </div>
        <div class="space-y-1.5">
          <Label>Land</Label>
          <Input bind:value={formData.country} />
        </div>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showEditDialog = false)}>Abbrechen</Button>
      <Button onclick={saveEdit}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
