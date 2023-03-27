<script>
    import Edge_More from "./EdgeMore.svelte";
    import TerminalWindow from "./TerminalWindow.svelte";
    import TextInput from "./TextInput.svelte";
    import ConfirmationWindow from "./ConfirmationWindow.svelte";
    import VertexPanel from "./VertexPanel.svelte";
</script>

<main class="container">

<!--  <div class="graph">-->
<!--    <TextInput />-->
<!--    <Vertex >-->

<!--    </Vertex>-->
<!--    <Edge_More />-->

<!--    <TerminalWindow >-->
<!--      <span slot="title">Vertice</span>-->
<!--      <span slot="CONTENT">Nice to meet you, traveller.</span>-->
<!--    </TerminalWindow>-->

<!--    <ConfirmationWindow>-->
<!--      <span slot="title">Are you sure?</span>-->
<!--      <span slot="CONTENT">This action cannot be undone.</span>-->
<!--    </ConfirmationWindow>-->

<!--  </div>-->

<VertexPanel />


</main>

<style>
    .container {
      padding: 0.5rem;
      display: grid;
      grid-template-columns: 1fr;
      justify-CONTENT: center;
      justify-items: baseline;
    }

    .graph {
        display: grid;
        gap: 1rem;
        grid-template-columns: 1fr 1fr;
        justify-CONTENT: center;
        justify-items: baseline;
    }

</style>