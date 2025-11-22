import type { RichTextEditor } from "../core/RichTextEditor";
import type { MemoryStats } from "../types";

/**
 * Helper class for document statistics operations
 */
export class StatsHelpers {
  private editor: RichTextEditor;

  constructor(editor: RichTextEditor) {
    this.editor = editor;
  }

  /**
   * Get the total character count in the document
   */
  getCharacterCount(): number {
    return this.editor.getLength();
  }

  /**
   * Get the word count in the document
   */
  getWordCount(): number {
    const wasmDoc = this.editor.getWasmDocument();
    return wasmDoc.getWordCount();
  }

  /**
   * Get the line count in the document
   */
  getLineCount(): number {
    const wasmDoc = this.editor.getWasmDocument();
    return wasmDoc.getLineCount();
  }

  /**
   * Get the character count of the current selection
   */
  getSelectedCharacterCount(): number {
    const selection = this.editor.getSelection();
    const start = Math.min(selection.anchor, selection.focus);
    const end = Math.max(selection.anchor, selection.focus);
    return end - start;
  }

  /**
   * Get the word count of the current selection
   */
  getSelectedWordCount(): number {
    const wasmDoc = this.editor.getWasmDocument();
    const selectedText = wasmDoc.getSelectedText();

    if (!selectedText || selectedText.length === 0) {
      return 0;
    }

    // Count words in selected text
    // Words are sequences of non-whitespace characters
    const words = selectedText.trim().split(/\s+/);
    return words.filter((word) => word.length > 0).length;
  }

  /**
   * Get memory usage statistics for the document
   */
  getMemoryStats(): MemoryStats {
    const wasmDoc = this.editor.getWasmDocument();
    return wasmDoc.getMemoryStats();
  }
}
