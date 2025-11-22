import { useEffect, useState } from "react";
import type {
  RichTextEditor,
  Selection,
  Format,
} from "@rockerrishabh/rich-text-editor-core";

/**
 * Editor state returned by useEditorState hook
 */
export interface EditorState {
  /** Current document content as plain text */
  content: string;
  /** Current selection */
  selection: Selection;
  /** Whether undo operation is available */
  canUndo: boolean;
  /** Whether redo operation is available */
  canRedo: boolean;
  /** Active formats at current selection */
  formats: Format[];
}

/**
 * Hook to subscribe to editor state changes and get reactive state
 *
 * This hook:
 * - Subscribes to editor change and selectionChange events
 * - Triggers re-renders when the editor state changes
 * - Handles null editor cases gracefully
 * - Cleans up event listeners on unmount
 *
 * @param editor - The RichTextEditor instance (can be null during initialization)
 * @returns EditorState object with current editor state
 *
 * @example
 * ```tsx
 * function MyEditor() {
 *   const { editor } = useEditor();
 *   const state = useEditorState(editor);
 *
 *   return (
 *     <div>
 *       <div>Characters: {state.content.length}</div>
 *       <button disabled={!state.canUndo} onClick={() => editor?.undo()}>
 *         Undo
 *       </button>
 *       <button disabled={!state.canRedo} onClick={() => editor?.redo()}>
 *         Redo
 *       </button>
 *       <div>Active formats: {state.formats.join(', ')}</div>
 *     </div>
 *   );
 * }
 * ```
 */
export function useEditorState(editor: RichTextEditor | null): EditorState {
  const [state, setState] = useState<EditorState>({
    content: "",
    selection: { anchor: 0, focus: 0 },
    canUndo: false,
    canRedo: false,
    formats: [],
  });

  useEffect(() => {
    if (!editor) return;

    // Function to update state from editor
    const updateState = () => {
      try {
        const selection = editor.getSelection();
        const start = Math.min(selection.anchor, selection.focus);

        setState({
          content: editor.getContent(),
          selection: selection,
          canUndo: editor.canUndo(),
          canRedo: editor.canRedo(),
          formats: editor.getFormatsAt(start),
        });
      } catch (error) {
        console.error("Error updating editor state:", error);
      }
    };

    // Initial state
    updateState();

    // Subscribe to editor events
    const handleChange = () => updateState();
    const handleSelectionChange = () => updateState();

    editor.on("change", handleChange);
    editor.on("selectionChange", handleSelectionChange);

    // Cleanup function
    return () => {
      editor.off("change", handleChange);
      editor.off("selectionChange", handleSelectionChange);
    };
  }, [editor]);

  return state;
}
