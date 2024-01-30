<script lang="ts">
    import GameCardSlot from "./GameCardSlot.svelte";
    import type { FieldSlot } from "$lib/types/field-slot";
    import { createEventDispatcher } from "svelte";
    import type { GameCard } from "$lib/types/card";
    import { getDrawerStore } from "@skeletonlabs/skeleton";

    export let inverted = false;
    export let selectableSlots: FieldSlot[] = [];

    export let field: { [K in FieldSlot]?: GameCard } = {};
    export let graveyard: GameCard[] = [];
    export let cardsInDeck: number = 20;

    const dispatch = createEventDispatcher<{ select: FieldSlot }>();
    const drawerStore = getDrawerStore();

    function onSelect(slot: FieldSlot) {
        if (!selectableSlots.includes(slot)) return;

        dispatch("select", slot);
    }

    function openGraveyard() {
        drawerStore.open({
            id: "card-list",
            meta: {
                cards: graveyard.toReversed(),
            },
        });
    }
</script>

<div class="flex {inverted ? 'flex-col-reverse mt-8' : 'flex-col mb-8'} gap-3">
    <div class="flex flex-row gap-3">
        <div
            class="mr-8"
            style="transform: translateY({inverted ? '-2rem' : '2rem'});"
        >
            <GameCardSlot
                letter="I"
                selectable={!!selectableSlots.includes("enchantment")}
                on:click={() => onSelect("enchantment")}
            />
        </div>

        <GameCardSlot
            letter="Y"
            selectable={!!selectableSlots.includes("yokai-1")}
            on:click={() => onSelect("yokai-1")}
            card={field["yokai-1"]}
        />
        <GameCardSlot
            letter="Y"
            selectable={!!selectableSlots.includes("yokai-2")}
            on:click={() => onSelect("yokai-2")}
            card={field["yokai-2"]}
        />
        <GameCardSlot
            letter="Y"
            selectable={!!selectableSlots.includes("yokai-3")}
            on:click={() => onSelect("yokai-3")}
            card={field["yokai-3"]}
        />

        <div
            class="ml-8"
            style="transform: translateY({inverted ? '-2rem' : '2rem'});"
        >
            <GameCardSlot
                letter="T"
                selectable={!!selectableSlots.includes("terrain")}
                on:click={() => onSelect("terrain")}
                card={field["terrain"]}
            />
        </div>
    </div>
    <div class="flex flex-row gap-3 h-min">
        <div
            class="mr-8"
            style="transform: translateY({inverted ? '-2rem' : '2rem'});"
        >
            <GameCardSlot letter="C" card={graveyard[graveyard.length - 1]} on:click={openGraveyard} />
        </div>

        <GameCardSlot
            letter="A"
            selectable={!!selectableSlots.includes("artifact-1")}
            on:click={() => onSelect("artifact-1")}
            card={field["artifact-1"]}
        />
        <GameCardSlot
            letter="A"
            selectable={!!selectableSlots.includes("artifact-2")}
            on:click={() => onSelect("artifact-2")}
            card={field["artifact-2"]}
        />
        <GameCardSlot
            letter="A"
            selectable={!!selectableSlots.includes("artifact-3")}
            on:click={() => onSelect("artifact-3")}
            card={field["artifact-3"]}
        />

        <div
            class="ml-8"
            style="transform: translateY({inverted ? '-2rem' : '2rem'});"
        >
            <GameCardSlot letter="M" showCardBack={cardsInDeck > 0} />
        </div>
    </div>
</div>
