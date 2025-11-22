import type { EventHandler } from "../core/EventController";
import type { RichTextEditor } from "../core/RichTextEditor";
/**
 * Handles clipboard operations (copy, cut, paste)
 */
export declare class ClipboardHandler implements EventHandler {
    private editor;
    private editorElement;
    constructor(editor: RichTextEditor);
    attach(): void;
    detach(): void;
    handleEvent(event: Event): void;
    /**
     * Handle copy event
     */
    private handleCopy;
    /**
     * Handle cut event
     */
    private handleCut;
    /**
     * Handle paste event
     */
    private handlePaste;
}
//# sourceMappingURL=ClipboardHandler.d.ts.map