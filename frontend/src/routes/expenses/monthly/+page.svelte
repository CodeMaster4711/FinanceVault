<script lang="ts">
  import { onMount } from "svelte";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import { CalendarDate } from "@internationalized/date";
  import Overview from "$lib/components/expenses/overview.svelte";
  import Expenses from "$lib/components/expenses/expenses.svelte";
  import ExpensesWithFilters from "$lib/components/expenses/expenses-with-filters.svelte";
  import Subscriptions from "$lib/components/expenses/subscritpions.svelte";
  import { ExpenseService, type Expense } from "$lib/services/expenses";
  import { SubscriptionService, type Subscription } from "$lib/services/subscriptions";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Select from "$lib/components/ui/select";
  import { toast } from "svelte-sonner";

  let loading = $state(true);
  let allExpenses: Expense[] = $state([]);
  let allSubscriptions: Subscription[] = $state([]);

  let selectedMonth = $state(new Date().getMonth() + 1);
  let selectedYear = $state(new Date().getFullYear());
  let selectedCategory = $state("all");
  let amountSearch = $state("");

  let showAddExpenseDialog = $state(false);
  let showEditExpenseDialog = $state(false);
  let editingExpense: Expense | null = $state(null);

  let showAddSubscriptionDialog = $state(false);
  let showEditSubscriptionDialog = $state(false);
  let editingSubscription: Subscription | null = $state(null);

  let expenseFormData = $state({
    title: "",
    amount: 0,
    date: "",
    category: "Lebensmittel",
  });

  let subscriptionFormData = $state({
    name: "",
    amount: 0,
    billing: "monthly",
    next_billing: "",
  });

  const expenseCategories = [
    "Lebensmittel",
    "Transport",
    "Unterhaltung",
    "Gesundheit",
    "Bildung",
    "Wohnen",
    "Sonstiges",
  ];

  const billingOptions = [
    { value: "monthly", label: "Monatlich" },
    { value: "yearly", label: "Jährlich" },
  ];

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    loading = true;
    try {
      [allExpenses, allSubscriptions] = await Promise.all([
        ExpenseService.getAll(),
        SubscriptionService.getAll(),
      ]);
    } catch {
      toast.error("Fehler beim Laden der Daten");
    } finally {
      loading = false;
    }
  }

  let expenses = $derived(
    allExpenses
      .filter((exp) => {
        const d = new Date(exp.date);
        const monthOk = d.getMonth() + 1 === selectedMonth;
        const yearOk = d.getFullYear() === selectedYear;
        const catOk = selectedCategory === "all" || exp.category === selectedCategory;
        let amtOk = true;
        if (amountSearch.trim()) {
          const n = parseFloat(amountSearch);
          if (!isNaN(n)) amtOk = Math.abs(exp.amount - n) < 0.01;
        }
        return monthOk && yearOk && catOk && amtOk;
      })
      .map((e) => ({
        id: e.id,
        date: new Date(e.date).toLocaleDateString("de-DE"),
        category: e.category,
        amount: e.amount,
        description: e.title,
      }))
  );

  let subscriptions = $derived(
    allSubscriptions.map((s) => ({
      id: s.id,
      name: s.name,
      amount: s.amount,
      billingDay: new Date(s.next_billing).getDate(),
      category: s.billing,
    }))
  );

  const now = new Date();
  let selectedDate = $state(
    new CalendarDate(now.getFullYear(), now.getMonth() + 1, now.getDate())
  );

  let totalExpenses = $derived(expenses.reduce((s, e) => s + e.amount, 0));
  let totalSubscriptions = $derived(
    allSubscriptions.reduce((s, sub) => {
      return s + (sub.billing === "yearly" ? sub.amount / 12 : sub.amount);
    }, 0)
  );
  let monthlyTotal = $derived(totalExpenses + totalSubscriptions);

  let categoryTotals = $derived(
    expenses.reduce(
      (acc, e) => {
        acc[e.category] = (acc[e.category] || 0) + e.amount;
        return acc;
      },
      {} as Record<string, number>
    )
  );

  function handleAddExpense() {
    expenseFormData = {
      title: "",
      amount: 0,
      date: new Date().toISOString().slice(0, 10),
      category: "Lebensmittel",
    };
    showAddExpenseDialog = true;
  }

  async function submitAddExpense() {
    try {
      await ExpenseService.create(expenseFormData);
      showAddExpenseDialog = false;
      await loadData();
      toast.success("Ausgabe erstellt");
    } catch {
      toast.error("Fehler beim Erstellen der Ausgabe");
    }
  }

  function handleEditExpense(id: string) {
    const exp = allExpenses.find((e) => e.id === id);
    if (!exp) return;
    editingExpense = exp;
    expenseFormData = {
      title: exp.title,
      amount: exp.amount,
      date: exp.date.slice(0, 10),
      category: exp.category,
    };
    showEditExpenseDialog = true;
  }

  async function submitEditExpense() {
    if (!editingExpense) return;
    try {
      await ExpenseService.update(editingExpense.id, expenseFormData);
      showEditExpenseDialog = false;
      await loadData();
      toast.success("Ausgabe aktualisiert");
    } catch {
      toast.error("Fehler beim Aktualisieren");
    }
  }

  async function handleDeleteExpense(id: string) {
    if (!confirm("Ausgabe wirklich löschen?")) return;
    try {
      await ExpenseService.delete(id);
      await loadData();
      toast.success("Ausgabe gelöscht");
    } catch {
      toast.error("Fehler beim Löschen");
    }
  }

  function handleAddSubscription() {
    subscriptionFormData = {
      name: "",
      amount: 0,
      billing: "monthly",
      next_billing: new Date().toISOString().slice(0, 10),
    };
    showAddSubscriptionDialog = true;
  }

  async function submitAddSubscription() {
    try {
      await SubscriptionService.create(subscriptionFormData);
      showAddSubscriptionDialog = false;
      await loadData();
      toast.success("Abonnement erstellt");
    } catch {
      toast.error("Fehler beim Erstellen des Abonnements");
    }
  }

  function handleEditSubscription(id: string) {
    const sub = allSubscriptions.find((s) => s.id === id);
    if (!sub) return;
    editingSubscription = sub;
    subscriptionFormData = {
      name: sub.name,
      amount: sub.amount,
      billing: sub.billing,
      next_billing: sub.next_billing.slice(0, 10),
    };
    showEditSubscriptionDialog = true;
  }

  async function submitEditSubscription() {
    if (!editingSubscription) return;
    try {
      await SubscriptionService.update(editingSubscription.id, subscriptionFormData);
      showEditSubscriptionDialog = false;
      await loadData();
      toast.success("Abonnement aktualisiert");
    } catch {
      toast.error("Fehler beim Aktualisieren");
    }
  }

  async function handleDeleteSubscription(id: string) {
    if (!confirm("Abonnement wirklich löschen?")) return;
    try {
      await SubscriptionService.delete(id);
      await loadData();
      toast.success("Abonnement gelöscht");
    } catch {
      toast.error("Fehler beim Löschen");
    }
  }
