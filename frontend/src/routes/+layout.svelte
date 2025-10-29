<script lang="ts">
  import "../app.css";
  import favicon from "$lib/assets/favicon.svg";
  import { onMount } from "svelte";
  import AppSidebar from "$lib/components/navbar/app-sidebar.svelte";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb/index.js";
  import { Separator } from "$lib/components/ui/separator/index.js";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import {
    theme,
    effectiveTheme,
    initThemeFromStorage,
  } from "$lib/stores/theme";
  import { page } from "$app/stores";
  import { authStore } from "$lib/stores/auth";

  let { children } = $props();

  // Check if we should show the sidebar (not on signin/signup pages)
  let showSidebar = $derived(
    !$page.url.pathname.startsWith("/signin") &&
      !$page.url.pathname.startsWith("/signup")
  );

  // On the client we initialize the store from localStorage and
  // subscribe to the effective theme to keep the <html> class in sync.
  onMount(() => {
    // Initialize auth store from localStorage
    authStore.initialize();

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

{#if showSidebar}
  <Sidebar.Provider>
    <AppSidebar />
    <Sidebar.Inset>
      <header
        class="group-has-data-[collapsible=icon]/sidebar-wrapper:h-12 flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear"
      >
        <div class="flex items-center gap-2 px-4">
          <Sidebar.Trigger class="-ml-1" />
          <Separator
            orientation="vertical"
            class="mr-2 data-[orientation=vertical]:h-4"
          />
          <Breadcrumb.Root>
            <Breadcrumb.List>
              <Breadcrumb.Item class="hidden md:block">
                <Breadcrumb.Link href="##"
                  >Building Your Application</Breadcrumb.Link
                >
              </Breadcrumb.Item>
              <Breadcrumb.Separator class="hidden md:block" />
              <Breadcrumb.Item>
                <Breadcrumb.Page>Data Fetching</Breadcrumb.Page>
              </Breadcrumb.Item>
            </Breadcrumb.List>
          </Breadcrumb.Root>
        </div>
      </header>
      <div class="flex flex-1 flex-col gap-4 p-4 pt-0">
        {@render children?.()}
      </div>
    </Sidebar.Inset>
  </Sidebar.Provider>
{:else}
  <!-- Content without sidebar for signin/signup pages -->
  {@render children?.()}
{/if}
