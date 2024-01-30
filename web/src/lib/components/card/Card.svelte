<script lang="ts">
    import { CardType, type GameCard } from "$lib/types/card";
    import { createEventDispatcher } from "svelte";
    import tilt from "svelte-tilt";

    export let card: GameCard;
    export let interactable = false;
    export let tiltScale = 1.05;
    export let effect3d = true;

    const dispatch = createEventDispatcher<{ click: void }>();
</script>

<button
    use:tilt={{ reverse: true, scale: tiltScale }}
    class="card relative {interactable
        ? ''
        : 'cursor-auto'} variant-filled aspect-[0.69] w-[18rem] text-left hover:shadow-2xl hover:z-20"
    on:click={() => dispatch("click")}
>
    <div
        class="p-4 h-full flex flex-col justify-between gap-3 transform-style-3d"
    >
        <header class="flex flex-col gap-3 transform-style-3d">
            <h1 class="text-2xl font-bold">
                {card.name}
            </h1>

            <img
                src={`/illustrations/${card.id}.png`}
                alt=""
                class="rounded-lg shadow-lg"
                class:translate-z-4={effect3d}
                style="aspect-ratio: 512/316;"
            />
        </header>

        <section class="h-full overflow-hidden text-sm">
            {card.description}
        </section>

        <footer class="flex flex-row justify-between transform-style-3d">
            {#if card.cardType === CardType.Yokai}
                <span
                    class="chip variant-filled-error min-w-24 shadow-lg cursor-auto"
                    class:translate-z-6={effect3d}
                >
                    <strong class="text-xl">{card.damage}</strong>
                </span>
                <span
                    class="chip variant-filled-success min-w-24 shadow-lg cursor-auto"
                    class:translate-z-6={effect3d}
                >
                    <strong class="text-xl">{card.health}</strong>
                </span>
            {/if}
        </footer>
    </div>
    <slot />
</button>
