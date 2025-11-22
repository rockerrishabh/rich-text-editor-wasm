/**
 * @rockerrishabh/rich-text-editor-solid
 *
 * Solid adapter for the rich-text-editor WASM-powered WYSIWYG editor.
 */

export { createEditor } from "./createEditor";
export { Editor } from "./Editor";

// Re-export types from vanilla library
export type {
  EditorOptions,
  RichTextEditor,
  Selection,
  Format,
  FormatType,
  SimpleFormatType,
  ValueFormatType,
  BlockType,
  EditorEventType,
  EditorEventCallback,
} from "@rockerrishabh/rich-text-editor-core";
