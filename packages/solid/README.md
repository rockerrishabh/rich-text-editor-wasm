# @rockerrishabh/rich-text-editor-solid

Solid adapter for the WASM-powered WYSIWYG rich text editor. This package provides Solid hooks and components for seamless integration of the editor into SolidJS applications.

## Installation

```bash
npm install @rockerrishabh/rich-text-editor-solid
# or
yarn add @rockerrishabh/rich-text-editor-solid
# or
pnpm add @rockerrishabh/rich-text-editor-solid
```

The vanilla library (`@rockerrishabh/rich-text-editor`) is included as a dependency and will be installed automatically.

## Quick Start

```tsx
import { Editor } from "@rockerrishabh/rich-text-editor-solid";

function App() {
  return (
    <Editor
      options={{
        placeholder: "Start typing...",
        onChange: (content) => console.log(content),
      }}
    />
  );
}

export default App;
```

## Usage Examples

### Basic Editor with createEditor Hook

```tsx
import { createEditor } from "@rockerrishabh/rich-text-editor-solid";
import { Show } from "solid-js";

function MyEditor() {
  const { editorRef, editor, isReady } = createEditor({
    placeholder: "Start typing...",
    onChange: (content) => {
      console.log("Content changed:", content);
    },
  });

  return (
    <div>
      <Show when={!isReady()}>
        <div>Loading editor...</div>
      </Show>
      <div ref={editorRef} />
    </div>
  );
}
```

### Editor with Reactive State

```tsx
import { createEditor } from "@rockerrishabh/rich-text-editor-solid";
import { Show, createEffect } from "solid-js";

function EditorWithState() {
  const { editorRef, editor, isReady } = createEditor();

  createEffect(() => {
    const editorInstance = editor();
    if (editorInstance) {
      console.log("Editor is ready!");
      console.log("Content:", editorInstance.getContent());
    }
  });

  return (
    <div>
      <div ref={editorRef} />
      <Show when={isReady()}>
        <div class="status-bar">
          <span>Length: {editor()?.getLength()}</span>
          <button
            disabled={!editor()?.canUndo()}
            onClick={() => editor()?.undo()}
          >
            Undo
          </button>
          <button
            disabled={!editor()?.canRedo()}
            onClick={() => editor()?.redo()}
          >
            Redo
          </button>
        </div>
      </Show>
    </div>
  );
}
```

### Custom Toolbar

```tsx
import { createEditor } from "@rockerrishabh/rich-text-editor-solid";
import { Show } from "solid-js";

function EditorWithCustomToolbar() {
  const { editorRef, editor, isReady } = createEditor();

  const applyFormat = (format: string) => {
    const editorInstance = editor();
    if (!editorInstance) return;

    const selection = editorInstance.getSelection();
    editorInstance.applyFormat(format, selection.anchor, selection.focus);
  };

  return (
    <div>
      <Show when={isReady()}>
        <div class="custom-toolbar">
          <button onClick={() => applyFormat("bold")}>Bold</button>
          <button onClick={() => applyFormat("italic")}>Italic</button>
          <button onClick={() => applyFormat("underline")}>Underline</button>
          <button onClick={() => editor()?.undo()}>Undo</button>
          <button onClick={() => editor()?.redo()}>Redo</button>
        </div>
      </Show>
      <div ref={editorRef} />
    </div>
  );
}
```

### With Initial Content

```tsx
import { Editor } from "@rockerrishabh/rich-text-editor-solid";

function EditorWithContent() {
  return (
    <Editor
      options={{
        initialContent: "# Welcome\n\nStart editing this document!",
        initialFormat: "markdown",
        placeholder: "Start typing...",
      }}
    />
  );
}
```

### Export and Import

```tsx
import { createEditor } from "@rockerrishabh/rich-text-editor-solid";
import { Show, createSignal } from "solid-js";

function EditorWithExport() {
  const { editorRef, editor, isReady } = createEditor();
  const [exportedHTML, setExportedHTML] = createSignal("");

  const handleExport = () => {
    const editorInstance = editor();
    if (!editorInstance) return;

    const html = editorInstance.toHTML();
    setExportedHTML(html);
    console.log("Exported HTML:", html);
  };

  const handleExportMarkdown = () => {
    const editorInstance = editor();
    if (!editorInstance) return;

    const markdown = editorInstance.toMarkdown();
    console.log("Exported Markdown:", markdown);
  };

  return (
    <div>
      <div ref={editorRef} />
      <Show when={isReady()}>
        <div>
          <button onClick={handleExport}>Export to HTML</button>
          <button onClick={handleExportMarkdown}>Export to Markdown</button>
          <Show when={exportedHTML()}>
            <pre>
              <code>{exportedHTML()}</code>
            </pre>
          </Show>
        </div>
      </Show>
    </div>
  );
}
```

