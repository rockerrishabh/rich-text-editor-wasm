import { WasmDocument } from "../../wasm/rte_core";
import { DOMRenderer } from "./DOMRenderer";
import { EventController } from "./EventController";
import { EditorState } from "./EditorState";
import type {
  EditorOptions,
  FormatType,
  ValueFormatType,
  BlockType,
  Format,
  Selection,
  EditorEventType,
  EditorEventCallback,
} from "../types";
import { EditorError, ErrorCodes } from "../types";
import {
  setupEditorAria,
  createLiveRegion,
  announce,
  formatTypeToLabel,
  blockTypeToLabel,
  setupEditorFocusManagement,
} from "../utils/accessibility";
import { logger } from "../utils/logger";
import {
  registerWasmInstance,
  unregisterWasmInstance,
} from "../utils/wasmManager";

/**
 * Main editor class that orchestrates all functionality
 */
export class RichTextEditor {
  // Core properties
  private wasmDoc: WasmDocument;
  private container: HTMLElement;
  private editorElement: HTMLElement;
  private renderer: DOMRenderer;
  private eventController: EventController;
  private state: EditorState;
  private options: EditorOptions;
  private isDestroyed: boolean = false;
  private eventCallbacks: Map<EditorEventType, Set<EditorEventCallback>> =
    new Map();
  private liveRegion: HTMLElement | null = null;

