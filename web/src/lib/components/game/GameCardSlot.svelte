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
    class="variant-glass-surface aspect-[0.69] w-[7.5rem] rounded-xl text-8xl text-surface-700 border hover:shadow-lg cursor-pointer flex items-center justify-center {selectable
        ? 'border-primary-700 border-2 shadow-xl'
        : 'border-surface-700'}"
>
    {#if card}
        <div class="h-full">
            <IngameCard {card} scale={0.35} interactable={false} tiltScale={1.6} />
        </div>
    {:else}
        {letter}
    {/if}
</button>
