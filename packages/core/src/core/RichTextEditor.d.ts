import { WasmDocument } from "../../wasm/rte_core";
import { DOMRenderer } from "./DOMRenderer";
import { EventController } from "./EventController";
import { EditorState } from "./EditorState";
import type { EditorOptions, FormatType, BlockType, Format, Selection, EditorEventType, EditorEventCallback } from "../types";
/**
 * Main editor class that orchestrates all functionality
 */
export declare class RichTextEditor {
    private wasmDoc;
    private container;
    private editorElement;
    private renderer;
    private eventController;
    private state;
    private options;
    private isDestroyed;
    private eventCallbacks;
    private liveRegion;
    constructor(container: HTMLElement, options?: EditorOptions);
    insertText(text: string, position: number): void;
    deleteRange(start: number, end: number): void;
    getContent(): string;
    getLength(): number;
    applyFormat(format: FormatType, start: number, end: number, value?: string): void;
    removeFormat(format: FormatType, start: number, end: number): void;
    getFormatsAt(position: number): Format[];
    setBlockType(blockType: BlockType, start: number, end: number): void;
    getBlockTypeAt(position: number): BlockType;
    setSelection(anchor: number, focus: number): void;
    getSelection(): Selection;
    undo(): void;
    redo(): void;
    canUndo(): boolean;
    canRedo(): boolean;
    toJSON(): string;
    toHTML(): string;
    toMarkdown(): string;
    toPlainText(): string;
    fromJSON(json: string): void;
    fromHTML(html: string): void;
    fromMarkdown(markdown: string): void;
    fromPlainText(text: string): void;
    destroy(): void;
    /**
     * Check if the editor has been destroyed
     */
    isEditorDestroyed(): boolean;
    /**
     * Ensure editor is not destroyed before operations
     */
    private ensureNotDestroyed;
    on(event: EditorEventType, callback: EditorEventCallback): void;
    off(event: EditorEventType, callback: EditorEventCallback): void;
    /**
     * Emit an event to all registered callbacks
     */
    private emit;
    /**
     * Get the editor element
     */
    getEditorElement(): HTMLElement;
    /**
     * Get the container element
     */
    getContainer(): HTMLElement;
    /**
     * Get the WASM document instance (for advanced use cases)
     */
    getWasmDocument(): WasmDocument;
    /**
     * Get the editor state
     */
    getState(): EditorState;
    /**
     * Get the renderer
     */
    getRenderer(): DOMRenderer;
    /**
     * Get the event controller
     */
    getEventController(): EventController;
    /**
     * Announce a message to screen readers
     */
    announce(message: string): void;
    /**
     * Get the live region element for screen reader announcements
     */
    getLiveRegion(): HTMLElement | null;
    /**
     * Enable or disable lazy rendering for large documents
     * @param enabled - Whether to enable lazy rendering
     * @param config - Optional viewport configuration
     */
    setLazyRendering(enabled: boolean, config?: Partial<import("../utils/performance").ViewportConfig>): void;
    /**
     * Enable or disable batched DOM updates
     * @param enabled - Whether to enable batched updates
     */
    setBatchedUpdates(enabled: boolean): void;
    /**
     * Flush any pending batched renders immediately
     */
    flushPendingRenders(): void;
}
//# sourceMappingURL=RichTextEditor.d.ts.map