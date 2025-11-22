import type { RichTextEditor } from "../core/RichTextEditor";
import type { BlockType } from "../types";

/**
 * Helper class for block-related operations
 */
export class BlockHelpers {
  private editor: RichTextEditor;

  // All available block types
  private readonly availableBlockTypes: BlockType[] = [
    "paragraph",
    "heading1",
    "heading2",
    "heading3",
    "heading4",
    "heading5",
    "heading6",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "bulletList",
    "numberedList",
    "blockQuote",
    "codeBlock",
  ];

  constructor(editor: RichTextEditor) {
    this.editor = editor;
  }

  /**
   * Get the block type at the current cursor position
   */
  getCurrentBlockType(): BlockType {
    const selection = this.editor.getSelection();
    const position = Math.min(selection.anchor, selection.focus);

    return this.editor.getBlockTypeAt(position);
  }

  /**
   * Set the block type for the current selection
   */
  setBlockType(blockType: BlockType): void {
    const selection = this.editor.getSelection();
    const start = Math.min(selection.anchor, selection.focus);
    const end = Math.max(selection.anchor, selection.focus);

    this.editor.setBlockType(blockType, start, end);
  }

  /**
   * Check if a specific block type is active at the current position
   */
  isBlockTypeActive(blockType: BlockType): boolean {
    const currentType = this.getCurrentBlockType();

    // Handle heading aliases (h1 === heading1, etc.)
    if (blockType === "h1" || blockType === "heading1") {
      return currentType === "h1" || currentType === "heading1";
    }
    if (blockType === "h2" || blockType === "heading2") {
      return currentType === "h2" || currentType === "heading2";
    }
    if (blockType === "h3" || blockType === "heading3") {
      return currentType === "h3" || currentType === "heading3";
    }
    if (blockType === "h4" || blockType === "heading4") {
      return currentType === "h4" || currentType === "heading4";
    }
    if (blockType === "h5" || blockType === "heading5") {
      return currentType === "h5" || currentType === "heading5";
    }
    if (blockType === "h6" || blockType === "heading6") {
      return currentType === "h6" || currentType === "heading6";
    }

    return currentType === blockType;
  }

  /**
   * Get all available block types
   */
  getAvailableBlockTypes(): BlockType[] {
    return [...this.availableBlockTypes];
  }
}
