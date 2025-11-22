import type { RichTextEditor } from "../core/RichTextEditor";
import type { ClipboardData } from "../types";
/**
 * Helper class for export/import and clipboard operations
 */
export declare class ExportHelpers {
    private editor;
    constructor(editor: RichTextEditor);
    /**
     * Export document to JSON format
     */
    toJSON(): string;
    /**
     * Export document to HTML format
     */
    toHTML(): string;
    /**
     * Export document to Markdown format
     */
    toMarkdown(): string;
    /**
     * Export document to plain text format
     */
    toPlainText(): string;
    /**
     * Import document from JSON format
     */
    fromJSON(json: string): void;
    /**
     * Import document from HTML format
     * HTML is automatically sanitized to prevent XSS attacks
     */
    fromHTML(html: string): void;
    /**
     * Import document from Markdown format
     */
    fromMarkdown(markdown: string): void;
    /**
     * Import document from plain text format
     */
    fromPlainText(text: string): void;
    /**
     * Copy the current selection to clipboard format
     */
    copy(): ClipboardData;
    /**
     * Cut the current selection to clipboard format
     */
    cut(): ClipboardData;
    /**
     * Paste HTML content at the current cursor position
     * HTML is automatically sanitized to prevent XSS attacks
     */
    paste(data: ClipboardData): void;
}
//# sourceMappingURL=exportHelpers.d.ts.map