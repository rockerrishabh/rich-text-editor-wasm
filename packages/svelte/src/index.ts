/**
 * @rockerrishabh/rich-text-editor-svelte
 *
 * Svelte adapter for @rockerrishabh/rich-text-editor
 *
 * This package provides Svelte stores and components for integrating
 * the rich text editor into Svelte applications.
 *
 * @packageDocumentation
 */

// Store
export { createEditorStore } from "./editorStore";
export type { EditorStore, EditorState } from "./editorStore";

// Component
export { default as Editor } from "./Editor.svelte";

// Re-export types from vanilla library for convenience
export type {
  RichTextEditor,
  EditorOptions,
  Selection,
  Format,
  FormatType,
  SimpleFormatType,
  ValueFormatType,
  BlockType,
  ToolbarConfig,
  StatusBarConfig,
  EditorEventType,
} from "@rockerrishabh/rich-text-editor-core";
