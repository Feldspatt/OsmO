<div class="container">
    <button><em>e</em>dit</button>
    <button><em>d</em>elete</button>
    <button><em>t</em>ags</button>
    <button><em>l</em>inks</button>
    <button><em>c</em>lone</button>
    <button><em>m</em>etadata</button>
</div>

<style>



    .container {
        display: grid;
        grid-template-columns: 1fr;
        color: var(--color-primary);

        font-weight: bold;
    }

    .container > * {
        padding: 0.4rem;
        border: var(--color-primary) 1px solid;
        border-top: none;
        /*font-size: 0.8rem;*/
    }

    .container > *:first-child {
        border-top: var(--color-primary) 1px solid;
    }

    .container > *:hover {
        color: var(--color-tertiary);
        cursor: pointer;
    }

</style>