import type { RichTextEditor } from "./RichTextEditor";
/**
 * Event handler interface
 */
export interface EventHandler {
    attach(): void;
    detach(): void;
    handleEvent(event: Event): void;
}
/**
 * Coordinates all event handlers
 *
 * Performance optimizations:
 * - Uses event delegation by attaching handlers to the editor container
 * - Individual handlers manage their own event listeners efficiently
 * - Debounced selection change events reduce overhead
 */
export declare class EventController {
    private editor;
    private handlers;
    private isAttached;
    constructor(editor: RichTextEditor);
    /**
     * Initialize all event handlers
     */
    private initializeHandlers;
    /**
     * Register an event handler
     */
    registerHandler(name: string, handler: EventHandler): void;
    /**
     * Unregister an event handler
     */
    unregisterHandler(name: string): void;
    /**
     * Get a registered handler by name
     */
    getHandler(name: string): EventHandler | undefined;
    /**
     * Handle an event by delegating to appropriate handler
     * This can be used for custom event delegation if needed
     */
    handleEvent(event: Event): void;
    /**
     * Attach all event handlers
     */
    attach(): void;
    /**
     * Detach all event handlers
     */
    detach(): void;
    /**
     * Check if event controller is attached
     */
    isEventControllerAttached(): boolean;
}
//# sourceMappingURL=EventController.d.ts.map