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
  import SearchIcon from "@lucide/svelte/icons/search";
  import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
  import LoaderIcon from "@lucide/svelte/icons/loader";

  interface Position {
    id: string;
    isin: string;
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

  // Add form
  let isinInput = $state("");
  let resolving = $state(false);
  let resolved: { ticker: string; name: string } | null = $state(null);
  let resolveError = $state("");
  let addQuantity = $state(0);
  let addAvgPrice = $state(0);
  let addCurrency = $state("EUR");
  let addCountry = $state("");
  let addAssetType = $state<Position["assetType"]>("stock");

  // Edit form
  let editForm = $state({
    isin: "", ticker: "", name: "", assetType: "stock" as Position["assetType"],
    quantity: 0, avgBuyPrice: 0, currency: "EUR", country: "",
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

  function persist() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(positions));
  }

  async function refreshQuotes() {
    if (positions.length === 0) return;
    loadingQuotes = true;
    try {
      const pairs: [string, string][] = positions.map((p) => [p.isin, p.ticker]);
      const data = await YahooService.fetchQuotes(pairs);
      const map: Record<string, Quote> = {};
      data.forEach((q) => { map[q.isin] = q; });
      quotes = map;
    } catch {
      toast.error("Kurse konnten nicht geladen werden");
    } finally {
      loadingQuotes = false;
    }
  }

  function openAdd() {
    isinInput = "";
    resolved = null;
    resolveError = "";
    addQuantity = 0;
    addAvgPrice = 0;
    addCurrency = "EUR";
    addCountry = "";
    addAssetType = "stock";
    showAddDialog = true;
  }

  async function resolveIsin() {
    const isin = isinInput.trim().toUpperCase();
    if (isin.length < 12) {
      resolveError = "Bitte eine gültige ISIN eingeben (12 Zeichen)";
      return;
    }
    resolving = true;
    resolveError = "";
    resolved = null;
    try {
      const [ticker, name] = await YahooService.resolveIsin(isin);
      resolved = { ticker, name };
    } catch {
      resolveError = `Keine Daten für ISIN ${isin} gefunden`;
    } finally {
      resolving = false;
    }
  }

  function addPosition() {
    if (!resolved) { toast.error("Bitte zuerst ISIN auflösen"); return; }
    if (!addQuantity || !addAvgPrice) { toast.error("Anzahl und Kaufpreis sind Pflicht"); return; }

    const pos: Position = {
      id: crypto.randomUUID(),
      isin: isinInput.trim().toUpperCase(),
      ticker: resolved.ticker,
      name: resolved.name,
      assetType: addAssetType,
      quantity: addQuantity,
      avgBuyPrice: addAvgPrice,
      currency: addCurrency,
      country: addCountry,
    };
    positions = [...positions, pos];
    persist();
    showAddDialog = false;
    refreshQuotes();
    toast.success(`${pos.name || pos.ticker} hinzugefügt`);
  }

  function openEdit(pos: Position) {
    editingPosition = pos;
    editForm = { isin: pos.isin, ticker: pos.ticker, name: pos.name, assetType: pos.assetType, quantity: pos.quantity, avgBuyPrice: pos.avgBuyPrice, currency: pos.currency, country: pos.country };
    showEditDialog = true;
  }

  function saveEdit() {
    if (!editingPosition) return;
    positions = positions.map((p) => p.id === editingPosition!.id ? { ...p, ...editForm } : p);
    persist();
    showEditDialog = false;
    refreshQuotes();
    toast.success("Position aktualisiert");
  }

  function deletePosition(id: string) {
    if (!confirm("Position wirklich löschen?")) return;
    positions = positions.filter((p) => p.id !== id);
    persist();
    toast.success("Position gelöscht");
  }

  function fmt(value: number, currency = "EUR") {
    return new Intl.NumberFormat("de-DE", { style: "currency", currency }).format(value);
  }

  function fmtPct(value: number) {
    return `${value >= 0 ? "+" : ""}${value.toFixed(2)}%`;
  }

  let totalInvested = $derived(positions.reduce((s, p) => s + p.quantity * p.avgBuyPrice, 0));
  let totalCurrent = $derived(positions.reduce((s, p) => {
    const q = quotes[p.isin];
    return s + p.quantity * (q ? q.price : p.avgBuyPrice);
  }, 0));
  let totalGain = $derived(totalCurrent - totalInvested);
  let totalGainPct = $derived(totalInvested > 0 ? (totalGain / totalInvested) * 100 : 0);

  let countryAllocation = $derived(
    Object.entries(
      positions.reduce((map, p) => {
        const val = p.quantity * (quotes[p.isin]?.price ?? p.avgBuyPrice);
        const c = p.country || "Unbekannt";
        map[c] = (map[c] ?? 0) + val;
        return map;
      }, {} as Record<string, number>)
    )
      .map(([country, value]) => ({ country, value, pct: totalCurrent > 0 ? (value / totalCurrent) * 100 : 0 }))
      .sort((a, b) => b.value - a.value)
  );

  let typeAllocation = $derived(
    Object.entries(
      positions.reduce((map, p) => {
        const val = p.quantity * (quotes[p.isin]?.price ?? p.avgBuyPrice);
        map[p.assetType] = (map[p.assetType] ?? 0) + val;
        return map;
      }, {} as Record<string, number>)
    )
      .map(([type, value]) => ({ type, value, pct: totalCurrent > 0 ? (value / totalCurrent) * 100 : 0 }))
      .sort((a, b) => b.value - a.value)
  );

  const typeColors: Record<string, string> = { stock: "bg-blue-500", etf: "bg-green-500", crypto: "bg-orange-500", other: "bg-muted-foreground" };
</script>

<div class="p-6 space-y-6">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-semibold tracking-tight">Portfolio</h1>
      <p class="text-sm text-muted-foreground">Kurse via Yahoo Finance — keine Daten werden hochgeladen</p>
    </div>
    <div class="flex gap-2">
      <Button variant="outline" size="sm" onclick={refreshQuotes} disabled={loadingQuotes}>
        <RefreshCwIcon class="size-4 mr-1.5 {loadingQuotes ? 'animate-spin' : ''}" />
        Aktualisieren
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
      {#each [
        { label: "Investiert", value: fmt(totalInvested), sub: null },
        { label: "Aktueller Wert", value: fmt(totalCurrent), sub: null },
        { label: "Gewinn / Verlust", value: fmt(totalGain), sub: null, colored: true, positive: totalGain >= 0 },
        { label: "Performance", value: fmtPct(totalGainPct), sub: `${positions.length} Positionen`, colored: true, positive: totalGainPct >= 0 },
      ] as item}
        <Card.Root>
          <Card.Header class="pb-1 pt-4 px-4">
            <Card.Title class="text-xs font-medium text-muted-foreground">{item.label}</Card.Title>
          </Card.Header>
          <Card.Content class="px-4 pb-4">
            <p class="text-lg font-semibold {item.colored ? (item.positive ? 'text-green-500' : 'text-destructive') : ''}">{item.value}</p>
            {#if item.sub}<p class="text-xs text-muted-foreground">{item.sub}</p>{/if}
          </Card.Content>
        </Card.Root>
      {/each}
    </div>

    <!-- Positions -->
    <Card.Root>
      <Card.Header>
        <Card.Title class="text-sm font-medium">Positionen</Card.Title>
      </Card.Header>
      <Card.Content class="p-0">
        <div class="overflow-x-auto">
          <table class="w-full text-sm">
            <thead class="border-b bg-muted/40">
              <tr>
                {#each ["ISIN", "Name", "Typ", "Anzahl", "Ø Kauf", "Kurs", "Wert", "G/V", "1T", ""] as h}
                  <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground whitespace-nowrap">{h}</th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each positions as pos}
                {@const q = quotes[pos.isin]}
                {@const curPrice = q?.price ?? pos.avgBuyPrice}
                {@const invested = pos.quantity * pos.avgBuyPrice}
                {@const current = pos.quantity * curPrice}
                {@const gain = current - invested}
                {@const gainPct = invested > 0 ? (gain / invested) * 100 : 0}
                <tr class="border-b last:border-0 hover:bg-muted/30 transition-colors">
                  <td class="px-4 py-3 font-mono text-xs text-muted-foreground">{pos.isin}</td>
                  <td class="px-4 py-3 max-w-[150px] truncate font-medium">{q?.name || pos.name || pos.ticker}</td>
                  <td class="px-4 py-3">
                    <Badge variant="secondary" class="text-xs">{assetTypes.find(t => t.value === pos.assetType)?.label}</Badge>
                  </td>
                  <td class="px-4 py-3 tabular-nums">{pos.quantity}</td>
                  <td class="px-4 py-3 tabular-nums">{fmt(pos.avgBuyPrice, pos.currency)}</td>
                  <td class="px-4 py-3 tabular-nums {!q ? 'text-muted-foreground' : ''}">{q ? fmt(q.price, q.currency) : "—"}</td>
                  <td class="px-4 py-3 tabular-nums font-medium">{fmt(current, pos.currency)}</td>
                  <td class="px-4 py-3 tabular-nums {gain >= 0 ? 'text-green-500' : 'text-destructive'}">
                    {fmt(gain, pos.currency)}<br/><span class="text-xs">{fmtPct(gainPct)}</span>
                  </td>
                  <td class="px-4 py-3 tabular-nums {(q?.change_pct ?? 0) >= 0 ? 'text-green-500' : 'text-destructive'}">
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
          {#each countryAllocation as item}
            <div class="space-y-1">
              <div class="flex justify-between text-sm">
                <span>{item.country}</span>
                <span class="text-muted-foreground">{fmt(item.value)} · {item.pct.toFixed(1)}%</span>
              </div>
              <div class="h-1.5 w-full rounded-full bg-muted overflow-hidden">
                <div class="h-full rounded-full bg-primary transition-all" style="width: {item.pct}%"></div>
              </div>
            </div>
          {:else}
            <p class="text-sm text-muted-foreground">Kein Land angegeben</p>
          {/each}
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Header>
          <Card.Title class="text-sm font-medium">Asset-Klassen</Card.Title>
        </Card.Header>
        <Card.Content class="space-y-3">
          {#each typeAllocation as item}
            <div class="space-y-1">
              <div class="flex justify-between text-sm">
                <span>{assetTypes.find(t => t.value === item.type)?.label ?? item.type}</span>
                <span class="text-muted-foreground">{fmt(item.value)} · {item.pct.toFixed(1)}%</span>
              </div>
              <div class="h-1.5 w-full rounded-full bg-muted overflow-hidden">
                <div class="h-full rounded-full transition-all {typeColors[item.type] ?? 'bg-primary'}" style="width: {item.pct}%"></div>
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
      <Dialog.Description>ISIN eingeben — Ticker und Name werden automatisch aufgelöst.</Dialog.Description>
    </Dialog.Header>

    <div class="space-y-4 py-2">
      <!-- ISIN Lookup -->
      <div class="space-y-1.5">
        <Label>ISIN</Label>
        <div class="flex gap-2">
          <Input
            bind:value={isinInput}
            placeholder="z.B. US0378331005"
            class="font-mono uppercase"
            maxlength={12}
            onkeydown={(e) => e.key === "Enter" && resolveIsin()}
          />
          <Button variant="outline" size="sm" onclick={resolveIsin} disabled={resolving}>
            {#if resolving}
              <LoaderIcon class="size-4 animate-spin" />
            {:else}
              <SearchIcon class="size-4" />
            {/if}
          </Button>
        </div>
        {#if resolveError}
          <p class="text-xs text-destructive">{resolveError}</p>
        {/if}
      </div>

      {#if resolved}
        <div class="flex items-center gap-2 rounded-md border border-green-500/30 bg-green-500/5 px-3 py-2">
          <CheckCircleIcon class="size-4 text-green-500 shrink-0" />
          <div class="text-sm">
            <span class="font-mono font-medium">{resolved.ticker}</span>
            {#if resolved.name}
              <span class="text-muted-foreground"> · {resolved.name}</span>
            {/if}
          </div>
        </div>

        <Separator />

        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-1.5">
            <Label>Anzahl</Label>
            <Input type="number" step="0.0001" min="0" bind:value={addQuantity} placeholder="0" />
          </div>
          <div class="space-y-1.5">
            <Label>Ø Kaufpreis</Label>
            <Input type="number" step="0.01" min="0" bind:value={addAvgPrice} placeholder="0.00" />
          </div>
          <div class="space-y-1.5">
            <Label>Währung</Label>
            <Input bind:value={addCurrency} placeholder="EUR" />
          </div>
          <div class="space-y-1.5">
            <Label>Land</Label>
            <Input bind:value={addCountry} placeholder="z.B. USA" />
          </div>
        </div>

        <div class="space-y-1.5">
          <Label>Asset-Typ</Label>
          <Select.Root type="single" bind:value={addAssetType}>
            <Select.Trigger>{assetTypes.find(t => t.value === addAssetType)?.label}</Select.Trigger>
            <Select.Content>
              {#each assetTypes as t}<Select.Item value={t.value}>{t.label}</Select.Item>{/each}
            </Select.Content>
          </Select.Root>
        </div>
      {/if}
    </div>

    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showAddDialog = false)}>Abbrechen</Button>
      <Button onclick={addPosition} disabled={!resolved}>Hinzufügen</Button>
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
          <Label>ISIN</Label>
          <Input bind:value={editForm.isin} class="font-mono uppercase" />
        </div>
        <div class="space-y-1.5">
          <Label>Typ</Label>
          <Select.Root type="single" bind:value={editForm.assetType}>
            <Select.Trigger>{assetTypes.find(t => t.value === editForm.assetType)?.label}</Select.Trigger>
            <Select.Content>
              {#each assetTypes as t}<Select.Item value={t.value}>{t.label}</Select.Item>{/each}
            </Select.Content>
          </Select.Root>
        </div>
      </div>
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5">
          <Label>Anzahl</Label>
          <Input type="number" step="0.0001" bind:value={editForm.quantity} />
        </div>
        <div class="space-y-1.5">
          <Label>Ø Kaufpreis</Label>
          <Input type="number" step="0.01" bind:value={editForm.avgBuyPrice} />
        </div>
        <div class="space-y-1.5">
          <Label>Währung</Label>
          <Input bind:value={editForm.currency} />
        </div>
        <div class="space-y-1.5">
          <Label>Land</Label>
          <Input bind:value={editForm.country} />
        </div>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showEditDialog = false)}>Abbrechen</Button>
      <Button onclick={saveEdit}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
