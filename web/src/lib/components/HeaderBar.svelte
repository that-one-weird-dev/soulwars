<script lang="ts">
    import { AppBar } from "@skeletonlabs/skeleton";
    import Login from "../../routes/Login.svelte";

</script>

<AppBar class="shadow-xl">
    <svelte:fragment slot="lead">
        <strong class="text-xl uppercase">Deck box</strong>
    </svelte:fragment>
    <svelte:fragment slot="trail">
        <Login />
    </svelte:fragment>
</AppBar>
