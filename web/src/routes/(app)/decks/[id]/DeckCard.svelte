<script lang="ts">
    import Card from "$lib/components/card/Card.svelte";
    import type { GameCard } from "$lib/types/card";
    import Icon from "@iconify/svelte";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher<{
        countChange: number;
        remove: void;
    }>();

    export let card: GameCard;
    export let count = 0;

    let editing = false;

    function add(amount: number) {
        count += amount;

        if (count < 0) {
            count = 0;
        } else if (count > 3) {
            count = 3;
        }
    }

    function confirm() {
        editing = false;

        if (count === 0) {
            dispatch("remove");
        } else {
            dispatch("countChange", count);
        }
    }
</script>

<div class="relative">
    <Card effect3d={!editing} {card}>
        {#if !editing}
            <div
                class="absolute w-full h-full top-0 left-0 p-3 transition translate-z-4 active:translate-z-2"
            >
                <div class="flex justify-end">
                    <button
                        class="btn variant-filled-primary"
                        on:click={() => (editing = true)}
                    >
                        <span>
                            <Icon icon="mdi:edit" class="text-lg" />
                        </span>
                        <span>{count}</span>
                    </button>
                </div>
            </div>
        {/if}
        <div
            class="absolute w-full h-full top-0 left-0 rounded-lg variant-glass p-4 transition-all transform-style-3d"
            class:pointer-events-none={!editing}
            class:opacity-100={editing}
            class:opacity-0={!editing}
        >
            <div class="flex flex-col gap-3 translate-z-4">
                <div class="flex gap-3">
                    <button class="btn variant-filled" on:click={() => add(-1)}>
                        <Icon icon="mdi:minus" />
                    </button>
                    <input
                        class="input variant-filled border-none"
                        type="number"
                        bind:value={count}
                        readonly
                    />
                    <button class="btn variant-filled" on:click={() => add(1)}>
                        <Icon icon="mdi:plus" />
                    </button>
                </div>
                {#if count === 0}
                    <button class="btn variant-filled-error" on:click={confirm}
                        >Remove</button
                    >
                {:else}
                    <button class="btn variant-filled" on:click={confirm}
                        >Confirm</button
                    >
                {/if}
            </div>
        </div>
    </Card>
</div>
