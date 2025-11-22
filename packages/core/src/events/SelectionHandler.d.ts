import type { EventHandler } from "../core/EventController";
import type { RichTextEditor } from "../core/RichTextEditor";
/**
 * Handles selection tracking and syncs with editor state
 */
export declare class SelectionHandler implements EventHandler {
    private editor;
    private editorElement;
    private lastSelection;
    private isUpdating;
    private debouncedHandleSelectionChange;
    constructor(editor: RichTextEditor);
    attach(): void;
    detach(): void;
    handleEvent(event: Event): void;
    /**
     * Handle focus event
     */
    private handleFocus;
    /**
     * Handle blur event
     */
    private handleBlur;
    /**
     * Handle selection change event (immediate, called by debounced wrapper)
     */
    private handleSelectionChangeImmediate;
    /**
     * Check if the DOM selection is within the editor
     */
    private isSelectionInEditor;
    /**
     * Convert DOM selection to editor selection (character positions)
     */
    private domSelectionToEditorSelection;
    /**
     * Check if selection has changed from last known selection
     */
    private hasSelectionChanged;
    /**
     * Update editor selection and emit events
     */
    private updateEditorSelection;
    /**
     * Programmatically set the DOM selection from editor positions
     */
    setDOMSelection(anchor: number, focus: number): void;
}
//# sourceMappingURL=SelectionHandler.d.ts.map