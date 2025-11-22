/**
 * Browser detection and feature detection utilities
 */

import type { Selection } from "../types";

/**
 * Detect if running in a browser environment
 */
export function isBrowser(): boolean {
  return typeof window !== "undefined" && typeof document !== "undefined";
}

/**
 * Detect browser type
 */
export function getBrowserType():
  | "chrome"
  | "firefox"
  | "safari"
  | "edge"
  | "unknown" {
  if (!isBrowser()) return "unknown";

  const ua = navigator.userAgent.toLowerCase();

  if (ua.indexOf("edg/") > -1) return "edge";
  if (ua.indexOf("chrome") > -1 && ua.indexOf("edg/") === -1) return "chrome";
  if (ua.indexOf("firefox") > -1) return "firefox";
  if (ua.indexOf("safari") > -1 && ua.indexOf("chrome") === -1) return "safari";

  return "unknown";
}

/**
 * Detect if running on mobile device
 */
export function isMobile(): boolean {
  if (!isBrowser()) return false;
  return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
    navigator.userAgent
  );
}

/**
 * Detect if running on iOS
 */
export function isIOS(): boolean {
  if (!isBrowser()) return false;
  return /iPad|iPhone|iPod/.test(navigator.userAgent);
}

/**
 * Get browser version
 */
export function getBrowserVersion(): number {
  if (!isBrowser()) return 0;

  const ua = navigator.userAgent;
  const browserType = getBrowserType();

  let match: RegExpMatchArray | null = null;

  switch (browserType) {
    case "chrome":
      match = ua.match(/Chrome\/(\d+)/);
      break;
    case "firefox":
      match = ua.match(/Firefox\/(\d+)/);
      break;
    case "safari":
      match = ua.match(/Version\/(\d+)/);
      break;
    case "edge":
      match = ua.match(/Edg\/(\d+)/);
      break;
  }

  return match ? parseInt(match[1], 10) : 0;
}

/**
 * Check if browser supports contenteditable
 */
export function hasContentEditable(): boolean {
  if (!isBrowser()) return false;
  return "contentEditable" in document.createElement("div");
}

/**
 * Check if browser supports Selection API
 */
export function hasSelectionAPI(): boolean {
  if (!isBrowser()) return false;
  return typeof window.getSelection === "function";
}

/**
 * Check if browser supports Clipboard API
 */
export function hasClipboardAPI(): boolean {
  if (!isBrowser()) return false;
  return typeof navigator.clipboard !== "undefined";
}

/**
 * Check if browser supports IME composition events
 */
export function hasIMESupport(): boolean {
  if (!isBrowser()) return false;
  return "oncompositionstart" in document.createElement("div");
}

/**
 * Check if browser is supported
 */
export function isBrowserSupported(): boolean {
  if (!isBrowser()) return false;

  return hasContentEditable() && hasSelectionAPI() && hasIMESupport();
}

/**
 * Get browser compatibility warnings
 */
export function getBrowserWarnings(): string[] {
  const warnings: string[] = [];

  if (!hasClipboardAPI()) {
    warnings.push("Clipboard API not supported - copy/paste may be limited");
  }

  const browserType = getBrowserType();
  const version = getBrowserVersion();

  if (browserType === "safari" && version < 14) {
    warnings.push("Safari version is outdated - some features may not work");
  }

  if (browserType === "firefox" && version < 78) {
    warnings.push("Firefox version is outdated - some features may not work");
  }

  if (browserType === "chrome" && version < 90) {
    warnings.push("Chrome version is outdated - some features may not work");
  }

  return warnings;
}

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
export class BrowserCompat {
  private static _instance: BrowserCompat;
  private _browserType: ReturnType<typeof getBrowserType>;
  private _browserVersion: number;
  private _isMobile: boolean;
  private _isIOS: boolean;

  private constructor() {
    this._browserType = getBrowserType();
    this._browserVersion = getBrowserVersion();
    this._isMobile = isMobile();
    this._isIOS = isIOS();
  }

  /**
   * Get singleton instance
   */
  static getInstance(): BrowserCompat {
    if (!BrowserCompat._instance) {
      BrowserCompat._instance = new BrowserCompat();
    }
    return BrowserCompat._instance;
  }

  /**
   * Get browser type
   */
  get browserType(): ReturnType<typeof getBrowserType> {
    return this._browserType;
  }

  /**
   * Get browser version
   */
  get browserVersion(): number {
    return this._browserVersion;
  }

  /**
   * Check if mobile device
   */
  get isMobile(): boolean {
    return this._isMobile;
  }

  /**
   * Check if iOS
   */
  get isIOS(): boolean {
    return this._isIOS;
  }

  /**
   * Check if browser supports contenteditable
   */
  hasContentEditable(): boolean {
    return hasContentEditable();
  }

  /**
   * Check if browser supports Selection API
   */
  hasSelectionAPI(): boolean {
    return hasSelectionAPI();
  }

  /**
   * Check if browser supports Clipboard API
   */
  hasClipboardAPI(): boolean {
    return hasClipboardAPI();
  }

  /**
   * Check if browser supports IME composition events
   */
  hasIMESupport(): boolean {
    return hasIMESupport();
  }

  /**
   * Normalize DOM Selection to editor Selection
   * Handles browser-specific quirks in selection API
   */
  normalizeSelection(
    domSelection: globalThis.Selection | null
  ): Selection | null {
    if (!domSelection || domSelection.rangeCount === 0) {
      return null;
    }

    // Placeholder - actual implementation would calculate position from range
    return {
      anchor: 0,
      focus: 0,
    };
  }

  /**
   * Normalize keyboard event across browsers
   * Handles differences in key codes, modifier keys, and behavior
   */
  normalizeKeyEvent(event: KeyboardEvent): NormalizedKeyEvent {
    return {
      key: event.key,
      code: event.code,
      ctrlKey: event.ctrlKey,
      shiftKey: event.shiftKey,
      altKey: event.altKey,
      metaKey: event.metaKey,
      isComposing: event.isComposing || false,
    };
  }

  /**
   * Normalize input event across browsers
   * Handles differences in inputType, data, and composition state
   */
  normalizeInputEvent(event: InputEvent): NormalizedInputEvent {
    return {
      inputType: event.inputType || "insertText",
      data: event.data || null,
      isComposing: event.isComposing || false,
    };
  }

  /**
   * Check if browser meets minimum version requirements
   */
  meetsMinimumVersion(): boolean {
    const minimumVersions: Record<string, number> = {
      chrome: 90,
      firefox: 78,
      safari: 14,
      edge: 90,
    };

    const minVersion = minimumVersions[this._browserType];
    if (!minVersion) return true;

    return this._browserVersion >= minVersion;
  }

  /**
   * Get unsupported browser error message
   */
  getUnsupportedBrowserMessage(): string | null {
    if (!isBrowserSupported()) {
      return "Your browser does not support the required features for this editor.";
    }

    if (!this.meetsMinimumVersion()) {
      return `Your ${this._browserType} version (${this._browserVersion}) is outdated. Please update to the latest version.`;
    }

    return null;
  }

  /**
   * Apply browser-specific fixes to an element
   */
  applyBrowserFixes(element: HTMLElement): void {
    // Ensure text is selectable across all browsers
    element.style.userSelect = "text";
  }

  /**
   * Check if event is a composition event (IME input)
   */
  isCompositionEvent(event: Event): boolean {
    return (
      event.type === "compositionstart" ||
      event.type === "compositionupdate" ||
      event.type === "compositionend"
    );
  }
}
