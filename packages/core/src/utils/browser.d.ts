/**
 * Browser detection and feature detection utilities
 */
import type { Selection } from "../types";
/**
 * Detect if running in a browser environment
 */
export declare function isBrowser(): boolean;
/**
 * Detect browser type
 */
export declare function getBrowserType(): "chrome" | "firefox" | "safari" | "edge" | "unknown";
/**
 * Detect if running on mobile device
 */
export declare function isMobile(): boolean;
/**
 * Detect if running on iOS
 */
export declare function isIOS(): boolean;
/**
 * Get browser version
 */
export declare function getBrowserVersion(): number;
/**
 * Check if browser supports contenteditable
 */
export declare function hasContentEditable(): boolean;
/**
 * Check if browser supports Selection API
 */
export declare function hasSelectionAPI(): boolean;
/**
 * Check if browser supports Clipboard API
 */
export declare function hasClipboardAPI(): boolean;
/**
 * Check if browser supports IME composition events
 */
export declare function hasIMESupport(): boolean;
/**
 * Check if browser is supported
 */
export declare function isBrowserSupported(): boolean;
/**
 * Get browser compatibility warnings
 */
export declare function getBrowserWarnings(): string[];
/**
 * Normalized key event interface
 */
export interface NormalizedKeyEvent {
    key: string;
    code: string;
    ctrlKey: boolean;
    shiftKey: boolean;
    altKey: boolean;
    metaKey: boolean;
    isComposing: boolean;
}
/**
 * Normalized input event interface
 */
export interface NormalizedInputEvent {
    inputType: string;
    data: string | null;
    isComposing: boolean;
}
/**
 * Browser compatibility utility class
 * Provides feature detection and behavior normalization across browsers
 */
export declare class BrowserCompat {
    private static _instance;
    private _browserType;
    private _browserVersion;
    private _isMobile;
    private _isIOS;
    private constructor();
    /**
     * Get singleton instance
     */
    static getInstance(): BrowserCompat;
    /**
     * Get browser type
     */
    get browserType(): ReturnType<typeof getBrowserType>;
    /**
     * Get browser version
     */
    get browserVersion(): number;
    /**
     * Check if mobile device
     */
    get isMobile(): boolean;
    /**
     * Check if iOS
     */
    get isIOS(): boolean;
    /**
     * Check if browser supports contenteditable
     */
    hasContentEditable(): boolean;
    /**
     * Check if browser supports Selection API
     */
    hasSelectionAPI(): boolean;
    /**
     * Check if browser supports Clipboard API
     */
    hasClipboardAPI(): boolean;
    /**
     * Check if browser supports IME composition events
     */
    hasIMESupport(): boolean;
    /**
     * Normalize DOM Selection to editor Selection
     * Handles browser-specific quirks in selection API
     */
    normalizeSelection(domSelection: globalThis.Selection | null): Selection | null;
    /**
     * Normalize keyboard event across browsers
     * Handles differences in key codes, modifier keys, and behavior
     */
    normalizeKeyEvent(event: KeyboardEvent): NormalizedKeyEvent;
    /**
     * Normalize input event across browsers
     * Handles differences in inputType, data, and composition state
     */
    normalizeInputEvent(event: InputEvent): NormalizedInputEvent;
    /**
     * Check if browser meets minimum version requirements
     */
    meetsMinimumVersion(): boolean;
    /**
     * Get unsupported browser error message
     */
    getUnsupportedBrowserMessage(): string | null;
    /**
     * Apply browser-specific fixes to an element
     */
    applyBrowserFixes(element: HTMLElement): void;
    /**
     * Check if event is a composition event (IME input)
     */
    isCompositionEvent(event: Event): boolean;
}
//# sourceMappingURL=browser.d.ts.map