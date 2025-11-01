<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { browser } from "$app/environment";
  import { authStore } from "$lib/stores/auth";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import { CalendarDate } from "@internationalized/date";
  import Overview from "$lib/components/expenses/overview.svelte";
  import Expenses from "$lib/components/expenses/expenses.svelte";
  import Subscriptions from "$lib/components/expenses/subscritpions.svelte";
  import {
    ExpenseService,
    type Expense as ExpenseType,
  } from "$lib/services/expenses";
  import {
    SubscriptionService,
    type Subscription as SubscriptionType,
  } from "$lib/services/subscriptions";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Select from "$lib/components/ui/select";
  import { toast } from "svelte-sonner";

  type Expense = {
    id: number;
    date: string;
    category: string;
    amount: number;
    description: string;
  };

  type Subscription = {
    id: number;
    name: string;
    amount: number;
    billingDay: number;
    category: string;
  };

  let loading = $state(true);
  let allExpenses: ExpenseType[] = $state([]);
  let allSubscriptions: SubscriptionType[] = $state([]);

  // Dialog states
  let showAddExpenseDialog = $state(false);
  let showEditExpenseDialog = $state(false);
  let editingExpense: ExpenseType | null = $state(null);

  let showAddSubscriptionDialog = $state(false);
  let showEditSubscriptionDialog = $state(false);
  let editingSubscription: SubscriptionType | null = $state(null);

  // Form states
  let expenseFormData = $state({
    description: "",
    amount: 0,
    date: "",
    category: "Lebensmittel",
  });

  let subscriptionFormData = $state({
    name: "",
    amount: 0,
    billing_cycle: "monthly",
    next_billing_date: "",
    category: "Streaming",
  });

  const expenseCategories = [
    "Lebensmittel",
    "Transport",
    "Unterhaltung",
    "Gesundheit",
    "Bildung",
    "Sonstiges",
  ];

  const subscriptionCategories = [
    "Streaming",
    "Software",
    "Cloud Services",
    "Fitness",
    "Zeitschriften",
    "Sonstiges",
  ];

  const billingCycles = [
    { value: "monthly", label: "Monatlich" },
    { value: "yearly", label: "Jährlich" },
    { value: "quarterly", label: "Quartalsweise" },
    { value: "weekly", label: "Wöchentlich" },
  ];

  onMount(async () => {
    if (browser) {
      // Check authentication
      if (!$authStore.isAuthenticated) {
        goto("/signin");
        return;
      }

      const token = $authStore.token;
      if (!token) {
        goto("/signin");
        return;
      }

      await loadData();
    }
  });

  async function loadData() {
    const token = $authStore.token;
    if (!token) return;

    try {
      loading = true;
      const [expensesData, subscriptionsData] = await Promise.all([
        ExpenseService.getExpenses(token),
        SubscriptionService.getSubscriptions(token),
      ]);
      allExpenses = expensesData;
      allSubscriptions = subscriptionsData;
    } catch (error) {
      console.error("Failed to load data:", error);
      toast.error("Fehler beim Laden der Daten");
    } finally {
      loading = false;
    }
  }

  // Filter expenses for current month
  let expenses = $derived(
    allExpenses
      .filter((exp) => {
        const expDate = new Date(exp.date);
        const now = new Date();
        return (
          expDate.getMonth() === now.getMonth() &&
          expDate.getFullYear() === now.getFullYear()
        );
      })
      .map((e) => ({
        id: parseInt(e.id.replace(/-/g, "").slice(0, 8), 16),
        date: new Date(e.date).toLocaleDateString("de-DE"),
        category: e.category,
        amount: parseFloat(e.amount.toString()),
        description: e.description,
      }))
  );

  // Transform subscriptions
  let subscriptions = $derived(
    allSubscriptions.map((s) => {
      const nextDate = new Date(s.next_billing_date);
      return {
        id: parseInt(s.id.replace(/-/g, "").slice(0, 8), 16),
        name: s.name,
        amount: parseFloat(s.amount.toString()),
        billingDay: nextDate.getDate(),
        category: s.category,
      };
    })
  );

  // Kalender-Konfiguration
  const now = new Date();
  let selectedDate = $state(
    new CalendarDate(now.getFullYear(), now.getMonth() + 1, now.getDate())
  );
  let markedDates = $derived(subscriptions.map((sub) => sub.billingDay));

  // Statistiken berechnen
  let totalExpenses = $derived(
    expenses.reduce((sum, exp) => sum + exp.amount, 0)
  );

  let totalSubscriptions = $derived(
    allSubscriptions.reduce((sum, s) => {
      const amount = parseFloat(s.amount.toString());
      switch (s.billing_cycle) {
        case "yearly":
          return sum + amount / 12;
        case "quarterly":
          return sum + amount / 3;
        case "weekly":
          return sum + amount * 4;
        default:
          return sum + amount;
      }
    }, 0)
  );

  let monthlyTotal = $derived(totalExpenses + totalSubscriptions);

  // Kategorien für Diagramm
  let categoryTotals = $derived(
    expenses.reduce(
      (acc, exp) => {
        acc[exp.category] = (acc[exp.category] || 0) + exp.amount;
        return acc;
      },
      {} as Record<string, number>
    )
  );

  // Expense handlers
  function handleAddExpense() {
    expenseFormData = {
      description: "",
      amount: 0,
      date: new Date().toISOString().slice(0, 19).replace("T", " "),
      category: "Lebensmittel",
    };
    showAddExpenseDialog = true;
  }

  async function submitAddExpense() {
    const token = $authStore.token;
    if (!token) return;

    try {
      await ExpenseService.createExpense(expenseFormData, token);
      showAddExpenseDialog = false;
      await loadData();
      toast.success("Ausgabe erfolgreich erstellt");
    } catch (error) {
      console.error("Failed to create expense:", error);
      toast.error("Fehler beim Erstellen der Ausgabe");
    }
  }

  function handleEditExpense(id: number) {
    const expense = allExpenses[expenses.findIndex((e) => e.id === id)];
    if (expense) {
      editingExpense = expense;
      expenseFormData = {
        description: expense.description,
        amount: parseFloat(expense.amount.toString()),
        date: expense.date,
        category: expense.category,
      };
      showEditExpenseDialog = true;
    }
  }

  async function submitEditExpense() {
    if (!editingExpense) return;

    const token = $authStore.token;
    if (!token) return;

    try {
      await ExpenseService.updateExpense(
        editingExpense.id,
        expenseFormData,
        token
      );
      showEditExpenseDialog = false;
      await loadData();
      toast.success("Ausgabe erfolgreich aktualisiert");
    } catch (error) {
      console.error("Failed to update expense:", error);
      toast.error("Fehler beim Aktualisieren der Ausgabe");
    }
  }

  async function handleDeleteExpense(id: number) {
    const expense = allExpenses[expenses.findIndex((e) => e.id === id)];
    if (!expense) return;

    if (!confirm("Möchten Sie diese Ausgabe wirklich löschen?")) return;

    const token = $authStore.token;
    if (!token) return;

    try {
      await ExpenseService.deleteExpense(expense.id, token);
      await loadData();
      toast.success("Ausgabe erfolgreich gelöscht");
    } catch (error) {
      console.error("Failed to delete expense:", error);
      toast.error("Fehler beim Löschen der Ausgabe");
    }
  }

  function handleUploadFile(type: "image" | "pdf") {
    toast.info(`${type === "image" ? "Bild" : "PDF"}-Upload kommt bald!`);
  }

  // Subscription handlers
  function handleAddSubscription() {
    const now = new Date();
    subscriptionFormData = {
      name: "",
      amount: 0,
      billing_cycle: "monthly",
      next_billing_date: now.toISOString().slice(0, 19).replace("T", " "),
      category: "Streaming",
    };
    showAddSubscriptionDialog = true;
  }

  async function submitAddSubscription() {
    const token = $authStore.token;
    if (!token) return;

    try {
      await SubscriptionService.createSubscription(subscriptionFormData, token);
      showAddSubscriptionDialog = false;
      await loadData();
      toast.success("Abonnement erfolgreich erstellt");
    } catch (error) {
      console.error("Failed to create subscription:", error);
      toast.error("Fehler beim Erstellen des Abonnements");
    }
  }

  function handleEditSubscription(id: number) {
    const subscription =
      allSubscriptions[subscriptions.findIndex((s) => s.id === id)];
    if (subscription) {
      editingSubscription = subscription;
      subscriptionFormData = {
        name: subscription.name,
        amount: parseFloat(subscription.amount.toString()),
        billing_cycle: subscription.billing_cycle,
        next_billing_date: subscription.next_billing_date,
        category: subscription.category,
      };
      showEditSubscriptionDialog = true;
    }
  }

  async function submitEditSubscription() {
    if (!editingSubscription) return;

    const token = $authStore.token;
    if (!token) return;

    try {
      await SubscriptionService.updateSubscription(
        editingSubscription.id,
        subscriptionFormData,
        token
      );
      showEditSubscriptionDialog = false;
      await loadData();
      toast.success("Abonnement erfolgreich aktualisiert");
    } catch (error) {
      console.error("Failed to update subscription:", error);
      toast.error("Fehler beim Aktualisieren des Abonnements");
    }
  }

  async function handleDeleteSubscription(id: number) {
    const subscription =
      allSubscriptions[subscriptions.findIndex((s) => s.id === id)];
    if (!subscription) return;

    if (!confirm("Möchten Sie dieses Abonnement wirklich löschen?")) return;

    const token = $authStore.token;
    if (!token) return;

    try {
      await SubscriptionService.deleteSubscription(subscription.id, token);
      await loadData();
      toast.success("Abonnement erfolgreich gelöscht");
    } catch (error) {
      console.error("Failed to delete subscription:", error);
      toast.error("Fehler beim Löschen des Abonnements");
    }
  }
