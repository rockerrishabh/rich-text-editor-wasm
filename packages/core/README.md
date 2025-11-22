# @rockerrishabh/rich-text-editor

Framework-agnostic vanilla JavaScript/TypeScript library for rich text editing, powered by WebAssembly.

## Features

- 🚀 **High Performance**: WebAssembly-powered document model with gap buffer text storage
- 🎨 **WYSIWYG Editing**: Real-time visual formatting as you type
- 📦 **Framework Agnostic**: Works with any JavaScript framework or vanilla JS
- 🔧 **Fully Typed**: Complete TypeScript support with comprehensive type definitions
- 🌐 **Browser Compatible**: Chrome 90+, Firefox 88+, Safari 14+, Edge 90+
- ♿ **Accessible**: WCAG 2.1 AA compliant with keyboard shortcuts and screen reader support
- 🎯 **Tree-Shakeable**: Modular exports for optimal bundle size
- 🎭 **Customizable**: Headless mode with utility functions to build your own UI
- 📝 **Rich Formatting**: Bold, italic, underline, strikethrough, code, links, colors
- 📋 **Block Types**: Headings (h1-h6), lists (bullet/numbered), quotes, code blocks
- ↩️ **Undo/Redo**: Full command history with configurable limits
- 📤 **Export/Import**: JSON, HTML, Markdown, and plain text formats
- 🔍 **Search**: Built-in search and replace functionality
- 📊 **Statistics**: Word count, character count, line count, memory usage

## Installation

```bash
npm install @rockerrishabh/rich-text-editor
```

## Quick Start

### Basic Editor

```typescript
import { RichTextEditor } from "@rockerrishabh/rich-text-editor";

// Create editor instance
const container = document.getElementById("editor-container");
const editor = new RichTextEditor(container, {
  placeholder: "Start typing...",
  onChange: (content) => {
    console.log("Content changed:", content);
  },
});

// Apply formatting
const selection = editor.getSelection();
editor.applyFormat("bold", selection.anchor, selection.focus);

// Export content
const html = editor.toHTML();
const markdown = editor.toMarkdown();
const json = editor.toJSON();
```

### With Initial Content

```typescript
const editor = new RichTextEditor(container, {
  initialContent: "<h1>Hello World</h1><p>Welcome to the editor!</p>",
  initialFormat: "html",
  placeholder: "Start typing...",
});
```

### Headless Mode (Build Your Own UI)

```typescript
import {
  RichTextEditor,
  FormatHelpers,
  BlockHelpers,
  StateHelpers,
  StatsHelpers,
} from "@rockerrishabh/rich-text-editor";

// Create editor without built-in UI
const editor = new RichTextEditor(container, {
  toolbar: false,
  statusBar: false,
  theme: "none",
});

// Create helper instances
const formatHelpers = new FormatHelpers(editor);
const blockHelpers = new BlockHelpers(editor);
const stateHelpers = new StateHelpers(editor);
const statsHelpers = new StatsHelpers(editor);

// Build your own toolbar
const toolbar = document.createElement("div");
toolbar.innerHTML = `
  <button id="bold-btn">Bold</button>
  <button id="italic-btn">Italic</button>
  <button id="h1-btn">Heading 1</button>
  <button id="undo-btn">Undo</button>
`;

// Wire up buttons
document.getElementById("bold-btn").addEventListener("click", () => {
  formatHelpers.toggleFormat("bold");
});

document.getElementById("italic-btn").addEventListener("click", () => {
  formatHelpers.toggleFormat("italic");
});

document.getElementById("h1-btn").addEventListener("click", () => {
  blockHelpers.setBlockType("heading1");
});

document.getElementById("undo-btn").addEventListener("click", () => {
  editor.undo();
});

// Update button states on selection change
editor.on("selectionChange", () => {
  const boldBtn = document.getElementById("bold-btn");
  boldBtn.classList.toggle("active", formatHelpers.isFormatActive("bold"));

  const undoBtn = document.getElementById("undo-btn");
  undoBtn.disabled = !stateHelpers.canUndo();
});

// Display statistics
editor.on("change", () => {
  console.log("Words:", statsHelpers.getWordCount());
  console.log("Characters:", statsHelpers.getCharacterCount());
});
```

