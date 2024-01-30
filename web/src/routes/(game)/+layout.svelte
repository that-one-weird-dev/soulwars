<script lang="ts">
    import HeaderBar from "$lib/components/HeaderBar.svelte";
    import CardDescriptionOverlay from "$lib/components/game/CardDescriptionOverlay.svelte";
    import { AppShell } from "@skeletonlabs/skeleton";
    import { gameCardOverlay } from "$lib/stores/gameCardOverlay";

</script>

<CardDescriptionOverlay card={$gameCardOverlay} />

<AppShell>
    <svelte:fragment slot="header">
        <HeaderBar />
    </svelte:fragment>

    <slot />
</AppShell>
