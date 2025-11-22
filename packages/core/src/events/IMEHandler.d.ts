import type { EventHandler } from "../core/EventController";
import type { RichTextEditor } from "../core/RichTextEditor";
import type { CompositionRange } from "../types";
/**
 * Handles IME (Input Method Editor) composition events for international input methods
 */
export declare class IMEHandler implements EventHandler {
    private editor;
    private editorElement;
    private compositionRange;
    private compositionData;
    constructor(editor: RichTextEditor);
    attach(): void;
    detach(): void;
    handleEvent(event: Event): void;
    /**
     * Handle composition start event
     */
    private handleCompositionStart;
    /**
     * Handle composition update event
     */
    private handleCompositionUpdate;
    /**
     * Handle composition end event
     */
    private handleCompositionEnd;
    /**
     * Sync composition result to WASM document
     */
    private syncCompositionToWASM;
    /**
     * Get current composition range
     */
    getCompositionRange(): CompositionRange | null;
    /**
     * Get current composition data
     */
    getCompositionData(): string;
    /**
     * Check if composition is in progress
     */
    isComposing(): boolean;
}
//# sourceMappingURL=IMEHandler.d.ts.map