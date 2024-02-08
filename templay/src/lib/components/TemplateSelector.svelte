<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  export let options: TemplateObject[] = [];
  export let updateSelectCallback: CallableFunction;
  let selected: TemplateObject = { id: 0, name: "", content: "" };

  function onChange() {
    console.info(selected);
    updateSelectCallback(selected);
  }
</script>

<div>
  <select bind:value={selected} on:change={onChange}>
    {#each options as option}
      <option value={option}>
        {option.name}
      </option>
    {/each}
  </select>
</div>
