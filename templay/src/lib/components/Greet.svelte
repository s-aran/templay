<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";

  let name = "";
  let greetMsg = "";

  let externalEditorIntervalId: number | undefined = undefined;

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });

    await message(`${greetMsg}`);
  }

  async function load_config() {
    console.info(await invoke("load_config"));
  }

  async function open_with_external_editor() {
    if (!externalEditorIntervalId) {
      clearInterval(externalEditorIntervalId);
    }

    // start interval handler
    externalEditorIntervalId = setInterval(async () => {
      const p = invoke("read_tempfile");
      // name = await invoke("read_tempfile");
      p.then((n) => {
        name = n as string;
      });
      p.catch((e) => {
        console.error(e);
      });
      console.info(name);
    }, 1000);

    await invoke("open_with_external_editor", { text: name });
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input
      id="greet-input"
      placeholder="Enter a name..."
      bind:value={name}
      on:input={() => {
        clearInterval(externalEditorIntervalId);
      }}
    />
    <button type="submit">Greet</button>
  </form>
  <button on:click={load_config}>load</button>
  <button on:click={open_with_external_editor}>Vim</button>
  <p>{greetMsg}</p>
</div>
