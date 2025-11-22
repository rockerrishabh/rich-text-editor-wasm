/**
 * Security utilities for sanitizing user input and preventing XSS attacks
 */

/**
 * Allowed HTML tags for sanitization
 */
const ALLOWED_TAGS = new Set([
  "p",
  "br",
  "strong",
  "b",
  "em",
  "i",
  "u",
  "s",
  "strike",
  "code",
  "pre",
  "a",
  "h1",
  "h2",
  "h3",
  "h4",
  "h5",
  "h6",
  "ul",
  "ol",
  "li",
  "blockquote",
  "span",
  "div",
]);

/**
 * Allowed attributes per tag
 */
const ALLOWED_ATTRIBUTES: Record<string, Set<string>> = {
  a: new Set(["href", "title"]),
  span: new Set(["style"]),
  div: new Set(["style"]),
  p: new Set(["style"]),
  h1: new Set(["style"]),
  h2: new Set(["style"]),
  h3: new Set(["style"]),
  h4: new Set(["style"]),
  h5: new Set(["style"]),
  h6: new Set(["style"]),
  strong: new Set(["style"]),
  b: new Set(["style"]),
  em: new Set(["style"]),
  i: new Set(["style"]),
  u: new Set(["style"]),
  s: new Set(["style"]),
  strike: new Set(["style"]),
  code: new Set(["style"]),
  pre: new Set(["style"]),
  blockquote: new Set(["style"]),
  ul: new Set(["style"]),
  ol: new Set(["style"]),
  li: new Set(["style"]),
};

/**
 * Allowed CSS properties for inline styles
 */
const ALLOWED_STYLE_PROPERTIES = new Set([
  "color",
  "background-color",
  "font-weight",
  "font-style",
  "text-decoration",
  "text-decoration-line",
]);

/**
 * URL protocols that are considered safe
 */
const SAFE_URL_PROTOCOLS = new Set(["http:", "https:", "mailto:", "tel:"]);

/**
 * Sanitize HTML content to prevent XSS attacks
 * @param html - Raw HTML string to sanitize
 * @returns Sanitized HTML string
 */
export function sanitizeHTML(html: string): string {
  if (!html || typeof html !== "string") {
    return "";
  }

  // Create a temporary DOM element to parse the HTML
  const temp = document.createElement("div");
  temp.innerHTML = html;

  // Recursively sanitize the DOM tree
  const sanitized = sanitizeNode(temp) as HTMLElement;

  return sanitized.innerHTML;
}

/**
 * Recursively sanitize a DOM node and its children
 * @param node - DOM node to sanitize
 * @returns Sanitized DOM node
 */
function sanitizeNode(node: Node): Node {
  // Handle text nodes - they're safe
  if (node.nodeType === Node.TEXT_NODE) {
    return node.cloneNode(false);
  }

  // Handle element nodes
  if (node.nodeType === Node.ELEMENT_NODE) {
    const element = node as Element;
    const tagName = element.tagName.toLowerCase();

    // Check if tag is allowed
    if (!ALLOWED_TAGS.has(tagName)) {
      // If tag is not allowed, return its text content as a text node
      const textNode = document.createTextNode(element.textContent || "");
      return textNode;
    }

    // Create a new clean element
    const cleanElement = document.createElement(tagName);

    // Copy allowed attributes
    const allowedAttrs = ALLOWED_ATTRIBUTES[tagName] || new Set();
    for (const attr of Array.from(element.attributes)) {
      if (allowedAttrs.has(attr.name)) {
        if (attr.name === "href") {
          // Sanitize URLs
          const sanitizedUrl = sanitizeURL(attr.value);
          if (sanitizedUrl) {
            cleanElement.setAttribute(attr.name, sanitizedUrl);
          }
        } else if (attr.name === "style") {
          // Sanitize inline styles
          const sanitizedStyle = sanitizeStyle(attr.value);
          if (sanitizedStyle) {
            cleanElement.setAttribute(attr.name, sanitizedStyle);
          }
        } else {
          cleanElement.setAttribute(attr.name, attr.value);
        }
      }
    }

    // Recursively sanitize children
    for (const child of Array.from(element.childNodes)) {
      const sanitizedChild = sanitizeNode(child);
      if (sanitizedChild) {
        cleanElement.appendChild(sanitizedChild);
      }
    }

    return cleanElement;
  }

  // For other node types (comments, etc.), skip them
  return document.createTextNode("");
}

/**
 * Sanitize a URL to prevent javascript: and data: URLs
 * @param url - URL string to sanitize
 * @returns Sanitized URL or null if unsafe
 */
export function sanitizeURL(url: string): string | null {
  if (!url || typeof url !== "string") {
    return null;
  }

  // Trim whitespace
  url = url.trim();

  // Check for empty URL
  if (url.length === 0) {
    return null;
  }

  // Check for dangerous protocols
  const lowerUrl = url.toLowerCase();

  // Block javascript: and data: URLs
  if (
    lowerUrl.startsWith("javascript:") ||
    lowerUrl.startsWith("data:") ||
    lowerUrl.startsWith("vbscript:") ||
    lowerUrl.startsWith("file:")
  ) {
    return null;
  }

  // If URL has a protocol, check if it's safe
  try {
    const urlObj = new URL(url, window.location.href);
    if (!SAFE_URL_PROTOCOLS.has(urlObj.protocol)) {
      return null;
    }
    return urlObj.href;
  } catch {
    // If URL parsing fails, it might be a relative URL
    // Check for protocol-like patterns that might bypass our checks
    if (lowerUrl.includes(":")) {
      return null;
    }
    // Return the original URL if it's relative
    return url;
  }
}

