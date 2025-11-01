<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { browser } from "$app/environment";
  import Expenses from "$lib/components/expenses/expenses.svelte";
  import {
    ExpenseService,
    type Expense,
    type CreateExpenseRequest,
  } from "$lib/services/expenses";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Select from "$lib/components/ui/select";
  import { toast } from "svelte-sonner";

  let expenses: Expense[] = $state([]);
  let loading = $state(true);
  let token = $state("");

  // Dialog state
  let showAddDialog = $state(false);
  let showEditDialog = $state(false);
  let editingExpense: Expense | null = $state(null);

  // Form state
  let formData = $state({
    description: "",
    amount: 0,
    date: "",
    category: "Lebensmittel",
  });

  const categories = [
    "Lebensmittel",
    "Transport",
    "Unterhaltung",
    "Gesundheit",
    "Bildung",
    "Sonstiges",
  ];

  onMount(async () => {
    if (browser) {
      const storedToken = localStorage.getItem("token");
      if (!storedToken) {
        goto("/signin");
        return;
      }
      token = storedToken;
      await loadExpenses();
    }
  });

  async function loadExpenses() {
    try {
      loading = true;
      expenses = await ExpenseService.getExpenses(token);
    } catch (error) {
      console.error("Failed to load expenses:", error);
      toast.error("Fehler beim Laden der Ausgaben");
    } finally {
      loading = false;
    }
  }

  function handleAddExpense() {
    formData = {
      description: "",
      amount: 0,
      date: new Date().toISOString().slice(0, 19).replace("T", " "),
      category: "Lebensmittel",
    };
    showAddDialog = true;
  }

  async function submitAddExpense() {
    try {
      const request: CreateExpenseRequest = {
        description: formData.description,
        amount: formData.amount,
        date: formData.date,
        category: formData.category,
      };

      await ExpenseService.createExpense(request, token);
      showAddDialog = false;
      await loadExpenses();
      toast.success("Ausgabe erfolgreich erstellt");
    } catch (error) {
      console.error("Failed to create expense:", error);
      toast.error("Fehler beim Erstellen der Ausgabe");
    }
  }

  function handleEditExpense(id: string) {
    const expense = expenses.find((e) => e.id === id);
    if (expense) {
      editingExpense = expense;
      formData = {
        description: expense.description,
        amount: parseFloat(expense.amount.toString()),
        date: expense.date,
        category: expense.category,
      };
      showEditDialog = true;
    }
  }

  async function submitEditExpense() {
    if (!editingExpense) return;

    try {
      await ExpenseService.updateExpense(
        editingExpense.id,
        {
          description: formData.description,
          amount: formData.amount,
          date: formData.date,
          category: formData.category,
        },
        token
      );
      showEditDialog = false;
      await loadExpenses();
      toast.success("Ausgabe erfolgreich aktualisiert");
    } catch (error) {
      console.error("Failed to update expense:", error);
      toast.error("Fehler beim Aktualisieren der Ausgabe");
    }
  }

  async function handleDeleteExpense(id: string) {
    if (!confirm("Möchten Sie diese Ausgabe wirklich löschen?")) return;

    try {
      await ExpenseService.deleteExpense(id, token);
      await loadExpenses();
      toast.success("Ausgabe erfolgreich gelöscht");
    } catch (error) {
      console.error("Failed to delete expense:", error);
      toast.error("Fehler beim Löschen der Ausgabe");
    }
  }

  function handleUploadFile(type: "image" | "pdf") {
    toast.info(`${type === "image" ? "Bild" : "PDF"}-Upload kommt bald!`);
  }

  // Transform expenses for the component using $derived
  let transformedExpenses = $derived(
    expenses.map((e) => ({
      id: parseInt(e.id.replace(/-/g, "").slice(0, 8), 16), // Simple ID transformation
      date: new Date(e.date).toLocaleDateString("de-DE"),
      category: e.category,
      amount: parseFloat(e.amount.toString()),
      description: e.description,
    }))
  );
</script>

<svelte:head>
  <title>Ausgaben Übersicht - FinanceVault</title>
</svelte:head>

<div class="container mx-auto py-6">
  {#if loading}
    <div class="flex items-center justify-center h-64">
      <p>Lade Ausgaben...</p>
    </div>
  {:else}
    <Expenses
      expenses={transformedExpenses}
      onAddExpense={handleAddExpense}
      onUploadFile={handleUploadFile}
      onEditExpense={(id) => {
        const expense =
          expenses[transformedExpenses.findIndex((e) => e.id === id)];
        if (expense) handleEditExpense(expense.id);
      }}
      onDeleteExpense={(id) => {
        const expense =
          expenses[transformedExpenses.findIndex((e) => e.id === id)];
        if (expense) handleDeleteExpense(expense.id);
      }}
    />
  {/if}
</div>

<!-- Add Expense Dialog -->
<Dialog.Root bind:open={showAddDialog}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Neue Ausgabe hinzufügen</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Label for="description">Beschreibung</Label>
        <Input
          id="description"
          bind:value={formData.description}
          placeholder="z.B. Einkauf bei Rewe"
        />
      </div>
      <div class="grid gap-2">
        <Label for="amount">Betrag (€)</Label>
        <Input
          id="amount"
          type="number"
          step="0.01"
          bind:value={formData.amount}
          placeholder="0.00"
        />
      </div>
      <div class="grid gap-2">
        <Label for="date">Datum</Label>
        <Input id="date" type="datetime-local" bind:value={formData.date} />
      </div>
      <div class="grid gap-2">
        <Label for="category">Kategorie</Label>
        <Select.Root>
          <Select.Trigger>
            <Select.Value placeholder="Kategorie wählen" />
          </Select.Trigger>
          <Select.Content>
            {#each categories as category}
              <Select.Item
                value={category}
                onclick={() => (formData.category = category)}
              >
                {category}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showAddDialog = false)}>
        Abbrechen
      </Button>
      <Button onclick={submitAddExpense}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Edit Expense Dialog -->
<Dialog.Root bind:open={showEditDialog}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Ausgabe bearbeiten</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Label for="edit-description">Beschreibung</Label>
        <Input id="edit-description" bind:value={formData.description} />
      </div>
      <div class="grid gap-2">
        <Label for="edit-amount">Betrag (€)</Label>
        <Input
          id="edit-amount"
          type="number"
          step="0.01"
          bind:value={formData.amount}
        />
      </div>
      <div class="grid gap-2">
        <Label for="edit-date">Datum</Label>
        <Input
          id="edit-date"
          type="datetime-local"
          bind:value={formData.date}
        />
      </div>
      <div class="grid gap-2">
        <Label for="edit-category">Kategorie</Label>
        <Select.Root>
          <Select.Trigger>
            <Select.Value placeholder={formData.category} />
          </Select.Trigger>
          <Select.Content>
            {#each categories as category}
              <Select.Item
                value={category}
                onclick={() => (formData.category = category)}
              >
                {category}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showEditDialog = false)}>
        Abbrechen
      </Button>
      <Button onclick={submitEditExpense}>Aktualisieren</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
