import type { EventHandler } from "../core/EventController";
import type { RichTextEditor } from "../core/RichTextEditor";
import type { CompositionRange } from "../types";

/**
 * Handles IME (Input Method Editor) composition events for international input methods
 */
export class IMEHandler implements EventHandler {
  private editor: RichTextEditor;
  private editorElement: HTMLElement;
  private compositionRange: CompositionRange | null = null;
  private compositionData: string = "";

  constructor(editor: RichTextEditor) {
    this.editor = editor;
    this.editorElement = editor.getEditorElement();
  }

  attach(): void {
    this.editorElement.addEventListener(
      "compositionstart",
      this.handleCompositionStart
    );
    this.editorElement.addEventListener(
      "compositionupdate",
      this.handleCompositionUpdate
    );
    this.editorElement.addEventListener(
      "compositionend",
      this.handleCompositionEnd
    );
  }

  detach(): void {
    this.editorElement.removeEventListener(
      "compositionstart",
      this.handleCompositionStart
    );
    this.editorElement.removeEventListener(
      "compositionupdate",
      this.handleCompositionUpdate
    );
    this.editorElement.removeEventListener(
      "compositionend",
      this.handleCompositionEnd
    );
  }

  handleEvent(event: Event): void {
    const type = event.type;
    if (type === "compositionstart") {
      this.handleCompositionStart(event as CompositionEvent);
    } else if (type === "compositionupdate") {
      this.handleCompositionUpdate(event as CompositionEvent);
    } else if (type === "compositionend") {
      this.handleCompositionEnd(event as CompositionEvent);
    }
  }

  /**
   * Handle composition start event
   */
  private handleCompositionStart = (event: CompositionEvent): void => {
    try {
      // Mark editor state as composing
      const state = this.editor.getState();
      state.setComposing(true);

      // Get current selection
      const selection = this.editor.getSelection();
      const start = Math.min(selection.anchor, selection.focus);
      const end = Math.max(selection.anchor, selection.focus);

      // Store composition range
      this.compositionRange = { start, end };

      // Initialize composition data
      this.compositionData = event.data || "";

      // Emit compositionStart event
      const options = this.editor["options"];
      if (options.onCompositionStart) {
        options.onCompositionStart(event);
      }
    } catch (error) {
      console.error("Error handling composition start:", error);
    }
  };

  /**
   * Handle composition update event
   */
  private handleCompositionUpdate = (event: CompositionEvent): void => {
    try {
      // Update composition data
      this.compositionData = event.data || "";

      // Emit compositionUpdate event
      const options = this.editor["options"];
      if (options.onCompositionUpdate) {
        options.onCompositionUpdate(event);
      }

      // Note: We don't update the WASM document during composition
      // The browser handles the visual representation during composition
      // We'll sync with WASM when composition ends
    } catch (error) {
      console.error("Error handling composition update:", error);
    }
  };

  /**
   * Handle composition end event
   */
  private handleCompositionEnd = (event: CompositionEvent): void => {
    try {
      // Get final composition data
      const finalData = event.data || this.compositionData;

      // Mark editor state as not composing
      const state = this.editor.getState();
      state.setComposing(false);

      // Sync composition result to WASM document
      if (this.compositionRange && finalData) {
        this.syncCompositionToWASM(finalData);
      }

      // Clear composition state
      this.compositionRange = null;
      this.compositionData = "";

      // Emit compositionEnd event
      const options = this.editor["options"];
      if (options.onCompositionEnd) {
        options.onCompositionEnd(event);
      }
    } catch (error) {
      console.error("Error handling composition end:", error);

      // Ensure we clear composing state even on error
      const state = this.editor.getState();
      state.setComposing(false);
      this.compositionRange = null;
      this.compositionData = "";
    }
  };

  /**
   * Sync composition result to WASM document
   */
  private syncCompositionToWASM(data: string): void {
    if (!this.compositionRange) {
      return;
    }

    try {
      const { start, end } = this.compositionRange;

      // Delete the original selection if any
      if (start !== end) {
        this.editor.deleteRange(start, end);
      }

      // Insert the composed text
      if (data) {
        this.editor.insertText(data, start);

        // Update selection to end of inserted text
        const newPosition = start + data.length;
        this.editor.setSelection(newPosition, newPosition);
      }

      // Re-render to ensure consistency
      const renderer = this.editor.getRenderer();
      renderer.render();
    } catch (error) {
      console.error("Error syncing composition to WASM:", error);

      // On error, try to recover by re-rendering from WASM
      try {
        const renderer = this.editor.getRenderer();
        renderer.render();
      } catch (renderError) {
        console.error(
          "Failed to recover from composition sync error:",
          renderError
        );
      }
    }
  }

  /**
   * Get current composition range
   */
  getCompositionRange(): CompositionRange | null {
    return this.compositionRange ? { ...this.compositionRange } : null;
  }

  /**
   * Get current composition data
   */
  getCompositionData(): string {
    return this.compositionData;
  }

  /**
   * Check if composition is in progress
   */
  isComposing(): boolean {
    const state = this.editor.getState();
    return state.isComposingState();
  }
}
