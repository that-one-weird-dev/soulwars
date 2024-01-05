<script lang="ts">
    import { page } from "$app/stores";
    import Icon from "@iconify/svelte";
    import { AppRail, AppRailAnchor } from "@skeletonlabs/skeleton";

    export let loggedIn: boolean;
</script>

<AppRail>
    <AppRailAnchor href="/" selected={$page.url.pathname === "/"}>
        <svelte:fragment slot="lead">
            <div class="flex justify-center">
                <Icon icon="mdi:home" style="font-size: 32px;" />
            </div>
        </svelte:fragment>
        <span>Home</span>
    </AppRailAnchor>
    <AppRailAnchor href="/cards" selected={$page.url.pathname === "/cards"}>
        <svelte:fragment slot="lead">
            <div class="flex justify-center">
                <Icon icon="mdi:cards" style="font-size: 32px;" />
            </div>
        </svelte:fragment>

        <span>Cards</span>
    </AppRailAnchor>
    {#if loggedIn}
        <AppRailAnchor href="/decks" selected={$page.url.pathname === "/decks"}>
            <svelte:fragment slot="lead">
                <div class="flex justify-center">
                    <Icon icon="mdi:treasure-chest" style="font-size: 32px;" />
                </div>
            </svelte:fragment>

            <span>Decks</span>
        </AppRailAnchor>
        <AppRailAnchor href="/match" selected={$page.url.pathname === "/match"}>
            <svelte:fragment slot="lead">
                <div class="flex justify-center">
                    <Icon icon="mdi:fencing" style="font-size: 32px;" />
                </div>
            </svelte:fragment>

            <span>Match</span>
        </AppRailAnchor>
    {/if}
</AppRail>