### Event Handling

```tsx
import { Editor } from "@rockerrishabh/rich-text-editor-solid";

function EditorWithEvents() {
  return (
    <Editor
      options={{
        onChange: (content) => console.log("Content:", content),
        onSelectionChange: (selection) => console.log("Selection:", selection),
        onFocus: () => console.log("Editor focused"),
        onBlur: () => console.log("Editor blurred"),
        onError: (error) => console.error("Editor error:", error),
      }}
    />
  );
}
```

### Controlled Component Pattern

```tsx
import { createEditor } from "@rockerrishabh/rich-text-editor-solid";
import { createSignal, createEffect } from "solid-js";

function ControlledEditor() {
  const { editorRef, editor, isReady } = createEditor();
  const [content, setContent] = createSignal("");

  createEffect(() => {
    const editorInstance = editor();
    if (editorInstance) {
      // Sync external state to editor
      const currentContent = editorInstance.getContent();
      if (currentContent !== content()) {
        // Update editor content if needed
      }
    }
  });

  const handleChange = (newContent: string) => {
    setContent(newContent);
    console.log("Content updated:", newContent);
  };

  return (
    <div>
      <div ref={editorRef} />
      <div>
        <p>Content length: {content().length}</p>
      </div>
    </div>
  );
}
```

## API Reference

### Hooks

#### `createEditor(options?: EditorOptions)`

Creates and manages a RichTextEditor instance with automatic cleanup using Solid's `onCleanup`.

**Parameters:**

- `options?: EditorOptions` - Configuration options for the editor (see EditorOptions below)

**Returns:**

```typescript
{
  editorRef: (el: HTMLDivElement) => void;  // Ref function to attach to container
  editor: Accessor<RichTextEditor | null>;  // Accessor for editor instance
  isReady: Accessor<boolean>;               // Accessor for ready state
}
```

**Example:**

```tsx
const { editorRef, editor, isReady } = createEditor({
  placeholder: "Start typing...",
  autoFocus: true,
});
```

### Components

#### `<Editor />`

A complete editor component with built-in lifecycle management.

**Props:**

```typescript
interface EditorProps {
  options?: EditorOptions; // Editor configuration
  class?: string; // CSS class for container
  onChange?: (content: string) => void; // Content change callback
  onSelectionChange?: (selection: Selection) => void; // Selection change callback
  onFocus?: () => void; // Focus event callback
  onBlur?: () => void; // Blur event callback
}
```

**Example:**

```tsx
<Editor
  options={{
    placeholder: "Start typing...",
    maxLength: 10000,
    spellCheck: true,
  }}
  class="my-editor"
  onChange={(content) => console.log(content)}
/>
```

### EditorOptions

Complete configuration options for the editor:

```typescript
interface EditorOptions {
  // Initial content
  initialContent?: string; // Initial content to load
  initialFormat?: "text" | "html" | "markdown" | "json"; // Format of initial content

  // Features
  toolbar?: boolean | ToolbarConfig; // Enable/configure toolbar
  statusBar?: boolean | StatusBarConfig; // Enable/configure status bar
  spellCheck?: boolean; // Browser spell checking (default: true)
  enableHistory?: boolean; // Undo/redo (default: true)
  enableSearch?: boolean; // Search functionality (default: true)
  enableClipboard?: boolean; // Clipboard operations (default: true)
  enableKeyboardShortcuts?: boolean; // Keyboard shortcuts (default: true)
  enableIME?: boolean; // IME composition (default: true)

  // Behavior
  placeholder?: string; // Placeholder text
  readOnly?: boolean; // Read-only mode (default: false)
  maxLength?: number; // Maximum character length
  historyLimit?: number; // Max undo/redo history (default: 100)
  autoFocus?: boolean; // Auto-focus on mount (default: false)
  tabBehavior?: "indent" | "tab" | "blur"; // Tab key behavior (default: "indent")

  // Styling
  className?: string; // CSS class for container
  classNames?: {
    // CSS classes for parts
    container?: string;
    editor?: string;
    toolbar?: string;
    statusBar?: string;
    placeholder?: string;
  };
  style?: Record<string, string>; // Inline styles
  theme?: "light" | "dark" | "auto" | "none"; // Theme preset (default: "light")
  cssVariables?: Record<string, string>; // Custom CSS variables

  // Events
  onChange?: (content: string) => void;
  onSelectionChange?: (selection: Selection) => void;
  onFocus?: () => void;
  onBlur?: () => void;
  onError?: (error: EditorError) => void;
  onBeforeInput?: (event: InputEvent) => boolean | void;
  onKeyDown?: (event: KeyboardEvent) => boolean | void;
  onKeyUp?: (event: KeyboardEvent) => boolean | void;
  onPaste?: (data: ClipboardData) => boolean | void;
  onCut?: (data: ClipboardData) => void;
  onCopy?: (data: ClipboardData) => void;
}
```

