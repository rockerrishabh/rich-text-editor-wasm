import { useEditor } from "./useEditor";
import type { EditorOptions, Selection } from "@rockerrishabh/rich-text-editor-core";

/**
 * Props for the Editor component
 */
export interface EditorProps extends EditorOptions {
  /** Optional CSS class name for the editor container */
  className?: string;
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
 * Editor component that wraps the RichTextEditor with React integration
 *
 * This component:
 * - Uses the useEditor hook to manage the editor instance
 * - Renders the editor container
 * - Forwards all EditorOptions as props
 * - Emits onChange, onSelectionChange, onFocus, onBlur events
 *
 * @example
 * ```tsx
 * function MyEditor() {
 *   const [content, setContent] = useState('');
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
export function Editor({
  className = "",
  onChange,
  onSelectionChange,
  onFocus,
  onBlur,
  ...editorOptions
}: EditorProps) {
  const { editorRef, isReady } = useEditor({
    ...editorOptions,
    // Override event handlers to forward to props
    onChange: onChange,
    onSelectionChange: onSelectionChange,
    onFocus: onFocus,
    onBlur: onBlur,
  });

  // Show loading state if editor is not ready
  if (!isReady) {
    return (
      <div className={`editor-loading ${className}`}>
        {editorOptions.placeholder || "Loading editor..."}
      </div>
    );
  }

  return <div ref={editorRef} className={className} />;
}
