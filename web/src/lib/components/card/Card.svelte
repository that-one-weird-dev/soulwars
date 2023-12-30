<script lang="ts">
    import { CardType, type GameCard } from "$lib/types/card";
    import { createEventDispatcher } from "svelte";

    export let card: GameCard;
    export let interactable = false;

    const dispatch = createEventDispatcher<{ click: void }>();
</script>

<button
    class="card {interactable
        ? 'card-hover'
        : 'cursor-auto'} variant-filled w-[18rem] h-[26rem] text-left"
    on:click={() => dispatch("click")}
>
    <section class="p-4 h-full flex flex-col justify-between gap-3">
        <header class="flex flex-col gap-3">
            <h1 class="text-2xl font-bold">{card.name}</h1>

            <img
                src={`/illustrations/${card.id}.png`}
                alt=""
                class="rounded-lg shadow-lg"
                style="aspect-ratio: 512/316;"
            />
        </header>

        <section
            class="h-[14rem] overflow-hidden text-sm"
        >
            {card.description}
        </section>

        <footer>
            {#if card.cardType === CardType.Yokai}
                <span
                    class="chip variant-filled-error min-w-24 shadow-lg cursor-auto"
                >
                    <strong class="text-xl">{card.damage}</strong>
                </span>
                <span
                    class="chip variant-filled-success min-w-24 float-end shadow-lg cursor-auto"
                >
                    <strong class="text-xl">{card.health}</strong>
                </span>
            {/if}
        </footer>
    </section>
</button>
