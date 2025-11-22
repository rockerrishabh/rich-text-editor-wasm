/**
 * Core type definitions for the rich text editor
 */

/**
 * Selection interface
 */
export interface Selection {
  anchor: number; // Position where selection started
  focus: number; // Position where selection ends
}

/**
 * Search match result
 */
export interface SearchMatch {
  start: number;
  end: number;
}

/**
 * Clipboard data
 */
export interface ClipboardData {
  text: string;
  html: string;
}

/**
 * Composition range during IME input
 */
export interface CompositionRange {
  start: number;
  end: number;
}

/**
 * Dirty region for incremental rendering
 */
export interface DirtyRegion {
  start: number;
  end: number;
  html: string;
}

/**
 * Memory usage statistics
 */
export interface MemoryStats {
  textLength: number;
  formatRuns: number;
  blocks: number;
  undoCommands: number;
  redoCommands: number;
  estimatedBytes: number;
  estimatedKB: number;
}

/**
 * Simple format types (no parameters)
 */
export type SimpleFormatType =
  | "bold"
  | "italic"
  | "underline"
  | "strikethrough"
  | "code";

/**
 * Format types that require a value parameter
 */
export type ValueFormatType =
  | "link"
  | "textColor"
  | "text-color"
  | "backgroundColor"
  | "background-color";

/**
 * All format types
 */
export type FormatType = SimpleFormatType | ValueFormatType;

/**
 * Format object
 */
export type Format =
  | SimpleFormatType
  | { type: "link"; url: string }
  | { type: "textColor" | "text-color"; color: string }
  | { type: "backgroundColor" | "background-color"; color: string };

/**
 * Block types
 */
export type BlockType =
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
  | "h6"
  | "bulletList"
  | "numberedList"
  | "blockQuote"
  | "codeBlock";

/**
 * Editor event types
 */
export type EditorEventType =
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

/**
 * Editor event callback
 */
export type EditorEventCallback<T = any> = (data: T) => void;

/**
 * Log level for debugging
 */
export type LogLevel = "debug" | "info" | "warn" | "error" | "none";

/**
 * Logging configuration
 */
export interface LoggingConfig {
  /** Enable or disable logging */
  enabled?: boolean;
  /** Log level threshold */
  level?: LogLevel;
  /** Custom log prefix */
  prefix?: string;
  /** Custom logger function */
  logger?: {
    debug?: (...args: any[]) => void;
    info?: (...args: any[]) => void;
    warn?: (...args: any[]) => void;
    error?: (...args: any[]) => void;
  };
}

/**
 * Editor options
 */
export interface EditorOptions {
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

  // Accessibility
  ariaDescribedBy?: string;
  keyboardShortcuts?: KeyboardShortcutConfig;

  // Debugging and logging
  logging?: boolean | LoggingConfig;
  debug?: boolean; // Shorthand for enabling debug logging

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

  // Security
  /** CSP nonce for inline styles (for nonce-based Content Security Policy) */
  cspNonce?: string;

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
  onCompositionStart?: (event: CompositionEvent) => void;
  onCompositionUpdate?: (event: CompositionEvent) => void;
  onCompositionEnd?: (event: CompositionEvent) => void;
}

/**
 * Toolbar configuration
 */
export interface ToolbarConfig {
  position?: "top" | "bottom" | "floating" | "none";
  buttons?: ToolbarButton[];
  groups?: ToolbarButtonGroup[];
  customButtons?: CustomToolbarButton[];
  sticky?: boolean;
  className?: string;
  style?: Record<string, string>;
  showLabels?: boolean;
  buttonSize?: "small" | "medium" | "large";
  tooltips?: boolean | "hover" | "focus";
  showShortcutHints?: boolean;
}

/**
 * Toolbar button types
 */
