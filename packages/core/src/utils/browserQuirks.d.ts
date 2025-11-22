/**
 * Browser-specific quirks and fixes
 * Handles rendering differences, IME composition, and mobile behavior
 */
/**
 * Safari-specific quirks
 */
export declare class SafariQuirks {
    static fixSelection(element: HTMLElement): void;
    static fixLineBreaks(element: HTMLElement): void;
}
/**
 * Firefox-specific quirks
 */
export declare class FirefoxQuirks {
    static fixSelection(element: HTMLElement): void;
    static fixPaste(html: string): string;
}
/**
 * Mobile-specific quirks
 */
export declare class MobileQuirks {
    static preventZoom(element: HTMLElement): void;
    static fixVirtualKeyboard(element: HTMLElement): void;
}
/**
 * Browser quirks manager
 * Applies appropriate fixes based on browser detection
 */
export declare class BrowserQuirksManager {
    private compat;
    constructor();
    /**
     * Apply all necessary browser fixes to an element
     */
    applyFixes(element: HTMLElement): void;
    /**
     * Fix block rendering based on browser
     */
    fixBlockRendering(blockElement: HTMLElement): void;
    /**
     * Fix list rendering based on browser
     */
    fixListRendering(listElement: HTMLUListElement | HTMLOListElement): void;
    /**
     * Normalize selection based on browser
     */
    normalizeSelection(_selection: globalThis.Selection): boolean;
    /**
     * Clean pasted content based on browser
     */
    cleanPastedContent(html: string): string;
    /**
     * Fix link element based on browser
     */
    fixLinkElement(linkElement: HTMLAnchorElement): void;
}
//# sourceMappingURL=browserQuirks.d.ts.map