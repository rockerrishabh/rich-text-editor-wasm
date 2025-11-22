import type { WasmDocument } from "../../wasm/rte_core";
import { type ViewportConfig } from "../utils/performance";
/**
 * Handles synchronization between WASM document and DOM
 */
export declare class DOMRenderer {
    private editor;
    private wasmDoc;
    private positionCache;
    private lazyRenderingEnabled;
    private viewportConfig;
    private lastRenderedRange;
    private renderBatcher;
    private batchedUpdatesEnabled;
    private cspNonce;
    constructor(editor: HTMLElement, wasmDoc: WasmDocument, cspNonce?: string);
    /**
     * Enable or disable lazy rendering for large documents
     */
    setLazyRendering(enabled: boolean, config?: Partial<ViewportConfig>): void;
    /**
     * Enable or disable batched DOM updates
     */
    setBatchedUpdates(enabled: boolean): void;
    /**
     * Render the entire document or visible portion if lazy rendering is enabled
     * Optionally batches the update using requestAnimationFrame
     */
    render(immediate?: boolean): void;
    /**
     * Render immediately without batching
     */
    private renderImmediate;
    /**
     * Render only the visible portion of the document (lazy rendering)
     */
    private renderLazy;
    /**
     * Update render if viewport has changed (for lazy rendering)
     */
    updateViewport(): void;
    /**
     * Render a specific range of the document
     */
    renderRange(start: number, end: number): void;
    /**
     * Sync DOM changes back to WASM document
     */
    syncFromDOM(): void;
    /**
     * Render formatted text with inline styles
     * This method is reserved for future use when manual DOM construction is needed
     * @internal
     */
    private renderFormats;
    /**
     * Create an HTML element for a specific format
     */
    private createFormatElement;
    /**
     * Get DOM node at a specific document position
     */
    getNodeAtPosition(position: number): {
        node: Node;
        offset: number;
    };
    /**
     * Get document position from DOM node
     */
    getPositionFromNode(node: Node, offset: number): number;
    /**
     * Clear the position cache
     * Should be called after any DOM modifications
     */
    clearPositionCache(): void;
    /**
     * Flush any pending batched renders immediately
     */
    flushPendingRenders(): void;
    /**
     * Cleanup resources
     */
    destroy(): void;
}
//# sourceMappingURL=DOMRenderer.d.ts.map