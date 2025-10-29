<script lang="ts">
  import * as Tabs from "$lib/components/ui/tabs/index.js";
</script>

<div class="h-screen w-full">
  <Tabs.Root value="overview" class="w-full h-full">
    <Tabs.List>
      <Tabs.Trigger value="overview">Overview</Tabs.Trigger>
      <Tabs.Trigger value="expenses">Expenses</Tabs.Trigger>
      <Tabs.Trigger value="subscription">Subscription</Tabs.Trigger>
    </Tabs.List>
    <Tabs.Content value="overview">
      Make changes to your account here.
    </Tabs.Content>
    <Tabs.Content value="expenses">Change your expenses here.</Tabs.Content>
    <Tabs.Content value="subscription"
      >Manage your subscription here.</Tabs.Content
    >
  </Tabs.Root>
</div>
