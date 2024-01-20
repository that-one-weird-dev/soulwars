<script lang="ts">
    import "../app.postcss";
    import {
        initializeStores,
        getDrawerStore,
        Modal,
        Drawer,
    } from "@skeletonlabs/skeleton";
    import {
        computePosition,
        autoUpdate,
        flip,
        shift,
        offset,
        arrow,
    } from "@floating-ui/dom";
    import { storePopup } from "@skeletonlabs/skeleton";
    import DeckView from "./(app)/decks/DeckView.svelte";
    import CardListDrawer from "$lib/components/drawer/CardListDrawer.svelte";

    initializeStores();
    storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

    const drawerStore = getDrawerStore();
</script>

<Modal />

<Drawer position="bottom">
    {#if $drawerStore.id === "deck"}
        <DeckView {...$drawerStore.meta} />
    {:else if $drawerStore.id === "card-select"}
        <CardListDrawer {...$drawerStore.meta} />
    {/if}
</Drawer>

<slot />
