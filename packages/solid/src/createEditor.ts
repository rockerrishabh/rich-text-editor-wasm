import { createSignal, onCleanup, type Accessor } from "solid-js";
import {
  RichTextEditor,
  type EditorOptions,
} from "@rockerrishabh/rich-text-editor-core";

/**
 * Return type for createEditor hook
 */
export interface CreateEditorReturn {
  /** Function to attach to the editor container element */
  editorRef: (el: HTMLDivElement) => void;
  /** Accessor for the RichTextEditor instance, or null if not yet initialized */
  editor: Accessor<RichTextEditor | null>;
  /** Accessor for whether the editor is ready to use */
  isReady: Accessor<boolean>;
}

/**
 * Hook to create and manage a RichTextEditor instance
 *
 * This hook handles the lifecycle of a RichTextEditor, including:
 * - Creating the editor instance when the ref is attached
 * - Initializing with the provided options
 * - Cleaning up (calling destroy()) on cleanup
 * - Proper null handling during initialization
 *
 * @param options - Optional editor configuration options
 * @returns Object containing editorRef function, editor accessor, and isReady accessor
 *
 * @example
 * ```tsx
 * function MyEditor() {
 *   const { editorRef, editor, isReady } = createEditor({
 *     initialContent: 'Hello World',
 *     placeholder: 'Start typing...',
 *   });
 *
 *   return (
 *     <Show when={isReady()} fallback={<div>Loading...</div>}>
 *       <div ref={editorRef} />
 *     </Show>
 *   );
 * }
 * ```
 */
export function createEditor(options?: EditorOptions): CreateEditorReturn {
  const [editor, setEditor] = createSignal<RichTextEditor | null>(null);
  const [isReady, setIsReady] = createSignal(false);

  let editorInstance: RichTextEditor | null = null;

  const editorRef = (el: HTMLDivElement) => {
    if (!el) return;

    // Initialize editor
    try {
      editorInstance = new RichTextEditor(el, options);
      setEditor(editorInstance);
      setIsReady(true);
    } catch (error) {
      console.error("Failed to initialize editor:", error);
      setIsReady(false);
    }
  };

  // Cleanup function
  onCleanup(() => {
    if (editorInstance) {
      try {
        editorInstance.destroy();
      } catch (error) {
        console.error("Error destroying editor:", error);
      }
      setEditor(null);
      setIsReady(false);
    }
  });

  return {
    editorRef,
    editor,
    isReady,
  };
}
