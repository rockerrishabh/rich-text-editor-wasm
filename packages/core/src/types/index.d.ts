/**
 * Core type definitions for the rich text editor
 */
/**
 * Selection interface
 */
export interface Selection {
    anchor: number;
    focus: number;
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
export type SimpleFormatType = "bold" | "italic" | "underline" | "strikethrough" | "code";
/**
 * Format types that require a value parameter
 */
export type ValueFormatType = "link" | "textColor" | "text-color" | "backgroundColor" | "background-color";
/**
 * All format types
 */
export type FormatType = SimpleFormatType | ValueFormatType;
/**
 * Format object
 */
export type Format = SimpleFormatType | {
    type: "link";
    url: string;
} | {
    type: "textColor" | "text-color";
    color: string;
} | {
    type: "backgroundColor" | "background-color";
    color: string;
};
/**
 * Block types
 */
export type BlockType = "paragraph" | "heading1" | "heading2" | "heading3" | "heading4" | "heading5" | "heading6" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "bulletList" | "numberedList" | "blockQuote" | "codeBlock";
/**
 * Editor event types
 */
export type EditorEventType = "change" | "selectionChange" | "focus" | "blur" | "beforeInput" | "input" | "keydown" | "keyup" | "paste" | "cut" | "copy" | "compositionStart" | "compositionUpdate" | "compositionEnd";
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
    initialContent?: string;
    initialFormat?: "text" | "html" | "markdown" | "json";
    toolbar?: boolean | ToolbarConfig;
    statusBar?: boolean | StatusBarConfig;
    spellCheck?: boolean;
    enableHistory?: boolean;
    enableSearch?: boolean;
    enableClipboard?: boolean;
    enableKeyboardShortcuts?: boolean;
    enableIME?: boolean;
    placeholder?: string;
    readOnly?: boolean;
    maxLength?: number;
    historyLimit?: number;
    autoFocus?: boolean;
    tabBehavior?: "indent" | "tab" | "blur";
    ariaDescribedBy?: string;
    keyboardShortcuts?: KeyboardShortcutConfig;
    logging?: boolean | LoggingConfig;
    debug?: boolean;
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
    /** CSP nonce for inline styles (for nonce-based Content Security Policy) */
    cspNonce?: string;
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
export type ToolbarButtonType = "bold" | "italic" | "underline" | "strikethrough" | "code" | "heading1" | "heading2" | "heading3" | "heading4" | "heading5" | "heading6" | "paragraph" | "bulletList" | "numberedList" | "blockQuote" | "codeBlock" | "link" | "textColor" | "backgroundColor" | "undo" | "redo" | "clear" | "export" | "exportJSON" | "exportHTML" | "exportMarkdown" | "separator" | "custom";
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
export type StatusBarItemType = "characterCount" | "wordCount" | "lineCount" | "selection" | "cursorPosition" | "blockType" | "formats" | "memoryUsage" | "custom";
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
export declare class EditorError extends Error {
    code: string;
    cause?: (Error | unknown) | undefined;
    /**
     * Create a new EditorError
     * @param message - Human-readable error message
     * @param code - Error code from ErrorCodes
     * @param cause - Optional underlying error that caused this error
     */
    constructor(message: string, code: string, cause?: (Error | unknown) | undefined);
    /**
     * Convert error to a plain object for serialization
     */
    toJSON(): {
        name: string;
        message: string;
        code: string;
        stack?: string;
        cause?: any;
    };
    /**
     * Get a user-friendly error message with suggestions
     */
    getUserMessage(): string;
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
export declare const ErrorCodes: {
    readonly WASM_INIT_FAILED: "WASM_INIT_FAILED";
    readonly INVALID_CONTAINER: "INVALID_CONTAINER";
    readonly INVALID_POSITION: "INVALID_POSITION";
    readonly INVALID_RANGE: "INVALID_RANGE";
    readonly INVALID_SELECTION: "INVALID_SELECTION";
    readonly INVALID_FORMAT: "INVALID_FORMAT";
    readonly INVALID_BLOCK_TYPE: "INVALID_BLOCK_TYPE";
    readonly BROWSER_NOT_SUPPORTED: "BROWSER_NOT_SUPPORTED";
    readonly EDITOR_DESTROYED: "EDITOR_DESTROYED";
    readonly CLIPBOARD_ERROR: "CLIPBOARD_ERROR";
    readonly IME_ERROR: "IME_ERROR";
    readonly RENDER_ERROR: "RENDER_ERROR";
    readonly SERIALIZATION_ERROR: "SERIALIZATION_ERROR";
    readonly EVENT_HANDLER_ERROR: "EVENT_HANDLER_ERROR";
    readonly VALIDATION_ERROR: "VALIDATION_ERROR";
    readonly MAX_LENGTH_EXCEEDED: "MAX_LENGTH_EXCEEDED";
    readonly HISTORY_ERROR: "HISTORY_ERROR";
    readonly DOM_SYNC_ERROR: "DOM_SYNC_ERROR";
};
/**
 * Type for error codes
 */
export type ErrorCode = (typeof ErrorCodes)[keyof typeof ErrorCodes];
//# sourceMappingURL=index.d.ts.map