  constructor(container: HTMLElement, options?: EditorOptions) {
    // Validate container
    if (!container || !(container instanceof HTMLElement)) {
      throw new EditorError(
        "Invalid container: must be an HTMLElement",
        ErrorCodes.INVALID_CONTAINER
      );
    }

    this.container = container;
    this.options = options || {};

    // Configure logger based on options
    if (this.options.debug) {
      // Shorthand for enabling debug logging
      logger.configure({ enabled: true, level: "debug" });
    } else if (
      this.options.logging !== undefined &&
      typeof this.options.logging !== "boolean"
    ) {
      logger.configure(this.options.logging);
    } else if (this.options.logging === true) {
      logger.configure({ enabled: true });
    }

    logger.info("Initializing RichTextEditor");

    try {
      // Initialize WASM document
      if (this.options.initialContent && this.options.initialFormat) {
        switch (this.options.initialFormat) {
          case "json":
            this.wasmDoc = WasmDocument.fromJSON(this.options.initialContent);
            break;
          case "html":
            this.wasmDoc = WasmDocument.fromHTML(this.options.initialContent);
            break;
          case "markdown":
            this.wasmDoc = WasmDocument.fromMarkdown(
              this.options.initialContent
            );
            break;
          case "text":
          default:
            this.wasmDoc = WasmDocument.fromText(this.options.initialContent);
            break;
        }
      } else if (this.options.initialContent) {
        // Default to plain text if no format specified
        this.wasmDoc = WasmDocument.fromText(this.options.initialContent);
      } else {
        // Create empty document
        this.wasmDoc = new WasmDocument();
      }

      // Set history limit if specified
      if (this.options.historyLimit !== undefined) {
        this.wasmDoc.setHistoryLimit(this.options.historyLimit);
      }

      // Create editor element
      this.editorElement = document.createElement("div");
      this.editorElement.contentEditable = "true";

      // Set up ARIA attributes for accessibility
      setupEditorAria(this.editorElement, {
        label: this.options.placeholder || "Rich text editor",
        describedBy: this.options.ariaDescribedBy,
        readOnly: this.options.readOnly,
      });

      // Apply options
      if (this.options.readOnly) {
        this.editorElement.contentEditable = "false";
      }

      if (this.options.spellCheck !== undefined) {
        this.editorElement.spellcheck = this.options.spellCheck;
      }

      // Apply CSS classes
      if (this.options.className) {
        this.editorElement.className = this.options.className;
      }
      if (this.options.classNames?.editor) {
        this.editorElement.classList.add(this.options.classNames.editor);
      }

      // Apply inline styles
      if (this.options.style) {
        Object.assign(this.editorElement.style, this.options.style);
      }

      // Apply theme
      if (this.options.theme && this.options.theme !== "none") {
        this.container.setAttribute("data-theme", this.options.theme);
      }

      // Apply CSS variables
      if (this.options.cssVariables) {
        Object.entries(this.options.cssVariables).forEach(([key, value]) => {
          this.container.style.setProperty(key, value);
        });
      }

      // Append editor to container
      this.container.appendChild(this.editorElement);

      // Create live region for screen reader announcements
      this.liveRegion = createLiveRegion(this.container, {
        politeness: "polite",
        atomic: true,
      });

      // Set up focus management for keyboard navigation
      setupEditorFocusManagement(this.editorElement);

      // Initialize renderer
      this.renderer = new DOMRenderer(
        this.editorElement,
        this.wasmDoc,
        this.options.cspNonce
      );

      // Initialize state
      this.state = new EditorState(this.wasmDoc);

      // Initialize event controller
      this.eventController = new EventController(this);

      // Set up WASM event listeners
      this.wasmDoc.onChange(() => {
        this.emit("change", this.getContent());
        if (this.options.onChange) {
          this.options.onChange(this.getContent());
        }
      });

      this.wasmDoc.onSelectionChange(() => {
        const selection = this.getSelection();
        this.emit("selectionChange", selection);
        if (this.options.onSelectionChange) {
          this.options.onSelectionChange(selection);
        }
      });

      // Auto-focus if requested
      if (this.options.autoFocus) {
        this.editorElement.focus();
      }

      // Initial render
      this.renderer.render();

      // Register this instance with the WASM manager for tracking
      registerWasmInstance();

      logger.info("RichTextEditor initialized successfully");
    } catch (error) {
      logger.error("Failed to initialize RichTextEditor:", error);
      throw new EditorError(
        `Failed to initialize editor: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.WASM_INIT_FAILED,
        error
      );
    }
  }

  // Document operations
  insertText(text: string, position: number): void {
    this.ensureNotDestroyed();

    try {
      logger.debug(`Inserting text at position ${position}:`, text);

      // Validate position
      const length = this.wasmDoc.getLength();
      if (position < 0 || position > length) {
        throw new EditorError(
          `Invalid position: ${position} (document length: ${length})`,
          ErrorCodes.INVALID_POSITION
        );
      }

      // Check max length if specified
      if (this.options.maxLength !== undefined) {
        const currentLength = this.wasmDoc.getLength();
        const newLength = currentLength + text.length;
        if (newLength > this.options.maxLength) {
          throw new EditorError(
            `Maximum length exceeded: ${this.options.maxLength} (attempted: ${newLength})`,
            ErrorCodes.MAX_LENGTH_EXCEEDED
          );
        }
      }

      this.wasmDoc.insertText(text, position);
      this.renderer.render();

      logger.debug("Text inserted successfully");
    } catch (error) {
      if (error instanceof EditorError) {
        throw error;
      }
      logger.error("Failed to insert text:", error);
      throw new EditorError(
        `Failed to insert text: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_POSITION,
        error
      );
    }
  }

