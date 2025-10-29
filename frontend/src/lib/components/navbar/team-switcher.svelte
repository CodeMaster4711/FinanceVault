<script lang="ts">
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import VaultIcon from "@lucide/svelte/icons/vault";
</script>

<Sidebar.Menu>
  <Sidebar.MenuItem>
    <div
      class="flex items-center justify-center gap-2 px-2 py-2 group-data-[collapsible=icon]:justify-center"
    >
      <div
        class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
      >
        <VaultIcon class="size-4" />
      </div>
      <div
        class="grid flex-1 text-left text-sm leading-tight group-data-[collapsible=icon]:hidden"
      >
        <span class="truncate font-semibold text-lg"> FinanceVault </span>
      </div>
    </div>
  </Sidebar.MenuItem>
</Sidebar.Menu>
