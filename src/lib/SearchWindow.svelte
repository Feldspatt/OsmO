<script>
import TerminalWindow from "./TerminalWindow.svelte";

export let title

</script>

<TerminalWindow title={title}>
    <span slot="title">{title}</span>
    <span slot="CONTENT" class="CONTENT">
            <label for="filterInput">filter</label>
            <input id="filterInput" type="text" placeholder="#tag1, ..."/>
            <label for="groupInput">group</label>
            <input id="groupInput" type="text" placeholder="#tag1 + #tag2, ..."/>
    </span>
</TerminalWindow>

<style>
    .CONTENT {
        display:grid;
        column-gap: 3rem;
        grid-template-columns: max-CONTENT 1fr;
        align-items: center;
    }

    input {
        caret-color: var(--color-primary);
    }

</style>

