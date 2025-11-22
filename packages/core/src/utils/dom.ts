/**
 * DOM utility functions
 */

/**
 * Check if an element is editable
 */
export function isEditable(element: HTMLElement): boolean {
  return (
    element.contentEditable === "true" ||
    element.isContentEditable ||
    element.tagName === "INPUT" ||
    element.tagName === "TEXTAREA"
  );
}

/**
 * Check if an element is a descendant of another element
 */
export function isDescendant(parent: Node, child: Node): boolean {
  let node: Node | null = child;
  while (node) {
    if (node === parent) {
      return true;
    }
    node = node.parentNode;
  }
  return false;
}

/**
 * Get the closest ancestor element matching a selector
 */
export function closest(
  element: HTMLElement,
  selector: string
): HTMLElement | null {
  if (element.closest) {
    return element.closest(selector);
  }

  // Fallback for older browsers
  let el: HTMLElement | null = element;
  while (el) {
    if (el.matches && el.matches(selector)) {
      return el;
    }
    el = el.parentElement;
  }
  return null;
}

/**
 * Get all text nodes within an element
 */
export function getTextNodes(element: Node): Text[] {
  const textNodes: Text[] = [];
  const walker = document.createTreeWalker(element, NodeFilter.SHOW_TEXT, null);

  let node: Node | null;
  while ((node = walker.nextNode())) {
    if (node.nodeType === Node.TEXT_NODE) {
      textNodes.push(node as Text);
    }
  }

  return textNodes;
}

/**
 * Get text content from an element, preserving line breaks
 */
export function getTextContent(element: HTMLElement): string {
  let text = "";
  const walker = document.createTreeWalker(
    element,
    NodeFilter.SHOW_TEXT | NodeFilter.SHOW_ELEMENT,
    null
  );

  let node: Node | null;
  while ((node = walker.nextNode())) {
    if (node.nodeType === Node.TEXT_NODE) {
      text += node.textContent || "";
    } else if (node.nodeType === Node.ELEMENT_NODE) {
      const el = node as HTMLElement;
      if (el.tagName === "BR" || el.tagName === "P" || el.tagName === "DIV") {
        text += "\n";
      }
    }
  }

  return text.trim();
}

/**
 * Remove all children from an element
 */
export function removeAllChildren(element: HTMLElement): void {
  while (element.firstChild) {
    element.removeChild(element.firstChild);
  }
}

/**
 * Create an element with attributes
 */
export function createElement(
  tagName: string,
  attributes?: Record<string, string>,
  children?: (Node | string)[]
): HTMLElement {
  const element = document.createElement(tagName);

  if (attributes) {
    Object.entries(attributes).forEach(([key, value]) => {
      element.setAttribute(key, value);
    });
  }

  if (children) {
    children.forEach((child) => {
      if (typeof child === "string") {
        element.appendChild(document.createTextNode(child));
      } else {
        element.appendChild(child);
      }
    });
  }

  return element;
}

/**
 * Set multiple attributes on an element
 */
export function setAttributes(
  element: HTMLElement,
  attributes: Record<string, string>
): void {
  Object.entries(attributes).forEach(([key, value]) => {
    element.setAttribute(key, value);
  });
}
