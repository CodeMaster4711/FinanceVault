<script lang="ts">
  import { onMount } from "svelte";
  import * as Card from "$lib/components/ui/card/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import { toast } from "svelte-sonner";
  import { ExpenseService, type Expense } from "$lib/services/expenses";
  import { SavingsService, type SavingsPlan } from "$lib/services/savings";
  import { YahooService } from "$lib/services/yahoo";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import SearchIcon from "@lucide/svelte/icons/search";
  import CheckCircleIcon from "@lucide/svelte/icons/check-circle";
  import LoaderIcon from "@lucide/svelte/icons/loader";

  // --- Expenses ---
  let allExpenses: Expense[] = $state([]);
  let loadingExpenses = $state(true);
  let selectedMonth = $state(new Date().getMonth() + 1);
  let selectedYear = $state(new Date().getFullYear());
  let selectedCategory = $state("all");
  let showAddExpense = $state(false);
  let showEditExpense = $state(false);
  let editingExpense: Expense | null = $state(null);
  let expenseForm = $state({ title: "", amount: 0, date: new Date().toISOString().slice(0, 10), category: "Lebensmittel" });

  const categories = ["Lebensmittel", "Transport", "Unterhaltung", "Gesundheit", "Bildung", "Wohnen", "Sonstiges"];
  const months = ["Jan", "Feb", "Mär", "Apr", "Mai", "Jun", "Jul", "Aug", "Sep", "Okt", "Nov", "Dez"];

  // --- Savings Plans ---
  let plans: SavingsPlan[] = $state([]);
  let loadingPlans = $state(true);
  let showAddPlan = $state(false);
  let showEditPlan = $state(false);
  let editingPlan: SavingsPlan | null = $state(null);
  let planForm = $state({ name: "", isin: "", ticker: "", amount: 0, currency: "EUR", interval: "monthly", next_date: new Date().toISOString().slice(0, 10) });
  let resolving = $state(false);
  let resolved: { ticker: string; name: string } | null = $state(null);
  let resolveError = $state("");

  const intervals = [
    { value: "monthly", label: "Monatlich" },
    { value: "quarterly", label: "Quartalsweise" },
    { value: "yearly", label: "Jährlich" },
  ];

  onMount(async () => {
    await Promise.all([loadExpenses(), loadPlans()]);
  });

  async function loadExpenses() {
    loadingExpenses = true;
    try { allExpenses = await ExpenseService.getAll(); }
    catch { toast.error("Fehler beim Laden der Ausgaben"); }
    finally { loadingExpenses = false; }
  }

  async function loadPlans() {
    loadingPlans = true;
    try { plans = await SavingsService.getAll(); }
    catch { toast.error("Fehler beim Laden der Sparpläne"); }
    finally { loadingPlans = false; }
  }

  let filteredExpenses = $derived(
    allExpenses.filter((e) => {
      const d = new Date(e.date);
      return d.getMonth() + 1 === selectedMonth
        && d.getFullYear() === selectedYear
        && (selectedCategory === "all" || e.category === selectedCategory);
    }).sort((a, b) => b.date.localeCompare(a.date))
  );

  let expenseTotal = $derived(filteredExpenses.reduce((s, e) => s + e.amount, 0));

  let categoryBreakdown = $derived(
    Object.entries(
      filteredExpenses.reduce((m, e) => { m[e.category] = (m[e.category] ?? 0) + e.amount; return m; }, {} as Record<string, number>)
    ).sort((a, b) => b[1] - a[1])
  );

  let plansMonthly = $derived(plans.reduce((s, p) => {
    if (p.interval === "monthly") return s + p.amount;
    if (p.interval === "quarterly") return s + p.amount / 3;
    if (p.interval === "yearly") return s + p.amount / 12;
    return s;
  }, 0));

  function fmt(v: number, currency = "EUR") {
    return new Intl.NumberFormat("de-DE", { style: "currency", currency }).format(v);
  }

  // Expense actions
  function openAddExpense() {
    expenseForm = { title: "", amount: 0, date: new Date().toISOString().slice(0, 10), category: "Lebensmittel" };
    showAddExpense = true;
  }

  async function submitAddExpense() {
    try {
      await ExpenseService.create(expenseForm);
      showAddExpense = false;
      await loadExpenses();
      toast.success("Ausgabe erstellt");
    } catch { toast.error("Fehler beim Erstellen"); }
  }

  function openEditExpense(e: Expense) {
    editingExpense = e;
    expenseForm = { title: e.title, amount: e.amount, date: e.date.slice(0, 10), category: e.category };
    showEditExpense = true;
  }

  async function submitEditExpense() {
    if (!editingExpense) return;
    try {
      await ExpenseService.update(editingExpense.id, expenseForm);
      showEditExpense = false;
      await loadExpenses();
      toast.success("Ausgabe aktualisiert");
    } catch { toast.error("Fehler beim Aktualisieren"); }
  }

  async function deleteExpense(id: string) {
    if (!confirm("Ausgabe löschen?")) return;
    try {
      await ExpenseService.delete(id);
      await loadExpenses();
      toast.success("Ausgabe gelöscht");
    } catch { toast.error("Fehler beim Löschen"); }
  }

  // Savings plan actions
  async function resolveIsin() {
    const isin = planForm.isin.trim().toUpperCase();
    if (isin.length < 12) { resolveError = "Ungültige ISIN"; return; }
    resolving = true;
    resolveError = "";
    resolved = null;
    try {
      const [ticker, name] = await YahooService.resolveIsin(isin);
      resolved = { ticker, name };
      planForm.ticker = ticker;
      if (!planForm.name) planForm.name = name;
    } catch {
      resolveError = `Nicht gefunden: ${isin}`;
    } finally {
      resolving = false;
    }
  }

  function openAddPlan() {
    planForm = { name: "", isin: "", ticker: "", amount: 0, currency: "EUR", interval: "monthly", next_date: new Date().toISOString().slice(0, 10) };
    resolved = null;
    resolveError = "";
    showAddPlan = true;
  }

  async function submitAddPlan() {
    if (!planForm.isin || !planForm.ticker) { toast.error("Bitte ISIN auflösen"); return; }
    if (!planForm.amount) { toast.error("Betrag fehlt"); return; }
    try {
      await SavingsService.create({ ...planForm, isin: planForm.isin.toUpperCase() });
      showAddPlan = false;
      await loadPlans();
      toast.success("Sparplan erstellt");
    } catch { toast.error("Fehler beim Erstellen"); }
  }

  function openEditPlan(p: SavingsPlan) {
    editingPlan = p;
    planForm = { name: p.name, isin: p.isin, ticker: p.ticker, amount: p.amount, currency: p.currency, interval: p.interval, next_date: p.next_date.slice(0, 10) };
    resolved = { ticker: p.ticker, name: p.name };
    showEditPlan = true;
  }

  async function submitEditPlan() {
    if (!editingPlan) return;
    try {
      await SavingsService.update(editingPlan.id, { name: planForm.name, amount: planForm.amount, currency: planForm.currency, interval: planForm.interval, next_date: planForm.next_date });
      showEditPlan = false;
      await loadPlans();
      toast.success("Sparplan aktualisiert");
    } catch { toast.error("Fehler beim Aktualisieren"); }
  }

  async function deletePlan(id: string) {
    if (!confirm("Sparplan löschen?")) return;
    try {
      await SavingsService.delete(id);
      await loadPlans();
      toast.success("Sparplan gelöscht");
    } catch { toast.error("Fehler beim Löschen"); }
  }
