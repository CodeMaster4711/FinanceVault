<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { vault } from "$lib/stores/vault";

  onMount(() => {
    return vault.subscribe((s) => {
      if (s.status === "unlocked") {
        goto("/expenses/monthly");
      }
    });
  });
</script>
