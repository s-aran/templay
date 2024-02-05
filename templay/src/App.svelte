<script lang="ts">
  import Editor from "./lib/components/Editor.svelte";
  import Greet from "./lib/components/Greet.svelte";
  import Selector from "./lib/components/Selector.svelte";
  import SettingsDialog from "./lib/components/SettingsDialog.svelte";

  let content = "";

  const updateContent = (newContent: string) => {
    content = newContent;
  };

  const button_click = () => {
    console.info(content);
  };

  let showDialog = false;
  const config = {};
</script>

<SettingsDialog bind:showDialog></SettingsDialog>

<main class="container">
  <div class="row">
    <Greet />
  </div>
  <div>
    <Selector />
  </div>
  <div>
    <Editor updateContentCallback={updateContent} />
  </div>

  <button on:click={button_click}>test</button>
  <button on:click={() => (showDialog = true)}>Settings</button>
</main>
