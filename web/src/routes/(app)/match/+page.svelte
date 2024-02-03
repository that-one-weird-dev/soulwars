<script lang="ts">
    import { io } from "socket.io-client";
    import { PUBLIC_BACKEND_ADDRESS } from "$env/static/public";
    import type { PageData } from "./$types";
    import { gameSocket } from "$lib/stores/game";
    import { goto } from "$app/navigation";
    import { getDrawerStore } from "@skeletonlabs/skeleton";
    import type { GameDeck } from "$lib/types/deck";

    export let data: PageData;

    const drawerStore = getDrawerStore();

    let currentState: "awaiting" | "searching" | "found" = "awaiting";
    let matchId: string | undefined = undefined;

    const socket = io(PUBLIC_BACKEND_ADDRESS, {
        transports: ["websocket"],
        auth: {
            sessionToken: data.sessionToken,
        },
    });

    socket.on("game:ready", async () => {
        console.log("game ready");

        $gameSocket = socket;
        goto("/game");
    })

    async function findMatch() {
        drawerStore.open({
            id: "deck-list",
            meta: {
                decks: data.decks,
                onSelect: connect,
            },
        })
    }

    async function connect(deck: GameDeck) {
        currentState = "searching";
        const matchInfo = await socket.emitWithAck("match:find");
        matchId = matchInfo.matchId;
        currentState = "found";

        const cards = deck.decksToCards.flatMap(dtc => new Array(dtc.count).fill(dtc.card.id));

        socket.emit("game:start", cards);
    }
</script>

<button class="btn variant-filled" on:click={findMatch} disabled={currentState !== "awaiting"}>Find match</button>

{#if currentState === "found"}
    <span>Match found with game id {matchId}</span>
{/if}
