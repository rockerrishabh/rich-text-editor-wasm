import type { RichTextEditor } from "../core/RichTextEditor";
import type { BlockType } from "../types";
/**
 * Helper class for block-related operations
 */
export declare class BlockHelpers {
    private editor;
    private readonly availableBlockTypes;
    constructor(editor: RichTextEditor);
    /**
     * Get the block type at the current cursor position
     */
    getCurrentBlockType(): BlockType;
    /**
     * Set the block type for the current selection
     */
    setBlockType(blockType: BlockType): void;
    /**
     * Check if a specific block type is active at the current position
     */
    isBlockTypeActive(blockType: BlockType): boolean;
    /**
     * Get all available block types
     */
    getAvailableBlockTypes(): BlockType[];
}
//# sourceMappingURL=blockHelpers.d.ts.map