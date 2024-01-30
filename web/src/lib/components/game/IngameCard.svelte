<script lang="ts">
    import type { GameCard } from "$lib/types/card";
    import { createEventDispatcher } from "svelte";
    import Card from "../card/Card.svelte";
    import { gameCardOverlay } from "$lib/stores/gameCardOverlay";

    export let card: GameCard;
    export let scale = 1;
    export let interactable = true;
    export let tiltScale = 1.05;

    const dispatch = createEventDispatcher<{ click: void }>();

    function showOverlay() {
        if (!interactable) return;

        $gameCardOverlay = card;
    }
</script>

<div
    role="button"
    tabindex="0"
    class="aspect-[0.69] hover:z-10"
    style="width: {18 * scale}rem;"
    on:contextmenu|preventDefault={showOverlay}
>
    <div
        class="absolute"
        style="transform-origin: 0 0; transform: scale({scale});"
    >
        <Card on:click={() => dispatch("click")} {card} {interactable} {tiltScale} />
    </div>
</div>
