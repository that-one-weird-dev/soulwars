<script lang="ts">
    import HeaderBar from "$lib/components/HeaderBar.svelte";
    import SideBar from "$lib/components/SideBar.svelte";
    import { AppShell } from "@skeletonlabs/skeleton";
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<AppShell>
    <svelte:fragment slot="header">
        <HeaderBar />
    </svelte:fragment>

    <svelte:fragment slot="sidebarLeft">
        <SideBar loggedIn={!!data.session} />
    </svelte:fragment>

    <div class="container mx-auto p-8 space-y-8">
        <slot />
    </div>
</AppShell>
