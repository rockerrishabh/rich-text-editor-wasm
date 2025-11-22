/**
 * Security utilities for sanitizing user input and preventing XSS attacks
 */
/**
 * Sanitize HTML content to prevent XSS attacks
 * @param html - Raw HTML string to sanitize
 * @returns Sanitized HTML string
 */
export declare function sanitizeHTML(html: string): string;
/**
 * Sanitize a URL to prevent javascript: and data: URLs
 * @param url - URL string to sanitize
 * @returns Sanitized URL or null if unsafe
 */
export declare function sanitizeURL(url: string): string | null;
/**
 * Sanitize inline CSS styles to prevent CSS injection
 * @param style - CSS style string to sanitize
 * @returns Sanitized CSS style string
 */
export declare function sanitizeStyle(style: string): string;
/**
 * Validate a color value to ensure it's safe
 * @param color - Color value to validate (hex, rgb, rgba, named color)
 * @returns Validated color value or null if invalid
 */
export declare function validateColor(color: string): string | null;
/**
 * Escape HTML special characters to prevent XSS
 * @param text - Text to escape
 * @returns Escaped text
 */
export declare function escapeHTML(text: string): string;
//# sourceMappingURL=security.d.ts.map