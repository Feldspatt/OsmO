<script>
    import TerminalWindow from "./TerminalWindow.svelte";
    import {objectToArray} from "./helpers.mjs";
    import SearchWindow from "./SearchWindow.svelte";

    const vertex = {
        metadata:{
            id: 1,
            created: "2019-01-01-00:00:00",
            updated: "2019-01-01-00:00:00",
        },
        data: {
            name: 'John',
            age: 30,
            city: 'New York',
            description: "A very long description that will be cut off after 20 characters"
        },
        links:{

        }
    }

    let fData = objectToArray(vertex.data);
    let fMetadata = objectToArray(vertex.metadata);

    let optionnalPanelValue = "metadata";

</script>

<TerminalWindow title={"Data"}>
    <div slot="CONTENT" class="CONTENT-container">
        {#each fData as item}
            <span>{item[0]}</span>
            <span>
                {#if item[1].length > 45}
                    {item[1].slice(0, 40)}<button>[...]</button>
                {:else}
                    {item[1]}
                {/if}
            </span>
        {/each}
    </div>
</TerminalWindow>

{#if optionnalPanelValue === "metadata"}
    <TerminalWindow title={"Metadata"}>
        <span slot="title">Metadata</span>

        <div slot="CONTENT" class="CONTENT-container">
            {#each fMetadata as item}
                <span>{item[0]}</span>
                <span>{item[1]}</span>
            {/each}
        </div>
    </TerminalWindow>
{/if}

<SearchWindow title={"Links"}>
    <div slot="CONTENT">
        <input type="text" placeholder="Search for a vertex" />
    </div>
</SearchWindow>

<style>

    .CONTENT-container {
        padding: 0;
        display:grid;
        grid-template-columns: max-CONTENT max-CONTENT;
        justify-CONTENT: space-between;
        column-gap: 3rem;
    }

    .CONTENT-container button {
        padding: 0
    }

</style>