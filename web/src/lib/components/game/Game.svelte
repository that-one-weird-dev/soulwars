<script lang="ts">
    import type { GameCard } from "$lib/types/card";
    import { onMount } from "svelte";
    import GameField from "./GameField.svelte";
    import wretch from "wretch";
    import Hand from "./Hand.svelte";
    import type { GameDeck } from "$lib/types/deck";
    import { shuffle } from "$lib/utils/shuffle";
    import IngameCard from "./IngameCard.svelte";
    import type { FieldSlot } from "$lib/types/field-slot";

    export let hand: GameCard[] = [];

    let selectedCard: number | undefined;
    let selectableSlots: FieldSlot[] = [];
    let field: { [K in FieldSlot]?: GameCard } = {};
    let graveyard: GameCard[] = [];

    onMount(async () => {
        const deck = await wretch("/decks/2/cards").get().json<GameDeck>();
        const cards = deck.decksToCards.flatMap((dtc) =>
            new Array(dtc.count).fill(dtc.card),
        );

        hand = shuffle(cards).slice(0, 5);
        graveyard = shuffle(cards).slice(0, 5);
    });

    function onSelectFromHand(event: CustomEvent<number>) {
        selectedCard = event.detail;
        selectableSlots = ["yokai-1", "yokai-2", "yokai-3"];
    }

    function onSelectSlot(event: CustomEvent<FieldSlot>) {
        if (selectedCard === undefined) return;
        if (field[event.detail] !== undefined) return;

        field[event.detail] = { ...hand[selectedCard] };

        hand = hand.toSpliced(selectedCard, 1);
        selectedCard = undefined;
        selectableSlots = [];
    }

    function onKeyDown(e: KeyboardEvent) {
        if (e.key === "Escape") {
            selectedCard = undefined;
            selectableSlots = [];
        }
    }
</script>

<main class="w-full h-full flex flex-col items-center gap-16 m-5">
    <GameField inverted={true} />
    <GameField {selectableSlots} {field} {graveyard} on:select={onSelectSlot} />
</main>

<section class="w-full h-full left-0 bottom-0 absolute pointer-events-none">
    <div class="w-full h-full absolute flex items-center">
        {#if selectedCard !== undefined}
            <div class="pointer-events-auto">
                <IngameCard card={hand[selectedCard]} interactable={false} />
            </div>
        {/if}
    </div>
    <div class="w-full h-full absolute flex justify-center items-end">
        <Hand {hand} {selectedCard} on:select={onSelectFromHand} />
    </div>
</section>

<svelte:window on:keydown={onKeyDown} />
