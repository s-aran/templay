<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let command = "";
  let args = "";
</script>

<div>
  <h1>設定</h1>

  <h2>外部エディター</h2>

  <h3>コマンドとパラメーター</h3>

  <h4>コマンド</h4>
  <input bind:value={command} placeholder="command..." />
  <h4>引数</h4>
  <input bind:value={args} placeholder="arguments..." />
</div>
