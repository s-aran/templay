<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  export let getText: CallableFunction = () => "";
  export let updateText: CallableFunction = (newText: string) => {};
  export let config: Config;

  let externalEditorIntervalId: number | undefined = undefined;
  export const stopExternalEditorInterval = () => {
    if (externalEditorIntervalId) {
      clearInterval(externalEditorIntervalId);
      console.info(externalEditorIntervalId);
      externalEditorIntervalId = undefined;
    }
  };

  async function open_with_external_editor() {
    if (!externalEditorIntervalId) {
      clearInterval(externalEditorIntervalId);
    }

    // start interval handler
    externalEditorIntervalId = setInterval(async () => {
      const p = invoke("read_tempfile");
      // name = await invoke("read_tempfile");
      p.then((n) => {
        console.info("read:", n);
        updateText(n);
        return n;
      });
      p.catch((e) => {
        console.error(e);
      });
    }, 1000);

    await invoke("open_with_external_editor", { text: getText() });
  }
</script>

<div>
  <button on:click={open_with_external_editor}
    >Edit in {config.external_editor.name}</button
  >
</div>
