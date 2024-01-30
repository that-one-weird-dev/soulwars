<script lang="ts">
    import { gameCardOverlay } from "$lib/stores/gameCardOverlay";
import type { GameCard } from "$lib/types/card";
    import IngameCard from "./IngameCard.svelte";

    export let card: GameCard | undefined;

    function dismiss() {
        $gameCardOverlay = undefined;
    }
</script>

{#if card}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div
        role="button"
        tabindex="0"
        class="absolute w-full h-full top-0 left-0 flex items-center justify-center z-40 variant-glass"
        on:click={dismiss}
    >
        <main class="flex flex-row gap-5">
            <IngameCard {card} interactable={false} />
            <section class="w-[32rem]">
                {card.description}
            </section>
        </main>
    </div>
{/if}
