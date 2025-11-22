import type { RichTextEditor } from "../core/RichTextEditor";
/**
 * Information about the current selection
 */
export interface SelectionInfo {
    hasSelection: boolean;
    start: number;
    end: number;
    length: number;
    isCollapsed: boolean;
}
/**
 * Helper class for editor state operations
 */
export declare class StateHelpers {
    private editor;
    constructor(editor: RichTextEditor);
    /**
     * Check if undo is available
     */
    canUndo(): boolean;
    /**
     * Check if redo is available
     */
    canRedo(): boolean;
    /**
     * Check if there is an active selection (not just a cursor)
     */
    hasSelection(): boolean;
    /**
     * Get detailed information about the current selection
     */
    getSelectionInfo(): SelectionInfo;
    /**
     * Check if the document is empty
     */
    isEmpty(): boolean;
    /**
     * Check if the document has been modified
     */
    isDirty(): boolean;
    /**
     * Get the current document length
     */
    getLength(): number;
    /**
     * Check if IME composition is in progress
     */
    isComposing(): boolean;
}
//# sourceMappingURL=stateHelpers.d.ts.map