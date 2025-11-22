# @rockerrishabh/rich-text-editor-react

React adapter for the WASM-powered WYSIWYG rich text editor. This package provides React hooks and components for seamless integration of the editor into React applications.

## Installation

```bash
npm install @rockerrishabh/rich-text-editor-react
# or
yarn add @rockerrishabh/rich-text-editor-react
# or
pnpm add @rockerrishabh/rich-text-editor-react
```

The vanilla library (`@rockerrishabh/rich-text-editor`) is included as a dependency and will be installed automatically.

## Quick Start

```tsx
import React from "react";
import { Editor } from "@rockerrishabh/rich-text-editor-react";

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

### Basic Editor with Hooks

```tsx
import React from "react";
import { useEditor } from "@rockerrishabh/rich-text-editor-react";

function MyEditor() {
  const { editorRef, editor, isReady } = useEditor({
    placeholder: "Start typing...",
    onChange: (content) => {
      console.log("Content changed:", content);
    },
  });

  return (
    <div>
      {!isReady && <div>Loading editor...</div>}
      <div ref={editorRef} />
    </div>
  );
}
```

### Editor with State Management

```tsx
import React from "react";
import {
  useEditor,
  useEditorState,
} from "@rockerrishabh/rich-text-editor-react";

function EditorWithState() {
  const { editorRef, editor, isReady } = useEditor();
  const state = useEditorState(editor);

  return (
    <div>
      <div ref={editorRef} />
      {isReady && (
        <div className="status-bar">
          <span>Characters: {state.content.length}</span>
          <span>
            Words: {state.content.split(/\s+/).filter(Boolean).length}
          </span>
          <button disabled={!state.canUndo} onClick={() => editor?.undo()}>
            Undo
          </button>
          <button disabled={!state.canRedo} onClick={() => editor?.redo()}>
            Redo
          </button>
        </div>
      )}
    </div>
  );
}
```

### Custom Toolbar

```tsx
import React from "react";
import { useEditor } from "@rockerrishabh/rich-text-editor-react";

function EditorWithCustomToolbar() {
  const { editorRef, editor, isReady } = useEditor();

  const applyFormat = (format: string) => {
    if (!editor) return;
    const selection = editor.getSelection();
    editor.applyFormat(format, selection.anchor, selection.focus);
  };

  return (
    <div>
      {isReady && (
        <div className="custom-toolbar">
          <button onClick={() => applyFormat("bold")}>Bold</button>
          <button onClick={() => applyFormat("italic")}>Italic</button>
          <button onClick={() => applyFormat("underline")}>Underline</button>
          <button onClick={() => editor?.undo()}>Undo</button>
          <button onClick={() => editor?.redo()}>Redo</button>
        </div>
      )}
      <div ref={editorRef} />
    </div>
  );
}
```

### With Initial Content

```tsx
import React from "react";
import { Editor } from "@rockerrishabh/rich-text-editor-react";

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
import React, { useState } from "react";
import { useEditor } from "@rockerrishabh/rich-text-editor-react";

function EditorWithExport() {
  const { editorRef, editor, isReady } = useEditor();
  const [exportedHTML, setExportedHTML] = useState("");

  const handleExport = () => {
    if (!editor) return;
    const html = editor.toHTML();
    setExportedHTML(html);
    console.log("Exported HTML:", html);
  };

  const handleImport = () => {
    if (!editor) return;
    const markdown = "# Imported Content\n\nThis was imported from Markdown!";
    // Import functionality would be implemented via editor methods
  };

  return (
    <div>
      <div ref={editorRef} />
      {isReady && (
        <div>
          <button onClick={handleExport}>Export to HTML</button>
          <button onClick={handleImport}>Import Markdown</button>
          {exportedHTML && (
            <pre>
              <code>{exportedHTML}</code>
            </pre>
          )}
        </div>
      )}
    </div>
  );
}
```

### Event Handling

```tsx
import React from "react";
import { Editor } from "@rockerrishabh/rich-text-editor-react";

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

## API Reference

### Hooks

#### `useEditor(options?: EditorOptions)`

Creates and manages a RichTextEditor instance with automatic cleanup.

**Parameters:**

- `options?: EditorOptions` - Configuration options for the editor (see EditorOptions below)

**Returns:**

```typescript
{
  editorRef: RefObject<HTMLDivElement>; // Ref to attach to container element
  editor: RichTextEditor | null; // Editor instance (null until ready)
  isReady: boolean; // Whether editor is initialized
}
```

**Example:**

```tsx
const { editorRef, editor, isReady } = useEditor({
  placeholder: "Start typing...",
  autoFocus: true,
});
```

#### `useEditorState(editor: RichTextEditor | null)`

Subscribes to editor state changes and returns reactive state.

**Parameters:**

- `editor: RichTextEditor | null` - The editor instance from useEditor

**Returns:**

```typescript
{
  content: string;      // Current document content
  selection: Selection; // Current selection state
  canUndo: boolean;     // Whether undo is available
  canRedo: boolean;     // Whether redo is available
  formats: Format[];    // Active formats at current selection
}
```

**Example:**

```tsx
const { editor } = useEditor();
const state = useEditorState(editor);

console.log(`Can undo: ${state.canUndo}`);
```

### Components

#### `<Editor />`

A complete editor component with built-in lifecycle management.

**Props:**

```typescript
interface EditorProps {
  options?: EditorOptions; // Editor configuration
  className?: string; // CSS class for container
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
  className="my-editor"
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
  style?: React.CSSProperties; // Inline styles
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

When using the `useEditor` hook, you get access to the editor instance with these methods:

```typescript
// Document operations
editor.insertText(text: string, position: number): void;
editor.deleteRange(start: number, end: number): void;
editor.getContent(): string;
editor.getLength(): number;

// Formatting operations
editor.applyFormat(format: FormatType, start: number, end: number): void;
editor.removeFormat(format: FormatType, start: number, end: number): void;
editor.getFormatsAt(position: number): Format[];

// Block operations
editor.setBlockType(blockType: BlockType, start: number, end: number): void;
editor.getBlockTypeAt(position: number): BlockType;

// Selection
editor.setSelection(anchor: number, focus: number): void;
editor.getSelection(): Selection;

// History
editor.undo(): void;
editor.redo(): void;
editor.canUndo(): boolean;
editor.canRedo(): boolean;

// Serialization
editor.toJSON(): string;
editor.toHTML(): string;
editor.toMarkdown(): string;
editor.toPlainText(): string;

// Lifecycle
editor.destroy(): void;

// Events
editor.on(event: EditorEvent, callback: EventCallback): void;
editor.off(event: EditorEvent, callback: EventCallback): void;
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

The `useEditor` hook automatically handles cleanup when the component unmounts. The editor's `destroy()` method is called to release all resources including WASM memory.

If you're manually managing the editor instance, make sure to call `destroy()`:

```tsx
useEffect(() => {
  return () => {
    editor?.destroy();
  };
}, [editor]);
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
} from "@rockerrishabh/rich-text-editor-react";
```

## License

MIT © Rishabh Kumar
