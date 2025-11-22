import type { RichTextEditor } from "../core/RichTextEditor";
import type { MemoryStats } from "../types";
/**
 * Helper class for document statistics operations
 */
export declare class StatsHelpers {
    private editor;
    constructor(editor: RichTextEditor);
    /**
     * Get the total character count in the document
     */
    getCharacterCount(): number;
    /**
     * Get the word count in the document
     */
    getWordCount(): number;
    /**
     * Get the line count in the document
     */
    getLineCount(): number;
    /**
     * Get the character count of the current selection
     */
    getSelectedCharacterCount(): number;
    /**
     * Get the word count of the current selection
     */
    getSelectedWordCount(): number;
    /**
     * Get memory usage statistics for the document
     */
    getMemoryStats(): MemoryStats;
}
//# sourceMappingURL=statsHelpers.d.ts.map