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
export const DEFAULT_VIEWPORT_CONFIG: ViewportConfig = {
  overscanAbove: 10,
  overscanBelow: 10,
  minDocumentSize: 5000, // Enable lazy rendering for documents > 5000 characters
};

/**
 * Calculate visible range based on viewport
 */
export function calculateVisibleRange(
  container: HTMLElement,
  documentLength: number,
  config: ViewportConfig = DEFAULT_VIEWPORT_CONFIG
): { start: number; end: number } | null {
  // Only use lazy rendering for large documents
  if (documentLength < config.minDocumentSize) {
    return null;
  }

  try {
    // Get container dimensions
    const containerRect = container.getBoundingClientRect();
    const scrollTop = container.scrollTop;
    const viewportHeight = containerRect.height;

    // Estimate line height (approximate)
    const computedStyle = window.getComputedStyle(container);
    const lineHeight = parseFloat(computedStyle.lineHeight) || 24;

    // Calculate visible line range
    const firstVisibleLine = Math.floor(scrollTop / lineHeight);
    const lastVisibleLine = Math.ceil(
      (scrollTop + viewportHeight) / lineHeight
    );

    // Add overscan
    const startLine = Math.max(0, firstVisibleLine - config.overscanAbove);
    const endLine = lastVisibleLine + config.overscanBelow;

    // Estimate character positions (rough approximation)
    // This is a simplified calculation - in production, you'd want more accurate tracking
    const avgCharsPerLine = 80;
    const start = startLine * avgCharsPerLine;
    const end = Math.min(endLine * avgCharsPerLine, documentLength);

    return { start, end };
  } catch (error) {
    console.error("Error calculating visible range:", error);
    return null;
  }
}

/**
 * Debounce function for performance optimization
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout> | null = null;

  return function (this: any, ...args: Parameters<T>) {
    const context = this;

    if (timeout !== null) {
      clearTimeout(timeout);
    }

    timeout = setTimeout(() => {
      func.apply(context, args);
      timeout = null;
    }, wait);
  };
}

/**
 * Throttle function for performance optimization
 */
export function throttle<T extends (...args: any[]) => any>(
  func: T,
  limit: number
): (...args: Parameters<T>) => void {
  let inThrottle: boolean = false;
  let lastResult: ReturnType<T>;

  return function (this: any, ...args: Parameters<T>): ReturnType<T> {
    const context = this;

    if (!inThrottle) {
      lastResult = func.apply(context, args);
      inThrottle = true;

      setTimeout(() => {
        inThrottle = false;
      }, limit);
    }

    return lastResult;
  };
}

/**
 * Request animation frame wrapper for batching DOM updates
 */
export class RenderBatcher {
  private pendingUpdates: Set<() => void> = new Set();
  private rafId: number | null = null;

  /**
   * Schedule a render update
   */
  schedule(update: () => void): void {
    this.pendingUpdates.add(update);

    if (this.rafId === null) {
      this.rafId = requestAnimationFrame(() => this.flush());
    }
  }

  /**
   * Flush all pending updates
   */
  private flush(): void {
    const updates = Array.from(this.pendingUpdates);
    this.pendingUpdates.clear();
    this.rafId = null;

    // Execute all updates in a single frame
    for (const update of updates) {
      try {
        update();
      } catch (error) {
        console.error("Error executing batched update:", error);
      }
    }
  }

  /**
   * Cancel all pending updates
   */
  cancel(): void {
    if (this.rafId !== null) {
      cancelAnimationFrame(this.rafId);
      this.rafId = null;
    }
    this.pendingUpdates.clear();
  }

  /**
   * Check if there are pending updates
   */
  hasPending(): boolean {
    return this.pendingUpdates.size > 0;
  }
}
