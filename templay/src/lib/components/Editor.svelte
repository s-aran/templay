<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  export let updateContentCallback: CallableFunction;

  let content = "";
  const onChange = (ev: Event) => {
    updateContentCallback(content);
  };
</script>

<div>
  <textarea bind:value={content} on:input={onChange} />
</div>
