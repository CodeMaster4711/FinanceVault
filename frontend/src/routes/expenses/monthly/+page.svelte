<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import { CalendarDate } from "@internationalized/date";
  import Overview from "$lib/components/expenses/overview.svelte";
  import Expenses from "$lib/components/expenses/expenses.svelte";
  import Subscriptions from "$lib/components/expenses/subscritpions.svelte";

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

  // Mock-Daten für Expenses
  let expenses = $state<Expense[]>([
    {
      id: 1,
      date: "2025-10-15",
      category: "Lebensmittel",
      amount: 125.5,
      description: "Supermarkt",
    },
    {
      id: 2,
      date: "2025-10-20",
      category: "Transport",
      amount: 45.0,
      description: "Tankstelle",
    },
    {
      id: 3,
      date: "2025-10-22",
      category: "Unterhaltung",
      amount: 89.99,
      description: "Kino & Essen",
    },
    {
      id: 4,
      date: "2025-10-25",
      category: "Sonstiges",
      amount: 34.5,
      description: "Amazon",
    },
  ]);

  // Mock-Daten für Subscriptions
  let subscriptions = $state<Subscription[]>([
    {
      id: 1,
      name: "Netflix",
      amount: 17.99,
      billingDay: 5,
      category: "Unterhaltung",
    },
    {
      id: 2,
      name: "Spotify",
      amount: 10.99,
      billingDay: 12,
      category: "Musik",
    },
    {
      id: 3,
      name: "Amazon Prime",
      amount: 8.99,
      billingDay: 18,
      category: "Shopping",
    },
    { id: 4, name: "Gym", amount: 49.9, billingDay: 1, category: "Fitness" },
  ]);

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
    subscriptions.reduce((sum, sub) => sum + sub.amount, 0)
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

  function handleAddExpense() {
    console.log("Add expense triggered");
  }

  function handleUploadFile(type: "image" | "pdf") {
    console.log(`Upload ${type} triggered`);
  }

  function handleEditExpense(id: number) {
    console.log("Edit expense:", id);
  }

  function handleDeleteExpense(id: number) {
    expenses = expenses.filter((e) => e.id !== id);
  }

  function handleAddSubscription() {
    console.log("Add subscription triggered");
  }

  function handleEditSubscription(id: number) {
    console.log("Edit subscription:", id);
  }

  function handleDeleteSubscription(id: number) {
    subscriptions = subscriptions.filter((s) => s.id !== id);
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
