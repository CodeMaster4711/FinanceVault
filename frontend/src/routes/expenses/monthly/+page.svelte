<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Calendar } from "$lib/components/ui/calendar/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import {
    createSvelteTable,
    FlexRender,
  } from "$lib/components/ui/data-table/index.js";
  import { renderComponent } from "$lib/components/ui/data-table/render-helpers.js";
  import {
    getCoreRowModel,
    getSortedRowModel,
    getFilteredRowModel,
    type ColumnDef,
    type SortingState,
    type ColumnFiltersState,
  } from "@tanstack/table-core";
  import {
    Plus,
    Upload,
    FileText,
    Image,
    MoreHorizontal,
    ArrowUpDown,
  } from "@lucide/svelte";
  import DataTableActions from "$lib/components/expenses/data-table-actions.svelte";
  import BadgeCell from "$lib/components/expenses/badge-cell.svelte";
  import { CalendarDate } from "@internationalized/date";

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

  // Table state
  let sorting = $state<SortingState>([]);
  let columnFilters = $state<ColumnFiltersState>([]);

  // Expense Columns
  const expenseColumns: ColumnDef<Expense>[] = [
    {
      accessorKey: "date",
      header: "Datum",
      cell: ({ row }) => row.getValue("date"),
    },
    {
      accessorKey: "category",
      header: "Kategorie",
      cell: ({ row }) => {
        const category = row.getValue("category") as string;
        return renderComponent(BadgeCell, {
          value: category,
          variant: "outline",
        });
      },
    },
    {
      accessorKey: "description",
      header: "Beschreibung",
      cell: ({ row }) => row.getValue("description"),
    },
    {
      accessorKey: "amount",
      header: "Betrag",
      cell: ({ row }) => {
        const amount = parseFloat(row.getValue("amount"));
        return `€${amount.toFixed(2)}`;
      },
    },
    {
      id: "actions",
      cell: ({ row }) => {
        return renderComponent(DataTableActions, {
          onEdit: () => handleEditExpense(row.original.id),
          onDelete: () => handleDeleteExpense(row.original.id),
        });
      },
    },
  ];

  // Subscription Columns
  const subscriptionColumns: ColumnDef<Subscription>[] = [
    {
      accessorKey: "name",
      header: "Name",
      cell: ({ row }) => row.getValue("name"),
    },
    {
      accessorKey: "category",
      header: "Kategorie",
      cell: ({ row }) => {
        const category = row.getValue("category") as string;
        return renderComponent(BadgeCell, {
          value: category,
          variant: "secondary",
        });
      },
    },
    {
      accessorKey: "billingDay",
      header: "Abbuchungstag",
      cell: ({ row }) => {
        const day = row.getValue("billingDay") as number;
        return renderComponent(BadgeCell, {
          value: `${day}. des Monats`,
          variant: "outline",
        });
      },
    },
    {
      accessorKey: "amount",
      header: "Betrag",
      cell: ({ row }) => {
        const amount = parseFloat(row.getValue("amount"));
        return `€${amount.toFixed(2)}`;
      },
    },
    {
      id: "actions",
      cell: ({ row }) => {
        return renderComponent(DataTableActions, {
          onEdit: () => handleEditSubscription(row.original.id),
          onDelete: () => handleDeleteSubscription(row.original.id),
        });
      },
    },
  ];

  // Create tables
  const expenseTable = createSvelteTable({
    get data() {
      return expenses;
    },
    columns: expenseColumns,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    state: {
      get sorting() {
        return sorting;
      },
      get columnFilters() {
        return columnFilters;
      },
    },
    onSortingChange: (updater) => {
      if (updater instanceof Function) {
        sorting = updater(sorting);
      } else {
        sorting = updater;
      }
    },
    onColumnFiltersChange: (updater) => {
      if (updater instanceof Function) {
        columnFilters = updater(columnFilters);
      } else {
        columnFilters = updater;
      }
    },
  });

  const subscriptionTable = createSvelteTable({
    get data() {
      return subscriptions;
    },
    columns: subscriptionColumns,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    state: {
      get sorting() {
        return sorting;
      },
    },
    onSortingChange: (updater) => {
      if (updater instanceof Function) {
        sorting = updater(sorting);
      } else {
        sorting = updater;
      }
    },
  });

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
    <Tabs.Content value="overview" class="flex-1 space-y-6">
      <div class="grid gap-4 md:grid-cols-3">
        <Card.Root>
          <Card.Header
            class="flex flex-row items-center justify-between space-y-0 pb-2"
          >
            <Card.Title class="text-sm font-medium">Gesamtausgaben</Card.Title>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              class="h-4 w-4 text-muted-foreground"
            >
              <path
                d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6"
              />
            </svg>
          </Card.Header>
          <Card.Content>
            <div class="text-2xl font-bold">€{totalExpenses.toFixed(2)}</div>
            <p class="text-xs text-muted-foreground">Einmalige Ausgaben</p>
          </Card.Content>
        </Card.Root>

        <Card.Root>
          <Card.Header
            class="flex flex-row items-center justify-between space-y-0 pb-2"
          >
            <Card.Title class="text-sm font-medium">Subscriptions</Card.Title>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              class="h-4 w-4 text-muted-foreground"
            >
              <rect width="20" height="14" x="2" y="5" rx="2" />
              <path d="M2 10h20" />
            </svg>
          </Card.Header>
          <Card.Content>
            <div class="text-2xl font-bold">
              €{totalSubscriptions.toFixed(2)}
            </div>
            <p class="text-xs text-muted-foreground">Monatliche Abos</p>
          </Card.Content>
        </Card.Root>

        <Card.Root>
          <Card.Header
            class="flex flex-row items-center justify-between space-y-0 pb-2"
          >
            <Card.Title class="text-sm font-medium">Gesamtsumme</Card.Title>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              class="h-4 w-4 text-muted-foreground"
            >
              <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" />
              <circle cx="9" cy="7" r="4" />
              <path d="M22 21v-2a4 4 0 0 0-3-3.87M16 3.13a4 4 0 0 1 0 7.75" />
            </svg>
          </Card.Header>
          <Card.Content>
            <div class="text-2xl font-bold">€{monthlyTotal.toFixed(2)}</div>
            <p class="text-xs text-muted-foreground">Diesen Monat</p>
          </Card.Content>
        </Card.Root>
      </div>

      <div class="grid gap-4 md:grid-cols-2">
        <Card.Root>
          <Card.Header>
            <Card.Title>Ausgaben nach Kategorie</Card.Title>
          </Card.Header>
          <Card.Content>
            <div class="space-y-4">
              {#each Object.entries(categoryTotals) as [category, amount]}
                <div class="flex items-center">
                  <div class="flex-1">
                    <p class="text-sm font-medium">{category}</p>
                    <div
                      class="mt-1 h-2 w-full bg-secondary rounded-full overflow-hidden"
                    >
                      <div
                        class="h-full bg-primary transition-all"
                        style="width: {(amount / totalExpenses) * 100}%"
                      ></div>
                    </div>
                  </div>
                  <div class="ml-4 text-sm font-medium">
                    €{amount.toFixed(2)}
                  </div>
                </div>
              {/each}
            </div>
          </Card.Content>
        </Card.Root>

        <Card.Root>
          <Card.Header>
            <Card.Title>Neueste Ausgaben</Card.Title>
          </Card.Header>
          <Card.Content>
            <div class="space-y-3">
              {#each expenses.slice(0, 4) as expense}
                <div class="flex items-center justify-between">
                  <div>
                    <p class="text-sm font-medium">{expense.description}</p>
                    <p class="text-xs text-muted-foreground">{expense.date}</p>
                  </div>
                  <Badge variant="outline">{expense.category}</Badge>
                  <span class="text-sm font-medium"
                    >€{expense.amount.toFixed(2)}</span
                  >
                </div>
              {/each}
            </div>
          </Card.Content>
        </Card.Root>
      </div>
    </Tabs.Content>

    <!-- Expenses Tab -->
    <Tabs.Content value="expenses" class="flex-1">
      <Card.Root class="h-full">
        <Card.Header>
          <div class="flex items-center justify-between">
            <Card.Title>Alle Ausgaben</Card.Title>
            <div class="flex gap-2">
              <Button
                size="sm"
                variant="outline"
                onclick={() => handleUploadFile("image")}
              >
                <Image class="mr-2 h-4 w-4" />
                Bild hochladen
              </Button>
              <Button
                size="sm"
                variant="outline"
                onclick={() => handleUploadFile("pdf")}
              >
                <FileText class="mr-2 h-4 w-4" />
                PDF hochladen
              </Button>
              <Button size="sm" onclick={handleAddExpense}>
                <Plus class="mr-2 h-4 w-4" />
                Neue Ausgabe
              </Button>
            </div>
          </div>
        </Card.Header>
        <Card.Content>
          <div class="rounded-md border">
            <table class="w-full">
              <thead class="border-b">
                {#each expenseTable.getHeaderGroups() as headerGroup}
                  <tr>
                    {#each headerGroup.headers as header}
                      <th
                        class="h-12 px-4 text-left align-middle font-medium text-muted-foreground"
                      >
                        {#if !header.isPlaceholder}
                          <FlexRender
                            content={header.column.columnDef.header}
                            context={header.getContext()}
                          />
                        {/if}
                      </th>
                    {/each}
                  </tr>
                {/each}
              </thead>
              <tbody>
                {#each expenseTable.getRowModel().rows as row}
                  <tr class="border-b transition-colors hover:bg-muted/50">
                    {#each row.getVisibleCells() as cell}
                      <td class="p-4 align-middle">
                        <FlexRender
                          content={cell.column.columnDef.cell}
                          context={cell.getContext()}
                        />
                      </td>
                    {/each}
                  </tr>
                {:else}
                  <tr>
                    <td
                      colspan={expenseColumns.length}
                      class="h-24 text-center"
                    >
                      Keine Ausgaben vorhanden.
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </Card.Content>
      </Card.Root>
    </Tabs.Content>

    <!-- Subscriptions Tab -->
    <Tabs.Content value="subscription" class="flex-1">
      <div class="grid grid-cols-[300px_1fr] gap-4 h-full">
        <!-- Kalender -->
        <Card.Root class="h-fit">
          <Card.Header>
            <Card.Title>Abbuchungskalender</Card.Title>
          </Card.Header>
          <Card.Content>
            <Calendar bind:value={selectedDate} class="rounded-md border" />
            <div class="mt-4 space-y-2">
              <p class="text-sm font-medium">Legende:</p>
              <div
                class="flex items-center gap-2 text-xs text-muted-foreground"
              >
                <div class="h-3 w-3 rounded-full bg-primary"></div>
                <span>Abbuchungstage</span>
              </div>
            </div>
          </Card.Content>
        </Card.Root>

        <!-- Subscriptions Tabelle -->
        <Card.Root>
          <Card.Header>
            <div class="flex items-center justify-between">
              <Card.Title>Meine Abonnements</Card.Title>
              <Button size="sm" onclick={handleAddExpense}>
                <Plus class="mr-2 h-4 w-4" />
                Neues Abo
              </Button>
            </div>
          </Card.Header>
          <Card.Content>
            <div class="rounded-md border">
              <table class="w-full">
                <thead class="border-b">
                  {#each subscriptionTable.getHeaderGroups() as headerGroup}
                    <tr>
                      {#each headerGroup.headers as header}
                        <th
                          class="h-12 px-4 text-left align-middle font-medium text-muted-foreground"
                        >
                          {#if !header.isPlaceholder}
                            <FlexRender
                              content={header.column.columnDef.header}
                              context={header.getContext()}
                            />
                          {/if}
                        </th>
                      {/each}
                    </tr>
                  {/each}
                </thead>
                <tbody>
                  {#each subscriptionTable.getRowModel().rows as row}
                    <tr class="border-b transition-colors hover:bg-muted/50">
                      {#each row.getVisibleCells() as cell}
                        <td class="p-4 align-middle">
                          <FlexRender
                            content={cell.column.columnDef.cell}
                            context={cell.getContext()}
                          />
                        </td>
                      {/each}
                    </tr>
                  {:else}
                    <tr>
                      <td
                        colspan={subscriptionColumns.length}
                        class="h-24 text-center"
                      >
                        Keine Abonnements vorhanden.
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>

            <div class="mt-4 pt-4 border-t">
              <div class="flex justify-between items-center">
                <span class="text-sm font-medium">Monatliche Gesamtsumme:</span>
                <span class="text-lg font-bold"
                  >€{totalSubscriptions.toFixed(2)}</span
                >
              </div>
            </div>
          </Card.Content>
        </Card.Root>
      </div>
    </Tabs.Content>
  </Tabs.Root>
</div>
