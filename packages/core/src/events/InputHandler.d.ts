import type { EventHandler } from "../core/EventController";
import type { RichTextEditor } from "../core/RichTextEditor";
/**
 * Handles text input events and syncs with WASM document
 */
export declare class InputHandler implements EventHandler {
    private editor;
    private editorElement;
    private isProcessing;
    constructor(editor: RichTextEditor);
    attach(): void;
    detach(): void;
    handleEvent(event: Event): void;
    /**
     * Handle beforeinput event - allows preventing default behavior
     */
    private handleBeforeInput;
    /**
     * Handle input event - sync changes to WASM document
     */
    private handleInput;
    /**
     * Sync DOM changes to WASM document
     */
    private syncDOMToWASM;
    /**
     * Handle text insertion
     */
    private handleTextInsertion;
    /**
     * Handle deletion
     */
    private handleDeletion;
    /**
     * Handle paste
     */
    private handlePaste;
    /**
     * Handle line break insertion
     */
    private handleLineBreak;
    /**
     * Full sync from DOM to WASM (fallback for complex changes)
     */
    private fullSync;
}
//# sourceMappingURL=InputHandler.d.ts.map