<script lang="ts">
  import Settings from "../pages/Settings.svelte";

  export let showDialog: boolean;

  let dialog: HTMLDialogElement;
  $: if (dialog && showDialog) dialog.showModal();
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<dialog bind:this={dialog} on:close={() => (showDialog = false)}>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div>
    <Settings />
    <hr />
    <!-- svelte-ignore a11y-autofocus -->
    <button autofocus on:click={() => dialog.close()}>OK</button>
  </div>
</dialog>