</script>

<div class="h-screen w-full p-6 overflow-auto">
  <Tabs.Root value="overview" class="w-full h-full flex flex-col">
    <Tabs.List class="mb-4">
      <Tabs.Trigger value="overview">Overview</Tabs.Trigger>
      <Tabs.Trigger value="expenses">Expenses</Tabs.Trigger>
      <Tabs.Trigger value="subscription">Subscriptions</Tabs.Trigger>
    </Tabs.List>

    <!-- Overview Tab -->
    <Tabs.Content value="overview" class="flex-1">
      <Overview
        {totalExpenses}
        {totalSubscriptions}
        {monthlyTotal}
        {categoryTotals}
        {expenses}
      />
    </Tabs.Content>

    <!-- Expenses Tab -->
    <Tabs.Content value="expenses" class="flex-1">
      <Expenses
        {expenses}
        onAddExpense={handleAddExpense}
        onUploadFile={handleUploadFile}
        onEditExpense={handleEditExpense}
        onDeleteExpense={handleDeleteExpense}
      />
    </Tabs.Content>

    <!-- Subscriptions Tab -->
    <Tabs.Content value="subscription" class="flex-1">
      <Subscriptions
        {subscriptions}
        {selectedDate}
        {totalSubscriptions}
        onAddSubscription={handleAddSubscription}
        onEditSubscription={handleEditSubscription}
        onDeleteSubscription={handleDeleteSubscription}
      />
    </Tabs.Content>
  </Tabs.Root>
