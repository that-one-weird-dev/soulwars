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
    import CardPickerDrawer from "$lib/components/drawer/CardPickerDrawer.svelte";
    import CardListDrawer from "$lib/components/drawer/CardListDrawer.svelte";
    import DeckListDrawer from "$lib/components/drawer/DeckListDrawer.svelte";

    initializeStores();
    storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

    const drawerStore = getDrawerStore();
</script>

<Modal />

<Drawer position="bottom">
    {#if $drawerStore.id === "deck"}
        <DeckView {...$drawerStore.meta} />
    {:else if $drawerStore.id === "card-picker"}
        <CardPickerDrawer {...$drawerStore.meta} />
    {:else if $drawerStore.id === "card-list"}
        <CardListDrawer {...$drawerStore.meta} />
    {:else if $drawerStore.id === "deck-list"}
        <DeckListDrawer {...$drawerStore.meta} />
    {/if}
</Drawer>

<slot />
