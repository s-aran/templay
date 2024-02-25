<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Editor from "./lib/components/Editor.svelte";
  import Greet from "./lib/components/Greet.svelte";
  import SettingsDialog from "./lib/components/SettingsDialog.svelte";
  import { onMount } from "svelte";
  import TemplateSelector from "./lib/components/TemplateSelector.svelte";
  import AboutDialog from "./lib/components/AboutDialog.svelte";
  import LaunchExternalEditorButton from "./lib/components/LaunchExternalEditorButton.svelte";

  let content = "";
  const getContent = () => content;
  const updateContent = (newContent: string) => {
    content = newContent;
  };

  const button_click = () => {
    console.info(content);
  };

  let visibleSettingsDialog = false;
  let config: Config = {
    version: 0,
    templates: [],
    external_editor: {
      name: "...",
      command: "",
      args: "",
    },
  };

  let visibleAboutDialog = false;

  const onSelectUpdate = (selected: TemplateObject) => {
    console.info(selected);
    updateContent(selected.content);
  };

  onMount(async () => {
    config = await invoke("load_config");
    console.info(config);
  });

  const showSettingsDialog = () => {
    visibleSettingsDialog = true;
  };

  const showAboutDialog = () => {
    visibleAboutDialog = true;
  };

  $: {
    if (!visibleSettingsDialog && config.version > 0) {
      invoke("save_config", { config });
    }
  }

  let stopExternalEditorInterval = () => {};
</script>

<SettingsDialog
  bind:showDialog={visibleSettingsDialog}
  settingsData={{ config }}
></SettingsDialog>
<AboutDialog bind:showDialog={visibleAboutDialog}></AboutDialog>

<header>
  <div id="selector-area">
    <TemplateSelector
      options={config.templates.map((e, i) => ({
        id: i,
        name: e.name,
        content: e.content,
      }))}
      updateSelectCallback={onSelectUpdate}
    />
  </div>
  <div id="header-buttons">
    <LaunchExternalEditorButton
      {config}
      getText={getContent}
      updateText={updateContent}
      bind:stopExternalEditorInterval
    ></LaunchExternalEditorButton>
    <button on:click={showSettingsDialog}>Settings</button>
    <button on:click={showAboutDialog}>About</button>
  </div>
</header>

<main class="container">
  <Editor
    {content}
    updateContentCallback={updateContent}
    onInput={stopExternalEditorInterval}
  />
</main>

<style lang="scss">
  header {
    position: fixed;
    z-index: 8192;
    left: 0;
    top: 0;
    width: 100%;
    box-sizing: border-box;
  }

  header > #selector-area {
    float: left;
  }

  header > #header-buttons {
    display: flex;
    justify-content: flex-end;
  }

  main {
    height: 20rem;
  }
</style>
