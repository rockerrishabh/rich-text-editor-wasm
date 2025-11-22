import type { EventHandler } from "../core/EventController";
import type { RichTextEditor } from "../core/RichTextEditor";
import type { KeyboardShortcutConfig } from "../types";

/**
 * Default keyboard shortcuts
 */
const DEFAULT_SHORTCUTS: Required<KeyboardShortcutConfig> = {
  bold: "Ctrl+B",
  italic: "Ctrl+I",
  underline: "Ctrl+U",
  undo: "Ctrl+Z",
  redo: "Ctrl+Shift+Z",
  link: "Ctrl+K",
};

/**
 * Handles keyboard events including shortcuts and special keys
 */
export class KeyboardHandler implements EventHandler {
  private editor: RichTextEditor;
  private editorElement: HTMLElement;
  private shortcuts: KeyboardShortcutConfig;

  constructor(editor: RichTextEditor) {
    this.editor = editor;
    this.editorElement = editor.getEditorElement();

    // Get custom shortcuts from options or use defaults
    const options = this.editor["options"];
    this.shortcuts = {
      ...DEFAULT_SHORTCUTS,
      ...(options.keyboardShortcuts || {}),
    };
  }

  attach(): void {
    this.editorElement.addEventListener("keydown", this.handleKeyDown);
    this.editorElement.addEventListener("keyup", this.handleKeyUp);
  }

  detach(): void {
    this.editorElement.removeEventListener("keydown", this.handleKeyDown);
    this.editorElement.removeEventListener("keyup", this.handleKeyUp);
  }

  handleEvent(event: Event): void {
    if (event.type === "keydown") {
      this.handleKeyDown(event as KeyboardEvent);
    } else if (event.type === "keyup") {
      this.handleKeyUp(event as KeyboardEvent);
    }
  }

  private handleKeyDown = (event: KeyboardEvent): void => {
    // Check if keyboard shortcuts are enabled
    const options = this.editor["options"];
    if (options.enableKeyboardShortcuts === false) {
      return;
    }

    // Call user's onKeyDown handler if provided
    if (options.onKeyDown) {
      const result = options.onKeyDown(event);
      if (result === false) {
        event.preventDefault();
        return;
      }
    }

    // Handle keyboard shortcuts
    const handled = this.handleShortcuts(event);
    if (handled) {
      event.preventDefault();
      return;
    }

    // Handle special keys
    this.handleSpecialKeys(event);
  };

  private handleKeyUp = (event: KeyboardEvent): void => {
    // Call user's onKeyUp handler if provided
    const options = this.editor["options"];
    if (options.onKeyUp) {
      const result = options.onKeyUp(event);
      if (result === false) {
        event.preventDefault();
        return;
      }
    }
  };

  /**
   * Detect if the user is on macOS
   */
  private isMacOS(): boolean {
    // Try modern API first
    if ("userAgentData" in navigator) {
      const uaData = (navigator as { userAgentData?: { platform?: string } })
        .userAgentData;
      if (uaData && uaData.platform) {
        return uaData.platform.toLowerCase().includes("mac");
      }
    }

    // Fallback to userAgent
    return /Mac|iPhone|iPad|iPod/.test(navigator.userAgent);
  }

  /**
   * Parse a shortcut string (e.g., "Ctrl+B") into components
   */
  private parseShortcut(shortcut: string): {
    key: string;
    ctrl: boolean;
    shift: boolean;
    alt: boolean;
    meta: boolean;
  } | null {
    if (!shortcut) return null;

    const parts = shortcut.split("+").map((p) => p.trim().toLowerCase());
    const result = {
      key: "",
      ctrl: false,
      shift: false,
      alt: false,
      meta: false,
    };

    for (const part of parts) {
      if (part === "ctrl" || part === "control") {
        result.ctrl = true;
      } else if (part === "shift") {
        result.shift = true;
      } else if (part === "alt" || part === "option") {
        result.alt = true;
      } else if (part === "meta" || part === "cmd" || part === "command") {
        result.meta = true;
      } else {
        result.key = part;
      }
    }

    return result.key ? result : null;
  }

