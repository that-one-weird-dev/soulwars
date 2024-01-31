<script lang="ts">
    import { io } from "socket.io-client";
    import { PUBLIC_BACKEND_ADDRESS } from "$env/static/public";
    import type { PageData } from "./$types";
    import { gameSocket } from "$lib/stores/game";
    import { goto } from "$app/navigation";

    export let data: PageData;

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

    async function connect() {
        currentState = "searching";
        const matchInfo = await socket.emitWithAck("match:find");
        matchId = matchInfo.matchId;
        currentState = "found";

        socket.emit("game:start", [1, 2, 3, 4]);
    }
</script>

<button class="btn variant-filled" on:click={connect} disabled={currentState !== "awaiting"}>Find match</button>

{#if currentState === "found"}
    <span>Match found with game id {matchId}</span>
{/if}