  deleteRange(start: number, end: number): void {
    this.ensureNotDestroyed();

    try {
      // Validate range
      const length = this.wasmDoc.getLength();
      if (start < 0 || end < start || end > length) {
        throw new EditorError(
          `Invalid range: start=${start}, end=${end} (document length: ${length})`,
          ErrorCodes.INVALID_RANGE
        );
      }

      this.wasmDoc.deleteRange(start, end);
      this.renderer.render();
    } catch (error) {
      if (error instanceof EditorError) {
        throw error;
      }
      throw new EditorError(
        `Failed to delete range: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_RANGE,
        error
      );
    }
  }

  getContent(): string {
    this.ensureNotDestroyed();
    return this.wasmDoc.getContent();
  }

  getLength(): number {
    this.ensureNotDestroyed();
    return this.wasmDoc.getLength();
  }

  // Formatting operations
  applyFormat(
    format: FormatType,
    start: number,
    end: number,
    value?: string
  ): void {
    this.ensureNotDestroyed();

    try {
      // Check if this is a value format type
      const valueFormats: ValueFormatType[] = [
        "link",
        "textColor",
        "text-color",
        "backgroundColor",
        "background-color",
      ];

      if (valueFormats.includes(format as ValueFormatType)) {
        if (!value) {
          throw new EditorError(
            `Format type "${format}" requires a value parameter`,
            ErrorCodes.INVALID_FORMAT
          );
        }

        // Validate and sanitize the value based on format type
        let sanitizedValue = value;

        if (
          format === "textColor" ||
          format === "text-color" ||
          format === "backgroundColor" ||
          format === "background-color"
        ) {
          // Validate color values to prevent CSS injection
          const { validateColor } = require("../utils/security");
          const validatedColor = validateColor(value);
          if (!validatedColor) {
            throw new EditorError(
              `Invalid color value: ${value}. Please use a valid hex, rgb, rgba, hsl, or named color.`,
              ErrorCodes.INVALID_FORMAT
            );
          }
          sanitizedValue = validatedColor;
        } else if (format === "link") {
          // Sanitize URLs to prevent XSS attacks
          const { sanitizeURL } = require("../utils/security");
          const sanitizedURL = sanitizeURL(value);
          if (!sanitizedURL) {
            throw new EditorError(
              `Invalid or unsafe URL: ${value}. Please use a valid http, https, mailto, or tel URL.`,
              ErrorCodes.INVALID_FORMAT
            );
          }
          sanitizedValue = sanitizedURL;
        }

        this.wasmDoc.applyFormatWithValue(format, sanitizedValue, start, end);
      } else {
        this.wasmDoc.applyFormat(format, start, end);
      }

      this.renderer.render();

      // Announce format change to screen readers
      const formatLabel = formatTypeToLabel(format);
      this.announce(`${formatLabel} applied`);
    } catch (error) {
      if (error instanceof EditorError) {
        throw error;
      }
      throw new EditorError(
        `Failed to apply format: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_FORMAT
      );
    }
  }

  removeFormat(format: FormatType, start: number, end: number): void {
    this.ensureNotDestroyed();

    try {
      this.wasmDoc.removeFormat(format, start, end);
      this.renderer.render();

      // Announce format removal to screen readers
      const formatLabel = formatTypeToLabel(format);
      this.announce(`${formatLabel} removed`);
    } catch (error) {
      throw new EditorError(
        `Failed to remove format: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_FORMAT
      );
    }
  }

  getFormatsAt(position: number): Format[] {
    this.ensureNotDestroyed();

    try {
      return this.wasmDoc.getFormatsAt(position);
    } catch (error) {
      throw new EditorError(
        `Failed to get formats: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_POSITION
      );
    }
  }

  // Block operations
  setBlockType(blockType: BlockType, start: number, end: number): void {
    this.ensureNotDestroyed();

    try {
      this.wasmDoc.setBlockType(blockType, start, end);
      this.renderer.render();

      // Announce block type change to screen readers
      const blockLabel = blockTypeToLabel(blockType);
      this.announce(`Changed to ${blockLabel}`);
    } catch (error) {
      throw new EditorError(
        `Failed to set block type: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_FORMAT
      );
    }
  }

  getBlockTypeAt(position: number): BlockType {
    this.ensureNotDestroyed();

    try {
      return this.wasmDoc.getBlockTypeAt(position) as BlockType;
    } catch (error) {
      throw new EditorError(
        `Failed to get block type: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_POSITION
      );
    }
  }

