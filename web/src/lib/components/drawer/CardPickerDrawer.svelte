<script lang="ts">
    import type { CardType, GameCard } from "$lib/types/card";
    import CardList from "../card/CardList.svelte";
    import Card from "../card/Card.svelte";
    import { getDrawerStore } from "@skeletonlabs/skeleton";
    import wretch from "wretch";
    import QueryStringAddon from "wretch/addons/queryString";

    const drawerStore = getDrawerStore();

    export let select: ((card: GameCard) => void) | undefined = undefined;
    export let excludeDeck: number | undefined = undefined;

    let cards: GameCard[] = [];

    wretch("/cards")
        .addon(QueryStringAddon)
        .query({
            notInDeck: excludeDeck,
        })
        .get()
        .json((c) => (cards = c));

    async function filter(name: string, types: CardType[]) {
        cards = await wretch("/cards")
            .addon(QueryStringAddon)
            .query({
                name,
                type: types.join(","),
                notInDeck: excludeDeck,
            })
            .get()
            .json();
    }

    function onClick(card: GameCard) {
        select?.(card);

        drawerStore.close();
    }
</script>

<div class="p-4">
    <CardList {cards} {filter} let:card>
        <Card {card} interactable={!!select} on:click={() => onClick(card)} />
    </CardList>
</div>
