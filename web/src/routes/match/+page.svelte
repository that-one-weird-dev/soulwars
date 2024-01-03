<script lang="ts">
    import { io } from "socket.io-client";
    import { PUBLIC_BACKEND_ADDRESS } from "$env/static/public";
    import type { PageData } from "./$types";

    export let data: PageData;

    let matchId: string | undefined = undefined;

    const socket = io(`${PUBLIC_BACKEND_ADDRESS}`, {
        transports: ["websocket"],
        auth: {
            sessionToken: data.sessionToken,
        },
    });

    async function connect() {
        const matchInfo = await socket.emitWithAck("match:find");
        matchId = matchInfo.matchId;
    }
</script>

<button class="btn variant-filled" on:click={connect}>Find match</button>
{#if matchId}
    <span>Match found with game id {matchId}</span>
{/if}
