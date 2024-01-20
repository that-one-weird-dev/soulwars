<script lang="ts">
    import GameCardSlot from "./GameCardSlot.svelte";
    import type { FieldSlot } from "$lib/types/field-slot";
    import { createEventDispatcher } from "svelte";
    import type { GameCard } from "$lib/types/card";

    export let inverted = false;
    export let selectableSlots: { [K in FieldSlot]?: boolean } = {};
    export let field: { [K in FieldSlot]?: GameCard } = {};

    const dispatch = createEventDispatcher<{ select: FieldSlot }>();

    function onSelect(slot: FieldSlot) {
        if (!selectableSlots[slot]) return;

        dispatch("select", slot);
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
                selectable={!!selectableSlots["enchantment"]}
                on:click={() => onSelect("enchantment")}
            />
        </div>

        <GameCardSlot
            letter="Y"
            selectable={!!selectableSlots["yokai-1"]}
            on:click={() => onSelect("yokai-1")}
            card={field["yokai-1"]}
        />
        <GameCardSlot
            letter="Y"
            selectable={!!selectableSlots["yokai-2"]}
            on:click={() => onSelect("yokai-2")}
            card={field["yokai-2"]}
        />
        <GameCardSlot
            letter="Y"
            selectable={!!selectableSlots["yokai-3"]}
            on:click={() => onSelect("yokai-3")}
            card={field["yokai-3"]}
        />

        <div
            class="ml-8"
            style="transform: translateY({inverted ? '-2rem' : '2rem'});"
        >
            <GameCardSlot
                letter="T"
                selectable={!!selectableSlots["terrain"]}
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
            <GameCardSlot letter="C" />
        </div>

        <GameCardSlot
            letter="A"
            selectable={!!selectableSlots["artifact-1"]}
            on:click={() => onSelect("artifact-1")}
            card={field["artifact-1"]}
        />
        <GameCardSlot
            letter="A"
            selectable={!!selectableSlots["artifact-2"]}
            on:click={() => onSelect("artifact-2")}
            card={field["artifact-2"]}
        />
        <GameCardSlot
            letter="A"
            selectable={!!selectableSlots["artifact-3"]}
            on:click={() => onSelect("artifact-3")}
            card={field["artifact-3"]}
        />

        <div
            class="ml-8"
            style="transform: translateY({inverted ? '-2rem' : '2rem'});"
        >
            <GameCardSlot letter="M" />
        </div>
    </div>
</div>
