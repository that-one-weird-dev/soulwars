<script lang="ts">
    import "../app.postcss";
    import {
        AppShell,
        AppBar,
        Drawer,
        initializeStores,
        getDrawerStore,
        Modal,
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
    import SideBar from "$lib/components/SideBar.svelte";
    import Login from "./Login.svelte";
    import DeckView from "./decks/DeckView.svelte";
    import CardListDrawer from "$lib/components/drawer/CardListDrawer.svelte";
    import type { PageData } from "./$types";

    export let data: PageData;

    initializeStores();
    storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

    const drawerStore = getDrawerStore();
</script>

<Drawer position="bottom">
    {#if $drawerStore.id === "deck"}
        <DeckView {...$drawerStore.meta} />
    {:else if $drawerStore.id === "card-select"}
        <CardListDrawer {...$drawerStore.meta} />
    {/if}
</Drawer>
<Modal />

<AppShell>
    <svelte:fragment slot="header">
        <AppBar class="shadow-xl">
            <svelte:fragment slot="lead">
                <strong class="text-xl uppercase">Deck box</strong>
            </svelte:fragment>
            <svelte:fragment slot="trail">
                <Login />
            </svelte:fragment>
        </AppBar>
    </svelte:fragment>

    <svelte:fragment slot="sidebarLeft">
        <SideBar loggedIn={!!data.session} />
    </svelte:fragment>

    <div class="container mx-auto p-8 space-y-8">
        <slot />
    </div>
</AppShell>
