<script lang="ts">
    import { io } from "socket.io-client";
    import { PUBLIC_BACKEND_ADDRESS } from "$env/static/public";
    import type { PageData } from "./$types";

    export let data: PageData;

    let currentState: "awaiting" | "searching" | "found" = "awaiting";
    let matchId: string | undefined = undefined;

    const socket = io(PUBLIC_BACKEND_ADDRESS, {
        transports: ["websocket"],
        auth: {
            sessionToken: data.sessionToken,
        },
    });

    socket.on("match:ready", async () => {
        console.log("match ready");
        console.log(await socket.emitWithAck("info:hand", "test"));
    })

    async function connect() {
        currentState = "searching";
        const matchInfo = await socket.emitWithAck("match:find");
        matchId = matchInfo.matchId;
        currentState = "found";
    }
</script>

<button class="btn variant-filled" on:click={connect} disabled={currentState !== "awaiting"}>Find match</button>

{#if currentState === "found"}
    <span>Match found with game id {matchId}</span>
{/if}
