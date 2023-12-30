<script lang="ts">
    import Icon from "@iconify/svelte";
    import { enhance } from "$app/forms";
    import {
        getDrawerStore,
        getModalStore,
        popup,
    } from "@skeletonlabs/skeleton";
    import { CardType, type GameCard } from "$lib/types/card";
    import Card from "$lib/components/card/Card.svelte";

    const drawerStore = getDrawerStore();
    const modalStore = getModalStore();

    export let id: number;
    export let name: string;
    export let cards: { card: GameCard; count: number }[];
</script>

<div class="h-full flex flex-col">
    <section class="p-4 flex flex-row justify-between">
        <header>
            <h1 class="text-4xl text-center">{name}</h1>
        </header>

        <footer class="flex flex-row gap-3">
            <a
                href={`/decks/${id}`}
                class="btn variant-filled"
                on:click={() => drawerStore.close()}
            >
                <span>
                    <Icon icon="mdi:edit" style="font-size: 1.3rem;" />
                </span>
                <span>Edit</span>
            </a>

            <form
                action={`/decks/${id}?/delete`}
                method="POST"
                use:enhance={async ({ cancel }) => {
                    const confirmed = await new Promise((resolve) => {
                        modalStore.trigger({
                            type: "confirm",
                            title: "Please confirm",
                            body: "Are you sure you want to delete this deck?",
                            response: resolve,
                        });
                    });

                    if (!confirmed) {
                        cancel();
                    }

                    return ({ update }) => {
                        drawerStore.close();

                        update();
                    };
                }}
            >
                <button class="btn variant-filled-error">
                    <span>
                        <Icon icon="mdi:trash" style="font-size: 1.3rem;" />
                    </span>
                    <span>Delete</span>
                </button>
            </form>
        </footer>
    </section>

    <section class="p-4 w-full">
        <div class="table-container">
            <table class="table table-hover">
                <thead>
                    <tr>
                        <th>Name</th>
                        <th>Description</th>
                        <th>Type</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {#each cards as { card, count }, i}
                        <tr
                            use:popup={{
                                event: "hover",
                                target: `card-popup-${i}`,
                                placement: "top",
                            }}
                        >
                            <td>
                                {card.name}
                                {#if count > 1}
                                    <span class="chip variant-filled-primary font-bold">
                                        x{count}
                                    </span>
                                {/if}
                            </td>
                            <td>{card.description}</td>
                            <td>{CardType[card.cardType]}</td>
                            <td class="flex flex-end gap-3">
                                {#if card.cardType === CardType.Yokai}
                                    <span
                                        class="chip variant-filled-error shadow-lg"
                                    >
                                        <strong>{card.damage}</strong>
                                    </span>
                                    <span
                                        class="chip variant-filled-success shadow-lg"
                                    >
                                        <strong>{card.health}</strong>
                                    </span>
                                {/if}
                            </td>
                        </tr>

                        <div data-popup={`card-popup-${i}`}>
                            <Card {card} />
                        </div>
                    {/each}
                </tbody>
            </table>
        </div>
    </section>
</div>
