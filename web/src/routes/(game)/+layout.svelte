<script lang="ts">
    import HeaderBar from "$lib/components/HeaderBar.svelte";
    import { AppShell } from "@skeletonlabs/skeleton";
</script>

<AppShell>
    <svelte:fragment slot="header">
        <HeaderBar />
    </svelte:fragment>

    <slot />
</AppShell>
