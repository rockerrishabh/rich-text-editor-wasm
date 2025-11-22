import { writable, type Writable } from "svelte/store";
import {
  RichTextEditor,
  type EditorOptions,
  type Selection,
  type Format,
} from "@rockerrishabh/rich-text-editor-core";

/**
 * Editor state interface
 */
export interface EditorState {
  /** The RichTextEditor instance */
  editor: RichTextEditor | null;
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
  /** Whether the editor is ready to use */
  isReady: boolean;
}

/**
 * Editor store interface with additional methods
 */
export interface EditorStore extends Writable<EditorState> {
  /** Get the current RichTextEditor instance */
  getEditor: () => RichTextEditor | null;
  /** Set the editor instance (used internally) */
  setEditor: (editor: RichTextEditor | null) => void;
  /** Cleanup method to destroy the editor */
  destroy: () => void;
}

/**
 * Creates a Svelte store wrapping a RichTextEditor instance
 *
 * This function:
 * - Creates a writable store with editor state
 * - Provides methods to get/set the editor instance
 * - Sets up reactive state updates on editor changes
 * - Provides a destroy method for cleanup
 *
 * @param container - The HTML element to attach the editor to
 * @param options - Optional editor configuration options
 * @returns EditorStore - A Svelte writable store with editor state and methods
 *
 * @example
 * ```svelte
 * <script>
 *   import { onMount, onDestroy } from 'svelte';
 *   import { createEditorStore } from '@rockerrishabh/rich-text-editor-svelte';
 *
 *   let container;
 *   let editorStore;
 *
 *   onMount(() => {
 *     editorStore = createEditorStore(container, {
 *       initialContent: 'Hello World',
 *       placeholder: 'Start typing...',
 *     });
 *   });
 *
 *   onDestroy(() => {
 *     editorStore?.destroy();
 *   });
 * </script>
 *
 * <div bind:this={container}></div>
 * ```
 */
export function createEditorStore(
  container: HTMLElement,
  options?: EditorOptions
): EditorStore {
  let editorInstance: RichTextEditor | null = null;

  // Create initial state
  const initialState: EditorState = {
    editor: null,
    content: "",
    selection: { anchor: 0, focus: 0 },
    canUndo: false,
    canRedo: false,
    formats: [],
    isReady: false,
  };

  // Create writable store
  const { subscribe, set, update } = writable<EditorState>(initialState);

  // Function to get current state from editor
  const getStateFromEditor = (editor: RichTextEditor): EditorState => {
    try {
      const selection = editor.getSelection();
      const start = Math.min(selection.anchor, selection.focus);

      return {
        editor,
        content: editor.getContent(),
        selection: selection,
        canUndo: editor.canUndo(),
        canRedo: editor.canRedo(),
        formats: editor.getFormatsAt(start),
        isReady: true,
      };
    } catch (error) {
      console.error("Error getting editor state:", error);
      return {
        editor,
        content: "",
        selection: { anchor: 0, focus: 0 },
        canUndo: false,
        canRedo: false,
        formats: [],
        isReady: true,
      };
    }
  };

  // Function to update store state
  const updateState = () => {
    if (editorInstance) {
      set(getStateFromEditor(editorInstance));
    }
  };

  // Initialize editor
  try {
    editorInstance = new RichTextEditor(container, options);

    // Set up event listeners for reactive updates
    editorInstance.on("change", updateState);
    editorInstance.on("selectionChange", updateState);

    // Update store with initial state
    updateState();
  } catch (error) {
    console.error("Failed to initialize editor:", error);
    set({
      ...initialState,
      isReady: false,
    });
  }

  // Get editor instance
  const getEditor = (): RichTextEditor | null => {
    return editorInstance;
  };

  // Set editor instance (used internally)
  const setEditor = (editor: RichTextEditor | null) => {
    editorInstance = editor;
    if (editor) {
      // Set up event listeners
      editor.on("change", updateState);
      editor.on("selectionChange", updateState);
      updateState();
    } else {
      set(initialState);
    }
  };

  // Cleanup function
  const destroy = () => {
    if (editorInstance) {
      try {
        editorInstance.destroy();
      } catch (error) {
        console.error("Error destroying editor:", error);
      }
      editorInstance = null;
      set(initialState);
    }
  };

  // Return store with additional methods
  return {
    subscribe,
    set,
    update,
    getEditor,
    setEditor,
    destroy,
  };
}