## API Overview

### Core Classes

#### RichTextEditor

Main editor class that orchestrates all functionality.

```typescript
class RichTextEditor {
  constructor(container: HTMLElement, options?: EditorOptions);

  // Document operations
  insertText(text: string, position: number): void;
  deleteRange(start: number, end: number): void;
  getContent(): string;
  getLength(): number;

  // Formatting operations
  applyFormat(
    format: FormatType,
    start: number,
    end: number,
    value?: string
  ): void;
  removeFormat(format: FormatType, start: number, end: number): void;
  getFormatsAt(position: number): Format[];

  // Block operations
  setBlockType(blockType: BlockType, start: number, end: number): void;
  getBlockTypeAt(position: number): BlockType;

  // Selection
  setSelection(anchor: number, focus: number): void;
  getSelection(): Selection;

  // History
  undo(): void;
  redo(): void;
  canUndo(): boolean;
  canRedo(): boolean;

  // Serialization
  toJSON(): string;
  toHTML(): string;
  toMarkdown(): string;
  toPlainText(): string;
  fromJSON(json: string): void;
  fromHTML(html: string): void;
  fromMarkdown(markdown: string): void;

  // Search
  find(query: string, caseSensitive?: boolean): SearchMatch[];
  replace(query: string, replacement: string, caseSensitive?: boolean): number;

  // Clipboard
  copy(): ClipboardData;
  cut(): ClipboardData;
  paste(data: ClipboardData): void;

  // Lifecycle
  destroy(): void;

  // Events
  on(event: EditorEventType, callback: EditorEventCallback): void;
  off(event: EditorEventType, callback: EditorEventCallback): void;
}
```

