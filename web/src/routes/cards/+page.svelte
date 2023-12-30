<script lang="ts">
    import { goto, invalidateAll } from "$app/navigation";
    import { page } from "$app/stores";
    import CardList from "$lib/components/card/CardList.svelte";
    import { CardType, cardTypeList } from "$lib/types/card";
    import type { PageData } from "./$types";

    export let data: PageData;

    const nameFilter = $page.url.searchParams.get("name") ?? "";
    const typeFilter = $page.url.searchParams
        .get("type")
        ?.split(",")
        .map(Number) ?? cardTypeList();

    async function filter(name: string, types: CardType[]) {
        await goto(`/cards?name=${name}&type=${types.join(",")}`);
        await invalidateAll();
    }
</script>

<CardList cards={data.cards} {nameFilter} {typeFilter} {filter} />
