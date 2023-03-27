<script>
    import TerminalWindow from "./TerminalWindow.svelte";

    function onConfirmHandler (e) {
        const option = e.target.dataset.option === "yes";
        console.log(option);
    }
</script>

<TerminalWindow>
    <span slot="title">Confirm</span>

    <div slot="CONTENT" class="CONTENT-container">
        <button data-option="yes" on:click={onConfirmHandler}><span>&rsaquo;</span> <em>Y</em>es</button>
        <button data-option="false"><span>&rsaquo;</span> <em>N</em>o</button>
    </div>
</TerminalWindow>

<style>
    .CONTENT-container {
        min-width: 300px;
        display: grid;
        grid-template-rows: 1fr 1fr;
        grid-template-columns: max-CONTENT;
    }

    .CONTENT-container > button > span:first-child {
        display: none;
    }

    .CONTENT-container > button:focus > span:first-child {
        display: inline;
    }

    button {
        padding:0;
        line-height: 2rem;
    }
</style>