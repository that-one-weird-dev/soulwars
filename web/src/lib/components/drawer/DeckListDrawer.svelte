<script lang="ts">
    import type { GameDeck } from "$lib/types/deck";
    import { getDrawerStore } from "@skeletonlabs/skeleton";
    import Deck from "../../../routes/(app)/decks/Deck.svelte";

    const drawerStore = getDrawerStore();

    export let decks: GameDeck[];
    export let onSelect: ((deck: GameDeck) => void) | undefined = undefined;

    function select(deck: GameDeck) {
        drawerStore.close();

        onSelect?.(deck);
    }
</script>

<div class="m-4 flex flex-row gap-3">
    {#each decks as deck}
        <Deck
            name={deck.name}
            preview={deck.decksToCards[0]
                ? `/illustrations/${deck.decksToCards[0]?.card?.id}.png`
                : ""}
            on:select={() => select(deck)}
        />
    {/each}
</div>
