<script lang="ts">
    import type { GameCard } from "$lib/types/card";
    import GameField from "./GameField.svelte";
    import Hand from "./Hand.svelte";
    import IngameCard from "./IngameCard.svelte";
    import type { FieldSlot } from "$lib/types/field-slot";
    import { createEventDispatcher } from "svelte";
    import type { CardLocation } from "$lib/types/card-location";

    const dispatch = createEventDispatcher<{
        cancelAction: void;
        activateCard: CardLocation;
    }>();

    export let hand: GameCard[] = [];
    export let graveyard: GameCard[] = [];

    export function selectSlot(slots: FieldSlot[], callback: (slot: FieldSlot) => void) {
        selectableSlots = slots;
        selectSlotCallback = callback;
    }

    let selectableSlots: FieldSlot[] = [];
    let selectSlotCallback: ((slot: FieldSlot) => void) | undefined;

    let selectedCard: GameCard | undefined = undefined;
    let field: { [K in FieldSlot]?: GameCard } = {};

    function onKeyDown(e: KeyboardEvent) {
        if (e.key === "Escape") {
            dispatch("cancelAction");
        }
    }

    function onSelectSlot(event: CustomEvent<FieldSlot>) {
        selectSlotCallback?.(event.detail);
        selectableSlots = [];
    }
</script>

<main class="w-full h-full flex flex-col items-center gap-16 m-5">
    <GameField inverted={true} />
    <GameField
        {selectableSlots}
        {field}
        {graveyard}
        on:select={onSelectSlot}
    />
</main>

<section class="w-full h-full left-0 bottom-0 absolute pointer-events-none">
    <div class="w-full h-full absolute flex items-center">
        {#if selectedCard !== undefined}
            <div class="pointer-events-auto">
                <IngameCard card={selectedCard} interactable={false} />
            </div>
        {/if}
    </div>
    <div class="w-full h-full absolute flex justify-center items-end">
        <Hand
            {hand}
            on:select={(event) =>
                dispatch("activateCard", { type: "hand", index: event.detail })}
        />
    </div>
</section>

<svelte:window on:keydown={onKeyDown} />
