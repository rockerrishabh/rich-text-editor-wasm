/**
 * @rockerrishabh/rich-text-editor-react
 *
 * React adapter for @rockerrishabh/rich-text-editor
 *
 * This package provides React hooks and components for integrating
 * the rich text editor into React applications.
 *
 * @packageDocumentation
 */

// Hooks
export { useEditor } from "./useEditor";
export type { UseEditorReturn } from "./useEditor";
export { useEditorState } from "./useEditorState";
export type { EditorState } from "./useEditorState";

// Components
export { Editor } from "./Editor";
export type { EditorProps } from "./Editor";

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
