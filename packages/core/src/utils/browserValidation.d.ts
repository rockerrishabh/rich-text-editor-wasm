/**
 * Browser validation and unsupported browser detection
 * Checks minimum browser versions and provides clear error messages
 */
/**
 * Minimum supported browser versions
 */
export declare const MINIMUM_BROWSER_VERSIONS: Record<string, number>;
/**
 * Browser support status
 */
export interface BrowserSupportStatus {
    supported: boolean;
    browserType: string;
    browserVersion: number;
    errors: string[];
    warnings: string[];
}
/**
 * Check if current browser is supported
 */
export declare function checkBrowserSupport(): BrowserSupportStatus;
/**
 * Get detailed error message for unsupported browser
 */
export declare function getUnsupportedBrowserMessage(status?: BrowserSupportStatus): string;
/**
 * Get user-friendly error message for unsupported browser
 */
export declare function getUserFriendlyErrorMessage(status?: BrowserSupportStatus): string;
/**
 * Validate browser and throw error if unsupported
 * @throws {EditorError} If browser is not supported
 */
export declare function validateBrowser(): void;
/**
 * Validate browser and show warning if issues detected
 * Returns true if browser is supported, false otherwise
 */
export declare function validateBrowserWithWarnings(onWarning?: (warning: string) => void): boolean;
/**
 * Get browser compatibility report
 * Useful for debugging and support
 */
export declare function getBrowserCompatibilityReport(): string;
/**
 * Create a user-facing error element for unsupported browsers
 */
export declare function createUnsupportedBrowserElement(): HTMLElement;
//# sourceMappingURL=browserValidation.d.ts.map