export type ToolbarButtonType =
  | "bold"
  | "italic"
  | "underline"
  | "strikethrough"
  | "code"
  | "heading1"
  | "heading2"
  | "heading3"
  | "heading4"
  | "heading5"
  | "heading6"
  | "paragraph"
  | "bulletList"
  | "numberedList"
  | "blockQuote"
  | "codeBlock"
  | "link"
  | "textColor"
  | "backgroundColor"
  | "undo"
  | "redo"
  | "clear"
  | "export"
  | "exportJSON"
  | "exportHTML"
  | "exportMarkdown"
  | "separator"
  | "custom";

/**
 * Toolbar button configuration
 */
export interface ToolbarButton {
  type: ToolbarButtonType;
  label?: string;
  icon?: string | HTMLElement;
  tooltip?: string;
  shortcut?: string;
  visible?: boolean;
  disabled?: boolean;
  onClick?: (editor: unknown) => void;
  className?: string;
  style?: Record<string, string>;
}

/**
 * Toolbar button group
 */
export interface ToolbarButtonGroup {
  name: string;
  buttons: ToolbarButton[];
  label?: string;
  className?: string;
}

/**
 * Custom toolbar button
 */
export interface CustomToolbarButton extends Omit<ToolbarButton, "type"> {
  id: string;
  label: string;
  onClick: (editor: unknown) => void;
  isActive?: (editor: unknown) => boolean;
  isDisabled?: (editor: unknown) => boolean;
}

/**
 * Status bar configuration
 */
export interface StatusBarConfig {
  position?: "top" | "bottom" | "none";
  items?: StatusBarItem[];
  className?: string;
  style?: Record<string, string>;
  updateFrequency?: number;
}

/**
 * Status bar item types
 */
export type StatusBarItemType =
  | "characterCount"
  | "wordCount"
  | "lineCount"
  | "selection"
  | "cursorPosition"
  | "blockType"
  | "formats"
  | "memoryUsage"
  | "custom";

/**
 * Status bar item configuration
 */
export interface StatusBarItem {
  type: StatusBarItemType;
  format?: string;
  visible?: boolean;
  render?: (editor: unknown) => string | HTMLElement;
  className?: string;
  style?: Record<string, string>;
}

/**
 * Editor error class
 * Extends the standard Error class with additional context for editor-specific errors
 */
