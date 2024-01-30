<script lang="ts">
    import CardList from "../card/CardList.svelte";
    import type { CardType, GameCard } from "$lib/types/card";

    export let cards: GameCard[];

    $: filteredCards = [...cards];

    function filter(name: string, types: CardType[]) {
        filteredCards = cards.filter((card) =>
            card.name.toLowerCase().includes(name.toLowerCase())
            && types.includes(card.cardType)
        );
    }
</script>

<div class="p-4">
    <CardList cards={filteredCards} {filter} />
</div>
