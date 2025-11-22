import type { EventHandler } from "../core/EventController";
import type { RichTextEditor } from "../core/RichTextEditor";
import type { KeyboardShortcutConfig } from "../types";
/**
 * Handles keyboard events including shortcuts and special keys
 */
export declare class KeyboardHandler implements EventHandler {
    private editor;
    private editorElement;
    private shortcuts;
    constructor(editor: RichTextEditor);
    attach(): void;
    detach(): void;
    handleEvent(event: Event): void;
    private handleKeyDown;
    private handleKeyUp;
    /**
     * Detect if the user is on macOS
     */
    private isMacOS;
    /**
     * Parse a shortcut string (e.g., "Ctrl+B") into components
     */
    private parseShortcut;
    /**
     * Check if an event matches a shortcut
     */
    private matchesShortcut;
    /**
     * Handle keyboard shortcuts (Ctrl+B, Ctrl+I, etc.)
     */
    private handleShortcuts;
    /**
     * Toggle a format at the current selection
     */
    private toggleFormat;
    /**
     * Get the configured keyboard shortcuts
     */
    getShortcuts(): KeyboardShortcutConfig;
    /**
     * Get a specific shortcut by action name
     */
    getShortcut(action: keyof KeyboardShortcutConfig): string | false;
    /**
     * Handle special keys (Enter, Backspace, Delete, Tab, Arrow keys)
     */
    private handleSpecialKeys;
    /**
     * Indent a list item (Tab in list)
     */
    private indentListItem;
    /**
     * Outdent a list item (Shift+Tab in list)
     */
    private outdentListItem;
    /**
     * Handle arrow key navigation to ensure proper focus management
     */
    private handleArrowKeyNavigation;
}
//# sourceMappingURL=KeyboardHandler.d.ts.map