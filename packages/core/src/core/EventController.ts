import type { RichTextEditor } from "./RichTextEditor";
import { KeyboardHandler } from "../events/KeyboardHandler";
import { InputHandler } from "../events/InputHandler";
import { SelectionHandler } from "../events/SelectionHandler";
import { ClipboardHandler } from "../events/ClipboardHandler";
import { IMEHandler } from "../events/IMEHandler";
import { EditorError, ErrorCodes } from "../types";
import { logger } from "../utils/logger";

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
export class EventController {
  private editor: RichTextEditor;
  private handlers: Map<string, EventHandler>;
  private isAttached: boolean = false;

  constructor(editor: RichTextEditor) {
    if (!editor) {
      throw new EditorError(
        "Invalid editor instance for EventController",
        ErrorCodes.INVALID_CONTAINER
      );
    }

    this.editor = editor;
    this.handlers = new Map();

    try {
      // Initialize all event handlers
      this.initializeHandlers();

      // Auto-attach handlers
      this.attach();

      logger.debug("EventController initialized");
    } catch (error) {
      logger.error("Failed to initialize EventController:", error);
      throw new EditorError(
        `Failed to initialize EventController: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.EVENT_HANDLER_ERROR,
        error
      );
    }
  }

  /**
   * Initialize all event handlers
   */
  private initializeHandlers(): void {
    try {
      logger.debug("Initializing event handlers");

      // Register keyboard handler
      this.registerHandler("keyboard", new KeyboardHandler(this.editor));

      // Register input handler
      this.registerHandler("input", new InputHandler(this.editor));

      // Register selection handler
      this.registerHandler("selection", new SelectionHandler(this.editor));

      // Register clipboard handler if enabled
      if (this.editor["options"].enableClipboard !== false) {
        this.registerHandler("clipboard", new ClipboardHandler(this.editor));
      }

      // Register IME handler if enabled
      if (this.editor["options"].enableIME !== false) {
        this.registerHandler("ime", new IMEHandler(this.editor));
      }

      logger.debug("Event handlers initialized successfully");
    } catch (error) {
      logger.error("Failed to initialize event handlers:", error);
      throw new EditorError(
        `Failed to initialize event handlers: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.EVENT_HANDLER_ERROR,
        error
      );
    }
  }

  /**
   * Register an event handler
   */
  registerHandler(name: string, handler: EventHandler): void {
    // If handler already exists, detach it first
    if (this.handlers.has(name)) {
      const existingHandler = this.handlers.get(name);
      if (existingHandler && this.isAttached) {
        existingHandler.detach();
      }
    }

    this.handlers.set(name, handler);

    // If already attached, attach the new handler immediately
    if (this.isAttached) {
      handler.attach();
    }
  }

  /**
   * Unregister an event handler
   */
  unregisterHandler(name: string): void {
    const handler = this.handlers.get(name);
    if (handler && this.isAttached) {
      handler.detach();
    }
    this.handlers.delete(name);
  }

  /**
   * Get a registered handler by name
   */
  getHandler(name: string): EventHandler | undefined {
    return this.handlers.get(name);
  }

  /**
   * Handle an event by delegating to appropriate handler
   * This can be used for custom event delegation if needed
   */
  handleEvent(event: Event): void {
    // Delegate to all handlers that can handle this event
    this.handlers.forEach((handler, name) => {
      try {
        handler.handleEvent(event);
      } catch (error) {
        logger.error(`Error in ${name} event handler:`, error);

        // Emit error event if handler is available
        const editorOptions = this.editor["options"];
        if (editorOptions.onError) {
          editorOptions.onError(
            new EditorError(
              `Event handler error in ${name}: ${
                error instanceof Error ? error.message : String(error)
              }`,
              ErrorCodes.EVENT_HANDLER_ERROR,
              error
            )
          );
        }
      }
    });
  }

  /**
   * Attach all event handlers
   */
  attach(): void {
    if (this.isAttached) {
      logger.debug("Event handlers already attached");
      return;
    }

    try {
      logger.debug("Attaching event handlers");
      this.handlers.forEach((handler, name) => {
        try {
          handler.attach();
          logger.debug(`${name} handler attached`);
        } catch (error) {
          logger.error(`Failed to attach ${name} handler:`, error);
          throw error;
        }
      });
      this.isAttached = true;
      logger.debug("All event handlers attached successfully");
    } catch (error) {
      logger.error("Failed to attach event handlers:", error);
      throw new EditorError(
        `Failed to attach event handlers: ${
          error instanceof Error ? error.message : String(error)
        }`,
        ErrorCodes.EVENT_HANDLER_ERROR,
        error
      );
    }
  }

  /**
   * Detach all event handlers
   */
  detach(): void {
    if (!this.isAttached) {
      logger.debug("Event handlers already detached");
      return;
    }

    try {
      logger.debug("Detaching event handlers");
      this.handlers.forEach((handler, name) => {
        try {
          handler.detach();
          logger.debug(`${name} handler detached`);
        } catch (error) {
          logger.error(`Failed to detach ${name} handler:`, error);
          // Continue detaching other handlers even if one fails
        }
      });
      this.isAttached = false;
      logger.debug("All event handlers detached");
    } catch (error) {
      logger.error("Failed to detach event handlers:", error);
      // Don't throw here as we're cleaning up
    }
  }

  /**
   * Check if event controller is attached
   */
  isEventControllerAttached(): boolean {
    return this.isAttached;
  }
}
