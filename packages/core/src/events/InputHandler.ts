import type { EventHandler } from "../core/EventController";
import type { RichTextEditor } from "../core/RichTextEditor";

/**
 * Handles text input events and syncs with WASM document
 */
export class InputHandler implements EventHandler {
  private editor: RichTextEditor;
  private editorElement: HTMLElement;
  private isProcessing: boolean = false;

  constructor(editor: RichTextEditor) {
    this.editor = editor;
    this.editorElement = editor.getEditorElement();
  }

  attach(): void {
    this.editorElement.addEventListener(
      "beforeinput",
      this.handleBeforeInput as EventListener
    );
    this.editorElement.addEventListener(
      "input",
      this.handleInput as EventListener
    );
  }

  detach(): void {
    this.editorElement.removeEventListener(
      "beforeinput",
      this.handleBeforeInput as EventListener
    );
    this.editorElement.removeEventListener(
      "input",
      this.handleInput as EventListener
    );
  }

  handleEvent(event: Event): void {
    if (event.type === "beforeinput") {
      this.handleBeforeInput(event as InputEvent);
    } else if (event.type === "input") {
      this.handleInput(event as InputEvent);
    }
  }

  /**
   * Handle beforeinput event - allows preventing default behavior
   */
  private handleBeforeInput = (event: InputEvent): void => {
    // Skip if we're already processing
    if (this.isProcessing) {
      return;
    }

    // Check if editor is read-only
    const options = this.editor["options"];
    if (options.readOnly) {
      event.preventDefault();
      return;
    }

    // Check if IME composition is in progress
    const state = this.editor.getState();
    if (state.isComposingState()) {
      // Let IME handler deal with composition
      return;
    }

    // Call user's onBeforeInput handler if provided
    if (options.onBeforeInput) {
      const result = options.onBeforeInput(event);
      if (result === false) {
        event.preventDefault();
        return;
      }
    }

    // Handle different input types
    const inputType = event.inputType;

    // For text insertion, check max length
    if (
      inputType === "insertText" ||
      inputType === "insertCompositionText" ||
      inputType === "insertFromPaste"
    ) {
      if (options.maxLength !== undefined) {
        const currentLength = this.editor.getLength();
        const dataLength = event.data?.length || 0;
        const selection = this.editor.getSelection();
        const selectionLength = Math.abs(selection.focus - selection.anchor);

        // Calculate new length (current - deleted + inserted)
        const newLength = currentLength - selectionLength + dataLength;

        if (newLength > options.maxLength) {
          event.preventDefault();
          return;
        }
      }
    }

    // Let the browser handle the input for now
    // We'll sync with WASM in the input event handler
  };

  /**
   * Handle input event - sync changes to WASM document
   */
  private handleInput = (event: InputEvent): void => {
    // Skip if we're already processing
    if (this.isProcessing) {
      return;
    }

    // Check if IME composition is in progress
    const state = this.editor.getState();
    if (state.isComposingState()) {
      // Let IME handler deal with composition
      return;
    }

    // Mark as processing to prevent recursion
    this.isProcessing = true;

    try {
      // Sync DOM changes to WASM document
      this.syncDOMToWASM(event);

      // Emit input event
      const options = this.editor["options"];
      if (options.onChange) {
        options.onChange(this.editor.getContent());
      }
    } catch (error) {
      console.error("Error syncing input to WASM:", error);

      // On error, re-render from WASM to restore consistent state
      const renderer = this.editor.getRenderer();
      renderer.render();
    } finally {
      this.isProcessing = false;
    }
  };

