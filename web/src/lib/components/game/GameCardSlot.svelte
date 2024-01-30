<script lang="ts">
    import type { GameCard } from "$lib/types/card";
    import { createEventDispatcher } from "svelte";
    import IngameCard from "./IngameCard.svelte";
    import { gameCardOverlay } from "$lib/stores/gameCardOverlay";

    export let letter = "";
    export let selectable = false;
    export let card: GameCard | undefined = undefined;

    const dispatch = createEventDispatcher<{ click: void }>();

    function showCard() {
        if (!card) return;

        $gameCardOverlay = card;
    }
</script>

<button
    on:click={() => dispatch("click")}
    on:contextmenu|preventDefault={showCard}
    class="aspect-[0.69] w-[7.5rem] relative flex justify-center items-center"
>
    <div class="absolute w-full h-full flex justify-center items-center {selectable
        ? 'border-primary-700 border-2 shadow-xl'
        : 'border-surface-700'} variant-glass-surface text-surface-700 text-8xl rounded-xl border hover:shadow-lg cursor-pointer">
        {letter}
    </div>

    {#if card}
        <IngameCard {card} scale={0.35} tiltScale={2} />
    {/if}
</button>
