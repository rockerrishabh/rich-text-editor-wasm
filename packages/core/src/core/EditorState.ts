import type { WasmDocument } from "../../wasm/rte_core";
import type { Selection, Format } from "../types";

/**
 * Manages editor state
 */
export class EditorState {
  private selection: Selection;
  private activeFormats: Format[];
  private isDirty: boolean;
  private isComposing: boolean;

  constructor(_wasmDoc: WasmDocument) {
    this.selection = { anchor: 0, focus: 0 };
    this.activeFormats = [];
    this.isDirty = false;
    this.isComposing = false;
  }

  /**
   * Get current selection
   */
  getSelection(): Selection {
    return { ...this.selection };
  }

  /**
   * Set current selection
   */
  setSelection(selection: Selection): void {
    this.selection = { ...selection };
  }

  /**
   * Get active formats at current selection
   */
  getActiveFormats(): Format[] {
    return [...this.activeFormats];
  }

  /**
   * Set active formats
   */
  setActiveFormats(formats: Format[]): void {
    this.activeFormats = [...formats];
  }

  /**
   * Check if document has been modified
   */
  isDirtyState(): boolean {
    return this.isDirty;
  }

  /**
   * Mark document as dirty
   */
  markDirty(): void {
    this.isDirty = true;
  }

  /**
   * Mark document as clean
   */
  markClean(): void {
    this.isDirty = false;
  }

  /**
   * Check if IME composition is in progress
   */
  isComposingState(): boolean {
    return this.isComposing;
  }

  /**
   * Set IME composition state
   */
  setComposing(composing: boolean): void {
    this.isComposing = composing;
  }
}