</script>

<div class="h-screen w-full p-6 overflow-auto">
  {#if loading}
    <div class="flex h-full items-center justify-center">
      <span class="text-muted-foreground text-sm">Laden...</span>
    </div>
  {:else}
    <Tabs.Root value="overview" class="w-full h-full flex flex-col">
      <Tabs.List class="mb-4">
        <Tabs.Trigger value="overview">Overview</Tabs.Trigger>
        <Tabs.Trigger value="expenses">Expenses</Tabs.Trigger>
        <Tabs.Trigger value="subscription">Subscriptions</Tabs.Trigger>
      </Tabs.List>

      <Tabs.Content value="overview" class="flex-1">
        <Overview
          {totalExpenses}
          {totalSubscriptions}
          {monthlyTotal}
          {categoryTotals}
          {selectedMonth}
          {selectedYear}
          {expenses}
        />
      </Tabs.Content>

      <Tabs.Content value="expenses" class="flex-1">
        <ExpensesWithFilters
          {expenses}
          {selectedMonth}
          {selectedYear}
          {selectedCategory}
          {amountSearch}
          onAddExpense={handleAddExpense}
          onUploadFile={() => toast.info("PDF-Import kommt bald")}
          onEditExpense={handleEditExpense}
          onDeleteExpense={handleDeleteExpense}
          onMonthChange={(m) => (selectedMonth = m)}
          onYearChange={(y) => (selectedYear = y)}
          onCategoryChange={(c) => (selectedCategory = c)}
          onAmountSearchChange={(a) => (amountSearch = a)}
        />
      </Tabs.Content>

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
  {/if}
</div>

<Dialog.Root bind:open={showAddExpenseDialog}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Neue Ausgabe</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Label for="title">Beschreibung</Label>
        <Input id="title" bind:value={expenseFormData.title} placeholder="z.B. Einkauf bei Rewe" />
      </div>
      <div class="grid gap-2">
        <Label for="amount">Betrag (€)</Label>
        <Input id="amount" type="number" step="0.01" bind:value={expenseFormData.amount} placeholder="0.00" />
      </div>
      <div class="grid gap-2">
        <Label for="date">Datum</Label>
        <Input id="date" type="date" bind:value={expenseFormData.date} />
      </div>
      <div class="grid gap-2">
        <Label>Kategorie</Label>
        <Select.Root type="single" bind:value={expenseFormData.category}>
          <Select.Trigger>{expenseFormData.category}</Select.Trigger>
          <Select.Content>
            {#each expenseCategories as cat}
              <Select.Item value={cat}>{cat}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showAddExpenseDialog = false)}>Abbrechen</Button>
      <Button onclick={submitAddExpense}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={showEditExpenseDialog}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Ausgabe bearbeiten</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Label for="edit-title">Beschreibung</Label>
        <Input id="edit-title" bind:value={expenseFormData.title} />
      </div>
      <div class="grid gap-2">
        <Label for="edit-amount">Betrag (€)</Label>
        <Input id="edit-amount" type="number" step="0.01" bind:value={expenseFormData.amount} />
      </div>
      <div class="grid gap-2">
        <Label for="edit-date">Datum</Label>
        <Input id="edit-date" type="date" bind:value={expenseFormData.date} />
      </div>
      <div class="grid gap-2">
        <Label>Kategorie</Label>
        <Select.Root type="single" bind:value={expenseFormData.category}>
          <Select.Trigger>{expenseFormData.category}</Select.Trigger>
          <Select.Content>
            {#each expenseCategories as cat}
              <Select.Item value={cat}>{cat}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showEditExpenseDialog = false)}>Abbrechen</Button>
      <Button onclick={submitEditExpense}>Aktualisieren</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={showAddSubscriptionDialog}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Neues Abonnement</Dialog.Title>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2">
        <Label for="sub-name">Name</Label>
        <Input id="sub-name" bind:value={subscriptionFormData.name} placeholder="z.B. Netflix" />
      </div>
      <div class="grid gap-2">
        <Label for="sub-amount">Betrag (€)</Label>
        <Input id="sub-amount" type="number" step="0.01" bind:value={subscriptionFormData.amount} placeholder="0.00" />
      </div>
      <div class="grid gap-2">
        <Label>Abrechnungszyklus</Label>
        <Select.Root type="single" bind:value={subscriptionFormData.billing}>
          <Select.Trigger>{billingOptions.find(b => b.value === subscriptionFormData.billing)?.label ?? subscriptionFormData.billing}</Select.Trigger>
          <Select.Content>
            {#each billingOptions as opt}
              <Select.Item value={opt.value}>{opt.label}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div class="grid gap-2">
        <Label for="sub-next">Nächste Abbuchung</Label>
        <Input id="sub-next" type="date" bind:value={subscriptionFormData.next_billing} />
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showAddSubscriptionDialog = false)}>Abbrechen</Button>
      <Button onclick={submitAddSubscription}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

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
        <Input id="edit-sub-amount" type="number" step="0.01" bind:value={subscriptionFormData.amount} />
      </div>
      <div class="grid gap-2">
        <Label>Abrechnungszyklus</Label>
        <Select.Root type="single" bind:value={subscriptionFormData.billing}>
          <Select.Trigger>{billingOptions.find(b => b.value === subscriptionFormData.billing)?.label ?? subscriptionFormData.billing}</Select.Trigger>
          <Select.Content>
            {#each billingOptions as opt}
              <Select.Item value={opt.value}>{opt.label}</Select.Item>
            {/each}
          </Select.Content>
        </Select.Root>
      </div>
      <div class="grid gap-2">
        <Label for="edit-sub-next">Nächste Abbuchung</Label>
        <Input id="edit-sub-next" type="date" bind:value={subscriptionFormData.next_billing} />
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showEditSubscriptionDialog = false)}>Abbrechen</Button>
      <Button onclick={submitEditSubscription}>Aktualisieren</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
