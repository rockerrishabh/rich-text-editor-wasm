import type { RichTextEditor } from "../core/RichTextEditor";
import type { FormatType, ValueFormatType, Format } from "../types";
/**
 * Helper class for format-related operations
 */
export declare class FormatHelpers {
    private editor;
    constructor(editor: RichTextEditor);
    /**
     * Check if a specific format is active at the current selection
     */
    isFormatActive(format: FormatType): boolean;
    /**
     * Get all active formats at the current selection
     */
    getActiveFormats(): Format[];
    /**
     * Apply a simple format to the current selection
     */
    applyFormat(format: FormatType): void;
    /**
     * Apply a format with a value to the current selection
     * For color formats, the value is validated to prevent CSS injection
     * For link formats, the URL is sanitized to prevent XSS attacks
     */
    applyFormatWithValue(format: ValueFormatType, value: string): void;
    /**
     * Remove a format from the current selection
     */
    removeFormat(format: FormatType): void;
    /**
     * Toggle a format on/off for the current selection
     */
    toggleFormat(format: FormatType, value?: string): void;
    /**
     * Check if a format can be applied to the current selection
     */
    canApplyFormat(): boolean;
}
//# sourceMappingURL=formatHelpers.d.ts.map