</script>

<div class="p-6 space-y-6 max-w-5xl">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-semibold tracking-tight">Ausgaben & Sparpläne</h1>
      <p class="text-sm text-muted-foreground">Übersicht, Kategorien und automatische Investitionen</p>
    </div>
  </div>

  <Tabs.Root value="expenses">
    <Tabs.List>
      <Tabs.Trigger value="expenses">Ausgaben</Tabs.Trigger>
      <Tabs.Trigger value="savings">Sparpläne</Tabs.Trigger>
    </Tabs.List>

    <!-- EXPENSES TAB -->
    <Tabs.Content value="expenses" class="space-y-4 mt-4">
      <div class="flex flex-wrap items-center gap-2">
        <Select.Root type="single" value={String(selectedMonth)} onValueChange={(v) => (selectedMonth = Number(v))}>
          <Select.Trigger class="w-28">{months[selectedMonth - 1]}</Select.Trigger>
          <Select.Content>{#each months as m, i}<Select.Item value={String(i + 1)}>{m}</Select.Item>{/each}</Select.Content>
        </Select.Root>
        <Select.Root type="single" value={String(selectedYear)} onValueChange={(v) => (selectedYear = Number(v))}>
          <Select.Trigger class="w-24">{selectedYear}</Select.Trigger>
          <Select.Content>{#each [2023,2024,2025,2026] as y}<Select.Item value={String(y)}>{y}</Select.Item>{/each}</Select.Content>
        </Select.Root>
        <Select.Root type="single" bind:value={selectedCategory}>
          <Select.Trigger class="w-36">{selectedCategory === "all" ? "Alle Kategorien" : selectedCategory}</Select.Trigger>
          <Select.Content>
            <Select.Item value="all">Alle Kategorien</Select.Item>
            {#each categories as c}<Select.Item value={c}>{c}</Select.Item>{/each}
          </Select.Content>
        </Select.Root>
        <div class="ml-auto">
          <Button size="sm" onclick={openAddExpense}><PlusIcon class="size-4 mr-1.5" />Ausgabe</Button>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4 sm:grid-cols-3">
        <Card.Root>
          <Card.Header class="pb-1 pt-4 px-4"><Card.Title class="text-xs text-muted-foreground">Gesamt</Card.Title></Card.Header>
          <Card.Content class="px-4 pb-4"><p class="text-lg font-semibold">{fmt(expenseTotal)}</p></Card.Content>
        </Card.Root>
        <Card.Root>
          <Card.Header class="pb-1 pt-4 px-4"><Card.Title class="text-xs text-muted-foreground">Buchungen</Card.Title></Card.Header>
          <Card.Content class="px-4 pb-4"><p class="text-lg font-semibold">{filteredExpenses.length}</p></Card.Content>
        </Card.Root>
        <Card.Root>
          <Card.Header class="pb-1 pt-4 px-4"><Card.Title class="text-xs text-muted-foreground">Ø pro Buchung</Card.Title></Card.Header>
          <Card.Content class="px-4 pb-4"><p class="text-lg font-semibold">{filteredExpenses.length ? fmt(expenseTotal / filteredExpenses.length) : "—"}</p></Card.Content>
        </Card.Root>
      </div>

      {#if categoryBreakdown.length > 0}
        <Card.Root>
          <Card.Header><Card.Title class="text-sm font-medium">Nach Kategorie</Card.Title></Card.Header>
          <Card.Content class="space-y-3">
            {#each categoryBreakdown as [cat, amount]}
              {@const pct = expenseTotal > 0 ? (amount / expenseTotal) * 100 : 0}
              <div class="space-y-1">
                <div class="flex justify-between text-sm">
                  <span>{cat}</span>
                  <span class="text-muted-foreground">{fmt(amount)} ({pct.toFixed(1)}%)</span>
                </div>
                <div class="h-1.5 w-full rounded-full bg-muted overflow-hidden">
                  <div class="h-full rounded-full bg-primary transition-all" style="width: {pct}%"></div>
                </div>
              </div>
            {/each}
          </Card.Content>
        </Card.Root>
      {/if}

      {#if loadingExpenses}
        <div class="flex justify-center h-24 items-center"><span class="text-sm text-muted-foreground">Laden...</span></div>
      {:else}
        <Card.Root>
          <Card.Content class="p-0">
            <table class="w-full text-sm">
              <thead class="border-b bg-muted/40">
                <tr>
                  {#each ["Datum", "Beschreibung", "Kategorie", "Betrag", ""] as h}
                    <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground">{h}</th>
                  {/each}
                </tr>
              </thead>
              <tbody>
                {#each filteredExpenses as e}
                  <tr class="border-b last:border-0 hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-2.5 text-muted-foreground whitespace-nowrap">{new Date(e.date).toLocaleDateString("de-DE")}</td>
                    <td class="px-4 py-2.5 max-w-[200px] truncate">{e.title}</td>
                    <td class="px-4 py-2.5"><Badge variant="secondary" class="text-xs">{e.category}</Badge></td>
                    <td class="px-4 py-2.5 tabular-nums font-medium">{fmt(e.amount, e.currency)}</td>
                    <td class="px-4 py-2.5">
                      <div class="flex gap-1">
                        <Button variant="ghost" size="icon" class="size-7" onclick={() => openEditExpense(e)}><PencilIcon class="size-3.5" /></Button>
                        <Button variant="ghost" size="icon" class="size-7 text-destructive" onclick={() => deleteExpense(e.id)}><TrashIcon class="size-3.5" /></Button>
                      </div>
                    </td>
                  </tr>
                {:else}
                  <tr><td colspan="5" class="px-4 py-8 text-center text-sm text-muted-foreground">Keine Ausgaben in diesem Monat.</td></tr>
                {/each}
              </tbody>
            </table>
          </Card.Content>
        </Card.Root>
      {/if}
    </Tabs.Content>

    <!-- SAVINGS PLANS TAB -->
    <Tabs.Content value="savings" class="space-y-4 mt-4">
      <div class="flex items-center justify-between">
        <div class="grid grid-cols-2 gap-4">
          <Card.Root>
            <Card.Header class="pb-1 pt-4 px-4"><Card.Title class="text-xs text-muted-foreground">Aktive Pläne</Card.Title></Card.Header>
            <Card.Content class="px-4 pb-4"><p class="text-lg font-semibold">{plans.length}</p></Card.Content>
          </Card.Root>
          <Card.Root>
            <Card.Header class="pb-1 pt-4 px-4"><Card.Title class="text-xs text-muted-foreground">Monatlich gesamt</Card.Title></Card.Header>
            <Card.Content class="px-4 pb-4"><p class="text-lg font-semibold">{fmt(plansMonthly)}</p></Card.Content>
          </Card.Root>
        </div>
        <Button size="sm" onclick={openAddPlan}><PlusIcon class="size-4 mr-1.5" />Sparplan</Button>
      </div>

      {#if loadingPlans}
        <div class="flex justify-center h-24 items-center"><span class="text-sm text-muted-foreground">Laden...</span></div>
      {:else if plans.length === 0}
        <Card.Root>
          <Card.Content class="flex flex-col items-center justify-center h-36 gap-2">
            <p class="text-sm text-muted-foreground">Noch keine Sparpläne angelegt.</p>
            <Button size="sm" onclick={openAddPlan}><PlusIcon class="size-4 mr-1.5" />Ersten Sparplan erstellen</Button>
          </Card.Content>
        </Card.Root>
      {:else}
        <Card.Root>
          <Card.Content class="p-0">
            <table class="w-full text-sm">
              <thead class="border-b bg-muted/40">
                <tr>
                  {#each ["Name", "ISIN", "Betrag", "Intervall", "Nächste Ausführung", ""] as h}
                    <th class="h-10 px-4 text-left text-xs font-medium text-muted-foreground whitespace-nowrap">{h}</th>
                  {/each}
                </tr>
              </thead>
              <tbody>
                {#each plans as p}
                  <tr class="border-b last:border-0 hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-2.5 font-medium">{p.name}</td>
                    <td class="px-4 py-2.5 font-mono text-xs text-muted-foreground">{p.isin}</td>
                    <td class="px-4 py-2.5 tabular-nums">{fmt(p.amount, p.currency)}</td>
                    <td class="px-4 py-2.5"><Badge variant="outline" class="text-xs">{intervals.find(i => i.value === p.interval)?.label ?? p.interval}</Badge></td>
                    <td class="px-4 py-2.5 text-muted-foreground">{new Date(p.next_date).toLocaleDateString("de-DE")}</td>
                    <td class="px-4 py-2.5">
                      <div class="flex gap-1">
                        <Button variant="ghost" size="icon" class="size-7" onclick={() => openEditPlan(p)}><PencilIcon class="size-3.5" /></Button>
                        <Button variant="ghost" size="icon" class="size-7 text-destructive" onclick={() => deletePlan(p.id)}><TrashIcon class="size-3.5" /></Button>
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </Card.Content>
        </Card.Root>
      {/if}
    </Tabs.Content>
  </Tabs.Root>
</div>

<!-- Add Expense Dialog -->
<Dialog.Root bind:open={showAddExpense}>
  <Dialog.Content class="max-w-sm">
    <Dialog.Header><Dialog.Title>Neue Ausgabe</Dialog.Title></Dialog.Header>
    <div class="grid gap-3 py-2">
      <div class="space-y-1.5"><Label>Beschreibung</Label><Input bind:value={expenseForm.title} placeholder="z.B. Einkauf" /></div>
      <div class="space-y-1.5"><Label>Betrag (€)</Label><Input type="number" step="0.01" bind:value={expenseForm.amount} /></div>
      <div class="space-y-1.5"><Label>Datum</Label><Input type="date" bind:value={expenseForm.date} /></div>
      <div class="space-y-1.5">
        <Label>Kategorie</Label>
        <Select.Root type="single" bind:value={expenseForm.category}>
          <Select.Trigger>{expenseForm.category}</Select.Trigger>
          <Select.Content>{#each categories as c}<Select.Item value={c}>{c}</Select.Item>{/each}</Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showAddExpense = false)}>Abbrechen</Button>
      <Button onclick={submitAddExpense}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Edit Expense Dialog -->
<Dialog.Root bind:open={showEditExpense}>
  <Dialog.Content class="max-w-sm">
    <Dialog.Header><Dialog.Title>Ausgabe bearbeiten</Dialog.Title></Dialog.Header>
    <div class="grid gap-3 py-2">
      <div class="space-y-1.5"><Label>Beschreibung</Label><Input bind:value={expenseForm.title} /></div>
      <div class="space-y-1.5"><Label>Betrag (€)</Label><Input type="number" step="0.01" bind:value={expenseForm.amount} /></div>
      <div class="space-y-1.5"><Label>Datum</Label><Input type="date" bind:value={expenseForm.date} /></div>
      <div class="space-y-1.5">
        <Label>Kategorie</Label>
        <Select.Root type="single" bind:value={expenseForm.category}>
          <Select.Trigger>{expenseForm.category}</Select.Trigger>
          <Select.Content>{#each categories as c}<Select.Item value={c}>{c}</Select.Item>{/each}</Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showEditExpense = false)}>Abbrechen</Button>
      <Button onclick={submitEditExpense}>Aktualisieren</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Add Savings Plan Dialog -->
<Dialog.Root bind:open={showAddPlan}>
  <Dialog.Content class="max-w-md">
    <Dialog.Header>
      <Dialog.Title>Neuer Sparplan</Dialog.Title>
      <Dialog.Description>Automatischer Kauf via ISIN — wird lokal gespeichert.</Dialog.Description>
    </Dialog.Header>
    <div class="space-y-3 py-2">
      <div class="space-y-1.5">
        <Label>ISIN</Label>
        <div class="flex gap-2">
          <Input bind:value={planForm.isin} placeholder="z.B. IE00B4L5Y983" class="font-mono uppercase" maxlength={12} onkeydown={(e) => e.key === "Enter" && resolveIsin()} />
          <Button variant="outline" size="sm" onclick={resolveIsin} disabled={resolving}>
            {#if resolving}<LoaderIcon class="size-4 animate-spin" />{:else}<SearchIcon class="size-4" />{/if}
          </Button>
        </div>
        {#if resolveError}<p class="text-xs text-destructive">{resolveError}</p>{/if}
        {#if resolved}
          <div class="flex items-center gap-2 rounded-md border border-green-500/30 bg-green-500/5 px-3 py-2">
            <CheckCircleIcon class="size-4 text-green-500 shrink-0" />
            <span class="text-sm font-mono font-medium">{resolved.ticker}</span>
            {#if resolved.name}<span class="text-sm text-muted-foreground"> · {resolved.name}</span>{/if}
          </div>
        {/if}
      </div>
      <div class="space-y-1.5"><Label>Name / Bezeichnung</Label><Input bind:value={planForm.name} placeholder="z.B. MSCI World ETF" /></div>
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5"><Label>Betrag</Label><Input type="number" step="0.01" bind:value={planForm.amount} /></div>
        <div class="space-y-1.5"><Label>Währung</Label><Input bind:value={planForm.currency} placeholder="EUR" /></div>
      </div>
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5">
          <Label>Intervall</Label>
          <Select.Root type="single" bind:value={planForm.interval}>
            <Select.Trigger>{intervals.find(i => i.value === planForm.interval)?.label}</Select.Trigger>
            <Select.Content>{#each intervals as i}<Select.Item value={i.value}>{i.label}</Select.Item>{/each}</Select.Content>
          </Select.Root>
        </div>
        <div class="space-y-1.5"><Label>Nächste Ausführung</Label><Input type="date" bind:value={planForm.next_date} /></div>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showAddPlan = false)}>Abbrechen</Button>
      <Button onclick={submitAddPlan} disabled={!resolved}>Erstellen</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Edit Savings Plan Dialog -->
<Dialog.Root bind:open={showEditPlan}>
  <Dialog.Content class="max-w-md">
    <Dialog.Header><Dialog.Title>Sparplan bearbeiten</Dialog.Title></Dialog.Header>
    <div class="space-y-3 py-2">
      <div class="space-y-1.5"><Label>Name</Label><Input bind:value={planForm.name} /></div>
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5"><Label>Betrag</Label><Input type="number" step="0.01" bind:value={planForm.amount} /></div>
        <div class="space-y-1.5"><Label>Währung</Label><Input bind:value={planForm.currency} /></div>
      </div>
      <div class="grid grid-cols-2 gap-3">
        <div class="space-y-1.5">
          <Label>Intervall</Label>
          <Select.Root type="single" bind:value={planForm.interval}>
            <Select.Trigger>{intervals.find(i => i.value === planForm.interval)?.label}</Select.Trigger>
            <Select.Content>{#each intervals as i}<Select.Item value={i.value}>{i.label}</Select.Item>{/each}</Select.Content>
          </Select.Root>
        </div>
        <div class="space-y-1.5"><Label>Nächste Ausführung</Label><Input type="date" bind:value={planForm.next_date} /></div>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showEditPlan = false)}>Abbrechen</Button>
      <Button onclick={submitEditPlan}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
