<script lang="ts">
    import CardList from "../card/CardList.svelte";
    import type { CardType, GameCard } from "$lib/types/card";

    export let cards: GameCard[];
    export let showFilters: boolean = true;

    $: filteredCards = [...cards];

    function filter(name: string, types: CardType[]) {
        filteredCards = cards.filter((card) => {
            if (!card) return true;

            return (
                card.name.toLowerCase().includes(name.toLowerCase()) &&
                types.includes(card.cardType)
            );
        });
    }
</script>

<div class="p-4">
    <CardList cards={filteredCards} {filter} {showFilters} />
</div>
