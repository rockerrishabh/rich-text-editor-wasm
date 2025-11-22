import type { WasmDocument } from "../../wasm/rte_core";
import type { Selection, Format } from "../types";
/**
 * Manages editor state
 */
export declare class EditorState {
    private selection;
    private activeFormats;
    private isDirty;
    private isComposing;
    constructor(_wasmDoc: WasmDocument);
    /**
     * Get current selection
     */
    getSelection(): Selection;
    /**
     * Set current selection
     */
    setSelection(selection: Selection): void;
    /**
     * Get active formats at current selection
     */
    getActiveFormats(): Format[];
    /**
     * Set active formats
     */
    setActiveFormats(formats: Format[]): void;
    /**
     * Check if document has been modified
     */
    isDirtyState(): boolean;
    /**
     * Mark document as dirty
     */
    markDirty(): void;
    /**
     * Mark document as clean
     */
    markClean(): void;
    /**
     * Check if IME composition is in progress
     */
    isComposingState(): boolean;
    /**
     * Set IME composition state
     */
    setComposing(composing: boolean): void;
}
//# sourceMappingURL=EditorState.d.ts.map