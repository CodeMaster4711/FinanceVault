<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card/index.js";
  import { enhance } from "$app/forms";
</script>

<div class="flex min-h-svh flex-col items-center justify-center p-6">
  <Card class="w-full max-w-md">
    <CardHeader>
      <CardTitle class="text-3xl">Welcome to FinanceVault</CardTitle>
      <CardDescription>You are successfully logged in!</CardDescription>
    </CardHeader>
    <CardContent class="flex flex-col gap-4">
      <p class="text-muted-foreground">
        This is your protected dashboard. Only authenticated users can access
        this page.
      </p>

      <form method="POST" action="?/logout" use:enhance>
        <Button type="submit" variant="destructive" class="w-full">
          Logout
        </Button>
      </form>
    </CardContent>
  </Card>
</div>
