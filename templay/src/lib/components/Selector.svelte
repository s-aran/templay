<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let selected = {};
  const templates = [
    { id: 1, name: "aaa1", text: "bbb1" },
    { id: 2, name: "aaa2", text: "bbb2" },
    { id: 3, name: "aaa3", text: "bbb3" },
    { id: 4, name: "aaa4", text: "bbb4" },
  ];

  function onChange() {
    console.info(selected);
  }
</script>

<div>
  <select bind:value={selected} on:change={onChange}>
    {#each templates as template}
      <option value={template}>
        {template.text}
      </option>
    {/each}
  </select>
</div>
