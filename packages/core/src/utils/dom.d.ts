/**
 * DOM utility functions
 */
/**
 * Check if an element is editable
 */
export declare function isEditable(element: HTMLElement): boolean;
/**
 * Check if an element is a descendant of another element
 */
export declare function isDescendant(parent: Node, child: Node): boolean;
/**
 * Get the closest ancestor element matching a selector
 */
export declare function closest(element: HTMLElement, selector: string): HTMLElement | null;
/**
 * Get all text nodes within an element
 */
export declare function getTextNodes(element: Node): Text[];
/**
 * Get text content from an element, preserving line breaks
 */
export declare function getTextContent(element: HTMLElement): string;
/**
 * Remove all children from an element
 */
export declare function removeAllChildren(element: HTMLElement): void;
/**
 * Create an element with attributes
 */
export declare function createElement(tagName: string, attributes?: Record<string, string>, children?: (Node | string)[]): HTMLElement;
/**
 * Set multiple attributes on an element
 */
export declare function setAttributes(element: HTMLElement, attributes: Record<string, string>): void;
//# sourceMappingURL=dom.d.ts.map