</div>

<!-- Add Expense Dialog -->
<Dialog.Root bind:open={showAddExpenseDialog}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Neue Ausgabe hinzufügen</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Label for="description">Beschreibung</Label>
        <Input
          id="description"
          bind:value={expenseFormData.description}
          placeholder="z.B. Einkauf bei Rewe"
        />
      </div>
      <div class="grid gap-2">
        <Label for="amount">Betrag (€)</Label>
        <Input
          id="amount"
          type="number"
          step="0.01"
          bind:value={expenseFormData.amount}
          placeholder="0.00"
        />
      </div>
      <div class="grid gap-2">
        <Label for="date">Datum</Label>
        <Input
          id="date"
          type="datetime-local"
          bind:value={expenseFormData.date}
        />
      </div>
      <div class="grid gap-2">
        <Label for="category">Kategorie</Label>
        <Select.Root>
          <Select.Trigger>
            <Select.Value placeholder="Kategorie wählen" />
          </Select.Trigger>
          <Select.Content>
            {#each expenseCategories as category}
              <Select.Item
                value={category}
                onclick={() => (expenseFormData.category = category)}
              >
                {category}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showAddExpenseDialog = false)}>
        Abbrechen
      </Button>
      <Button onclick={submitAddExpense}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Edit Expense Dialog -->
