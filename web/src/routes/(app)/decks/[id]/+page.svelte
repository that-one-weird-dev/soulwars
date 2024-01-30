<script lang="ts">
    import Icon from "@iconify/svelte";
    import type { PageData } from "./$types";
    import { getDrawerStore } from "@skeletonlabs/skeleton";
    import type { GameCard } from "$lib/types/card";
    import { page } from "$app/stores";
    import { goto, invalidateAll } from "$app/navigation";
    import wretch from "wretch";
    import DeckCard from "./DeckCard.svelte";
    import { enhance } from "$app/forms";

    export let data: PageData;

    let changeNameForm: HTMLFormElement | undefined;

    const drawerStore = getDrawerStore();

    function insertCard() {
        drawerStore.open({
            id: "card-picker",
            position: "right",
            meta: {
                select: addCard,
                excludeDeck: data.deck.id,
            },
        });
    }

    async function addCard(card: GameCard) {
        await wretch(`/decks/${$page.params.id}/addCard`)
            .post({ id: card.id })
            .res();

        await invalidateAll();
    }

    async function removeCard(card: GameCard) {
        await wretch(`/decks/${$page.params.id}/removeCard`)
            .post({ id: card.id })
            .res();

        await invalidateAll();
    }

    async function updateCardCount(card: GameCard, count: number) {
        await wretch(`/decks/${$page.params.id}/updateCardCount`)
            .post({ id: card.id, count })
            .res();

        await invalidateAll();
    }

    function back() {
        goto(`/decks?open=${data.deck.id}`);
    }
</script>

<div class="flex gap-3">
    <button class="btn variant-filled" on:click={back}>
        <span>
            <Icon icon="mdi:arrow-left" />
        </span>
        <span>Back</span>
    </button>
    <form
        bind:this={changeNameForm}
        action="?/changeName"
        method="POST"
        use:enhance={() => {
            return ({ update }) => update({ reset: false });
        }}
    >
        <input
            class="p-0 text-4xl font-bold bg-transparent border-transparent outline-transparent focus:border-transparent focus:outline-transparent focus:ring-0"
            autocomplete="off"
            type="text"
            placeholder="Name"
            bind:value={data.deck.name}
            on:focusout={() => changeNameForm?.requestSubmit()}
            name="name"
        />
    </form>
</div>

<div class="flex flex-row flex-wrap gap-3">
    {#each data.deck.decksToCards as dtc}
        <DeckCard
            card={dtc.card}
            count={dtc.count}
            on:remove={() => removeCard(dtc.card)}
            on:countChange={(event) => updateCardCount(dtc.card, event.detail)}
        />
    {/each}

    <button
        class="card card-hover w-[18rem] h-[26rem] variant-filled flex justify-center items-center"
        on:click={insertCard}
    >
        <Icon icon="mdi:plus" class="text-4xl" />
    </button>
</div>
