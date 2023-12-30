<script lang="ts">
    import Icon from "@iconify/svelte";

    type T = $$Generic;

    export let options: Record<string, T>;
    export let name: string | undefined = undefined;
    export let value: T[] = [...Object.values(options)];
    export let formValue: string = "";


    function toggle(key: string) {
        value = value.includes(options[key])
            ? value.filter(v => v !== options[key])
            : [...value, options[key]];

        formValue = value.join(",");
    }
</script>

{#if name}
    <input class="hidden" {name} value={formValue} />
{/if}

<div class="flex flex-row gap-3">
    {#each Object.entries(options) as [optionKey, optionValue]}
        <button
            class="chip {value.includes(optionValue) ? 'variant-filled' : 'variant-soft'}"
            type="button"
            on:click={() => {
                toggle(optionKey);
            }}
            on:keypress
        >
            {#if value.includes(optionValue)}<span><Icon icon="mdi:check" /></span>{/if}
            <span class="capitalize">{optionKey}</span>
        </button>
    {/each}
</div>