  // Selection
  setSelection(anchor: number, focus: number): void {
    this.ensureNotDestroyed();

    try {
      // Validate selection
      const length = this.wasmDoc.getLength();
      if (anchor < 0 || anchor > length || focus < 0 || focus > length) {
        throw new EditorError(
          `Invalid selection: anchor=${anchor}, focus=${focus} (document length: ${length})`,
          ErrorCodes.INVALID_SELECTION
        );
      }

      this.wasmDoc.setSelection(anchor, focus);
      this.state.setSelection({ anchor, focus });
    } catch (error) {
      if (error instanceof EditorError) {
        throw error;
      }
      throw new EditorError(
        `Failed to set selection: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_SELECTION,
        error
      );
    }
  }

  getSelection(): Selection {
    this.ensureNotDestroyed();

    try {
      const selection = this.wasmDoc.getSelection();
      return {
        anchor: selection.anchor,
        focus: selection.focus,
      };
    } catch (error) {
      throw new EditorError(
        `Failed to get selection: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.INVALID_POSITION
      );
    }
  }

  // History
  undo(): void {
    this.ensureNotDestroyed();

    if (!this.options.enableHistory) {
      return;
    }

    try {
      const canUndo = this.wasmDoc.canUndo();
      if (canUndo) {
        this.wasmDoc.undo();
        this.renderer.render();

        // Announce undo action to screen readers
        this.announce("Undo");
      }
    } catch (error) {
      // Log warning but don't throw
      console.warn("Undo failed:", error);
      if (this.options.onError) {
        this.options.onError(
          new EditorError(
            `Undo operation failed: ${
              error instanceof Error ? error.message : String(error)
            }`,
            ErrorCodes.HISTORY_ERROR,
            error
          )
        );
      }
    }
  }

  redo(): void {
    this.ensureNotDestroyed();

    if (!this.options.enableHistory) {
      return;
    }

    try {
      const canRedo = this.wasmDoc.canRedo();
      if (canRedo) {
        this.wasmDoc.redo();
        this.renderer.render();

        // Announce redo action to screen readers
        this.announce("Redo");
      }
    } catch (error) {
      // Log warning but don't throw
      console.warn("Redo failed:", error);
      if (this.options.onError) {
        this.options.onError(
          new EditorError(
            `Redo operation failed: ${
              error instanceof Error ? error.message : String(error)
            }`,
            ErrorCodes.HISTORY_ERROR,
            error
          )
        );
      }
    }
  }

  canUndo(): boolean {
    this.ensureNotDestroyed();

    if (!this.options.enableHistory) {
      return false;
    }

    return this.wasmDoc.canUndo();
  }

  canRedo(): boolean {
    this.ensureNotDestroyed();

    if (!this.options.enableHistory) {
      return false;
    }

    return this.wasmDoc.canRedo();
  }

  // Serialization
  toJSON(): string {
    this.ensureNotDestroyed();

    try {
      return this.wasmDoc.toJSON();
    } catch (error) {
      throw new EditorError(
        `Failed to export to JSON: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.SERIALIZATION_ERROR,
        error
      );
    }
  }

  toHTML(): string {
    this.ensureNotDestroyed();

    try {
      return this.wasmDoc.toHTML();
    } catch (error) {
      throw new EditorError(
        `Failed to export to HTML: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.SERIALIZATION_ERROR,
        error
      );
    }
  }

  toMarkdown(): string {
    this.ensureNotDestroyed();

    try {
      return this.wasmDoc.toMarkdown();
    } catch (error) {
      throw new EditorError(
        `Failed to export to Markdown: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.SERIALIZATION_ERROR,
        error
      );
    }
  }

  toPlainText(): string {
    this.ensureNotDestroyed();

    try {
      return this.wasmDoc.toPlainText();
    } catch (error) {
      throw new EditorError(
        `Failed to export to plain text: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.SERIALIZATION_ERROR,
        error
      );
    }
  }

