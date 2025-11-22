/**
 * Accessibility utilities for ARIA attributes and screen reader support
 */
export interface AriaConfig {
    label?: string;
    describedBy?: string;
    readOnly?: boolean;
}
export interface LiveRegionConfig {
    politeness?: "polite" | "assertive";
    atomic?: boolean;
}
/**
 * Set up ARIA attributes on editor element
 */
export declare function setupEditorAria(element: HTMLElement, config: AriaConfig): void;
/**
 * Create a live region for screen reader announcements
 */
export declare function createLiveRegion(container: HTMLElement, config: LiveRegionConfig): HTMLElement;
/**
 * Announce a message to screen readers
 */
export declare function announce(liveRegion: HTMLElement, message: string): void;
/**
 * Convert format type to accessible label
 */
export declare function formatTypeToLabel(format: string): string;
/**
 * Convert block type to accessible label
 */
export declare function blockTypeToLabel(blockType: string): string;
/**
 * Set up focus management for keyboard navigation
 */
export declare function setupEditorFocusManagement(element: HTMLElement): void;
//# sourceMappingURL=accessibility.d.ts.map