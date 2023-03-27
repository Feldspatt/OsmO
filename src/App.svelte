<script>
    import {page} from "./lib/stores.mjs";

  import CreateView from "./lib/CreateView.svelte";
  import NavBar from "./lib/NavBar.svelte";
  import HomeView from "./lib/HomeView.svelte";

  let pageValue

  page.subscribe(value => {
    console.log("page value changed to", value)
      pageValue = value
  })

</script>

  <NavBar />
  {#if pageValue === "home"}
    <HomeView />
  {:else if pageValue === "create"}
    <CreateView />
  {/if}


