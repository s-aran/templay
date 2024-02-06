<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  export let data: Config;

  $: {
    console.warn(data);
  }
</script>

<div>
  <h1>設定</h1>

  <h2>外部エディター</h2>

  <h3>コマンドとパラメーター</h3>

  <h4>コマンド</h4>
  <input bind:value={data.external_editor.command} placeholder="command..." />
  <h4>引数</h4>
  <input bind:value={data.external_editor.args} placeholder="arguments..." />
</div>
