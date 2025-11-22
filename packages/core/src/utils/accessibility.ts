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
export function setupEditorAria(
  element: HTMLElement,
  config: AriaConfig
): void {
  element.setAttribute("role", "textbox");
  element.setAttribute("aria-multiline", "true");

  if (config.label) {
    element.setAttribute("aria-label", config.label);
  }

  if (config.describedBy) {
    element.setAttribute("aria-describedby", config.describedBy);
  }

  if (config.readOnly) {
    element.setAttribute("aria-readonly", "true");
  }
}

/**
 * Create a live region for screen reader announcements
 */
export function createLiveRegion(
  container: HTMLElement,
  config: LiveRegionConfig
): HTMLElement {
  const liveRegion = document.createElement("div");
  liveRegion.setAttribute("role", "status");
  liveRegion.setAttribute("aria-live", config.politeness || "polite");
  liveRegion.setAttribute("aria-atomic", String(config.atomic !== false));
  liveRegion.style.position = "absolute";
  liveRegion.style.left = "-10000px";
  liveRegion.style.width = "1px";
  liveRegion.style.height = "1px";
  liveRegion.style.overflow = "hidden";

  container.appendChild(liveRegion);
  return liveRegion;
}

/**
 * Announce a message to screen readers
 */
export function announce(liveRegion: HTMLElement, message: string): void {
  liveRegion.textContent = message;

  // Clear after announcement
  setTimeout(() => {
    liveRegion.textContent = "";
  }, 1000);
}

/**
 * Convert format type to accessible label
 */
export function formatTypeToLabel(format: string): string {
  const labels: Record<string, string> = {
    bold: "Bold",
    italic: "Italic",
    underline: "Underline",
    strikethrough: "Strikethrough",
    code: "Code",
    link: "Link",
    "text-color": "Text Color",
    "background-color": "Background Color",
  };

  return labels[format] || format;
}

/**
 * Convert block type to accessible label
 */
export function blockTypeToLabel(blockType: string): string {
  const labels: Record<string, string> = {
    paragraph: "Paragraph",
    heading1: "Heading 1",
    heading2: "Heading 2",
    heading3: "Heading 3",
    "bullet-list": "Bullet List",
    "ordered-list": "Numbered List",
    "list-item": "List Item",
    blockquote: "Blockquote",
    "code-block": "Code Block",
  };

  return labels[blockType] || blockType;
}

/**
 * Set up focus management for keyboard navigation
 */
export function setupEditorFocusManagement(element: HTMLElement): void {
  element.setAttribute("tabindex", "0");
}