export class EditorError extends Error {
  /**
   * Create a new EditorError
   * @param message - Human-readable error message
   * @param code - Error code from ErrorCodes
   * @param cause - Optional underlying error that caused this error
   */
  constructor(
    message: string,
    public code: string,
    public cause?: Error | unknown
  ) {
    super(message);
    this.name = "EditorError";

    // Maintain proper stack trace for where our error was thrown (only available on V8)
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, EditorError);
    }

    // Include cause in the stack trace if available
    if (cause instanceof Error && cause.stack) {
      this.stack = `${this.stack}\nCaused by: ${cause.stack}`;
    }
  }

  /**
   * Convert error to a plain object for serialization
   */
  toJSON(): {
    name: string;
    message: string;
    code: string;
    stack?: string;
    cause?: any;
  } {
    return {
      name: this.name,
      message: this.message,
      code: this.code,
      stack: this.stack,
      cause:
        this.cause instanceof Error
          ? {
              name: this.cause.name,
              message: this.cause.message,
              stack: this.cause.stack,
            }
          : this.cause,
    };
  }

  /**
   * Get a user-friendly error message with suggestions
   */
  getUserMessage(): string {
    const suggestions: Record<string, string> = {
      [ErrorCodes.WASM_INIT_FAILED]:
        "Failed to initialize the editor. Please ensure WebAssembly is supported in your browser and the WASM module is properly loaded.",
      [ErrorCodes.INVALID_CONTAINER]:
        "The container element is invalid. Please provide a valid HTMLElement.",
      [ErrorCodes.INVALID_POSITION]:
        "The specified position is out of bounds. Please ensure the position is within the document length.",
      [ErrorCodes.INVALID_RANGE]:
        "The specified range is invalid. Please ensure start is less than or equal to end and both are within bounds.",
      [ErrorCodes.INVALID_FORMAT]:
        "The format operation failed. Please check the format type and parameters.",
      [ErrorCodes.INVALID_BLOCK_TYPE]:
        "The block type is invalid. Please use a supported block type.",
      [ErrorCodes.INVALID_SELECTION]:
        "The selection is invalid. Please ensure anchor and focus positions are within bounds.",
      [ErrorCodes.BROWSER_NOT_SUPPORTED]:
        "Your browser is not supported. Please use a modern browser (Chrome 90+, Firefox 88+, Safari 14+, Edge 90+).",
      [ErrorCodes.EDITOR_DESTROYED]:
        "Cannot perform operation on a destroyed editor. Please create a new editor instance.",
      [ErrorCodes.CLIPBOARD_ERROR]:
        "Clipboard operation failed. Please ensure clipboard permissions are granted.",
      [ErrorCodes.IME_ERROR]:
        "IME composition error. Please try again or disable IME temporarily.",
      [ErrorCodes.RENDER_ERROR]:
        "Failed to render the document. Please check the console for more details.",
      [ErrorCodes.SERIALIZATION_ERROR]:
        "Failed to serialize/deserialize the document. Please check the format and data.",
      [ErrorCodes.EVENT_HANDLER_ERROR]:
        "An error occurred in an event handler. Please check your event handler code.",
      [ErrorCodes.VALIDATION_ERROR]:
        "Input validation failed. Please check the provided parameters.",
      [ErrorCodes.MAX_LENGTH_EXCEEDED]:
        "Maximum document length exceeded. Please reduce the content size.",
      [ErrorCodes.HISTORY_ERROR]:
        "Undo/redo operation failed. The history may be corrupted.",
      [ErrorCodes.DOM_SYNC_ERROR]:
        "Failed to synchronize DOM with document state. Please refresh the editor.",
    };

    const suggestion = suggestions[this.code];
    return suggestion
      ? `${this.message}\n\nSuggestion: ${suggestion}`
      : this.message;
  }
}

/**
 * Keyboard shortcut configuration
 */
export interface KeyboardShortcutConfig {
  bold?: string | false;
  italic?: string | false;
  underline?: string | false;
  undo?: string | false;
  redo?: string | false;
  link?: string | false;
  [key: string]: string | false | undefined;
}

/**
 * Error codes for editor operations
 * These codes help identify the type of error that occurred
 */
export const ErrorCodes = {
  // Initialization errors
  WASM_INIT_FAILED: "WASM_INIT_FAILED",
  INVALID_CONTAINER: "INVALID_CONTAINER",

  // Position and range errors
  INVALID_POSITION: "INVALID_POSITION",
  INVALID_RANGE: "INVALID_RANGE",
  INVALID_SELECTION: "INVALID_SELECTION",

  // Format and block errors
  INVALID_FORMAT: "INVALID_FORMAT",
  INVALID_BLOCK_TYPE: "INVALID_BLOCK_TYPE",

  // Browser compatibility errors
  BROWSER_NOT_SUPPORTED: "BROWSER_NOT_SUPPORTED",

  // Lifecycle errors
  EDITOR_DESTROYED: "EDITOR_DESTROYED",

  // Operation errors
  CLIPBOARD_ERROR: "CLIPBOARD_ERROR",
  IME_ERROR: "IME_ERROR",
  RENDER_ERROR: "RENDER_ERROR",
  SERIALIZATION_ERROR: "SERIALIZATION_ERROR",
  EVENT_HANDLER_ERROR: "EVENT_HANDLER_ERROR",

  // Validation errors
  VALIDATION_ERROR: "VALIDATION_ERROR",
  MAX_LENGTH_EXCEEDED: "MAX_LENGTH_EXCEEDED",

  // History errors
  HISTORY_ERROR: "HISTORY_ERROR",

  // DOM synchronization errors
  DOM_SYNC_ERROR: "DOM_SYNC_ERROR",
} as const;

/**
 * Type for error codes
 */
export type ErrorCode = (typeof ErrorCodes)[keyof typeof ErrorCodes];
