/**
 * @rockerrishabh/rich-text-editor
 *
 * Framework-agnostic vanilla JavaScript/TypeScript library for rich text editing
 */

// Core classes
export { RichTextEditor } from "./core/RichTextEditor";
export { DOMRenderer } from "./core/DOMRenderer";
export { EventController } from "./core/EventController";
export { EditorState } from "./core/EditorState";

// Event handlers
export { KeyboardHandler } from "./events/KeyboardHandler";
export { InputHandler } from "./events/InputHandler";
export { SelectionHandler } from "./events/SelectionHandler";
export { ClipboardHandler } from "./events/ClipboardHandler";
export { IMEHandler } from "./events/IMEHandler";

// Helper classes
export { FormatHelpers } from "./helpers/formatHelpers";
export { BlockHelpers } from "./helpers/blockHelpers";
export { StateHelpers } from "./helpers/stateHelpers";
export { StatsHelpers } from "./helpers/statsHelpers";
export { ExportHelpers } from "./helpers/exportHelpers";

// Utilities
export * from "./utils/browser";
export * from "./utils/browserQuirks";
export * from "./utils/browserValidation";
export * from "./utils/dom";
export { logger, createLogger } from "./utils/logger";
export type { CustomLogger } from "./utils/logger";
export {
  getWasmMemoryStats,
  getActiveInstanceCount,
} from "./utils/wasmManager";
export {
  debounce,
  throttle,
  RenderBatcher,
  calculateVisibleRange,
  DEFAULT_VIEWPORT_CONFIG,
} from "./utils/performance";
export type { ViewportConfig } from "./utils/performance";
export {
  sanitizeHTML,
  sanitizeURL,
  sanitizeStyle,
  validateColor,
  escapeHTML,
} from "./utils/security";

// Types
export type {
  Selection,
  SearchMatch,
  ClipboardData,
  CompositionRange,
  DirtyRegion,
  MemoryStats,
  SimpleFormatType,
  ValueFormatType,
  FormatType,
  Format,
  BlockType,
  EditorEventType,
  EditorEventCallback,
  EditorOptions,
  ToolbarConfig,
  ToolbarButtonType,
  ToolbarButton,
  ToolbarButtonGroup,
  CustomToolbarButton,
  StatusBarConfig,
  StatusBarItemType,
  StatusBarItem,
  LogLevel,
  LoggingConfig,
  ErrorCode,
} from "./types";

export { EditorError, ErrorCodes } from "./types";

// Re-export SelectionInfo from stateHelpers
export type { SelectionInfo } from "./helpers/stateHelpers";
