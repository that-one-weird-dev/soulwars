<script lang="ts">
    import Card from "$lib/components/card/Card.svelte";
    import ChipsMultiSelect from "$lib/components/ui/ChipsMultiSelect.svelte";
    import { cardTypeList, type CardType, type GameCard } from "$lib/types/card";

    export let cards: GameCard[];
    export let filter: (name: string, types: CardType[]) => void;

    export let nameFilter: string = "";
    export let typeFilter: CardType[] = cardTypeList();
</script>

<div class="flex flex-col gap-3">
    <form
        class="flex flex-col gap-3"
        on:submit|preventDefault={() => filter(nameFilter, typeFilter)}
    >
        <div class="flex gap-3 items-center">
            <input
                class="input"
                type="text"
                placeholder="Name"
                bind:value={nameFilter}
            />
            <ChipsMultiSelect
                options={{ normal: 0, spell: 1, trap: 2 }}
                bind:value={typeFilter}
            />
        </div>
        <button class="btn variant-filled w-32">Filter</button>
    </form>

    <div class="flex gap-3 flex-wrap">
        {#each cards as card}
            <slot {card}>
                <Card {card} />
            </slot>
        {/each}
    </div>
</div>