  // Import methods
  fromJSON(json: string): void {
    this.ensureNotDestroyed();

    try {
      // Validate input
      if (!json || typeof json !== "string") {
        throw new EditorError(
          "Invalid JSON input: must be a non-empty string",
          ErrorCodes.VALIDATION_ERROR
        );
      }

      // Create new document from JSON
      const newDoc = WasmDocument.fromJSON(json);

      // Free old document
      this.wasmDoc.free();

      // Replace with new document
      this.wasmDoc = newDoc;

      // Re-render
      this.renderer.render();
    } catch (error) {
      if (error instanceof EditorError) {
        throw error;
      }
      throw new EditorError(
        `Failed to import from JSON: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.SERIALIZATION_ERROR,
        error
      );
    }
  }

  fromHTML(html: string): void {
    this.ensureNotDestroyed();

    try {
      // Validate input
      if (html === null || html === undefined) {
        throw new EditorError(
          "Invalid HTML input: must not be null or undefined",
          ErrorCodes.VALIDATION_ERROR
        );
      }

      // Sanitize HTML to prevent XSS attacks
      const { sanitizeHTML } = require("../utils/security");
      const sanitizedHTML = sanitizeHTML(html);

      // Create new document from sanitized HTML
      const newDoc = WasmDocument.fromHTML(sanitizedHTML);

      // Free old document
      this.wasmDoc.free();

      // Replace with new document
      this.wasmDoc = newDoc;

      // Re-render
      this.renderer.render();

      logger.debug("HTML imported successfully (sanitized)");
    } catch (error) {
      if (error instanceof EditorError) {
        throw error;
      }
      throw new EditorError(
        `Failed to import from HTML: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.SERIALIZATION_ERROR,
        error
      );
    }
  }

  fromMarkdown(markdown: string): void {
    this.ensureNotDestroyed();

    try {
      // Validate input
      if (markdown === null || markdown === undefined) {
        throw new EditorError(
          "Invalid Markdown input: must not be null or undefined",
          ErrorCodes.VALIDATION_ERROR
        );
      }

      // Create new document from Markdown
      const newDoc = WasmDocument.fromMarkdown(markdown);

      // Free old document
      this.wasmDoc.free();

      // Replace with new document
      this.wasmDoc = newDoc;

      // Re-render
      this.renderer.render();
    } catch (error) {
      if (error instanceof EditorError) {
        throw error;
      }
      throw new EditorError(
        `Failed to import from Markdown: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.SERIALIZATION_ERROR,
        error
      );
    }
  }

  fromPlainText(text: string): void {
    this.ensureNotDestroyed();

    try {
      // Validate input
      if (text === null || text === undefined) {
        throw new EditorError(
          "Invalid text input: must not be null or undefined",
          ErrorCodes.VALIDATION_ERROR
        );
      }

      // Create new document from plain text
      const newDoc = WasmDocument.fromText(text);

      // Free old document
      this.wasmDoc.free();

      // Replace with new document
      this.wasmDoc = newDoc;

      // Re-render
      this.renderer.render();
    } catch (error) {
      if (error instanceof EditorError) {
        throw error;
      }
      throw new EditorError(
        `Failed to import from plain text: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.SERIALIZATION_ERROR,
        error
      );
    }
  }

  // Lifecycle
  destroy(): void {
    if (this.isDestroyed) {
      return;
    }

    try {
      // Detach event controller
      if (this.eventController) {
        this.eventController.detach();
      }

      // Clear all event callbacks
      this.eventCallbacks.clear();

      // Cleanup renderer resources
      if (this.renderer) {
        this.renderer.destroy();
      }

      // Remove editor element from DOM
      if (this.editorElement && this.editorElement.parentNode) {
        this.editorElement.parentNode.removeChild(this.editorElement);
      }

      // Free WASM resources
      if (this.wasmDoc) {
        this.wasmDoc.free();
      }

      // Unregister this instance from the WASM manager
      unregisterWasmInstance();

      this.isDestroyed = true;
    } catch (error) {
      console.error("Error during editor cleanup:", error);
    }
  }

