<script>
    export let title = null;
</script>

<div class="container box-shadow" tabindex="0">
    {#if title}
        <span class="window-title">{title}</span>
    {/if}
    <span class="CONTENT">
            <slot name="CONTENT"/>
    </span>
</div>

<style>
    * {
        margin: 0;
        padding:0
    }

    .container {
        display: grid;

        position:relative;
        border: 1px solid var(--color-primary);
        padding: 0.5rem 0.5rem 0.5rem 0.5rem;
        margin: calc(0.5rem - 1px)
    }

    .container:focus {
        outline: none;
        border: 1px solid var(--color-tertiary);
    }

    .CONTENT {
        margin: 1rem 0.5rem 0.5rem 0.5rem;
        padding:0;
    }

    .window-title {
        position: absolute;
        top: -0.75rem;
        padding: 0 0.5rem 0 0.5rem;
        margin-left: calc(0.5rem - 1px);

        background-color: var(--color-grey-base);
    }


</style>