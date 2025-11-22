import type { RichTextEditor } from "../core/RichTextEditor";
import type { ClipboardData } from "../types";

/**
 * Helper class for export/import and clipboard operations
 */
export class ExportHelpers {
  private editor: RichTextEditor;

  constructor(editor: RichTextEditor) {
    this.editor = editor;
  }

  /**
   * Export document to JSON format
   */
  toJSON(): string {
    return this.editor.toJSON();
  }

  /**
   * Export document to HTML format
   */
  toHTML(): string {
    return this.editor.toHTML();
  }

  /**
   * Export document to Markdown format
   */
  toMarkdown(): string {
    return this.editor.toMarkdown();
  }

  /**
   * Export document to plain text format
   */
  toPlainText(): string {
    return this.editor.toPlainText();
  }

  /**
   * Import document from JSON format
   */
  fromJSON(json: string): void {
    this.editor.fromJSON(json);
  }

  /**
   * Import document from HTML format
   * HTML is automatically sanitized to prevent XSS attacks
   */
  fromHTML(html: string): void {
    this.editor.fromHTML(html);
  }

  /**
   * Import document from Markdown format
   */
  fromMarkdown(markdown: string): void {
    this.editor.fromMarkdown(markdown);
  }

  /**
   * Import document from plain text format
   */
  fromPlainText(text: string): void {
    this.editor.fromPlainText(text);
  }

  /**
   * Copy the current selection to clipboard format
   */
  copy(): ClipboardData {
    const wasmDoc = this.editor.getWasmDocument();
    const clipboardData = wasmDoc.copy();

    return {
      text: clipboardData.text || "",
      html: clipboardData.html || "",
    };
  }

  /**
   * Cut the current selection to clipboard format
   */
  cut(): ClipboardData {
    const wasmDoc = this.editor.getWasmDocument();
    const clipboardData = wasmDoc.cut();

    return {
      text: clipboardData.text || "",
      html: clipboardData.html || "",
    };
  }

  /**
   * Paste HTML content at the current cursor position
   * HTML is automatically sanitized to prevent XSS attacks
   */
  paste(data: ClipboardData): void {
    const wasmDoc = this.editor.getWasmDocument();

    // Prefer HTML if available, otherwise use plain text
    if (data.html && data.html.length > 0) {
      // Sanitize HTML before pasting
      const { sanitizeHTML } = require("../utils/security");
      const sanitizedHTML = sanitizeHTML(data.html);
      wasmDoc.pasteHtml(sanitizedHTML);
    } else if (data.text && data.text.length > 0) {
      wasmDoc.pastePlainText(data.text);
    }

    // Re-render after paste
    const renderer = this.editor.getRenderer();
    renderer.render();
  }
}
