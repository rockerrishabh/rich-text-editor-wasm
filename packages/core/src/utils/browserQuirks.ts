/**
 * Browser-specific quirks and fixes
 * Handles rendering differences, IME composition, and mobile behavior
 */

import { BrowserCompat } from "./browser";

/**
 * Safari-specific quirks
 */
export class SafariQuirks {
  static fixSelection(element: HTMLElement): void {
    element.style.userSelect = "text";
  }

  static fixLineBreaks(element: HTMLElement): void {
    // Safari sometimes adds extra <br> tags
    const brs = element.querySelectorAll("br");
    brs.forEach((br) => {
      if (br.nextSibling && br.nextSibling.nodeName === "BR") {
        br.remove();
      }
    });
  }
}

/**
 * Firefox-specific quirks
 */
export class FirefoxQuirks {
  static fixSelection(element: HTMLElement): void {
    element.style.userSelect = "text";
  }

  static fixPaste(html: string): string {
    // Firefox sometimes includes extra metadata
    return html.replace(/<meta[^>]*>/gi, "");
  }
}

/**
 * Mobile-specific quirks
 */
export class MobileQuirks {
  static preventZoom(element: HTMLElement): void {
    element.style.fontSize = "16px"; // Prevents iOS zoom on focus
  }

  static fixVirtualKeyboard(element: HTMLElement): void {
    // Ensure element scrolls into view when keyboard appears
    element.addEventListener("focus", () => {
      setTimeout(() => {
        element.scrollIntoView({ behavior: "smooth", block: "center" });
      }, 300);
    });
  }
}

/**
 * Browser quirks manager
 * Applies appropriate fixes based on browser detection
 */
export class BrowserQuirksManager {
  private compat: BrowserCompat;

  constructor() {
    this.compat = BrowserCompat.getInstance();
  }

  /**
   * Apply all necessary browser fixes to an element
   */
  applyFixes(element: HTMLElement): void {
    this.compat.applyBrowserFixes(element);

    if (this.compat.browserType === "safari") {
      SafariQuirks.fixSelection(element);
    }

    if (this.compat.browserType === "firefox") {
      FirefoxQuirks.fixSelection(element);
    }

    if (this.compat.isMobile) {
      MobileQuirks.preventZoom(element);
      MobileQuirks.fixVirtualKeyboard(element);
    }
  }

  /**
   * Fix block rendering based on browser
   */
  fixBlockRendering(blockElement: HTMLElement): void {
    if (this.compat.browserType === "safari") {
      SafariQuirks.fixLineBreaks(blockElement);
    }
  }

  /**
   * Fix list rendering based on browser
   */
  fixListRendering(listElement: HTMLUListElement | HTMLOListElement): void {
    // Ensure consistent list styling across browsers
    listElement.style.paddingLeft = "40px";
  }

  /**
   * Normalize selection based on browser
   */
  normalizeSelection(_selection: globalThis.Selection): boolean {
    // Return true if selection was normalized
    return false;
  }

  /**
   * Clean pasted content based on browser
   */
  cleanPastedContent(html: string): string {
    let cleaned = html;

    if (this.compat.browserType === "firefox") {
      cleaned = FirefoxQuirks.fixPaste(cleaned);
    }

    // Remove common unwanted elements
    cleaned = cleaned.replace(/<script[^>]*>.*?<\/script>/gi, "");
    cleaned = cleaned.replace(/<style[^>]*>.*?<\/style>/gi, "");

    return cleaned;
  }

  /**
   * Fix link element based on browser
   */
  fixLinkElement(linkElement: HTMLAnchorElement): void {
    // Ensure links open in new tab
    linkElement.target = "_blank";
    linkElement.rel = "noopener noreferrer";
  }
}
