import { Show, type Component, mergeProps, createEffect, on } from "solid-js";
import { createEditor } from "./createEditor";
import type {
  EditorOptions,
  Selection,
} from "@rockerrishabh/rich-text-editor-core";

/**
 * Props for the Editor component
 */
export interface EditorProps extends EditorOptions {
  /** Optional CSS class name for the editor container */
  class?: string;
  /** Callback fired when content changes */
  onChange?: (content: string) => void;
  /** Callback fired when selection changes */
  onSelectionChange?: (selection: Selection) => void;
  /** Callback fired when editor gains focus */
  onFocus?: () => void;
  /** Callback fired when editor loses focus */
  onBlur?: () => void;
}

/**
 * Editor component that wraps the RichTextEditor with Solid integration
 *
 * This component:
 * - Uses the createEditor hook to manage the editor instance
 * - Renders the editor container
 * - Forwards all EditorOptions as props
 * - Emits onChange, onSelectionChange, onFocus, onBlur events
 *
 * @example
 * ```tsx
 * function MyEditor() {
 *   const [content, setContent] = createSignal('');
 *
 *   return (
 *     <Editor
 *       initialContent="Hello World"
 *       placeholder="Start typing..."
 *       onChange={setContent}
 *       onSelectionChange={(sel) => console.log(sel)}
 *     />
 *   );
 * }
 * ```
 */
export const Editor: Component<EditorProps> = (props) => {
  const merged = mergeProps(
    {
      class: "",
    },
    props
  );

  // Extract event handlers and class from props
  const {
    class: className,
    onChange,
    onSelectionChange,
    onFocus,
    onBlur,
    ...editorOptions
  } = merged;

  const { editorRef, editor, isReady } = createEditor(editorOptions);

  createEffect(
    on(
      editor,
      (editorInstance) => {
        if (!editorInstance) return;

        // Create disposable event listeners. We collect cleanup functions only.
        const disposers: (() => void)[] = [];

        const addListener = <T extends Function>(
          event: string,
          handler?: T
        ) => {
          if (!handler) return;
          // editorInstance.on may return a disposer function or void; handle both cases.
          const d = (editorInstance as any).on?.(event, handler);
          if (typeof d === "function") {
            disposers.push(d as () => void);
          } else if (typeof (editorInstance as any).off === "function") {
            disposers.push(() => (editorInstance as any).off(event, handler));
          }
        };

        addListener("change", onChange);
        addListener("selectionChange", onSelectionChange);
        addListener("focus", onFocus);
        addListener("blur", onBlur);

        // Cleanup on effect re-run or component unmount
        return () => {
          disposers.forEach((dispose) => {
            dispose();
          });
        };
      },
      { defer: true }
    )
  );

  return (
    <Show
      when={isReady()}
      fallback={
        <div class={`editor-loading ${className}`}>
          {editorOptions.placeholder || "Loading editor..."}
        </div>
      }
    >
      <div ref={editorRef} class={className} />
    </Show>
  );
};
