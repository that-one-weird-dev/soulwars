<script lang="ts">
    import Game from "$lib/components/game/Game.svelte";
    import { type GameCard } from "$lib/types/card";
    import type { Socket } from "socket.io-client";
    import wretch from "wretch";

    export let socket: Socket;

    let hand: GameCard[] = [];

    socket.on("game:select:field-slot", (_, callback: (fieldSlot: string) => void) => {
        console.log("requested field slot selection");
        callback("artifact-1");
    });

    socket
        .emitWithAck("info:hand")
        .then(({ cards }: { cards: number[] }) => {

            return Promise.all(cards.map((card) => {
                return wretch(`/cards/${card}`).get().json<GameCard>()
            }));
        })
        .then(cards => hand = cards);
</script>

<Game {hand} />
