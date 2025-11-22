# @rockerrishabh/rich-text-editor-svelte

Svelte adapter for the WASM-powered WYSIWYG rich text editor. This package provides Svelte stores and components for seamless integration of the editor into Svelte applications.

## Installation

```bash
npm install @rockerrishabh/rich-text-editor-svelte
# or
yarn add @rockerrishabh/rich-text-editor-svelte
# or
pnpm add @rockerrishabh/rich-text-editor-svelte
```

The vanilla library (`@rockerrishabh/rich-text-editor`) is included as a dependency and will be installed automatically.

## Quick Start

```svelte
<script>
  import { Editor } from "@rockerrishabh/rich-text-editor-svelte";
</script>

<Editor
  options={{
    placeholder: "Start typing...",
    onChange: (content) => console.log(content),
  }}
/>
```

## Usage Examples

### Basic Editor with createEditorStore

```svelte
<script>
  import { createEditorStore } from "@rockerrishabh/rich-text-editor-svelte";
  import { onDestroy } from "svelte";

  const editorStore = createEditorStore({
    placeholder: "Start typing...",
    onChange: (content) => {
      console.log("Content changed:", content);
    },
  });

  onDestroy(() => {
    editorStore.destroy();
  });
</script>

<div bind:this={editorStore.container} />
```

### Editor Component (Recommended)

```svelte
<script>
  import { Editor } from "@rockerrishabh/rich-text-editor-svelte";

  function handleChange(content) {
    console.log("Content:", content);
  }
</script>

<Editor
  options={{
    placeholder: "Start typing...",
  }}
  on:change={(e) => handleChange(e.detail)}
/>
```

### Editor with Reactive State

```svelte
<script>
  import { Editor } from "@rockerrishabh/rich-text-editor-svelte";

  let editor;
  let canUndo = false;
  let canRedo = false;

  function handleEditorReady(e) {
    editor = e.detail;
    updateState();
  }

  function updateState() {
    if (editor) {
      canUndo = editor.canUndo();
      canRedo = editor.canRedo();
    }
  }

  function handleChange() {
    updateState();
  }
</script>

<Editor
  options={{
    onChange: handleChange,
  }}
  on:ready={handleEditorReady}
/>

{#if editor}
  <div class="status-bar">
    <span>Length: {editor.getLength()}</span>
    <button disabled={!canUndo} on:click={() => editor.undo()}>
      Undo
    </button>
    <button disabled={!canRedo} on:click={() => editor.redo()}>
      Redo
    </button>
  </div>
{/if}
```

### Custom Toolbar

```svelte
<script>
  import { Editor } from "@rockerrishabh/rich-text-editor-svelte";

  let editor;

  function applyFormat(format) {
    if (!editor) return;
    const selection = editor.getSelection();
    editor.applyFormat(format, selection.anchor, selection.focus);
  }
</script>

{#if editor}
  <div class="custom-toolbar">
    <button on:click={() => applyFormat("bold")}>Bold</button>
    <button on:click={() => applyFormat("italic")}>Italic</button>
    <button on:click={() => applyFormat("underline")}>Underline</button>
    <button on:click={() => editor.undo()}>Undo</button>
    <button on:click={() => editor.redo()}>Redo</button>
  </div>
{/if}

<Editor
  on:ready={(e) => (editor = e.detail)}
/>
```

### With Initial Content

```svelte
<script>
  import { Editor } from "@rockerrishabh/rich-text-editor-svelte";
</script>

<Editor
  options={{
    initialContent: "# Welcome\n\nStart editing this document!",
    initialFormat: "markdown",
    placeholder: "Start typing...",
  }}
/>
```

### Export and Import

```svelte
<script>
  import { Editor } from "@rockerrishabh/rich-text-editor-svelte";

  let editor;
  let exportedHTML = "";

  function handleExport() {
    if (!editor) return;
    exportedHTML = editor.toHTML();
    console.log("Exported HTML:", exportedHTML);
  }

  function handleExportMarkdown() {
    if (!editor) return;
    const markdown = editor.toMarkdown();
    console.log("Exported Markdown:", markdown);
  }

  function handleExportJSON() {
    if (!editor) return;
    const json = editor.toJSON();
    console.log("Exported JSON:", json);
  }
</script>

<Editor on:ready={(e) => (editor = e.detail)} />

{#if editor}
  <div>
    <button on:click={handleExport}>Export to HTML</button>
    <button on:click={handleExportMarkdown}>Export to Markdown</button>
    <button on:click={handleExportJSON}>Export to JSON</button>

    {#if exportedHTML}
      <pre><code>{exportedHTML}</code></pre>
    {/if}
  </div>
{/if}
```