[Full API documentation →](#detailed-api-reference)

### Helper Classes

These classes provide utility functions for building custom UIs:

#### FormatHelpers

```typescript
class FormatHelpers {
  constructor(editor: RichTextEditor);

  isFormatActive(format: SimpleFormatType): boolean;
  getActiveFormats(): Format[];
  applyFormat(format: SimpleFormatType): void;
  applyFormatWithValue(format: ValueFormatType, value: string): void;
  removeFormat(format: FormatType): void;
  toggleFormat(format: SimpleFormatType): void;
  canApplyFormat(): boolean;
}
```

#### BlockHelpers

```typescript
class BlockHelpers {
  constructor(editor: RichTextEditor);

  getCurrentBlockType(): BlockType;
  setBlockType(blockType: BlockType): void;
  isBlockTypeActive(blockType: BlockType): boolean;
  getAvailableBlockTypes(): BlockType[];
}
```

#### StateHelpers

```typescript
class StateHelpers {
  constructor(editor: RichTextEditor);

  canUndo(): boolean;
  canRedo(): boolean;
  hasSelection(): boolean;
  getSelectionInfo(): SelectionInfo;
  isEmpty(): boolean;
  isDirty(): boolean;
  getLength(): number;
  isComposing(): boolean;
}
```

#### StatsHelpers

```typescript
class StatsHelpers {
  constructor(editor: RichTextEditor);

  getCharacterCount(): number;
  getWordCount(): number;
  getLineCount(): number;
  getSelectedCharacterCount(): number;
  getSelectedWordCount(): number;
  getMemoryStats(): MemoryStats;
}
```

#### ExportHelpers

```typescript
class ExportHelpers {
  constructor(editor: RichTextEditor);

  toJSON(): string;
  toHTML(): string;
  toMarkdown(): string;
  toPlainText(): string;
  fromJSON(json: string): void;
  fromHTML(html: string): void;
  fromMarkdown(markdown: string): void;
  fromPlainText(text: string): void;
  copy(): ClipboardData;
  cut(): ClipboardData;
  paste(data: ClipboardData): void;
  pasteHTML(html: string): void;
  pastePlainText(text: string): void;
}
```

### Event Handlers

Internal event handlers (exported for advanced use cases):

- **KeyboardHandler**: Keyboard shortcuts and text input
- **InputHandler**: Text insertion and deletion
- **SelectionHandler**: Selection tracking and updates
- **ClipboardHandler**: Copy, cut, and paste operations
- **IMEHandler**: IME composition for international input

### Types

```typescript
// Format types
type SimpleFormatType =
  | "bold"
  | "italic"
  | "underline"
  | "strikethrough"
  | "code";
type ValueFormatType = "link" | "textColor" | "backgroundColor";
type FormatType = SimpleFormatType | ValueFormatType;

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
  anchor: number;
  focus: number;
}

interface SelectionInfo extends Selection {
  isCollapsed: boolean;
  direction: "forward" | "backward" | "none";
  length: number;
  start: number;
  end: number;
}

// Events
type EditorEventType =
  | "change"
  | "selectionChange"
  | "focus"
  | "blur"
  | "beforeInput"
  | "input"
  | "keydown"
  | "keyup"
  | "paste"
  | "cut"
  | "copy"
  | "compositionStart"
  | "compositionUpdate"
  | "compositionEnd";
```

[Full type definitions →](#type-reference)

## Configuration Options

```typescript
interface EditorOptions {
  // Initial content
  initialContent?: string;
  initialFormat?: "text" | "html" | "markdown" | "json";

  // Features
  toolbar?: boolean | ToolbarConfig;
  statusBar?: boolean | StatusBarConfig;
  spellCheck?: boolean;
  enableHistory?: boolean;
  enableSearch?: boolean;
  enableClipboard?: boolean;
  enableKeyboardShortcuts?: boolean;
  enableIME?: boolean;

  // Behavior
  placeholder?: string;
  readOnly?: boolean;
  maxLength?: number;
  historyLimit?: number;
  autoFocus?: boolean;
  tabBehavior?: "indent" | "tab" | "blur";

  // Styling
  className?: string;
  classNames?: {
    container?: string;
    editor?: string;
    toolbar?: string;
    statusBar?: string;
    placeholder?: string;
  };
  style?: Record<string, string>;
  theme?: "light" | "dark" | "auto" | "none";
  cssVariables?: Record<string, string>;

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

## Examples

### Custom Toolbar Configuration

```typescript
const editor = new RichTextEditor(container, {
  toolbar: {
    position: "top",
    buttons: [
      { type: "bold", tooltip: "Make text bold (Ctrl+B)" },
      { type: "italic" },
      { type: "underline" },
      { type: "separator" },
      { type: "heading1" },
      { type: "heading2" },
      { type: "separator" },
      { type: "bulletList" },
      { type: "numberedList" },
      { type: "separator" },
      { type: "link" },
      { type: "separator" },
      { type: "undo" },
      { type: "redo" },
    ],
    sticky: true,
    showLabels: false,
    buttonSize: "medium",
  },
});
```

### Custom Status Bar

```typescript
const editor = new RichTextEditor(container, {
  statusBar: {
    position: "bottom",
    items: [
      { type: "characterCount", format: "{count} characters" },
      { type: "wordCount", format: "{count} words" },
      { type: "lineCount" },
      {
        type: "custom",
        render: (editor) => {
          const stats = new StatsHelpers(editor).getMemoryStats();
          return `Memory: ${stats.estimatedKB.toFixed(1)} KB`;
        },
      },
    ],
  },
});
```

### Event Handling

```typescript
const editor = new RichTextEditor(container, {
  onChange: (content) => {
    console.log("Content changed:", content);
    localStorage.setItem("draft", content);
  },
  onSelectionChange: (selection) => {
    console.log("Selection:", selection);
  },
  onFocus: () => {
    console.log("Editor focused");
  },
  onBlur: () => {
    console.log("Editor blurred");
  },
  onError: (error) => {
    console.error("Editor error:", error);
  },
});
```

### Import/Export

```typescript
// Export to different formats
const json = editor.toJSON();
const html = editor.toHTML();
const markdown = editor.toMarkdown();
const plainText = editor.toPlainText();

// Import from different formats
editor.fromHTML("<h1>Hello</h1><p>World</p>");
editor.fromMarkdown("# Hello\n\nWorld");
editor.fromJSON('{"version":"1.0","content":"..."}');
```

### Search and Replace

```typescript
// Find all matches
const matches = editor.find("hello", false); // case-insensitive
console.log(`Found ${matches.length} matches`);

// Replace all occurrences
const replacedCount = editor.replace("hello", "hi", false);
console.log(`Replaced ${replacedCount} occurrences`);
```

### Theming

```typescript
// Use built-in theme
const editor = new RichTextEditor(container, {
  theme: "dark",
});

// Custom theme with CSS variables
const editor = new RichTextEditor(container, {
  theme: "none",
  cssVariables: {
    "--rte-editor-bg": "#1e1e1e",
    "--rte-editor-color": "#ffffff",
    "--rte-toolbar-bg": "#2d2d2d",
    "--rte-button-color": "#ffffff",
    "--rte-selection-bg": "#264f78",
    "--rte-focus-outline": "2px solid #0066cc",
  },
});
```

### Read-Only Mode

```typescript
const editor = new RichTextEditor(container, {
  readOnly: true,
  initialContent: "<p>This content cannot be edited</p>",
  initialFormat: "html",
});

// Toggle read-only mode later
editor.setReadOnly(false);
```

## Keyboard Shortcuts

The editor supports standard keyboard shortcuts:

| Shortcut                            | Action             |
| ----------------------------------- | ------------------ |
| `Ctrl+B` / `Cmd+B`                  | Toggle bold        |
| `Ctrl+I` / `Cmd+I`                  | Toggle italic      |
| `Ctrl+U` / `Cmd+U`                  | Toggle underline   |
| `Ctrl+Z` / `Cmd+Z`                  | Undo               |
| `Ctrl+Y` / `Cmd+Y` / `Ctrl+Shift+Z` | Redo               |
| `Ctrl+K` / `Cmd+K`                  | Insert link        |
| `Tab`                               | Indent (in lists)  |
| `Shift+Tab`                         | Outdent (in lists) |

## Browser Support

| Browser       | Minimum Version | Notes                   |
| ------------- | --------------- | ----------------------- |
| Chrome        | 90+             | Full support            |
| Firefox       | 88+             | Full support            |
| Safari        | 14+             | Full support            |
| Edge          | 90+             | Full support            |
| iOS Safari    | 14+             | Full support with touch |
| Chrome Mobile | 90+             | Full support with touch |

### Browser Compatibility Features

- Automatic detection of unsupported browsers with clear error messages
- Normalization of browser-specific behaviors (selection API, IME, clipboard)
- Handling of browser quirks (Firefox block rendering, Safari IME)
- Mobile-optimized touch and virtual keyboard support

## Accessibility

The editor is designed to be fully accessible:

- **WCAG 2.1 AA Compliant**: Meets accessibility standards
- **Keyboard Navigation**: Full keyboard support without mouse
- **Screen Reader Support**: Announces formatting changes and document structure
- **ARIA Attributes**: Proper roles and labels for all interactive elements
- **Focus Management**: Logical focus order through toolbar and content
- **Color Contrast**: Sufficient contrast ratios for all UI elements

## Performance

The editor is optimized for performance:

- **Initialization**: < 100ms for editor ready
- **Typing**: < 16ms per keystroke (60 FPS)
- **Large Documents**: Maintains 60 FPS with 10,000+ characters
- **Memory**: < 50KB per editor instance (excluding WASM module)
- **WASM Module Sharing**: Single WASM instance shared across all editors

### Performance Features

- Lazy rendering for large documents (viewport-based)
- Debounced DOM updates to minimize reflows
- Event delegation for efficient event handling
- Format caching to avoid recalculation
- Automatic memory cleanup on destroy

## Error Handling

The editor provides comprehensive error handling:

```typescript
import { EditorError, ErrorCodes } from "@rockerrishabh/rich-text-editor";

try {
  const editor = new RichTextEditor(container);
} catch (error) {
  if (error instanceof EditorError) {
    switch (error.code) {
      case ErrorCodes.WASM_INIT_FAILED:
        console.error("Failed to initialize WASM module");
        break;
      case ErrorCodes.INVALID_CONTAINER:
        console.error("Invalid container element");
        break;
      case ErrorCodes.BROWSER_NOT_SUPPORTED:
        console.error("Browser not supported");
        break;
      default:
        console.error("Unknown error:", error.message);
    }
  }
}
```

## Security

The editor includes comprehensive security features to protect against XSS attacks and other vulnerabilities:

- **HTML Sanitization**: Automatically sanitizes HTML input during import and paste operations
- **URL Sanitization**: Validates and escapes user-provided URLs in links to prevent `javascript:` and `data:` URL attacks
- **Color Validation**: Validates color values to prevent CSS injection attacks
- **Style Sanitization**: Sanitizes inline styles to block dangerous CSS patterns
- **XSS Protection**: Uses textContent instead of innerHTML where possible

### Content Security Policy (CSP)

The WASM module requires `wasm-unsafe-eval` in your Content Security Policy:

```
Content-Security-Policy: script-src 'self' 'wasm-unsafe-eval';
```

**Recommended CSP configuration for production:**

```
Content-Security-Policy:
  default-src 'self';
  script-src 'self' 'wasm-unsafe-eval';
  style-src 'self' 'unsafe-inline';
  img-src 'self' data: https:;
  font-src 'self' data:;
  connect-src 'self';
  object-src 'none';
  base-uri 'self';
  form-action 'self';
  frame-ancestors 'none';
```

**For detailed security information, including:**

- XSS prevention strategies
- CSP configuration examples for different servers
- Nonce-based CSP support
- Input sanitization utilities
- Security best practices
- Security checklist

**See the [Security Guide](./SECURITY.md) for complete documentation.**

## Framework Adapters

For framework-specific integrations, see:

- [React Adapter](https://www.npmjs.com/package/@rockerrishabh/rich-text-editor-react)
- [Solid Adapter](https://www.npmjs.com/package/@rockerrishabh/rich-text-editor-solid)
- [Svelte Adapter](https://www.npmjs.com/package/@rockerrishabh/rich-text-editor-svelte)

## Detailed API Reference

### RichTextEditor

#### Constructor

```typescript
constructor(container: HTMLElement, options?: EditorOptions)
```

Creates a new editor instance.

**Parameters:**

- `container`: The HTML element to mount the editor in
- `options`: Optional configuration options

**Throws:**

- `EditorError` with code `INVALID_CONTAINER` if container is invalid
- `EditorError` with code `WASM_INIT_FAILED` if WASM initialization fails
- `EditorError` with code `BROWSER_NOT_SUPPORTED` if browser is not supported

#### Document Operations

##### insertText

```typescript
insertText(text: string, position: number): void
```

Inserts text at the specified position.

##### deleteRange

```typescript
deleteRange(start: number, end: number): void
```

Deletes text in the specified range.

##### getContent

```typescript
getContent(): string
```

Returns the current document content as plain text.

##### getLength

```typescript
getLength(): number
```

Returns the length of the document in characters.

#### Formatting Operations

##### applyFormat

```typescript
applyFormat(format: FormatType, start: number, end: number, value?: string): void
```

Applies formatting to the specified range.

**Parameters:**

- `format`: The format type to apply
- `start`: Start position
- `end`: End position
- `value`: Optional value for formats like "link", "textColor", "backgroundColor"

##### removeFormat

```typescript
removeFormat(format: FormatType, start: number, end: number): void
```

Removes formatting from the specified range.

##### getFormatsAt

```typescript
getFormatsAt(position: number): Format[]
```

Returns all active formats at the specified position.

#### Block Operations

##### setBlockType

```typescript
setBlockType(blockType: BlockType, start: number, end: number): void
```

Sets the block type for the specified range.

##### getBlockTypeAt

```typescript
getBlockTypeAt(position: number): BlockType
```

Returns the block type at the specified position.

#### Selection

##### setSelection

```typescript
setSelection(anchor: number, focus: number): void
```

Sets the selection range.

##### getSelection

```typescript
getSelection(): Selection
```

Returns the current selection.

#### History

##### undo

```typescript
undo(): void
```

Undoes the last operation.

##### redo

```typescript
redo(): void
```

Redoes the last undone operation.

##### canUndo

```typescript
canUndo(): boolean
```

Returns whether undo is available.

##### canRedo

```typescript
canRedo(): boolean
```

Returns whether redo is available.

#### Serialization

##### toJSON

```typescript
toJSON(): string
```

Exports the document as JSON.

##### toHTML

```typescript
toHTML(): string
```

Exports the document as HTML.

##### toMarkdown

```typescript
toMarkdown(): string
```

Exports the document as Markdown.

##### toPlainText

```typescript
toPlainText(): string
```

Exports the document as plain text.

##### fromJSON

```typescript
fromJSON(json: string): void
```

Imports a document from JSON.

##### fromHTML

```typescript
fromHTML(html: string): void
```

Imports a document from HTML.

##### fromMarkdown

```typescript
fromMarkdown(markdown: string): void
```

Imports a document from Markdown.

#### Search

##### find

```typescript
find(query: string, caseSensitive?: boolean): SearchMatch[]
```

Finds all matches of the query string.

##### replace

```typescript
replace(query: string, replacement: string, caseSensitive?: boolean): number
```

Replaces all matches of the query string and returns the count.

#### Clipboard

##### copy

```typescript
copy(): ClipboardData
```

Copies the current selection to clipboard.

##### cut

```typescript
cut(): ClipboardData
```

Cuts the current selection to clipboard.

##### paste

```typescript
paste(data: ClipboardData): void
```

Pastes clipboard data at the current position.

#### Lifecycle

##### destroy

```typescript
destroy(): void
```

Destroys the editor instance and frees all resources.

#### Events

##### on

```typescript
on(event: EditorEventType, callback: EditorEventCallback): void
```

Registers an event listener.

##### off

```typescript
off(event: EditorEventType, callback: EditorEventCallback): void
```

Unregisters an event listener.

## Type Reference

### EditorOptions

Complete configuration options for the editor. See [Configuration Options](#configuration-options) section.

### Format Types

```typescript
type SimpleFormatType =
  | "bold"
  | "italic"
  | "underline"
  | "strikethrough"
  | "code";
type ValueFormatType = "link" | "textColor" | "backgroundColor";
type FormatType = SimpleFormatType | ValueFormatType;

type Format =
  | SimpleFormatType
  | { type: "link"; url: string }
  | { type: "textColor"; color: string }
  | { type: "backgroundColor"; color: string };
```

### Block Types

```typescript
type BlockType =
  | "paragraph"
  | "heading1"
  | "heading2"
  | "heading3"
  | "heading4"
  | "heading5"
  | "heading6"
  | "h1"
  | "h2"
  | "h3"
  | "h4"
  | "h5"
  | "h6" // Aliases
  | "bulletList"
  | "numberedList"
  | "blockQuote"
  | "codeBlock";
```

### Selection Types

```typescript
interface Selection {
  anchor: number;
  focus: number;
}

interface SelectionInfo extends Selection {
  isCollapsed: boolean;
  direction: "forward" | "backward" | "none";
  length: number;
  start: number;
  end: number;
}
```

### Event Types

```typescript
type EditorEventType =
  | "change"
  | "selectionChange"
  | "focus"
  | "blur"
  | "beforeInput"
  | "input"
  | "keydown"
  | "keyup"
  | "paste"
  | "cut"
  | "copy"
  | "compositionStart"
  | "compositionUpdate"
  | "compositionEnd";

type EditorEventCallback<T = any> = (data: T) => void;
```

### Other Types

```typescript
interface SearchMatch {
  start: number;
  end: number;
}

interface ClipboardData {
  text: string;
  html: string;
}

interface MemoryStats {
  textLength: number;
  formatRuns: number;
  blocks: number;
  undoCommands: number;
  redoCommands: number;
  estimatedBytes: number;
  estimatedKB: number;
}

interface CompositionRange {
  start: number;
  end: number;
}

interface DirtyRegion {
  start: number;
  end: number;
  html: string;
}
```

## Contributing

Contributions are welcome! Please see the [contributing guidelines](../../CONTRIBUTING.md) for details.

## License

MIT © Rishabh Kumar

## Links

- [GitHub Repository](https://github.com/rockerrishabh/rich-text-editor-wasm)
- [Issue Tracker](https://github.com/rockerrishabh/rich-text-editor-wasm/issues)
- [NPM Package](https://www.npmjs.com/package/@rockerrishabh/rich-text-editor)
- [React Adapter](https://www.npmjs.com/package/@rockerrishabh/rich-text-editor-react)
- [Solid Adapter](https://www.npmjs.com/package/@rockerrishabh/rich-text-editor-solid)
- [Svelte Adapter](https://www.npmjs.com/package/@rockerrishabh/rich-text-editor-svelte)