  /**
   * Check if the editor has been destroyed
   */
  isEditorDestroyed(): boolean {
    return this.isDestroyed;
  }

  /**
   * Ensure editor is not destroyed before operations
   */
  private ensureNotDestroyed(): void {
    if (this.isDestroyed) {
      throw new EditorError(
        "Cannot perform operation on destroyed editor",
        ErrorCodes.EDITOR_DESTROYED
      );
    }
  }

  // Events
  on(event: EditorEventType, callback: EditorEventCallback): void {
    this.ensureNotDestroyed();

    if (!this.eventCallbacks.has(event)) {
      this.eventCallbacks.set(event, new Set());
    }

    this.eventCallbacks.get(event)!.add(callback);
  }

  off(event: EditorEventType, callback: EditorEventCallback): void {
    this.ensureNotDestroyed();

    const callbacks = this.eventCallbacks.get(event);
    if (callbacks) {
      callbacks.delete(callback);

      // Clean up empty sets
      if (callbacks.size === 0) {
        this.eventCallbacks.delete(event);
      }
    }
  }

  /**
   * Emit an event to all registered callbacks
   */
  private emit(event: EditorEventType, data?: any): void {
    const callbacks = this.eventCallbacks.get(event);
    if (callbacks) {
      callbacks.forEach((callback) => {
        try {
          callback(data);
        } catch (error) {
          console.error(`Error in event callback for "${event}":`, error);
          if (this.options.onError) {
            this.options.onError(
              new EditorError(
                `Event callback error: ${
                  error instanceof Error ? error.message : String(error)
                }`,
                "EVENT_CALLBACK_ERROR"
              )
            );
          }
        }
      });
    }
  }

  /**
   * Get the editor element
   */
  getEditorElement(): HTMLElement {
    return this.editorElement;
  }

  /**
   * Get the container element
   */
  getContainer(): HTMLElement {
    return this.container;
  }

  /**
   * Get the WASM document instance (for advanced use cases)
   */
  getWasmDocument(): WasmDocument {
    this.ensureNotDestroyed();
    return this.wasmDoc;
  }

  /**
   * Get the editor state
   */
  getState(): EditorState {
    this.ensureNotDestroyed();
    return this.state;
  }

  /**
   * Get the renderer
   */
  getRenderer(): DOMRenderer {
    this.ensureNotDestroyed();
    return this.renderer;
  }

  /**
   * Get the event controller
   */
  getEventController(): EventController {
    this.ensureNotDestroyed();
    return this.eventController;
  }

  /**
   * Announce a message to screen readers
   */
  announce(message: string): void {
    if (this.liveRegion) {
      announce(this.liveRegion, message);
    }
  }

  /**
   * Get the live region element for screen reader announcements
   */
  getLiveRegion(): HTMLElement | null {
    return this.liveRegion;
  }

  // Performance optimization methods

  /**
   * Enable or disable lazy rendering for large documents
   * @param enabled - Whether to enable lazy rendering
   * @param config - Optional viewport configuration
   */
  setLazyRendering(
    enabled: boolean,
    config?: Partial<import("../utils/performance").ViewportConfig>
  ): void {
    this.ensureNotDestroyed();
    this.renderer.setLazyRendering(enabled, config);
  }

  /**
   * Enable or disable batched DOM updates
   * @param enabled - Whether to enable batched updates
   */
  setBatchedUpdates(enabled: boolean): void {
    this.ensureNotDestroyed();
    this.renderer.setBatchedUpdates(enabled);
  }

  /**
   * Flush any pending batched renders immediately
   */
  flushPendingRenders(): void {
    this.ensureNotDestroyed();
    this.renderer.flushPendingRenders();
  }
}
