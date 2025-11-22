import type { RichTextEditor } from "../core/RichTextEditor";
import type { FormatType, ValueFormatType, Format } from "../types";

/**
 * Helper class for format-related operations
 */
export class FormatHelpers {
  private editor: RichTextEditor;

  constructor(editor: RichTextEditor) {
    this.editor = editor;
  }

  /**
   * Check if a specific format is active at the current selection
   */
  isFormatActive(format: FormatType): boolean {
    const selection = this.editor.getSelection();
    const position = Math.min(selection.anchor, selection.focus);

    // Get formats at the start of selection
    const formats = this.editor.getFormatsAt(position);

    // Check if the format exists in the formats array
    return formats.some((f) => {
      if (typeof f === "string") {
        return f === format;
      }
      return f.type === format;
    });
  }

  /**
   * Get all active formats at the current selection
   */
  getActiveFormats(): Format[] {
    const selection = this.editor.getSelection();
    const position = Math.min(selection.anchor, selection.focus);

    return this.editor.getFormatsAt(position);
  }

  /**
   * Apply a simple format to the current selection
   */
  applyFormat(format: FormatType): void {
    const selection = this.editor.getSelection();
    const start = Math.min(selection.anchor, selection.focus);
    const end = Math.max(selection.anchor, selection.focus);

    this.editor.applyFormat(format, start, end);
  }

  /**
   * Apply a format with a value to the current selection
   * For color formats, the value is validated to prevent CSS injection
   * For link formats, the URL is sanitized to prevent XSS attacks
   */
  applyFormatWithValue(format: ValueFormatType, value: string): void {
    const selection = this.editor.getSelection();
    const start = Math.min(selection.anchor, selection.focus);
    const end = Math.max(selection.anchor, selection.focus);

    // Validate and sanitize the value based on format type
    let sanitizedValue = value;

    if (
      format === "textColor" ||
      format === "text-color" ||
      format === "backgroundColor" ||
      format === "background-color"
    ) {
      // Validate color values to prevent CSS injection
      const { validateColor } = require("../utils/security");
      const validatedColor = validateColor(value);
      if (!validatedColor) {
        throw new Error(
          `Invalid color value: ${value}. Please use a valid hex, rgb, rgba, hsl, or named color.`
        );
      }
      sanitizedValue = validatedColor;
    } else if (format === "link") {
      // Sanitize URLs to prevent XSS attacks
      const { sanitizeURL } = require("../utils/security");
      const sanitizedURL = sanitizeURL(value);
      if (!sanitizedURL) {
        throw new Error(
          `Invalid or unsafe URL: ${value}. Please use a valid http, https, mailto, or tel URL.`
        );
      }
      sanitizedValue = sanitizedURL;
    }

    this.editor.applyFormat(format, start, end, sanitizedValue);
  }

  /**
   * Remove a format from the current selection
   */
  removeFormat(format: FormatType): void {
    const selection = this.editor.getSelection();
    const start = Math.min(selection.anchor, selection.focus);
    const end = Math.max(selection.anchor, selection.focus);

    this.editor.removeFormat(format, start, end);
  }

  /**
   * Toggle a format on/off for the current selection
   */
  toggleFormat(format: FormatType, value?: string): void {
    if (this.isFormatActive(format)) {
      this.removeFormat(format);
    } else {
      if (value !== undefined) {
        this.applyFormatWithValue(format as ValueFormatType, value);
      } else {
        this.applyFormat(format);
      }
    }
  }

  /**
   * Check if a format can be applied to the current selection
   */
  canApplyFormat(): boolean {
    const selection = this.editor.getSelection();
    // Can apply format if there's a selection (not just a cursor)
    return selection.anchor !== selection.focus;
  }
}
