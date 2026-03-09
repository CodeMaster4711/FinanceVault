<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import { toast } from "svelte-sonner";
  import { YahooService, type Quote, type FundData } from "$lib/services/yahoo";
  import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
  import TrendingDownIcon from "@lucide/svelte/icons/trending-down";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import SearchIcon from "@lucide/svelte/icons/search";
  import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
  import LoaderIcon from "@lucide/svelte/icons/loader";
  import BarChart2Icon from "@lucide/svelte/icons/bar-chart-2";

  interface Position {
    id: string;
    isin: string;
    ticker: string;
    name: string;
    asset_type: "stock" | "etf" | "crypto" | "other";
    quantity: number;
    avg_buy_price: number;
    currency: string;
    country: string;
    created_at: string;
  }

  let positions: Position[] = $state([]);
  let quotes: Record<string, Quote> = $state({});
  let loadingQuotes = $state(false);
  let showAddDialog = $state(false);
  let showEditDialog = $state(false);
  let editingPosition: Position | null = $state(null);
  let showFundDialog = $state(false);
  let fundData: FundData | null = $state(null);
  let fundDataLoading = $state(false);
  let fundDataTicker = $state("");

  // Add form
  let isinInput = $state("");
  let resolving = $state(false);
  let resolved: { ticker: string; name: string } | null = $state(null);
  let resolveError = $state("");
  let addQuantity = $state(0);
  let addAvgPrice = $state(0);
  let addCurrency = $state("EUR");
  let addCountry = $state("");
  let addAssetType = $state<Position["asset_type"]>("stock");

  // Edit form
  let editForm = $state({
    asset_type: "stock" as Position["asset_type"],
    quantity: 0,
    avg_buy_price: 0,
    currency: "EUR",
    country: "",
  });

  const assetTypes = [
    { value: "stock", label: "Aktie" },
    { value: "etf", label: "ETF" },
    { value: "crypto", label: "Krypto" },
    { value: "other", label: "Sonstiges" },
  ];

  onMount(async () => {
    await loadPositions();
    if (positions.length > 0) refreshQuotes();
  });

  async function loadPositions() {
    try {
      positions = await invoke<Position[]>("get_positions");
    } catch {
      toast.error("Positionen konnten nicht geladen werden");
    }
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

  async function addPosition() {
    if (!resolved) { toast.error("Bitte zuerst ISIN auflösen"); return; }
    if (!addQuantity || !addAvgPrice) { toast.error("Anzahl und Kaufpreis sind Pflicht"); return; }

    try {
      const pos = await invoke<Position>("create_position", {
        input: {
          isin: isinInput.trim().toUpperCase(),
          ticker: resolved.ticker,
          name: resolved.name,
          asset_type: addAssetType,
          quantity: addQuantity,
          avg_buy_price: addAvgPrice,
          currency: addCurrency,
          country: addCountry,
        },
      });
      positions = [...positions, pos];
      showAddDialog = false;
      refreshQuotes();
      toast.success(`${pos.name || pos.ticker} hinzugefügt`);
    } catch {
      toast.error("Position konnte nicht gespeichert werden");
    }
  }

  function openEdit(pos: Position) {
    editingPosition = pos;
    editForm = {
      asset_type: pos.asset_type,
      quantity: pos.quantity,
      avg_buy_price: pos.avg_buy_price,
      currency: pos.currency,
      country: pos.country,
    };
    showEditDialog = true;
  }

  async function saveEdit() {
    if (!editingPosition) return;
    try {
      await invoke("update_position", {
        id: editingPosition.id,
        input: editForm,
      });
      positions = positions.map((p) =>
        p.id === editingPosition!.id ? { ...p, ...editForm } : p
      );
      showEditDialog = false;
      refreshQuotes();
      toast.success("Position aktualisiert");
    } catch {
      toast.error("Fehler beim Aktualisieren");
    }
  }

  async function deletePosition(id: string) {
    if (!confirm("Position wirklich löschen?")) return;
    try {
      await invoke("delete_position", { id });
      positions = positions.filter((p) => p.id !== id);
      toast.success("Position gelöscht");
    } catch {
      toast.error("Fehler beim Löschen");
    }
  }

  async function openFundData(ticker: string) {
    fundDataTicker = ticker;
    fundData = null;
    fundDataLoading = true;
    showFundDialog = true;
    try {
      fundData = await YahooService.fetchFundData(ticker);
    } catch {
      toast.error(`Keine Fund-Daten für ${ticker} verfügbar`);
      showFundDialog = false;
    } finally {
      fundDataLoading = false;
    }
  }

  const sectorLabels: Record<string, string> = {
    technology: "Technologie",
    financial_services: "Finanzen",
    healthcare: "Gesundheit",
    consumer_cyclical: "Konsum (zyklisch)",
    consumer_defensive: "Konsum (defensiv)",
    industrials: "Industrie",
    communication_services: "Kommunikation",
    energy: "Energie",
    basic_materials: "Rohstoffe",
    utilities: "Versorger",
    realestate: "Immobilien",
  };

  function fmt(value: number, currency = "EUR") {
    return new Intl.NumberFormat("de-DE", { style: "currency", currency }).format(value);
  }

  function fmtPct(value: number) {
    return `${value >= 0 ? "+" : ""}${value.toFixed(2)}%`;
  }

  let totalInvested = $derived(positions.reduce((s, p) => s + p.quantity * p.avg_buy_price, 0));
  let totalCurrent = $derived(positions.reduce((s, p) => {
    const q = quotes[p.isin];
    return s + p.quantity * (q ? q.price : p.avg_buy_price);
  }, 0));
  let totalGain = $derived(totalCurrent - totalInvested);
  let totalGainPct = $derived(totalInvested > 0 ? (totalGain / totalInvested) * 100 : 0);

  let countryAllocation = $derived(
    Object.entries(
      positions.reduce((map, p) => {
        const val = p.quantity * (quotes[p.isin]?.price ?? p.avg_buy_price);
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
        const val = p.quantity * (quotes[p.isin]?.price ?? p.avg_buy_price);
        map[p.asset_type] = (map[p.asset_type] ?? 0) + val;
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
                {#each ["ISIN", "Name", "Typ", "Anzahl", "Ø Kauf", "Kurs", "Wert", "Anteil", "G/V", "1T", ""] as h}
                  <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground whitespace-nowrap">{h}</th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each positions as pos}
                {@const q = quotes[pos.isin]}
                {@const curPrice = q?.price ?? pos.avg_buy_price}
                {@const invested = pos.quantity * pos.avg_buy_price}
                {@const current = pos.quantity * curPrice}
                {@const gain = current - invested}
                {@const gainPct = invested > 0 ? (gain / invested) * 100 : 0}
                {@const portfolioPct = totalCurrent > 0 ? (current / totalCurrent) * 100 : 0}
                <tr class="border-b last:border-0 hover:bg-muted/30 transition-colors">
                  <td class="px-4 py-3 font-mono text-xs text-muted-foreground">{pos.isin}</td>
                  <td class="px-4 py-3 max-w-[150px] truncate font-medium">{q?.name || pos.name || pos.ticker}</td>
                  <td class="px-4 py-3">
                    <Badge variant="secondary" class="text-xs">{assetTypes.find(t => t.value === pos.asset_type)?.label}</Badge>
                  </td>
                  <td class="px-4 py-3 tabular-nums">{pos.quantity}</td>
                  <td class="px-4 py-3 tabular-nums">{fmt(pos.avg_buy_price, pos.currency)}</td>
                  <td class="px-4 py-3 tabular-nums {!q ? 'text-muted-foreground' : ''}">{q ? fmt(q.price, q.currency) : "—"}</td>
                  <td class="px-4 py-3 tabular-nums font-medium">{fmt(current, pos.currency)}</td>
                  <td class="px-4 py-3 min-w-[90px]">
                    <div class="flex items-center gap-1.5">
                      <div class="h-1 w-14 rounded-full bg-muted overflow-hidden">
                        <div class="h-full rounded-full bg-primary transition-all" style="width: {portfolioPct}%"></div>
                      </div>
                      <span class="tabular-nums text-xs text-muted-foreground">{portfolioPct.toFixed(1)}%</span>
                    </div>
                  </td>
                  <td class="px-4 py-3 tabular-nums {gain >= 0 ? 'text-green-500' : 'text-destructive'}">
                    {fmt(gain, pos.currency)}<br/><span class="text-xs">{fmtPct(gainPct)}</span>
                  </td>
                  <td class="px-4 py-3 tabular-nums {(q?.change_pct ?? 0) >= 0 ? 'text-green-500' : 'text-destructive'}">
                    {q ? fmtPct(q.change_pct) : "—"}
                  </td>
                  <td class="px-4 py-3">
                    <div class="flex gap-1">
                      <Button variant="ghost" size="icon" class="size-7" onclick={() => openFundData(pos.ticker)} title="Holdings & Sektoren">
                        <BarChart2Icon class="size-3.5" />
                      </Button>
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
      <div class="space-y-1.5">
        <Label>Typ</Label>
        <Select.Root type="single" bind:value={editForm.asset_type}>
          <Select.Trigger>{assetTypes.find(t => t.value === editForm.asset_type)?.label}</Select.Trigger>
          <Select.Content>
            {#each assetTypes as t}<Select.Item value={t.value}>{t.label}</Select.Item>{/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5">
          <Label>Anzahl</Label>
          <Input type="number" step="0.0001" bind:value={editForm.quantity} />
        </div>
        <div class="space-y-1.5">
          <Label>Ø Kaufpreis</Label>
          <Input type="number" step="0.01" bind:value={editForm.avg_buy_price} />
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

<!-- Fund Data Dialog -->
<Dialog.Root bind:open={showFundDialog}>
  <Dialog.Content class="max-w-lg">
    <Dialog.Header>
      <Dialog.Title>{fundDataTicker} — Holdings & Sektoren</Dialog.Title>
      <Dialog.Description>Daten via Yahoo Finance · keine Daten werden hochgeladen</Dialog.Description>
    </Dialog.Header>

    {#if fundDataLoading}
      <div class="flex items-center justify-center h-32">
        <LoaderIcon class="size-5 animate-spin text-muted-foreground" />
      </div>
    {:else if fundData}
      <div class="space-y-5 py-2 max-h-[60vh] overflow-y-auto pr-1">

        {#if fundData.holdings.length > 0}
          <div class="space-y-2">
            <p class="text-sm font-medium">Top Holdings</p>
            {#each fundData.holdings as h}
              {@const pct = h.percent * 100}
              <div class="space-y-1">
                <div class="flex justify-between text-sm">
                  <span class="truncate max-w-[260px]">{h.name || h.symbol}</span>
                  <span class="tabular-nums text-muted-foreground shrink-0 ml-2">{pct.toFixed(2)}%</span>
                </div>
                <div class="h-1.5 w-full rounded-full bg-muted overflow-hidden">
                  <div class="h-full rounded-full bg-primary transition-all" style="width: {Math.min(pct * 3, 100)}%"></div>
                </div>
              </div>
            {/each}
          </div>
        {/if}

        {#if fundData.holdings.length > 0 && fundData.sector_weights.length > 0}
          <Separator />
        {/if}

        {#if fundData.sector_weights.length > 0}
          {@const maxSector = Math.max(...fundData.sector_weights.map(s => s.percent))}
          <div class="space-y-2">
            <p class="text-sm font-medium">Sektorgewichtung</p>
            {#each fundData.sector_weights.sort((a, b) => b.percent - a.percent) as s}
              {@const pct = s.percent * 100}
              <div class="space-y-1">
                <div class="flex justify-between text-sm">
                  <span>{sectorLabels[s.sector] ?? s.sector}</span>
                  <span class="tabular-nums text-muted-foreground">{pct.toFixed(2)}%</span>
                </div>
                <div class="h-1.5 w-full rounded-full bg-muted overflow-hidden">
                  <div class="h-full rounded-full bg-blue-500 transition-all" style="width: {maxSector > 0 ? (s.percent / maxSector) * 100 : 0}%"></div>
                </div>
              </div>
            {/each}
          </div>
        {/if}

        {#if fundData.holdings.length === 0 && fundData.sector_weights.length === 0}
          <p class="text-sm text-muted-foreground text-center py-4">Keine Daten verfügbar.</p>
        {/if}
      </div>
    {/if}

    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showFundDialog = false)}>Schließen</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
