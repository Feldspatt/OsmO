<div class="container">
    <h1>
        Create
    </h1>

    <div class="options">
        <button>
            Vertex
        </button>
        <button>
            Edge
        </button>
        <button>
            Tag
        </button>
    </div>
</div>

<style>
    .container {
        display: grid;
        width: 100vw;
        height: 100vh;
        grid-template-rows: 1fr 1fr;
        justify-items: center;
        align-items: center;
    }

    .options {
        display: grid;
        width: 100%;
        grid-template-columns: repeat(3, max-CONTENT);
        justify-CONTENT: space-evenly;
        justify-items: center;
        align-items: center;
    }

    button {
        border: var(--color-primary) 1px dashed;
        padding: 2rem
    }

    button:focus, button:hover {
        color: var(--color-tertiary);
        border-color: var(--color-tertiary);
    }
</style>