<script lang="ts">
    import { Avatar } from "@skeletonlabs/skeleton";
    import { signIn, signOut } from "@auth/sveltekit/client";
    import { page } from "$app/stores";

    const session = $page.data.session;

    $: isLogged = Object.keys(session ?? {}).length > 0;
</script>

{#if isLogged && session}
    {#if session.user?.image}
        <Avatar src={session.user?.image} on:click={() => signOut()} class="cursor-pointer" />
    {/if}
{:else}
    <button class="btn variant-filled" on:click={() => signIn("discord")}
        >Sign in</button
    >
{/if}
