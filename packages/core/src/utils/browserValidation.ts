/**
 * Browser validation and unsupported browser detection
 * Checks minimum browser versions and provides clear error messages
 */

import {
  getBrowserType,
  getBrowserVersion,
  isBrowserSupported,
} from "./browser";
import { EditorError, ErrorCodes } from "../types";

/**
 * Minimum supported browser versions
 */
export const MINIMUM_BROWSER_VERSIONS: Record<string, number> = {
  chrome: 90,
  firefox: 78,
  safari: 14,
  edge: 90,
};

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
export function checkBrowserSupport(): BrowserSupportStatus {
  const browserType = getBrowserType();
  const browserVersion = getBrowserVersion();
  const errors: string[] = [];
  const warnings: string[] = [];

  // Check basic browser support
  if (!isBrowserSupported()) {
    errors.push(
      "Browser does not support required features (contenteditable, Selection API, IME)"
    );
  }

  // Check minimum version
  const minVersion = MINIMUM_BROWSER_VERSIONS[browserType];
  if (minVersion && browserVersion < minVersion) {
    errors.push(
      `${browserType} version ${browserVersion} is below minimum supported version ${minVersion}`
    );
  }

  // Add warnings for older but supported versions
  if (browserType === "safari" && browserVersion < 15) {
    warnings.push("Safari 15+ recommended for best experience");
  }

  return {
    supported: errors.length === 0,
    browserType,
    browserVersion,
    errors,
    warnings,
  };
}

/**
 * Get detailed error message for unsupported browser
 */
export function getUnsupportedBrowserMessage(
  status?: BrowserSupportStatus
): string {
  const s = status || checkBrowserSupport();

  if (s.supported) {
    return "";
  }

  let message = `Browser not supported: ${s.browserType} ${s.browserVersion}\n\n`;
  message += "Errors:\n";
  s.errors.forEach((error) => {
    message += `- ${error}\n`;
  });

  message += "\nSupported browsers:\n";
  Object.entries(MINIMUM_BROWSER_VERSIONS).forEach(([browser, version]) => {
    message += `- ${browser} ${version}+\n`;
  });

  return message;
}

/**
 * Get user-friendly error message for unsupported browser
 */
export function getUserFriendlyErrorMessage(
  status?: BrowserSupportStatus
): string {
  const s = status || checkBrowserSupport();

  if (s.supported) {
    return "";
  }

  return `Your browser (${s.browserType} ${s.browserVersion}) is not supported. Please update to the latest version or use a modern browser like Chrome, Firefox, Safari, or Edge.`;
}

/**
 * Validate browser and throw error if unsupported
 * @throws {EditorError} If browser is not supported
 */
export function validateBrowser(): void {
  const status = checkBrowserSupport();

  if (!status.supported) {
    throw new EditorError(
      getUserFriendlyErrorMessage(status),
      ErrorCodes.BROWSER_NOT_SUPPORTED
    );
  }
}

/**
 * Validate browser and show warning if issues detected
 * Returns true if browser is supported, false otherwise
 */
export function validateBrowserWithWarnings(
  onWarning?: (warning: string) => void
): boolean {
  const status = checkBrowserSupport();

  if (!status.supported) {
    return false;
  }

  if (status.warnings.length > 0 && onWarning) {
    status.warnings.forEach((warning) => onWarning(warning));
  }

  return true;
}

/**
 * Get browser compatibility report
 * Useful for debugging and support
 */
export function getBrowserCompatibilityReport(): string {
  const status = checkBrowserSupport();

  let report = "Browser Compatibility Report\n";
  report += "============================\n\n";
  report += `Browser: ${status.browserType} ${status.browserVersion}\n`;
  report += `Supported: ${status.supported ? "Yes" : "No"}\n\n`;

  if (status.errors.length > 0) {
    report += "Errors:\n";
    status.errors.forEach((error) => {
      report += `- ${error}\n`;
    });
    report += "\n";
  }

  if (status.warnings.length > 0) {
    report += "Warnings:\n";
    status.warnings.forEach((warning) => {
      report += `- ${warning}\n`;
    });
    report += "\n";
  }

  report += "Minimum Supported Versions:\n";
  Object.entries(MINIMUM_BROWSER_VERSIONS).forEach(([browser, version]) => {
    report += `- ${browser}: ${version}+\n`;
  });

  return report;
}

/**
 * Create a user-facing error element for unsupported browsers
 */
export function createUnsupportedBrowserElement(): HTMLElement {
  const status = checkBrowserSupport();
  const container = document.createElement("div");

  container.style.cssText = `
    padding: 20px;
    background-color: #fff3cd;
    border: 1px solid #ffc107;
    border-radius: 4px;
    color: #856404;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    font-size: 14px;
    line-height: 1.5;
  `;

  container.innerHTML = `
    <h3 style="margin-top: 0;">Browser Not Supported</h3>
    <p>${getUserFriendlyErrorMessage(status)}</p>
    <p style="margin-bottom: 0;">
      <strong>Supported browsers:</strong><br>
      ${Object.entries(MINIMUM_BROWSER_VERSIONS)
        .map(([browser, version]) => `${browser} ${version}+`)
        .join(", ")}
    </p>
  `;

  return container;
}
