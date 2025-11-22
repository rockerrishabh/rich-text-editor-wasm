import type { WasmDocument } from "../../wasm/rte_core";
import type { Format } from "../types";
import { EditorError, ErrorCodes } from "../types";
import { logger } from "../utils/logger";
import {
  calculateVisibleRange,
  DEFAULT_VIEWPORT_CONFIG,
  type ViewportConfig,
  RenderBatcher,
} from "../utils/performance";

/**
 * Handles synchronization between WASM document and DOM
 */
export class DOMRenderer {
  private editor: HTMLElement;
  private wasmDoc: WasmDocument;
  private positionCache: Map<Node, number> = new Map();
  private lazyRenderingEnabled: boolean = false;
  private viewportConfig: ViewportConfig = DEFAULT_VIEWPORT_CONFIG;
  private lastRenderedRange: { start: number; end: number } | null = null;
  private renderBatcher: RenderBatcher = new RenderBatcher();
  private batchedUpdatesEnabled: boolean = true;
  private cspNonce: string | undefined;

  constructor(editor: HTMLElement, wasmDoc: WasmDocument, cspNonce?: string) {
    if (!editor || !(editor instanceof HTMLElement)) {
      throw new EditorError(
        "Invalid editor element for DOMRenderer",
        ErrorCodes.INVALID_CONTAINER
      );
    }

    if (!wasmDoc) {
      throw new EditorError(
        "Invalid WASM document for DOMRenderer",
        ErrorCodes.WASM_INIT_FAILED
      );
    }

    this.editor = editor;
    this.wasmDoc = wasmDoc;
    this.cspNonce = cspNonce;

    logger.debug("DOMRenderer initialized", { cspNonce: !!cspNonce });
  }

  /**
   * Enable or disable lazy rendering for large documents
   */
  setLazyRendering(enabled: boolean, config?: Partial<ViewportConfig>): void {
    this.lazyRenderingEnabled = enabled;
    if (config) {
      this.viewportConfig = { ...DEFAULT_VIEWPORT_CONFIG, ...config };
    }
    logger.debug(
      `Lazy rendering ${enabled ? "enabled" : "disabled"}`,
      this.viewportConfig
    );
  }

  /**
   * Enable or disable batched DOM updates
   */
  setBatchedUpdates(enabled: boolean): void {
    this.batchedUpdatesEnabled = enabled;
    if (!enabled) {
      this.renderBatcher.cancel();
    }
    logger.debug(`Batched updates ${enabled ? "enabled" : "disabled"}`);
  }

  /**
   * Render the entire document or visible portion if lazy rendering is enabled
   * Optionally batches the update using requestAnimationFrame
   */
  render(immediate: boolean = false): void {
    if (this.batchedUpdatesEnabled && !immediate) {
      // Schedule batched render
      this.renderBatcher.schedule(() => this.renderImmediate());
    } else {
      // Render immediately
      this.renderImmediate();
    }
  }