/**
 * Sanitize inline CSS styles to prevent CSS injection
 * @param style - CSS style string to sanitize
 * @returns Sanitized CSS style string
 */
export function sanitizeStyle(style: string): string {
  if (!style || typeof style !== "string") {
    return "";
  }

  // Parse the style string into individual declarations
  const declarations = style.split(";").filter((d) => d.trim());

  const sanitizedDeclarations: string[] = [];

  for (const declaration of declarations) {
    const [property, value] = declaration.split(":").map((s) => s.trim());

    if (!property || !value) {
      continue;
    }

    // Check if property is allowed
    if (!ALLOWED_STYLE_PROPERTIES.has(property.toLowerCase())) {
      continue;
    }

    // Sanitize the value
    const sanitizedValue = sanitizeStyleValue(value);
    if (sanitizedValue) {
      sanitizedDeclarations.push(`${property}: ${sanitizedValue}`);
    }
  }

  return sanitizedDeclarations.join("; ");
}

/**
 * Sanitize a CSS style value to prevent injection
 * @param value - CSS value to sanitize
 * @returns Sanitized CSS value or null if unsafe
 */
function sanitizeStyleValue(value: string): string | null {
  if (!value || typeof value !== "string") {
    return null;
  }

  value = value.trim();

  // Block dangerous patterns
  const dangerousPatterns = [
    /javascript:/i,
    /expression\(/i,
    /import/i,
    /@import/i,
    /url\(/i, // Block url() to prevent loading external resources
    /behavior:/i,
    /-moz-binding/i,
  ];

  for (const pattern of dangerousPatterns) {
    if (pattern.test(value)) {
      return null;
    }
  }

  return value;
}

/**
 * Validate a color value to ensure it's safe
 * @param color - Color value to validate (hex, rgb, rgba, named color)
 * @returns Validated color value or null if invalid
 */
export function validateColor(color: string): string | null {
  if (!color || typeof color !== "string") {
    return null;
  }

  color = color.trim().toLowerCase();

  // Check for empty color
  if (color.length === 0) {
    return null;
  }

  // Hex color pattern (#RGB or #RRGGBB or #RRGGBBAA)
  const hexPattern = /^#([0-9a-f]{3}|[0-9a-f]{6}|[0-9a-f]{8})$/i;
  if (hexPattern.test(color)) {
    return color;
  }

  // RGB/RGBA pattern
  const rgbPattern =
    /^rgba?\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*(,\s*(0|1|0?\.\d+)\s*)?\)$/;
  const rgbMatch = color.match(rgbPattern);
  if (rgbMatch) {
    const r = parseInt(rgbMatch[1], 10);
    const g = parseInt(rgbMatch[2], 10);
    const b = parseInt(rgbMatch[3], 10);

    // Validate RGB values are in range 0-255
    if (r >= 0 && r <= 255 && g >= 0 && g <= 255 && b >= 0 && b <= 255) {
      return color;
    }
    return null;
  }

  // HSL/HSLA pattern
  const hslPattern =
    /^hsla?\(\s*(\d{1,3})\s*,\s*(\d{1,3})%\s*,\s*(\d{1,3})%\s*(,\s*(0|1|0?\.\d+)\s*)?\)$/;
  const hslMatch = color.match(hslPattern);
  if (hslMatch) {
    const h = parseInt(hslMatch[1], 10);
    const s = parseInt(hslMatch[2], 10);
    const l = parseInt(hslMatch[3], 10);

    // Validate HSL values
    if (h >= 0 && h <= 360 && s >= 0 && s <= 100 && l >= 0 && l <= 100) {
      return color;
    }
    return null;
  }

  // Named colors (basic set)
  const namedColors = new Set([
    "black",
    "white",
    "red",
    "green",
    "blue",
    "yellow",
    "cyan",
    "magenta",
    "gray",
    "grey",
    "orange",
    "purple",
    "pink",
    "brown",
    "navy",
    "teal",
    "olive",
    "lime",
    "aqua",
    "maroon",
    "silver",
    "fuchsia",
    "transparent",
  ]);

  if (namedColors.has(color)) {
    return color;
  }

  // If none of the patterns match, reject the color
  return null;
}

/**
 * Escape HTML special characters to prevent XSS
 * @param text - Text to escape
 * @returns Escaped text
 */
export function escapeHTML(text: string): string {
  if (!text || typeof text !== "string") {
    return "";
  }

  const escapeMap: Record<string, string> = {
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    '"': "&quot;",
    "'": "&#x27;",
    "/": "&#x2F;",
  };

  return text.replace(/[&<>"'/]/g, (char) => escapeMap[char] || char);
}
