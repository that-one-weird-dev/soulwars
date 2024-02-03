<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let name: string;
    export let preview: string;
    export let cardCount: number | undefined = undefined;

    const dispatch = createEventDispatcher<{ select: void }>();

    function countColor(count: number) {
        if (count < 20) {
            return "variant-filled-warning";
        } else if (count > 40) {
            return "variant-filled-warning";
        } else  {
            return "variant-filled-primary"
        }
    }
</script>

<button
    class="block card card-hover cursor-pointer overflow-hidden w-64"
    on:click={() => dispatch("select")}
>
    <header class="relative inline-block">
        <img src={preview} alt="" class="aspect-square" />
        {#if cardCount}
            <span class="chip {countColor(cardCount)} m-2 absolute top-0 right-0 font-bold text-xl">{cardCount}</span>
        {/if}
    </header>
    <section class="p-4">
        <h1 class="text-2xl">{name}</h1>
    </section>
</button>
