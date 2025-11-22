import type { RichTextEditor } from "../core/RichTextEditor";

/**
 * Information about the current selection
 */
export interface SelectionInfo {
  hasSelection: boolean;
  start: number;
  end: number;
  length: number;
  isCollapsed: boolean;
}

/**
 * Helper class for editor state operations
 */
export class StateHelpers {
  private editor: RichTextEditor;

  constructor(editor: RichTextEditor) {
    this.editor = editor;
  }

  /**
   * Check if undo is available
   */
  canUndo(): boolean {
    return this.editor.canUndo();
  }

  /**
   * Check if redo is available
   */
  canRedo(): boolean {
    return this.editor.canRedo();
  }

  /**
   * Check if there is an active selection (not just a cursor)
   */
  hasSelection(): boolean {
    const selection = this.editor.getSelection();
    return selection.anchor !== selection.focus;
  }

  /**
   * Get detailed information about the current selection
   */
  getSelectionInfo(): SelectionInfo {
    const selection = this.editor.getSelection();
    const start = Math.min(selection.anchor, selection.focus);
    const end = Math.max(selection.anchor, selection.focus);
    const length = end - start;

    return {
      hasSelection: length > 0,
      start,
      end,
      length,
      isCollapsed: length === 0,
    };
  }

  /**
   * Check if the document is empty
   */
  isEmpty(): boolean {
    return this.editor.getLength() === 0;
  }

  /**
   * Check if the document has been modified
   */
  isDirty(): boolean {
    const state = this.editor.getState();
    return state.isDirtyState();
  }

  /**
   * Get the current document length
   */
  getLength(): number {
    return this.editor.getLength();
  }

  /**
   * Check if IME composition is in progress
   */
  isComposing(): boolean {
    const state = this.editor.getState();
    return state.isComposingState();
  }
}