  /**
   * Check if an event matches a shortcut
   */
  private matchesShortcut(
    event: KeyboardEvent,
    shortcut: string | false
  ): boolean {
    if (shortcut === false) return false;

    const parsed = this.parseShortcut(shortcut);
    if (!parsed) return false;

    const isMac = this.isMacOS();
    const eventKey = event.key.toLowerCase();

    // On Mac, Ctrl in shortcut string means Cmd key
    // On Windows/Linux, Ctrl means Ctrl key
    const modKeyMatch = isMac
      ? parsed.ctrl
        ? event.metaKey
        : true
      : parsed.ctrl
      ? event.ctrlKey
      : true;

    return (
      eventKey === parsed.key &&
      modKeyMatch &&
      event.shiftKey === parsed.shift &&
      event.altKey === parsed.alt &&
      (isMac ? event.metaKey === parsed.meta : true)
    );
  }

  /**
   * Handle keyboard shortcuts (Ctrl+B, Ctrl+I, etc.)
   */
  private handleShortcuts(event: KeyboardEvent): boolean {
    // Check if history is enabled
    const options = this.editor["options"];
    const historyEnabled = options.enableHistory !== false;

    // Get current selection
    const selection = this.editor.getSelection();
    const hasSelection = selection.anchor !== selection.focus;

    // Bold
    if (
      this.shortcuts.bold &&
      this.matchesShortcut(event, this.shortcuts.bold)
    ) {
      if (hasSelection) {
        this.toggleFormat("bold");
      }
      return true;
    }

    // Italic
    if (
      this.shortcuts.italic &&
      this.matchesShortcut(event, this.shortcuts.italic)
    ) {
      if (hasSelection) {
        this.toggleFormat("italic");
      }
      return true;
    }

    // Underline
    if (
      this.shortcuts.underline &&
      this.matchesShortcut(event, this.shortcuts.underline)
    ) {
      if (hasSelection) {
        this.toggleFormat("underline");
      }
      return true;
    }

    // Undo
    if (
      this.shortcuts.undo &&
      this.matchesShortcut(event, this.shortcuts.undo)
    ) {
      if (historyEnabled) {
        this.editor.undo();
      }
      return true;
    }

    // Redo
    if (
      this.shortcuts.redo &&
      this.matchesShortcut(event, this.shortcuts.redo)
    ) {
      if (historyEnabled) {
        this.editor.redo();
      }
      return true;
    }

    // Link
    if (
      this.shortcuts.link &&
      this.matchesShortcut(event, this.shortcuts.link)
    ) {
      if (hasSelection) {
        // For now, just prevent default
        // In a full implementation, this would open a link dialog
        console.log("Insert link shortcut triggered");
      }
      return true;
    }

    return false;
  }

  /**
   * Toggle a format at the current selection
   */
  private toggleFormat(format: "bold" | "italic" | "underline"): void {
    const selection = this.editor.getSelection();
    const start = Math.min(selection.anchor, selection.focus);
    const end = Math.max(selection.anchor, selection.focus);

    // Check if format is already applied
    const formats = this.editor.getFormatsAt(start);
    const isActive = formats.some((f) =>
      typeof f === "string" ? f === format : false
    );

    if (isActive) {
      this.editor.removeFormat(format, start, end);
    } else {
      this.editor.applyFormat(format, start, end);
    }

    // Note: Announcements are handled in applyFormat/removeFormat methods
  }

  /**
   * Get the configured keyboard shortcuts
   */
  getShortcuts(): KeyboardShortcutConfig {
    return { ...this.shortcuts };
  }

  /**
   * Get a specific shortcut by action name
   */
  getShortcut(action: keyof KeyboardShortcutConfig): string | false {
    return this.shortcuts[action] ?? false;
  }

