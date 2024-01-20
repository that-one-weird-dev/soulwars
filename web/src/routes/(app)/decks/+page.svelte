<script lang="ts">
    import { getDrawerStore } from "@skeletonlabs/skeleton";
    import Deck from "./Deck.svelte";
    import type { PageData } from "./$types";
    import Icon from "@iconify/svelte";
    import { enhance } from "$app/forms";
    import { page } from "$app/stores";

    export let data: PageData;

    const drawerStore = getDrawerStore();

    if ($page.url.searchParams.get("open")) {
        const openId = Number($page.url.searchParams.get("open"));
        const deck = data.decks.find((deck) => deck.id === openId);

        if (deck) {
            open(deck);
        }
    }

    function open(deck: (typeof data.decks)[0]) {
        drawerStore.open({
            id: "deck",
            meta: {
                id: deck.id,
                name: deck.name,
                cards: deck.decksToCards,
            },
        });
    }

    function cardCount(deck: (typeof data.decks)[0]): number {
        return deck.decksToCards.reduce((prev, dtc) => prev + dtc.count, 0);
    }
</script>

<div class="flex flex-row gap-3 flex-wrap">
    {#each data.decks as deck}
        <Deck
            name={deck.name}
            preview={deck.decksToCards[0] ? `/illustrations/${deck.decksToCards[0]?.card?.id}.png` : ""}
            cardCount={cardCount(deck)}
            on:select={() => open(deck)}
        />
    {/each}

    <div class="card card-hover w-64 variant-filled">
        <form method="POST" action="?/create" class="h-full" use:enhance>
            <button class="h-full w-full flex items-center justify-center">
                <Icon icon="mdi:plus" class="text-4xl" />
            </button>
        </form>
    </div>
</div>
