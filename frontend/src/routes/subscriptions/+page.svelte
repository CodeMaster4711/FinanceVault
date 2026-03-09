<script lang="ts">
  import { onMount } from "svelte";
  import { today, getLocalTimeZone } from "@internationalized/date";
  import Subscriptions from "$lib/components/expenses/subscritpions.svelte";
  import { SubscriptionService, type Subscription } from "$lib/services/subscriptions";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Select from "$lib/components/ui/select";
  import { toast } from "svelte-sonner";

  let allSubs: Subscription[] = $state([]);
  let loading = $state(true);
  let selectedDate = $state(today(getLocalTimeZone()));
  let showAddDialog = $state(false);
  let showEditDialog = $state(false);
  let editingSub: Subscription | null = $state(null);
  let formData = $state({ name: "", amount: 0, billing: "monthly", next_billing: "" });

  const billingOptions = [
    { value: "monthly", label: "Monatlich" },
    { value: "yearly", label: "Jährlich" },
  ];

  onMount(async () => {
    try {
      allSubs = await SubscriptionService.getAll();
    } catch {
      toast.error("Fehler beim Laden der Abonnements");
    } finally {
      loading = false;
    }
  });

  let subscriptions = $derived(
    allSubs.map((s) => ({
      id: s.id,
      name: s.name,
      amount: s.amount,
      billingDay: new Date(s.next_billing).getDate(),
      category: s.billing,
    }))
  );

  let totalSubscriptions = $derived(
    allSubs.reduce((sum, s) => sum + (s.billing === "yearly" ? s.amount / 12 : s.amount), 0)
  );

  function handleAddSubscription() {
    formData = { name: "", amount: 0, billing: "monthly", next_billing: new Date().toISOString().slice(0, 10) };
    showAddDialog = true;
  }

  async function submitAdd() {
    try {
      await SubscriptionService.create(formData);
      showAddDialog = false;
      allSubs = await SubscriptionService.getAll();
      toast.success("Abonnement erstellt");
    } catch { toast.error("Fehler beim Erstellen"); }
  }

  function handleEditSubscription(id: string) {
    const sub = allSubs.find((s) => s.id === id);
    if (!sub) return;
    editingSub = sub;
    formData = { name: sub.name, amount: sub.amount, billing: sub.billing, next_billing: sub.next_billing.slice(0, 10) };
    showEditDialog = true;
  }

  async function submitEdit() {
    if (!editingSub) return;
    try {
      await SubscriptionService.update(editingSub.id, formData);
      showEditDialog = false;
      allSubs = await SubscriptionService.getAll();
      toast.success("Abonnement aktualisiert");
    } catch { toast.error("Fehler beim Aktualisieren"); }
  }

  async function handleDeleteSubscription(id: string) {
    if (!confirm("Abonnement wirklich löschen?")) return;
    try {
      await SubscriptionService.delete(id);
      allSubs = await SubscriptionService.getAll();
      toast.success("Abonnement gelöscht");
    } catch { toast.error("Fehler beim Löschen"); }
  }
</script>

<div class="container mx-auto py-6">
  {#if loading}
    <div class="flex items-center justify-center h-64">
      <span class="text-muted-foreground text-sm">Laden...</span>
    </div>
  {:else}
    <Subscriptions
      {subscriptions} {selectedDate} {totalSubscriptions}
      onAddSubscription={handleAddSubscription}
      onEditSubscription={handleEditSubscription}
      onDeleteSubscription={handleDeleteSubscription}
    />
  {/if}
</div>

<Dialog.Root bind:open={showAddDialog}>
  <Dialog.Content>
    <Dialog.Header><Dialog.Title>Neues Abonnement</Dialog.Title></Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2"><Label>Name</Label><Input bind:value={formData.name} placeholder="z.B. Netflix" /></div>
      <div class="grid gap-2"><Label>Betrag (€)</Label><Input type="number" step="0.01" bind:value={formData.amount} /></div>
      <div class="grid gap-2">
        <Label>Abrechnungszyklus</Label>
        <Select.Root type="single" bind:value={formData.billing}>
          <Select.Trigger>{billingOptions.find(b => b.value === formData.billing)?.label}</Select.Trigger>
          <Select.Content>{#each billingOptions as opt}<Select.Item value={opt.value}>{opt.label}</Select.Item>{/each}</Select.Content>
        </Select.Root>
      </div>
      <div class="grid gap-2"><Label>Nächste Abbuchung</Label><Input type="date" bind:value={formData.next_billing} /></div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showAddDialog = false)}>Abbrechen</Button>
      <Button onclick={submitAdd}>Speichern</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={showEditDialog}>
  <Dialog.Content>
    <Dialog.Header><Dialog.Title>Abonnement bearbeiten</Dialog.Title></Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="grid gap-2"><Label>Name</Label><Input bind:value={formData.name} /></div>
      <div class="grid gap-2"><Label>Betrag (€)</Label><Input type="number" step="0.01" bind:value={formData.amount} /></div>
      <div class="grid gap-2">
        <Label>Abrechnungszyklus</Label>
        <Select.Root type="single" bind:value={formData.billing}>
          <Select.Trigger>{billingOptions.find(b => b.value === formData.billing)?.label}</Select.Trigger>
          <Select.Content>{#each billingOptions as opt}<Select.Item value={opt.value}>{opt.label}</Select.Item>{/each}</Select.Content>
        </Select.Root>
      </div>
      <div class="grid gap-2"><Label>Nächste Abbuchung</Label><Input type="date" bind:value={formData.next_billing} /></div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (showEditDialog = false)}>Abbrechen</Button>
      <Button onclick={submitEdit}>Aktualisieren</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