  /**
   * Render immediately without batching
   */
  private renderImmediate(): void {
    try {
      logger.debug("Rendering document");

      // Clear position cache
      this.positionCache.clear();

      const documentLength = this.wasmDoc.getLength();

      // Check if we should use lazy rendering
      if (
        this.lazyRenderingEnabled &&
        documentLength >= this.viewportConfig.minDocumentSize
      ) {
        this.renderLazy();
        return;
      }

      // Get HTML from WASM document
      const html = this.wasmDoc.toHTML();

      // Update editor content
      this.editor.innerHTML = html;

      // If document is empty, ensure editor is editable
      if (this.wasmDoc.isEmpty()) {
        this.editor.innerHTML = "<p><br></p>";
      }

      this.lastRenderedRange = null;
      logger.debug("Document rendered successfully");
    } catch (error) {
      logger.error("Failed to render document:", error);
      throw new EditorError(
        `Failed to render document: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.RENDER_ERROR,
        error
      );
    }
  }

  /**
   * Render only the visible portion of the document (lazy rendering)
   */
  private renderLazy(): void {
    try {
      const documentLength = this.wasmDoc.getLength();
      const visibleRange = calculateVisibleRange(
        this.editor,
        documentLength,
        this.viewportConfig
      );

      if (!visibleRange) {
        // Fall back to full render
        const html = this.wasmDoc.toHTML();
        this.editor.innerHTML = html;
        this.lastRenderedRange = null;
        return;
      }

      // Check if we need to update the rendered range
      if (
        this.lastRenderedRange &&
        this.lastRenderedRange.start === visibleRange.start &&
        this.lastRenderedRange.end === visibleRange.end
      ) {
        logger.debug("Visible range unchanged, skipping render");
        return;
      }

      logger.debug(
        `Lazy rendering range: ${visibleRange.start}-${visibleRange.end}`
      );

      // Render the visible range
      // Note: This is a simplified implementation. A production version would need
      // more sophisticated virtual scrolling with placeholder elements
      const html = this.wasmDoc.toHTML();
      this.editor.innerHTML = html;

      this.lastRenderedRange = visibleRange;
      logger.debug("Lazy render completed");
    } catch (error) {
      logger.error("Failed to lazy render:", error);
      // Fall back to full render
      const html = this.wasmDoc.toHTML();
      this.editor.innerHTML = html;
      this.lastRenderedRange = null;
    }
  }

  /**
   * Update render if viewport has changed (for lazy rendering)
   */
  updateViewport(): void {
    if (
      this.lazyRenderingEnabled &&
      this.wasmDoc.getLength() >= this.viewportConfig.minDocumentSize
    ) {
      this.renderLazy();
    }
  }

  /**
   * Render a specific range of the document
   */
  renderRange(start: number, end: number): void {
    try {
      // Validate range
      if (start < 0 || end < start) {
        throw new EditorError(
          `Invalid range: start=${start}, end=${end}`,
          ErrorCodes.INVALID_RANGE
        );
      }

      const length = this.wasmDoc.getLength();
      if (start > length || end > length) {
        throw new EditorError(
          `Range out of bounds: start=${start}, end=${end}, length=${length}`,
          ErrorCodes.INVALID_RANGE
        );
      }

      logger.debug(`Rendering range: ${start}-${end}`);

      // Clear position cache for affected range
      this.positionCache.clear();

      // Get HTML for the specific range
      const html = this.wasmDoc.toHTMLRange(start, end);

      // Find the DOM nodes that correspond to this range
      const startNode = this.getNodeAtPosition(start);
      const endNode = this.getNodeAtPosition(end);

      // Create a temporary container to parse the HTML
      const temp = document.createElement("div");
      temp.innerHTML = html;

      // Replace the content between start and end nodes
      // This is a simplified implementation - a production version would need
      // more sophisticated DOM manipulation to preserve selection
      const range = document.createRange();
      range.setStart(startNode.node, startNode.offset);
      range.setEnd(endNode.node, endNode.offset);
      range.deleteContents();

      // Insert the new content
      const fragment = document.createDocumentFragment();
      while (temp.firstChild) {
        fragment.appendChild(temp.firstChild);
      }
      range.insertNode(fragment);

      logger.debug("Range rendered successfully");
    } catch (error) {
      if (error instanceof EditorError) {
        throw error;
      }
      logger.error("Failed to render range:", error);
      throw new EditorError(
        `Failed to render range: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.RENDER_ERROR,
        error
      );
    }
  }

