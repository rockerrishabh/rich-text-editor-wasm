import { useEffect, useRef, useState, type RefObject } from "react";
import {
  RichTextEditor,
  type EditorOptions,
} from "@rockerrishabh/rich-text-editor-core";

/**
 * Return type for useEditor hook
 */
export interface UseEditorReturn {
  /** Ref to attach to the editor container element */
  editorRef: RefObject<HTMLDivElement | null>;
  /** The RichTextEditor instance, or null if not yet initialized */
  editor: RichTextEditor | null;
  /** Whether the editor is ready to use */
  isReady: boolean;
}

/**
 * Hook to create and manage a RichTextEditor instance
 *
 * This hook handles the lifecycle of a RichTextEditor, including:
 * - Creating the editor instance on mount
 * - Initializing with the provided options
 * - Cleaning up (calling destroy()) on unmount
 * - Proper null handling during initialization
 *
 * @param options - Optional editor configuration options
 * @returns Object containing editorRef, editor instance, and isReady state
 *
 * @example
 * ```tsx
 * function MyEditor() {
 *   const { editorRef, editor, isReady } = useEditor({
 *     initialContent: 'Hello World',
 *     placeholder: 'Start typing...',
 *   });
 *
 *   if (!isReady) {
 *     return <div>Loading...</div>;
 *   }
 *
 *   return <div ref={editorRef} />;
 * }
 * ```
 */
export function useEditor(options?: EditorOptions): UseEditorReturn {
  const editorRef = useRef<HTMLDivElement>(null);
  const [editor, setEditor] = useState<RichTextEditor | null>(null);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    // Initialize only when the container element is available
    const container = editorRef.current;
    console.debug("[useEditor] editorRef.current =", container);
    if (!container) return;

    let editorInstance: RichTextEditor | null = null;

    // Initialize editor
    try {
      editorInstance = new RichTextEditor(container, options);
      setEditor(editorInstance);
      console.debug("[useEditor] RichTextEditor initialized", editorInstance);
      setIsReady(true);
    } catch (error) {
      console.error("Failed to initialize editor:", error);
      setIsReady(false);
    }

    // Cleanup function
    return () => {
      if (editorInstance) {
        try {
          editorInstance.destroy();
        } catch (error) {
          console.error("Error destroying editor:", error);
        }
        setEditor(null);
        setIsReady(false);
      }
    };
    // Re-run when the ref's current element changes
  }, [editorRef.current]);

  return {
    editorRef,
    editor,
    isReady,
  };
}
