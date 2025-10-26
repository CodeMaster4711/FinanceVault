<script lang="ts">
  import "../app.css";
  import favicon from "$lib/assets/favicon.svg";
  import { onMount } from "svelte";
  import {
    theme,
    effectiveTheme,
    initThemeFromStorage,
  } from "$lib/stores/theme";

  let { children } = $props();

  // On the client we initialize the store from localStorage and
  // subscribe to the effective theme to keep the <html> class in sync.
  onMount(() => {
    initThemeFromStorage();

    const unsub = effectiveTheme.subscribe((t) => {
      try {
        if (t === "dark") document.documentElement.classList.add("dark");
        else document.documentElement.classList.remove("dark");
        // reflect chosen preference for debugging/other CSS
        try {
          document.documentElement.setAttribute(
            "data-theme",
            t === "dark" ? "dark" : "light"
          );
        } catch (e) {}
      } catch (e) {
        // ignore
      }
    });

    return unsub;
  });
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

{@render children?.()}
