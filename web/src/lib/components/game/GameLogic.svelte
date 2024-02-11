<script lang="ts">
    import Game from "$lib/components/game/Game.svelte";
    import { type GameCard } from "$lib/types/card";
    import type { Socket } from "socket.io-client";
    import type { CardLocation } from "$lib/types/card-location";
    import wretch from "wretch";
    import type { FieldSlot } from "$lib/types/field-slot";

    export let socket: Socket;

    let hand: GameCard[] = [];
    let game: Game;

    socket
        .emitWithAck("info:hand")
        .then(({ cards }: { cards: number[] }) => {
            return Promise.all(
                cards.map((card) => {
                    return wretch(`/cards/${card}`).get().json<GameCard>();
                }),
            );
        })
        .then((cards) => (hand = cards));

    socket.emit("game:activate");

    socket.on(
        "game:select:field-slot",
        (slots: FieldSlot[], callback: (fieldSlot: string) => void) => {
            game.selectSlot(slots, (slot) => callback(slot));
        },
    );

    function onActivateCard(event: CustomEvent<CardLocation>) {
        socket.emit("game:activate", event.detail);
    }
</script>

<Game bind:this={game} {hand} on:activateCard={onActivateCard} />
