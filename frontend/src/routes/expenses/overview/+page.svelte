<script lang="ts">
  import { onMount } from "svelte";
  import ExpensesWithFilters from "$lib/components/expenses/expenses-with-filters.svelte";
  import { ExpenseService, type Expense } from "$lib/services/expenses";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Select from "$lib/components/ui/select";
  import { toast } from "svelte-sonner";

  let allExpenses: Expense[] = $state([]);
  let loading = $state(true);
  let selectedMonth = $state(new Date().getMonth() + 1);
  let selectedYear = $state(new Date().getFullYear());
  let selectedCategory = $state("all");
  let amountSearch = $state("");
  let showAddDialog = $state(false);
  let showEditDialog = $state(false);
  let editingExpense: Expense | null = $state(null);
  let formData = $state({ title: "", amount: 0, date: "", category: "Lebensmittel" });

  const categories = ["Lebensmittel", "Transport", "Unterhaltung", "Gesundheit", "Bildung", "Wohnen", "Sonstiges"];

  onMount(async () => {
    try {
      allExpenses = await ExpenseService.getAll();
    } catch {
      toast.error("Fehler beim Laden der Ausgaben");
    } finally {
      loading = false;
    }
  });

  let expenses = $derived(
    allExpenses
      .filter((exp) => {
        const d = new Date(exp.date);
        const catOk = selectedCategory === "all" || exp.category === selectedCategory;
        let amtOk = true;
        if (amountSearch.trim()) {
          const n = parseFloat(amountSearch);
          if (!isNaN(n)) amtOk = Math.abs(exp.amount - n) < 0.01;
        }
        return d.getMonth() + 1 === selectedMonth && d.getFullYear() === selectedYear && catOk && amtOk;
      })
      .map((e) => ({ id: e.id, date: new Date(e.date).toLocaleDateString("de-DE"), category: e.category, amount: e.amount, description: e.title }))
  );

  function handleAddExpense() {
    formData = { title: "", amount: 0, date: new Date().toISOString().slice(0, 10), category: "Lebensmittel" };
    showAddDialog = true;
  }

  async function submitAdd() {
    try {
      await ExpenseService.create(formData);
      showAddDialog = false;
      allExpenses = await ExpenseService.getAll();
      toast.success("Ausgabe erstellt");
    } catch { toast.error("Fehler beim Erstellen"); }
  }

  function handleEditExpense(id: string) {
    const exp = allExpenses.find((e) => e.id === id);
    if (!exp) return;
    editingExpense = exp;
    formData = { title: exp.title, amount: exp.amount, date: exp.date.slice(0, 10), category: exp.category };
    showEditDialog = true;
  }

  async function submitEdit() {
    if (!editingExpense) return;
    try {
      await ExpenseService.update(editingExpense.id, formData);
      showEditDialog = false;
      allExpenses = await ExpenseService.getAll();
      toast.success("Ausgabe aktualisiert");
    } catch { toast.error("Fehler beim Aktualisieren"); }
  }

  async function handleDeleteExpense(id: string) {
    if (!confirm("Ausgabe wirklich löschen?")) return;
    try {
      await ExpenseService.delete(id);
      allExpenses = await ExpenseService.getAll();
      toast.success("Ausgabe gelöscht");
    } catch { toast.error("Fehler beim Löschen"); }
  }
</script>

<div class="container mx-auto py-6">
  {#if loading}
    <div class="flex items-center justify-center h-64">
      <span class="text-muted-foreground text-sm">Laden...</span>
    </div>
  {:else}
    <ExpensesWithFilters
      {expenses} {selectedMonth} {selectedYear} {selectedCategory} {amountSearch}
      onAddExpense={handleAddExpense}
      onUploadFile={() => toast.info("PDF-Import kommt bald")}
      onEditExpense={handleEditExpense}
      onDeleteExpense={handleDeleteExpense}
      onMonthChange={(m) => (selectedMonth = m)}
      onYearChange={(y) => (selectedYear = y)}
      onCategoryChange={(c) => (selectedCategory = c)}
      onAmountSearchChange={(a) => (amountSearch = a)}
    />
  {/if}
</div>

<Dialog.Root bind:open={showAddDialog}>
  <Dialog.Content>
    <Dialog.Header><Dialog.Title>Neue Ausgabe</Dialog.Title></Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2"><Label>Beschreibung</Label><Input bind:value={formData.title} placeholder="z.B. Einkauf" /></div>
      <div class="grid gap-2"><Label>Betrag (€)</Label><Input type="number" step="0.01" bind:value={formData.amount} /></div>
      <div class="grid gap-2"><Label>Datum</Label><Input type="date" bind:value={formData.date} /></div>
      <div class="grid gap-2">
        <Label>Kategorie</Label>
        <Select.Root type="single" bind:value={formData.category}>
          <Select.Trigger>{formData.category}</Select.Trigger>
          <Select.Content>{#each categories as cat}<Select.Item value={cat}>{cat}</Select.Item>{/each}</Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showAddDialog = false)}>Abbrechen</Button>
      <Button onclick={submitAdd}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={showEditDialog}>
  <Dialog.Content>
    <Dialog.Header><Dialog.Title>Ausgabe bearbeiten</Dialog.Title></Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2"><Label>Beschreibung</Label><Input bind:value={formData.title} /></div>
      <div class="grid gap-2"><Label>Betrag (€)</Label><Input type="number" step="0.01" bind:value={formData.amount} /></div>
      <div class="grid gap-2"><Label>Datum</Label><Input type="date" bind:value={formData.date} /></div>
      <div class="grid gap-2">
        <Label>Kategorie</Label>
        <Select.Root type="single" bind:value={formData.category}>
          <Select.Trigger>{formData.category}</Select.Trigger>
          <Select.Content>{#each categories as cat}<Select.Item value={cat}>{cat}</Select.Item>{/each}</Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showEditDialog = false)}>Abbrechen</Button>
      <Button onclick={submitEdit}>Aktualisieren</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
