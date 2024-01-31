<script lang="ts">
    import { goto } from "$app/navigation";
    import GameLogic from "$lib/components/game/GameLogic.svelte";
    import { gameSocket } from "$lib/stores/game";
    import { onMount } from "svelte";

    onMount(() => {
        if (!$gameSocket?.connected) {
            goto("/match");
        }
    });
</script>

{#if $gameSocket?.connected}
    <GameLogic socket={$gameSocket} />
{/if}
