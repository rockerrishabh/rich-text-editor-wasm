import type { EventHandler } from "../core/EventController";
import type { RichTextEditor } from "../core/RichTextEditor";
import type { ClipboardData } from "../types";

/**
 * Handles clipboard operations (copy, cut, paste)
 */
export class ClipboardHandler implements EventHandler {
  private editor: RichTextEditor;
  private editorElement: HTMLElement;

  constructor(editor: RichTextEditor) {
    this.editor = editor;
    this.editorElement = editor.getEditorElement();
  }

  attach(): void {
    this.editorElement.addEventListener("copy", this.handleCopy);
    this.editorElement.addEventListener("cut", this.handleCut);
    this.editorElement.addEventListener("paste", this.handlePaste);
  }

  detach(): void {
    this.editorElement.removeEventListener("copy", this.handleCopy);
    this.editorElement.removeEventListener("cut", this.handleCut);
    this.editorElement.removeEventListener("paste", this.handlePaste);
  }

  handleEvent(event: Event): void {
    const type = event.type;
    if (type === "copy") {
      this.handleCopy(event as ClipboardEvent);
    } else if (type === "cut") {
      this.handleCut(event as ClipboardEvent);
    } else if (type === "paste") {
      this.handlePaste(event as ClipboardEvent);
    }
  }

  /**
   * Handle copy event
   */
  private handleCopy = (event: ClipboardEvent): void => {
    try {
      // Get current selection
      const selection = this.editor.getSelection();
      const start = Math.min(selection.anchor, selection.focus);
      const end = Math.max(selection.anchor, selection.focus);

      // If no selection, nothing to copy
      if (start === end) {
        return;
      }

      // Get clipboard data from WASM
      const wasmDoc = this.editor.getWasmDocument();
      const clipboardData = wasmDoc.copy() as ClipboardData;

      // Set clipboard data
      if (event.clipboardData) {
        event.preventDefault();

        // Set both plain text and HTML
        event.clipboardData.setData("text/plain", clipboardData.text);
        event.clipboardData.setData("text/html", clipboardData.html);
      }

      // Call user's onCopy handler if provided
      const options = this.editor["options"];
      if (options.onCopy) {
        options.onCopy(clipboardData);
      }
    } catch (error) {
      console.error("Error handling copy:", error);
    }
  };

  /**
   * Handle cut event
   */
  private handleCut = (event: ClipboardEvent): void => {
    try {
      // Check if editor is read-only
      const options = this.editor["options"];
      if (options.readOnly) {
        event.preventDefault();
        return;
      }

      // Get current selection
      const selection = this.editor.getSelection();
      const start = Math.min(selection.anchor, selection.focus);
      const end = Math.max(selection.anchor, selection.focus);

      // If no selection, nothing to cut
      if (start === end) {
        return;
      }

      // Get clipboard data from WASM (cut also deletes the selection)
      const wasmDoc = this.editor.getWasmDocument();
      const clipboardData = wasmDoc.cut() as ClipboardData;

      // Set clipboard data
      if (event.clipboardData) {
        event.preventDefault();

        // Set both plain text and HTML
        event.clipboardData.setData("text/plain", clipboardData.text);
        event.clipboardData.setData("text/html", clipboardData.html);

        // Re-render after cut
        const renderer = this.editor.getRenderer();
        renderer.render();

        // Update selection to cut position (already done by WASM)
        this.editor.setSelection(start, start);
      }

      // Call user's onCut handler if provided
      if (options.onCut) {
        options.onCut(clipboardData);
      }
    } catch (error) {
      console.error("Error handling cut:", error);
    }
  };

  /**
   * Handle paste event
   */
  private handlePaste = (event: ClipboardEvent): void => {
    try {
      // Check if editor is read-only
      const options = this.editor["options"];
      if (options.readOnly) {
        event.preventDefault();
        return;
      }

      // Prevent default paste behavior
      event.preventDefault();

      if (!event.clipboardData) {
        return;
      }

      // Try to get HTML first, fall back to plain text
      let html = event.clipboardData.getData("text/html");
      const text = event.clipboardData.getData("text/plain");

      // Create clipboard data object
      const clipboardData: ClipboardData = {
        text: text || "",
        html: html || "",
      };

      // Call user's onPaste handler if provided
      if (options.onPaste) {
        const result = options.onPaste(clipboardData);
        if (result === false) {
          return;
        }
      }

      // Get current selection
      const selection = this.editor.getSelection();
      const start = Math.min(selection.anchor, selection.focus);
      const end = Math.max(selection.anchor, selection.focus);

      // Delete selected range if any
      if (start !== end) {
        this.editor.deleteRange(start, end);
      }

      // Check max length if specified
      if (options.maxLength !== undefined) {
        const currentLength = this.editor.getLength();
        const pasteLength = text.length;
        const newLength = currentLength - (end - start) + pasteLength;

        if (newLength > options.maxLength) {
          console.warn("Paste would exceed maximum length");
          return;
        }
      }

      // Paste content using WASM
      const wasmDoc = this.editor.getWasmDocument();

      // Try to paste HTML if available, otherwise paste plain text
      if (html) {
        try {
          wasmDoc.pasteHtml(html);
        } catch (error) {
          // If HTML paste fails, fall back to plain text
          console.warn("HTML paste failed, falling back to plain text:", error);
          wasmDoc.pastePlainText(text);
        }
      } else if (text) {
        wasmDoc.pastePlainText(text);
      }

      // Re-render after paste
      const renderer = this.editor.getRenderer();
      renderer.render();

      // Update selection to end of pasted content
      const newPosition = start + text.length;
      this.editor.setSelection(newPosition, newPosition);
    } catch (error) {
      console.error("Error handling paste:", error);

      // On error, try simple text insertion as fallback
      try {
        const text = event.clipboardData?.getData("text/plain");
        if (text) {
          const selection = this.editor.getSelection();
          const start = Math.min(selection.anchor, selection.focus);
          const end = Math.max(selection.anchor, selection.focus);

          if (start !== end) {
            this.editor.deleteRange(start, end);
          }

          this.editor.insertText(text, start);
          this.editor.setSelection(start + text.length, start + text.length);
        }
      } catch (fallbackError) {
        console.error("Fallback paste also failed:", fallbackError);
      }
    }
  };
}
