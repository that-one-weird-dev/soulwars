<script lang="ts">
    import type { GameCard } from "$lib/types/card";
    import { createEventDispatcher } from "svelte";
    import IngameCard from "./IngameCard.svelte";

    export let hand: GameCard[];

    const dispatch = createEventDispatcher<{ select: number }>();

    let hover = false;
    let hoverCardIndex: number | undefined = undefined;

    function select(index: number) {
        dispatch("select", index);
    }
</script>

<div
    tabindex="0"
    role="toolbar"
    class="flex flex-row items-end gap-3 transition pointer-events-auto {hover
        ? ''
        : 'translate-y-32'}"
    on:mouseenter={() => (hover = true)}
    on:mouseleave={() => {
        hover = false;
        hoverCardIndex = undefined;
    }}
>
    {#each hand as card, i}
            {@const scale = i === hoverCardIndex ? 1.2 : 1}

            <div
                tabindex="0"
                role="toolbar"
                on:focus={() => (hoverCardIndex = i)}
                on:mouseover={() => (hoverCardIndex = i)}
            >
                <div
                    style="transform: scale({scale}); transform-origin: bottom center; z-index: {scale >
                    1
                        ? '10'
                        : 'auto'}"
                    class="transition relative"
                >
                    <IngameCard
                        {card}
                        scale={0.65}
                        on:click={() => select(i)}
                    />
                </div>
            </div>
    {/each}
</div>