### RichTextEditor Methods

When using the `createEditor` hook, you get access to the editor instance with these methods:

```typescript
// Document operations
editor().insertText(text: string, position: number): void;
editor().deleteRange(start: number, end: number): void;
editor().getContent(): string;
editor().getLength(): number;

// Formatting operations
editor().applyFormat(format: FormatType, start: number, end: number): void;
editor().removeFormat(format: FormatType, start: number, end: number): void;
editor().getFormatsAt(position: number): Format[];

// Block operations
editor().setBlockType(blockType: BlockType, start: number, end: number): void;
editor().getBlockTypeAt(position: number): BlockType;

// Selection
editor().setSelection(anchor: number, focus: number): void;
editor().getSelection(): Selection;

// History
editor().undo(): void;
editor().redo(): void;
editor().canUndo(): boolean;
editor().canRedo(): boolean;

// Serialization
editor().toJSON(): string;
editor().toHTML(): string;
editor().toMarkdown(): string;
editor().toPlainText(): string;

// Lifecycle
editor().destroy(): void;

// Events
editor().on(event: EditorEvent, callback: EventCallback): void;
editor().off(event: EditorEvent, callback: EventCallback): void;
```

### Types

```typescript
// Format types
type FormatType =
  | "bold"
  | "italic"
  | "underline"
  | "strikethrough"
  | "code"
  | "link"
  | "textColor"
  | "backgroundColor";

// Block types
type BlockType =
  | "paragraph"
  | "heading1"
  | "heading2"
  | "heading3"
  | "heading4"
  | "heading5"
  | "heading6"
  | "bulletList"
  | "numberedList"
  | "blockQuote"
  | "codeBlock";

// Selection
interface Selection {
  anchor: number; // Start position
  focus: number; // End position
}

// Format object
type Format =
  | "bold"
  | "italic"
  | "underline"
  | "strikethrough"
  | "code"
  | { type: "link"; url: string }
  | { type: "textColor"; color: string }
  | { type: "backgroundColor"; color: string };
```

## Styling

The editor uses CSS variables for theming. You can customize the appearance by overriding these variables:

```css
.my-editor {
  --rte-editor-bg: #ffffff;
  --rte-editor-color: #000000;
  --rte-editor-font-family: "Inter", sans-serif;
  --rte-editor-font-size: 16px;
  --rte-editor-line-height: 1.6;
  --rte-editor-padding: 20px;
  --rte-placeholder-color: #999999;
  --rte-selection-bg: #b3d4fc;
}
```

Or use the `cssVariables` option:

```tsx
<Editor
  options={{
    cssVariables: {
      "--rte-editor-bg": "#fafafa",
      "--rte-editor-color": "#2c3e50",
      "--rte-editor-font-family": "'Inter', sans-serif",
    },
  }}
/>
```

## Keyboard Shortcuts

- **Ctrl+B** / **Cmd+B**: Toggle bold
- **Ctrl+I** / **Cmd+I**: Toggle italic
- **Ctrl+U** / **Cmd+U**: Toggle underline
- **Ctrl+Z** / **Cmd+Z**: Undo
- **Ctrl+Y** / **Cmd+Y** / **Ctrl+Shift+Z**: Redo
- **Ctrl+K** / **Cmd+K**: Insert link
- **Tab**: Indent (in lists)
- **Shift+Tab**: Outdent (in lists)

## Browser Support

- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+
- iOS Safari 14+
- Chrome Mobile 90+

## Memory Management

The `createEditor` hook automatically handles cleanup using Solid's `onCleanup`. The editor's `destroy()` method is called to release all resources including WASM memory when the component is unmounted.

If you're manually managing the editor instance, make sure to call `destroy()`:

```tsx
onCleanup(() => {
  editor()?.destroy();
});
```

## TypeScript Support

This package includes full TypeScript definitions. All types are exported for your convenience:

```typescript
import type {
  EditorOptions,
  RichTextEditor,
  Selection,
  Format,
  FormatType,
  BlockType,
} from "@rockerrishabh/rich-text-editor-solid";
```

## Reactivity

The editor integrates seamlessly with Solid's reactivity system. Use `createEffect` to react to editor changes:

```tsx
import { createEditor } from "@rockerrishabh/rich-text-editor-solid";
import { createEffect } from "solid-js";

function ReactiveEditor() {
  const { editorRef, editor, isReady } = createEditor();

  createEffect(() => {
    if (isReady()) {
      console.log("Editor is ready!");
      const content = editor()?.getContent();
      console.log("Current content:", content);
    }
  });

  return <div ref={editorRef} />;
}
```

## License

MIT © Rishabh Kumar
