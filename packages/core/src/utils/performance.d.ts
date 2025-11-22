/**
 * Performance optimization utilities for the editor
 */
/**
 * Viewport-based rendering configuration
 */
export interface ViewportConfig {
    /** Number of lines to render above the viewport */
    overscanAbove: number;
    /** Number of lines to render below the viewport */
    overscanBelow: number;
    /** Minimum document size to enable lazy rendering */
    minDocumentSize: number;
}
/**
 * Default viewport configuration
 */
export declare const DEFAULT_VIEWPORT_CONFIG: ViewportConfig;
/**
 * Calculate visible range based on viewport
 */
export declare function calculateVisibleRange(container: HTMLElement, documentLength: number, config?: ViewportConfig): {
    start: number;
    end: number;
} | null;
/**
 * Debounce function for performance optimization
 */
export declare function debounce<T extends (...args: any[]) => any>(func: T, wait: number): (...args: Parameters<T>) => void;
/**
 * Throttle function for performance optimization
 */
export declare function throttle<T extends (...args: any[]) => any>(func: T, limit: number): (...args: Parameters<T>) => void;
/**
 * Request animation frame wrapper for batching DOM updates
 */
export declare class RenderBatcher {
    private pendingUpdates;
    private rafId;
    /**
     * Schedule a render update
     */
    schedule(update: () => void): void;
    /**
     * Flush all pending updates
     */
    private flush;
    /**
     * Cancel all pending updates
     */
    cancel(): void;
    /**
     * Check if there are pending updates
     */
    hasPending(): boolean;
}
//# sourceMappingURL=performance.d.ts.map