<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from "svelte";
  import {
    createEditorStore,
    type EditorStore,
    type EditorState,
  } from "./editorStore";
  import type {
    EditorOptions,
    RichTextEditor,
    Selection,
  } from "@rockerrishabh/rich-text-editor-core";

  const dispatch = createEventDispatcher<{
    change: string;
    selectionChange: Selection;
    focus: void;
    blur: void;
  }>();

  // Public props
  export let className: string = "";
  export let options: EditorOptions = {};

  let containerElement: HTMLDivElement | null = null;
  let editorStore: EditorStore | null = null;
  let editor: RichTextEditor | null = null;
  let unsubscribe: (() => void) | null = null;

  // Reactive state from store
  let state: EditorState | null = null;
  $: isReady = state?.isReady ?? false;

  onMount(() => {
    // Ensure container is present in DOM
    if (!containerElement) return;

    try {
      editorStore = createEditorStore(containerElement, options);
      editor = editorStore.getEditor();

      // Subscribe to store updates
      unsubscribe = editorStore.subscribe((s) => {
        state = s;
      });

      if (editor) {
        // Forward events using the dispatcher
        editor.on("change", (content) => dispatch("change", content));
        editor.on("selectionChange", (selection) =>
          dispatch("selectionChange", selection)
        );
        editor.on("focus", () => dispatch("focus"));
        editor.on("blur", () => dispatch("blur"));
      }
    } catch (error) {
      console.error("Failed to create editor store:", error);
      state = null;
    }
  });

  onDestroy(() => {
    unsubscribe?.();
    if (editorStore) {
      try {
        editorStore.destroy();
      } catch (e) {
        console.error("Error destroying editor store:", e);
      }
      editorStore = null;
    }
  });
</script>

<div class={`rte-wrapper ${className}`}>
  <div bind:this={containerElement} class="rte-container"></div>

  {#if !isReady}
    <div class="editor-loading">
      {options.placeholder || "Loading editor..."}
    </div>
  {/if}
</div>

<style>
  .editor-loading {
    padding: 12px;
    color: #999;
    border: 1px solid #ccc;
    border-radius: 4px;
    min-height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .rte-wrapper {
    position: relative;
  }

  .rte-container {
    min-height: 160px;
  }

  .editor-loading {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(255, 255, 255, 0.9);
  }
</style>
