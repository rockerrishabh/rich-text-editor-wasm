<script lang="ts">
  import { Editor } from "@rockerrishabh/rich-text-editor-svelte";

  let content = "<p>Hello from Svelte!</p>";
</script>

<main>
  <h1>Svelte Rich Text Editor</h1>

  <Editor
    initialContent={content}
    on:change={(e) => (content = e.detail)}
  />

  <hr />

  <h2>Output:</h2>
  {@html content}
</main>

<style>
  main {
    font-family: sans-serif;
    padding: 2rem;
  }

  :global(.ProseMirror) {
    border: 1px solid #ccc;
    padding: 1rem;
  }
</style>