<Dialog.Root bind:open={showEditExpenseDialog}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Ausgabe bearbeiten</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Label for="edit-description">Beschreibung</Label>
        <Input id="edit-description" bind:value={expenseFormData.description} />
      </div>
      <div class="grid gap-2">
        <Label for="edit-amount">Betrag (€)</Label>
        <Input
          id="edit-amount"
          type="number"
          step="0.01"
          bind:value={expenseFormData.amount}
        />
      </div>
      <div class="grid gap-2">
        <Label for="edit-date">Datum</Label>
        <Input
          id="edit-date"
          type="datetime-local"
          bind:value={expenseFormData.date}
        />
      </div>
      <div class="grid gap-2">
        <Label for="edit-category">Kategorie</Label>
        <Select.Root>
          <Select.Trigger>
            <Select.Value placeholder={expenseFormData.category} />
          </Select.Trigger>
          <Select.Content>
            {#each expenseCategories as category}
              <Select.Item
                value={category}
                onclick={() => (expenseFormData.category = category)}
              >
                {category}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showEditExpenseDialog = false)}>
        Abbrechen
      </Button>
      <Button onclick={submitEditExpense}>Aktualisieren</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Add Subscription Dialog -->
<Dialog.Root bind:open={showAddSubscriptionDialog}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Neues Abonnement hinzufügen</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Label for="sub-name">Name</Label>
        <Input
          id="sub-name"
          bind:value={subscriptionFormData.name}
          placeholder="z.B. Netflix"
        />
      </div>
      <div class="grid gap-2">
        <Label for="sub-amount">Betrag (€)</Label>
        <Input
          id="sub-amount"
          type="number"
          step="0.01"
          bind:value={subscriptionFormData.amount}
          placeholder="0.00"
        />
      </div>
      <div class="grid gap-2">
        <Label for="sub-billing_cycle">Abrechnungszyklus</Label>
        <Select.Root>
          <Select.Trigger>
            <Select.Value placeholder="Abrechnungszyklus wählen" />
          </Select.Trigger>
          <Select.Content>
            {#each billingCycles as cycle}
              <Select.Item
                value={cycle.value}
                onclick={() =>
                  (subscriptionFormData.billing_cycle = cycle.value)}
              >
                {cycle.label}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div class="grid gap-2">
        <Label for="sub-next_billing_date">Nächste Abbuchung</Label>
        <Input
          id="sub-next_billing_date"
          type="datetime-local"
          bind:value={subscriptionFormData.next_billing_date}
        />
      </div>
      <div class="grid gap-2">
        <Label for="sub-category">Kategorie</Label>
        <Select.Root>
          <Select.Trigger>
            <Select.Value placeholder="Kategorie wählen" />
          </Select.Trigger>
          <Select.Content>
            {#each subscriptionCategories as category}
              <Select.Item
                value={category}
                onclick={() => (subscriptionFormData.category = category)}
              >
                {category}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button
        variant="outline"
        onclick={() => (showAddSubscriptionDialog = false)}
      >
        Abbrechen
      </Button>
      <Button onclick={submitAddSubscription}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<!-- Edit Subscription Dialog -->
<Dialog.Root bind:open={showEditSubscriptionDialog}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Abonnement bearbeiten</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Label for="edit-sub-name">Name</Label>
        <Input id="edit-sub-name" bind:value={subscriptionFormData.name} />
      </div>
      <div class="grid gap-2">
        <Label for="edit-sub-amount">Betrag (€)</Label>
        <Input
          id="edit-sub-amount"
          type="number"
          step="0.01"
          bind:value={subscriptionFormData.amount}
        />
      </div>
      <div class="grid gap-2">
        <Label for="edit-sub-billing_cycle">Abrechnungszyklus</Label>
        <Select.Root>
          <Select.Trigger>
            <Select.Value placeholder={subscriptionFormData.billing_cycle} />
          </Select.Trigger>
          <Select.Content>
            {#each billingCycles as cycle}
              <Select.Item
                value={cycle.value}
                onclick={() =>
                  (subscriptionFormData.billing_cycle = cycle.value)}
              >
                {cycle.label}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div class="grid gap-2">
        <Label for="edit-sub-next_billing_date">Nächste Abbuchung</Label>
        <Input
          id="edit-sub-next_billing_date"
          type="datetime-local"
          bind:value={subscriptionFormData.next_billing_date}
        />
      </div>
      <div class="grid gap-2">
        <Label for="edit-sub-category">Kategorie</Label>
        <Select.Root>
          <Select.Trigger>
            <Select.Value placeholder={subscriptionFormData.category} />
          </Select.Trigger>
          <Select.Content>
            {#each subscriptionCategories as category}
              <Select.Item
                value={category}
                onclick={() => (subscriptionFormData.category = category)}
              >
                {category}
              </Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button
        variant="outline"
        onclick={() => (showEditSubscriptionDialog = false)}
      >
        Abbrechen
      </Button>
      <Button onclick={submitEditSubscription}>Aktualisieren</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