  /**
   * Sync DOM changes to WASM document
   */
  private syncDOMToWASM(event: InputEvent): void {
    const inputType = event.inputType;

    // Get current DOM content
    const domContent = this.editorElement.textContent || "";

    // Get current WASM content
    const wasmContent = this.editor.getContent();

    // If content is the same, no need to sync
    if (domContent === wasmContent) {
      return;
    }

    // Handle different input types
    switch (inputType) {
      case "insertText":
      case "insertCompositionText":
        this.handleTextInsertion(event);
        break;

      case "deleteContentBackward":
      case "deleteContentForward":
      case "deleteByCut":
        this.handleDeletion(event);
        break;

      case "insertFromPaste":
        this.handlePaste(event);
        break;

      case "insertParagraph":
      case "insertLineBreak":
        this.handleLineBreak(event);
        break;

      default:
        // For other input types, do a full sync
        this.fullSync();
        break;
    }
  }

  /**
   * Handle text insertion
   */
  private handleTextInsertion(event: InputEvent): void {
    const data = event.data;
    if (!data) {
      return;
    }

    // Get selection before insertion
    const selection = this.editor.getSelection();
    const start = Math.min(selection.anchor, selection.focus);
    const end = Math.max(selection.anchor, selection.focus);

    // Delete selected range if any
    if (start !== end) {
      this.editor.deleteRange(start, end);
    }

    // Insert text
    this.editor.insertText(data, start);

    // Update selection
    const newPosition = start + data.length;
    this.editor.setSelection(newPosition, newPosition);
  }

  /**
   * Handle deletion
   */
  private handleDeletion(event: InputEvent): void {
    const selection = this.editor.getSelection();
    const start = Math.min(selection.anchor, selection.focus);
    const end = Math.max(selection.anchor, selection.focus);

    if (start === end) {
      // No selection, delete one character
      if (event.inputType === "deleteContentBackward") {
        // Delete character before cursor
        if (start > 0) {
          this.editor.deleteRange(start - 1, start);
          this.editor.setSelection(start - 1, start - 1);
        }
      } else if (event.inputType === "deleteContentForward") {
        // Delete character after cursor
        const length = this.editor.getLength();
        if (start < length) {
          this.editor.deleteRange(start, start + 1);
          this.editor.setSelection(start, start);
        }
      }
    } else {
      // Delete selected range
      this.editor.deleteRange(start, end);
      this.editor.setSelection(start, start);
    }
  }

  /**
   * Handle paste
   */
  private handlePaste(event: InputEvent): void {
    // Paste is handled by ClipboardHandler
    // This is a fallback for browsers that don't support clipboard events
    const data = event.data;
    if (data) {
      const selection = this.editor.getSelection();
      const start = Math.min(selection.anchor, selection.focus);
      const end = Math.max(selection.anchor, selection.focus);

      // Delete selected range if any
      if (start !== end) {
        this.editor.deleteRange(start, end);
      }

      // Insert pasted text
      this.editor.insertText(data, start);

      // Update selection
      const newPosition = start + data.length;
      this.editor.setSelection(newPosition, newPosition);
    }
  }

  /**
   * Handle line break insertion
   */
  private handleLineBreak(_event: InputEvent): void {
    const selection = this.editor.getSelection();
    const start = Math.min(selection.anchor, selection.focus);
    const end = Math.max(selection.anchor, selection.focus);

    // Delete selected range if any
    if (start !== end) {
      this.editor.deleteRange(start, end);
    }

    // Insert newline
    this.editor.insertText("\n", start);

    // Update selection
    const newPosition = start + 1;
    this.editor.setSelection(newPosition, newPosition);
  }

  /**
   * Full sync from DOM to WASM (fallback for complex changes)
   */
  private fullSync(): void {
    // Get DOM content
    const domContent = this.editorElement.textContent || "";

    // Replace entire WASM content
    // This is a simple approach - a more sophisticated implementation
    // would use diff algorithms to minimize changes
    const wasmDoc = this.editor.getWasmDocument();
    const currentLength = wasmDoc.getLength();

    // Delete all content
    if (currentLength > 0) {
      wasmDoc.deleteRange(0, currentLength);
    }

    // Insert new content
    if (domContent.length > 0) {
      wasmDoc.insertText(domContent, 0);
    }

    // Re-render to ensure consistency
    const renderer = this.editor.getRenderer();
    renderer.render();
  }
}
