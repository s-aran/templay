<script lang="ts">
  import { writeText } from "@tauri-apps/api/clipboard";

  export let content = "";

  let copied_flag = false;
  const onClick = async () => {
    await writeText(content);

    copied_flag = true;
    setTimeout(() => {
      copied_flag = false;
    }, 1000);
  };
</script>

<button on:click={onClick}>{copied_flag ? "Copied!" : "Copy"}</button>
