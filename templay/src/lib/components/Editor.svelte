<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  export let content = "";
  export let updateContentCallback: CallableFunction;
  export let onInput: CallableFunction = () => {};

  const onChange = (ev: Event) => {
    onInput();
    updateContentCallback(content);
  };
</script>

<div>
  <textarea bind:value={content} on:input={onChange} />
</div>

<style lang="scss">
  div {
    width: 100%;
    height: 100%;
  }
  div > textarea {
    width: 100%;
    height: 100%;
  }
</style>
