<script>
    import {page} from './stores.mjs'

    function navigateToPage(e) {
        page.set(e.target.dataset.page)
    }

</script>

<div class="container">

<button>
    <h1 class="watermark">OsmOdb</h1>
</button>

<nav>
    <button>
        query
    </button>
    <button on:click={navigateToPage} data-page="create">
        create
    </button>
    <button on:click={navigateToPage} data-page="home">
        explore
    </button>
    <button>
        import
    </button>
    <button>
        connect
    </button>
</nav>

</div>

<style>

    * {
        margin: 0;
        padding: 0;
    }

    .container {
        position: fixed;
        top: 0;
        right: 0;
        padding: 0.5rem;
    }

    nav {
        display:none
    }

    .container:focus-within nav {
        margin-top: 0.5rem;
        display: grid;
        gap: 0.5rem;
        grid-template-columns: max-CONTENT;
        grid-template-rows: repeat(5, max-CONTENT);
        justify-items: end;
        justify-CONTENT: end;
    }

    nav > * {
        /*border-bottom: var(--color-primary) 1px solid;*/
        /*border-right: var(--color-primary) 1px solid;*/
    }
</style>