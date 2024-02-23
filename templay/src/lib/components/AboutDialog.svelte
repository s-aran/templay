<script lang="ts">
  import About from "../pages/About.svelte";

  export let showDialog: boolean;

  let dialog: HTMLDialogElement;
  $: {
    if (dialog && showDialog) dialog.showModal();
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog
  bind:this={dialog}
  on:close={() => (showDialog = false)}
  on:click|self={() => {
    dialog.close();
  }}
>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div>
    <About></About>
  </div>
</dialog>