  /**
   * Sync DOM changes back to WASM document
   */
  syncFromDOM(): void {
    try {
      logger.debug("Syncing DOM to WASM");

      // Get the current text content from the DOM
      const domText = this.editor.textContent || "";

      // Get the current text from WASM
      const wasmText = this.wasmDoc.getContent();

      // If they're the same, no sync needed
      if (domText === wasmText) {
        logger.debug("DOM and WASM are in sync, no changes needed");
        return;
      }

      // Simple sync: replace entire content
      // A more sophisticated implementation would use diff algorithms
      // to find minimal changes and preserve formatting
      const length = this.wasmDoc.getLength();
      if (length > 0) {
        this.wasmDoc.deleteRange(0, length);
      }
      if (domText.length > 0) {
        this.wasmDoc.insertText(domText, 0);
      }

      logger.debug("DOM synced to WASM successfully");
    } catch (error) {
      logger.error("Failed to sync DOM to WASM:", error);
      throw new EditorError(
        `Failed to sync DOM to WASM: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.DOM_SYNC_ERROR,
        error
      );
    }
  }

  /**
   * Render formatted text with inline styles
   * This method is reserved for future use when manual DOM construction is needed
   * @internal
   */
  // @ts-ignore - Reserved for future use
  private renderFormats(text: string, formats: Format[]): HTMLElement {
    let element: HTMLElement = document.createElement("span");

    // If no formats, just return text in a span
    if (formats.length === 0) {
      element.textContent = text;
      return element;
    }

    // Apply each format by wrapping in appropriate elements
    for (const format of formats) {
      const wrapper = this.createFormatElement(format);
      if (element.textContent === "") {
        element.textContent = text;
      }
      wrapper.appendChild(element);
      element = wrapper;
    }

    return element;
  }

  /**
   * Create an HTML element for a specific format
   */
  private createFormatElement(format: Format): HTMLElement {
    if (typeof format === "string") {
      // Simple format types
      switch (format) {
        case "bold":
          return document.createElement("strong");
        case "italic":
          return document.createElement("em");
        case "underline": {
          const u = document.createElement("u");
          return u;
        }
        case "strikethrough": {
          const s = document.createElement("s");
          return s;
        }
        case "code":
          return document.createElement("code");
        default: {
          const span = document.createElement("span");
          return span;
        }
      }
    } else {
      // Format with value
      const span = document.createElement("span");

      if (format.type === "link") {
        const a = document.createElement("a");
        a.href = format.url;
        a.target = "_blank";
        a.rel = "noopener noreferrer";
        return a;
      } else if (format.type === "textColor" || format.type === "text-color") {
        span.style.color = format.color;
        // Add CSP nonce if provided
        if (this.cspNonce) {
          span.setAttribute("nonce", this.cspNonce);
        }
        return span;
      } else if (
        format.type === "backgroundColor" ||
        format.type === "background-color"
      ) {
        span.style.backgroundColor = format.color;
        // Add CSP nonce if provided
        if (this.cspNonce) {
          span.setAttribute("nonce", this.cspNonce);
        }
        return span;
      }

      return span;
    }
  }

  // Note: renderBlock method removed as it's not needed with current toHTML() approach
  // If manual DOM construction is needed in the future, it can be re-added

  // Note: normalizeBlockType method removed along with renderBlock
  // Not needed with current toHTML() approach

  /**
   * Get DOM node at a specific document position
   */
  getNodeAtPosition(position: number): { node: Node; offset: number } {
    try {
      // Handle edge cases
      if (position < 0) {
        logger.warn(`Position ${position} is negative, clamping to 0`);
        return { node: this.editor, offset: 0 };
      }

      const length = this.wasmDoc.getLength();
      if (position > length) {
        logger.warn(
          `Position ${position} exceeds document length ${length}, clamping to end`
        );
        return { node: this.editor, offset: this.editor.childNodes.length };
      }

      // Walk through the DOM tree to find the node at the given position
      let currentPosition = 0;
      const walker = document.createTreeWalker(
        this.editor,
        NodeFilter.SHOW_TEXT | NodeFilter.SHOW_ELEMENT,
        null
      );

      let node: Node | null = walker.currentNode;

      while (node) {
        if (node.nodeType === Node.TEXT_NODE) {
          const textLength = node.textContent?.length || 0;

          if (currentPosition + textLength >= position) {
            // Found the text node containing this position
            const offset = position - currentPosition;
            return { node, offset };
          }

          currentPosition += textLength;
        } else if (node.nodeType === Node.ELEMENT_NODE) {
          const element = node as HTMLElement;

          // Handle line breaks
          if (element.tagName === "BR") {
            if (currentPosition === position) {
              return { node: element.parentNode || this.editor, offset: 0 };
            }
            currentPosition += 1;
          }
        }

        node = walker.nextNode();
      }

      // If we didn't find the exact position, return the last node
      logger.debug(
        `Position ${position} not found exactly, returning last node`
      );
      return { node: this.editor, offset: this.editor.childNodes.length };
    } catch (error) {
      logger.error("Failed to get node at position:", error);
      throw new EditorError(
        `Failed to get node at position ${position}: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_POSITION,
        error
      );
    }
  }

  /**
   * Get document position from DOM node
   */
  getPositionFromNode(node: Node, offset: number): number {
    try {
      // Validate offset
      if (offset < 0) {
        logger.warn(`Offset ${offset} is negative, clamping to 0`);
        offset = 0;
      }

      // Check cache first
      const cacheKey = node;
      if (this.positionCache.has(cacheKey)) {
        return this.positionCache.get(cacheKey)! + offset;
      }

      // Walk through the DOM tree to calculate position
      let position = 0;
      const walker = document.createTreeWalker(
        this.editor,
        NodeFilter.SHOW_TEXT | NodeFilter.SHOW_ELEMENT,
        null
      );

      let currentNode: Node | null = walker.currentNode;

      while (currentNode) {
        if (currentNode === node) {
          // Found the node, cache and return position
          this.positionCache.set(node, position);
          return position + offset;
        }

        if (currentNode.nodeType === Node.TEXT_NODE) {
          position += currentNode.textContent?.length || 0;
        } else if (currentNode.nodeType === Node.ELEMENT_NODE) {
          const element = currentNode as HTMLElement;

          // Handle line breaks
          if (element.tagName === "BR") {
            position += 1;
          }
        }

        currentNode = walker.nextNode();
      }

      // If node not found, return 0
      logger.warn("Node not found in editor, returning position 0");
      return 0;
    } catch (error) {
      logger.error("Failed to get position from node:", error);
      throw new EditorError(
        `Failed to get position from node: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_POSITION,
        error
      );
    }
  }

  /**
   * Clear the position cache
   * Should be called after any DOM modifications
   */
  clearPositionCache(): void {
    this.positionCache.clear();
  }

  /**
   * Flush any pending batched renders immediately
   */
  flushPendingRenders(): void {
    if (this.renderBatcher.hasPending()) {
      this.renderBatcher.cancel();
      this.renderImmediate();
    }
  }

  /**
   * Cleanup resources
   */
  destroy(): void {
    this.renderBatcher.cancel();
    this.positionCache.clear();
    this.lastRenderedRange = null;
  }
}