### Event Handling

```svelte
<script>
  import { Editor } from "@rockerrishabh/rich-text-editor-svelte";

  function handleChange(e) {
    console.log("Content:", e.detail);
  }

  function handleSelectionChange(e) {
    console.log("Selection:", e.detail);
  }

  function handleFocus() {
    console.log("Editor focused");
  }

  function handleBlur() {
    console.log("Editor blurred");
  }
</script>

<Editor
  on:change={handleChange}
  on:selectionChange={handleSelectionChange}
  on:focus={handleFocus}
  on:blur={handleBlur}
/>
```

### Using Store Pattern

```svelte
<script>
  import { createEditorStore } from "@rockerrishabh/rich-text-editor-svelte";
  import { onDestroy } from "svelte";

  const store = createEditorStore({
    placeholder: "Start typing...",
  });

  $: editor = $store.editor;
  $: isReady = $store.isReady;

  onDestroy(() => {
    store.destroy();
  });
</script>

{#if !isReady}
  <div>Loading editor...</div>
{/if}

<div bind:this={store.container} />

{#if isReady && editor}
  <div>
    <button on:click={() => editor.undo()}>Undo</button>
    <button on:click={() => editor.redo()}>Redo</button>
  </div>
{/if}
```

## API Reference

### Functions

#### `createEditorStore(options?: EditorOptions)`

Creates a Svelte store that manages an editor instance.

**Parameters:**

- `options?: EditorOptions` - Configuration options for the editor (see EditorOptions below)

**Returns:**

```typescript
{
  subscribe: (callback: (state: EditorState) => void) => () => void;
  getEditor: () => RichTextEditor | null;
  setEditor: (editor: RichTextEditor) => void;
  destroy: () => void;
  container: HTMLDivElement | null;  // Container element to bind
}
```

**Store State:**

```typescript
{
  editor: RichTextEditor | null; // Editor instance
  isReady: boolean; // Whether editor is initialized
}
```

**Example:**

```svelte
<script>
  import { createEditorStore } from "@rockerrishabh/rich-text-editor-svelte";
  import { onDestroy } from "svelte";

  const store = createEditorStore({
    placeholder: "Start typing...",
  });

  onDestroy(() => store.destroy());
</script>

<div bind:this={store.container} />
```

### Components

#### `<Editor />`

A complete editor component with built-in lifecycle management.

**Props:**

```typescript
interface EditorProps {
  options?: EditorOptions; // Editor configuration
  class?: string; // CSS class for container
}
```

**Events:**

```typescript
on:ready={(e) => { /* e.detail is RichTextEditor */ }}
on:change={(e) => { /* e.detail is content string */ }}
on:selectionChange={(e) => { /* e.detail is Selection */ }}
on:focus={() => { /* editor focused */ }}
on:blur={() => { /* editor blurred */ }}
```

**Example:**

```svelte
<Editor
  options={{
    placeholder: "Start typing...",
    maxLength: 10000,
  }}
  class="my-editor"
  on:change={(e) => console.log(e.detail)}
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

When you get the editor instance (via `on:ready` event or store), you have access to these methods:

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

```svelte
<Editor
  options={{
    cssVariables: {
      '--rte-editor-bg': '#fafafa',
      '--rte-editor-color': '#2c3e50',
      '--rte-editor-font-family': "'Inter', sans-serif",
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

When using the `createEditorStore` function, make sure to call `destroy()` in `onDestroy`:

```svelte
<script>
  import { createEditorStore } from "@rockerrishabh/rich-text-editor-svelte";
  import { onDestroy } from "svelte";

  const store = createEditorStore();

  onDestroy(() => {
    store.destroy();
  });
</script>
```

The `<Editor>` component handles cleanup automatically.

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
} from "@rockerrishabh/rich-text-editor-svelte";
```

## Svelte Stores

The editor integrates with Svelte's store system. You can use the `$` syntax to access reactive state:

```svelte
<script>
  import { createEditorStore } from "@rockerrishabh/rich-text-editor-svelte";

  const store = createEditorStore();

  $: editor = $store.editor;
  $: isReady = $store.isReady;
</script>

{#if isReady}
  <p>Editor is ready!</p>
{/if}
```

## License

MIT © Rishabh Kumar
