<script lang="ts" module>
  import BookOpenIcon from "@lucide/svelte/icons/book-open";
  import CalendarIcon from "@lucide/svelte/icons/calendar";
  import LayoutDashboardIcon from "@lucide/svelte/icons/layout-dashboard";
  import CarIcon from "@lucide/svelte/icons/car";
  import MapPinIcon from "@lucide/svelte/icons/map-pin";
  import Wrench from "@lucide/svelte/icons/wrench";
  import UsersIcon from "@lucide/svelte/icons/users";
  import FuelIcon from "@lucide/svelte/icons/fuel";
  import ClipboardListIcon from "@lucide/svelte/icons/clipboard-list";
  import ChartPieIcon from "@lucide/svelte/icons/chart-pie";
  import DollarSignIcon from "@lucide/svelte/icons/dollar-sign";
  import TrendingUpIcon from "@lucide/svelte/icons/trending-up";
  import PiggyBankIcon from "@lucide/svelte/icons/piggy-bank";
  import BarChart3Icon from "@lucide/svelte/icons/bar-chart-3";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import ReceiptIcon from "@lucide/svelte/icons/receipt";
  import CalendarDaysIcon from "@lucide/svelte/icons/calendar-days";
  import TargetIcon from "@lucide/svelte/icons/target";

  // Data for FinanceVault app
  const data = {
    user: {
      name: "Max Mustermann",
      email: "max@example.com",
      avatar: "/avatars/user.jpg",
    },
    teams: [
      {
        name: "FinanceVault",
        plan: "Premium",
      },
    ],
    navMain: [
      {
        title: "Dashboard",
        url: "/",
        icon: LayoutDashboardIcon,
        isActive: true,
      },
      {
        title: "Monthly Expenses",
        url: "/expenses/monthly",
        icon: CalendarDaysIcon,
      },
      {
        title: "Expense Overview",
        url: "/expenses/overview",
        icon: ReceiptIcon,
      },
      {
        title: "Reports",
        url: "/reports",
        icon: ChartPieIcon,
      },
    ],
  };
</script>

<script lang="ts">
  import NavMain from "./nav-main.svelte";
  import NavProjects from "./nav-projects.svelte";
  import NavUser from "./nav-user.svelte";
  import TeamSwitcher from "./team-switcher.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import type { ComponentProps } from "svelte";

  let {
    ref = $bindable(null),
    collapsible = "icon",
    ...restProps
  }: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root {collapsible} {...restProps}>
  <Sidebar.Header>
    <TeamSwitcher />
  </Sidebar.Header>
  <Sidebar.Content>
    <NavMain items={data.navMain} />
  </Sidebar.Content>
  <Sidebar.Footer>
    <NavUser />
  </Sidebar.Footer>
  <Sidebar.Rail />
</Sidebar.Root>
