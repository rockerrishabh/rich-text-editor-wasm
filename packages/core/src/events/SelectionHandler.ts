import type { EventHandler } from "../core/EventController";
import type { RichTextEditor } from "../core/RichTextEditor";
import type { Selection } from "../types";
import { debounce } from "../utils/performance";

/**
 * Handles selection tracking and syncs with editor state
 */
export class SelectionHandler implements EventHandler {
  private editor: RichTextEditor;
  private editorElement: HTMLElement;
  private lastSelection: Selection | null = null;
  private isUpdating: boolean = false;
  private debouncedHandleSelectionChange: (event: Event) => void;

  constructor(editor: RichTextEditor) {
    this.editor = editor;
    this.editorElement = editor.getEditorElement();

    // Debounce selection change events to reduce overhead
    // 50ms is a good balance between responsiveness and performance
    this.debouncedHandleSelectionChange = debounce(
      this.handleSelectionChangeImmediate.bind(this),
      50
    );
  }

  attach(): void {
    // Listen to document-level selectionchange event (debounced)
    document.addEventListener(
      "selectionchange",
      this.debouncedHandleSelectionChange
    );

    // Also listen to focus/blur events on the editor
    this.editorElement.addEventListener("focus", this.handleFocus);
    this.editorElement.addEventListener("blur", this.handleBlur);
  }

  detach(): void {
    document.removeEventListener(
      "selectionchange",
      this.debouncedHandleSelectionChange
    );
    this.editorElement.removeEventListener("focus", this.handleFocus);
    this.editorElement.removeEventListener("blur", this.handleBlur);
  }

  handleEvent(event: Event): void {
    if (event.type === "selectionchange") {
      this.debouncedHandleSelectionChange(event);
    } else if (event.type === "focus") {
      this.handleFocus(event);
    } else if (event.type === "blur") {
      this.handleBlur(event);
    }
  }

  /**
   * Handle focus event
   */
  private handleFocus = (_event: Event): void => {
    const options = this.editor["options"];
    if (options.onFocus) {
      options.onFocus();
    }
  };

  /**
   * Handle blur event
   */
  private handleBlur = (_event: Event): void => {
    const options = this.editor["options"];
    if (options.onBlur) {
      options.onBlur();
    }
  };

  /**
   * Handle selection change event (immediate, called by debounced wrapper)
   */
  private handleSelectionChangeImmediate(_event: Event): void {
    // Skip if we're updating the selection ourselves
    if (this.isUpdating) {
      return;
    }

    // Get the current DOM selection
    const domSelection = window.getSelection();
    if (!domSelection) {
      return;
    }

    // Check if the selection is within our editor
    if (!this.isSelectionInEditor(domSelection)) {
      return;
    }

    try {
      // Convert DOM selection to editor positions
      const editorSelection = this.domSelectionToEditorSelection(domSelection);

      if (!editorSelection) {
        return;
      }

      // Check if selection has actually changed
      if (this.hasSelectionChanged(editorSelection)) {
        // Update editor state
        this.updateEditorSelection(editorSelection);

        // Store last selection
        this.lastSelection = editorSelection;
      }
    } catch (error) {
      console.error("Error handling selection change:", error);
    }
  }

  /**
   * Check if the DOM selection is within the editor
   */
  private isSelectionInEditor(domSelection: globalThis.Selection): boolean {
    const anchorNode = domSelection.anchorNode;
    const focusNode = domSelection.focusNode;

    if (!anchorNode || !focusNode) {
      return false;
    }

    // Check if both anchor and focus are within the editor element
    return (
      this.editorElement.contains(anchorNode) &&
      this.editorElement.contains(focusNode)
    );
  }

  /**
   * Convert DOM selection to editor selection (character positions)
   */
  private domSelectionToEditorSelection(
    domSelection: globalThis.Selection
  ): Selection | null {
    try {
      const renderer = this.editor.getRenderer();

      // Get anchor position
      const anchorNode = domSelection.anchorNode;
      const anchorOffset = domSelection.anchorOffset;
      const anchor = anchorNode
        ? renderer.getPositionFromNode(anchorNode, anchorOffset)
        : 0;

      // Get focus position
      const focusNode = domSelection.focusNode;
      const focusOffset = domSelection.focusOffset;
      const focus = focusNode
        ? renderer.getPositionFromNode(focusNode, focusOffset)
        : 0;

      return { anchor, focus };
    } catch (error) {
      console.error("Error converting DOM selection:", error);
      return null;
    }
  }

  /**
   * Check if selection has changed from last known selection
   */
  private hasSelectionChanged(newSelection: Selection): boolean {
    if (!this.lastSelection) {
      return true;
    }

    return (
      this.lastSelection.anchor !== newSelection.anchor ||
      this.lastSelection.focus !== newSelection.focus
    );
  }

  /**
   * Update editor selection and emit events
   */
  private updateEditorSelection(selection: Selection): void {
    // Mark as updating to prevent recursion
    this.isUpdating = true;

    try {
      // Update editor state
      const state = this.editor.getState();
      state.setSelection(selection);

      // Update WASM document selection
      this.editor.setSelection(selection.anchor, selection.focus);

      // Get active formats at selection
      const position = Math.min(selection.anchor, selection.focus);
      if (position >= 0 && position <= this.editor.getLength()) {
        const formats = this.editor.getFormatsAt(position);
        state.setActiveFormats(formats);
      }

      // Emit selectionChange event
      const options = this.editor["options"];
      if (options.onSelectionChange) {
        options.onSelectionChange(selection);
      }
    } catch (error) {
      console.error("Error updating editor selection:", error);
    } finally {
      this.isUpdating = false;
    }
  }

  /**
   * Programmatically set the DOM selection from editor positions
   */
  setDOMSelection(anchor: number, focus: number): void {
    this.isUpdating = true;

    try {
      const renderer = this.editor.getRenderer();
      const domSelection = window.getSelection();

      if (!domSelection) {
        return;
      }

      // Get DOM nodes and offsets for anchor and focus
      const anchorNode = renderer.getNodeAtPosition(anchor);
      const focusNode = renderer.getNodeAtPosition(focus);

      if (!anchorNode || !focusNode) {
        return;
      }

      // Create a new range
      const range = document.createRange();
      range.setStart(anchorNode.node, anchorNode.offset);
      range.setEnd(focusNode.node, focusNode.offset);

      // Update DOM selection
      domSelection.removeAllRanges();
      domSelection.addRange(range);

      // Update last selection
      this.lastSelection = { anchor, focus };
    } catch (error) {
      console.error("Error setting DOM selection:", error);
    } finally {
      this.isUpdating = false;
    }
  }
}
