<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";

  let name = "";
  let greetMsg = "";

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });

    await message(`${greetMsg}`);
  }

  async function load_config() {
    console.info(await invoke("load_config"));
  }

  async function open_by_external_editor() {
    const result = (await invoke("open_by_external_editor", {
      text: name,
    })) as string;
    name = result;
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <button on:click={load_config}>load</button>
  <button on:click={open_by_external_editor}>Vim</button>
  <p>{greetMsg}</p>
</div>