  /**
   * Handle special keys (Enter, Backspace, Delete, Tab, Arrow keys)
   */
  private handleSpecialKeys(event: KeyboardEvent): void {
    const options = this.editor["options"];

    // Handle Tab key based on tabBehavior option
    if (event.key === "Tab" && !event.shiftKey) {
      const tabBehavior = options.tabBehavior || "indent";

      if (tabBehavior === "indent") {
        // Insert tab character or indent list
        event.preventDefault();
        const selection = this.editor.getSelection();
        const position = Math.min(selection.anchor, selection.focus);

        // Check if we're in a list
        const blockType = this.editor.getBlockTypeAt(position);
        if (blockType === "bulletList" || blockType === "numberedList") {
          // Indent list item
          this.indentListItem(position);
        } else {
          // Insert tab character
          this.editor.insertText("\t", position);
        }
      } else if (tabBehavior === "tab") {
        // Insert tab character
        event.preventDefault();
        const selection = this.editor.getSelection();
        const position = Math.min(selection.anchor, selection.focus);
        this.editor.insertText("\t", position);
      } else if (tabBehavior === "blur") {
        // Allow default tab behavior (focus next element)
        // Don't prevent default
      }
    }

    // Handle Shift+Tab for outdenting
    if (event.key === "Tab" && event.shiftKey) {
      event.preventDefault();
      const selection = this.editor.getSelection();
      const position = Math.min(selection.anchor, selection.focus);

      // Check if we're in a list
      const blockType = this.editor.getBlockTypeAt(position);
      if (blockType === "bulletList" || blockType === "numberedList") {
        // Outdent list item
        this.outdentListItem(position);
      }
    }

    // Handle arrow keys in lists for better navigation
    if (
      event.key === "ArrowUp" ||
      event.key === "ArrowDown" ||
      event.key === "ArrowLeft" ||
      event.key === "ArrowRight"
    ) {
      // Allow default arrow key behavior
      // The browser's contenteditable handles this well
      // We just ensure the editor maintains focus
      this.handleArrowKeyNavigation(event);
    }

    // Handle Enter key
    if (event.key === "Enter") {
      // Let the browser handle Enter for now
      // In a full implementation, this would handle list continuation, etc.
      const selection = this.editor.getSelection();
      const position = Math.min(selection.anchor, selection.focus);
      const blockType = this.editor.getBlockTypeAt(position);

      if (blockType === "bulletList" || blockType === "numberedList") {
        // Browser will handle list continuation
        // We could enhance this in the future
      }
    }

    // Handle Backspace
    if (event.key === "Backspace") {
      // Let the browser handle Backspace for now
      // In a full implementation, this would handle special cases like
      // deleting at the start of a list item
    }

    // Handle Delete
    if (event.key === "Delete") {
      // Let the browser handle Delete for now
    }
  }

  /**
   * Indent a list item (Tab in list)
   */
  private indentListItem(position: number): void {
    // Placeholder for list indentation
    // This would require more complex logic to handle nested lists
    console.log("List indentation at position:", position);
    this.editor.announce("List item indented");
  }

  /**
   * Outdent a list item (Shift+Tab in list)
   */
  private outdentListItem(position: number): void {
    // Placeholder for list outdentation
    // This would require more complex logic to handle nested lists
    console.log("List outdentation at position:", position);
    this.editor.announce("List item outdented");
  }

  /**
   * Handle arrow key navigation to ensure proper focus management
   */
  private handleArrowKeyNavigation(event: KeyboardEvent): void {
    // Ensure the editor maintains focus during arrow key navigation
    // The browser's contenteditable handles the actual navigation
    // We just need to ensure accessibility features work correctly

    const selection = this.editor.getSelection();
    const position = Math.min(selection.anchor, selection.focus);
    const blockType = this.editor.getBlockTypeAt(position);

    // For lists, we could announce the current item when navigating
    // but this might be too verbose, so we'll keep it simple for now
    if (
      (blockType === "bulletList" || blockType === "numberedList") &&
      (event.key === "ArrowUp" || event.key === "ArrowDown")
    ) {
      // Navigation within lists - browser handles this well
      // We maintain focus order by not preventing default
    }
  }
}
