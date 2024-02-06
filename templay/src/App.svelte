<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Editor from "./lib/components/Editor.svelte";
  import Greet from "./lib/components/Greet.svelte";
  import Selector from "./lib/components/Selector.svelte";
  import SettingsDialog from "./lib/components/SettingsDialog.svelte";
  import { onMount } from "svelte";

  let content = "";

  const updateContent = (newContent: string) => {
    content = newContent;
  };

  const button_click = () => {
    console.info(content);
  };

  let showDialog = false;
  let config: Config = {
    version: 0,
    templates: [],
    external_editor: {
      command: "",
      args: "",
    },
  };

  onMount(async () => {
    config = await invoke("load_config");
    console.info(config);
  });

  $: {
    if (!showDialog && config.version > 0) {
      invoke("save_config", { config });
    }
  }
</script>

<SettingsDialog bind:showDialog settingsData={{ config }}></SettingsDialog>

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
