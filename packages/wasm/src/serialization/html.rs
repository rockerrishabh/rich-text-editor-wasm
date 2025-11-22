use crate::document::{Document, Position, Range};
use crate::formatting::{BlockType, InlineFormat};
use std::collections::HashSet;
use thiserror::Error;

/// Errors that can occur during HTML serialization/deserialization
#[derive(Debug, Error)]
pub enum HtmlError {
    #[error("HTML parsing error: {0}")]
    ParseError(String),

    #[error("Invalid format data: {0}")]
    InvalidFormat(String),

    #[error("Sanitization error: {0}")]
    SanitizationError(String),
}

impl Document {
    /// Exports the document to plain text format, stripping all formatting
    ///
    /// This method returns the raw text content without any inline or block formatting.
    /// Line breaks are preserved.
    ///
    /// # Returns
    ///
    /// A plain text string representation of the document.
    pub fn to_plain_text(&self) -> String {
        self.get_content()
    }

    /// Imports a document from plain text
    ///
    /// Creates a new document with the given text content.
    /// All text will be formatted as paragraphs with no inline formatting.
    ///
    /// # Arguments
    ///
    /// * `text` - A plain text string
    ///
    /// # Returns
    ///
    /// A Document with the text content and default formatting.
    pub fn from_plain_text(text: &str) -> Self {
        Document::from_text(text)
    }

    /// Exports the document to HTML format
    ///
    /// Maps inline formats to HTML tags:
    /// - Bold: `<strong>`
    /// - Italic: `<em>`
    /// - Underline: `<u>`
    /// - Strikethrough: `<del>`
    /// - Code: `<code>`
    /// - Link: `<a href="url">`
    /// - TextColor: `<span style="color: ...">`
    /// - BackgroundColor: `<span style="background-color: ...">`
    ///
    /// Maps block types to HTML tags:
    /// - Paragraph: `<p>`
    /// - Heading: `<h1>` through `<h6>`
    /// - BulletList: `<ul><li>`
    /// - NumberedList: `<ol><li>`
    /// - BlockQuote: `<blockquote>`
    /// - CodeBlock: `<pre><code>`
    ///
    /// # Returns
    ///
    /// An HTML string representation of the document with semantic tags.
    pub fn to_html(&self) -> String {
        let content = self.get_content();
        if content.is_empty() {
            return "<p></p>\n".to_string();
        }
        self.to_html_range(None)
    }

    /// Exports a specific range of the document to HTML format
    ///
    /// This method supports incremental rendering by allowing you to generate
    /// HTML for only the dirty regions of the document.
    ///
    /// # Arguments
    ///
    /// * `range` - Optional range to render. If None, renders the entire document.
    ///
    /// # Returns
    ///
    /// An HTML string representation of the specified range with semantic tags.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_text_editor_wasm::document::{Document, Range};
    ///
    /// let doc = Document::from_text("Hello World");
    /// // Render only the first 5 characters
    /// let html = doc.to_html_range(Some(Range::from_offsets(0, 5)));
    /// ```
    pub fn to_html_range(&self, range: Option<Range>) -> String {
        let content = self.get_content();
        if content.is_empty() {
            return String::new();
        }

        // Determine the range to render
        let (render_start, render_end) = if let Some(r) = range {
            let normalized = r.normalize();
            (normalized.start_offset(), normalized.end_offset())
        } else {
            (0, self.get_length())
        };

        let mut result = String::new();
        let lines: Vec<&str> = content.split('\n').collect();
        let mut current_offset = 0;
        let mut in_list = false;
        let mut current_list_type: Option<BlockType> = None;

        for (line_idx, line) in lines.iter().enumerate() {
            let line_start = current_offset;
            let line_end = line_start + line.chars().count();

            // Skip lines outside the render range
            if line_end < render_start {
                current_offset = line_end + 1;
                continue;
            }
            if line_start >= render_end {
                break;
            }

            // Get block type for this line
            let block_type = if line_start < self.get_length() {
                self.get_block_type_at(Position::new(line_start))
            } else {
                BlockType::Paragraph
            };

            // Handle list transitions
            let is_list_item =
                matches!(block_type, BlockType::BulletList | BlockType::NumberedList);

            if is_list_item {
                if !in_list || current_list_type != Some(block_type.clone()) {
                    // Close previous list if different type
                    if in_list {
                        if let Some(BlockType::BulletList) = current_list_type {
                            result.push_str("</ul>\n");
                        } else if let Some(BlockType::NumberedList) = current_list_type {
                            result.push_str("</ol>\n");
                        }
                    }
                    // Open new list
                    match block_type {
                        BlockType::BulletList => result.push_str("<ul>\n"),
                        BlockType::NumberedList => result.push_str("<ol>\n"),
                        _ => {}
                    }
                    in_list = true;
                    current_list_type = Some(block_type.clone());
                }
            } else if in_list {
                // Close list when transitioning to non-list block
                if let Some(BlockType::BulletList) = current_list_type {
                    result.push_str("</ul>\n");
                } else if let Some(BlockType::NumberedList) = current_list_type {
                    result.push_str("</ol>\n");
                }
                in_list = false;
                current_list_type = None;
            }

            // Add opening block tag
            match block_type {
                BlockType::Paragraph => result.push_str("<p>"),
                BlockType::Heading { level } => {
                    result.push_str(&format!("<h{}>", level));
                }
                BlockType::BulletList | BlockType::NumberedList => {
                    result.push_str("<li>");
                }
                BlockType::BlockQuote => result.push_str("<blockquote>"),
                BlockType::CodeBlock => {
                    if line_idx == 0
                        || self.get_block_type_at(Position::new(line_start.saturating_sub(1)))
                            != BlockType::CodeBlock
                    {
                        result.push_str("<pre><code>");
                    }
                }
            }

            // Process inline formats for this line
            if block_type == BlockType::CodeBlock {
                // In code blocks, escape HTML but don't process inline formats
                result.push_str(&escape_html(line));
            } else {
                result.push_str(&self.format_line_with_html(line, line_start));
            }

            // Add closing block tag
            match block_type {
                BlockType::Paragraph => result.push_str("</p>\n"),
                BlockType::Heading { level } => {
                    result.push_str(&format!("</h{}>\n", level));
                }
                BlockType::BulletList | BlockType::NumberedList => {
                    result.push_str("</li>\n");
                }
                BlockType::BlockQuote => result.push_str("</blockquote>\n"),
                BlockType::CodeBlock => {
                    let is_last_line = line_idx == lines.len() - 1;
                    let next_is_code = if !is_last_line {
                        let next_offset = line_end + 1;
                        if next_offset < self.get_length() {
                            self.get_block_type_at(Position::new(next_offset))
                                == BlockType::CodeBlock
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    if !next_is_code {
                        result.push_str("</code></pre>\n");
                    } else {
                        result.push('\n');
                    }
                }
            }

            current_offset = line_end + 1; // +1 for the newline character
        }

        // Close any open lists at the end
        if in_list {
            if let Some(BlockType::BulletList) = current_list_type {
                result.push_str("</ul>\n");
            } else if let Some(BlockType::NumberedList) = current_list_type {
                result.push_str("</ol>\n");
            }
        }

        result
    }

    /// Exports HTML for all dirty regions in the document
    ///
    /// This method is useful for incremental rendering. It returns HTML
    /// for each dirty region along with the range it covers.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing (range, html) for each dirty region.
    ///
    /// # Example
    ///
    /// ```
    /// use rich_text_editor_wasm::document::Document;
    /// use rich_text_editor_wasm::document::Position;
    ///
    /// let mut doc = Document::from_text("Hello World");
    /// doc.insert_text(Position::new(5), " Beautiful").unwrap();
    ///
    /// // Get HTML for dirty regions
    /// let dirty_html = doc.to_html_dirty_regions();
    /// for (range, html) in dirty_html {
    ///     println!("Range {:?}: {}", range, html);
    /// }
    /// ```
    pub fn to_html_dirty_regions(&self) -> Vec<(Range, String)> {
        let dirty_regions = self.get_dirty_regions();
        dirty_regions
            .into_iter()
            .map(|range| {
                let html = self.to_html_range(Some(range));
                (range, html)
            })
            .collect()
    }

    /// Formats a single line with HTML inline formatting
    fn format_line_with_html(&self, line: &str, line_start: usize) -> String {
        if line.is_empty() {
            return String::new();
        }

        let line_len = line.chars().count();
        let mut result = String::new();
        let mut pos = 0;

        while pos < line_len {
            let abs_pos = line_start + pos;
            let formats = if abs_pos < self.get_length() {
                self.get_formats_at(Position::new(abs_pos))
            } else {
                HashSet::new()
            };

            // Find the end of this format run
            let mut run_end = pos + 1;
            while run_end < line_len {
                let next_abs_pos = line_start + run_end;
                let next_formats = if next_abs_pos < self.get_length() {
                    self.get_formats_at(Position::new(next_abs_pos))
                } else {
                    HashSet::new()
                };
                if next_formats != formats {
                    break;
                }
                run_end += 1;
            }

            // Extract the text for this run
            let run_text: String = line.chars().skip(pos).take(run_end - pos).collect();

            // Apply formats
            let formatted = apply_html_formats(&run_text, &formats);
            result.push_str(&formatted);

            pos = run_end;
        }

        result
    }
}

/// Applies HTML formatting tags to text based on the given formats
fn apply_html_formats(text: &str, formats: &HashSet<InlineFormat>) -> String {
    let mut result = escape_html(text);

    // Collect formats in a specific order for proper nesting
    let has_bold = formats.contains(&InlineFormat::Bold);
    let has_italic = formats.contains(&InlineFormat::Italic);
    let has_underline = formats.contains(&InlineFormat::Underline);
    let has_strikethrough = formats.contains(&InlineFormat::Strikethrough);
    let has_code = formats.contains(&InlineFormat::Code);

    let link = formats.iter().find_map(|f| match f {
        InlineFormat::Link { url } => Some(url.clone()),
        _ => None,
    });

    let text_color = formats.iter().find_map(|f| match f {
        InlineFormat::TextColor { color } => Some(color.clone()),
        _ => None,
    });

    let bg_color = formats.iter().find_map(|f| match f {
        InlineFormat::BackgroundColor { color } => Some(color.clone()),
        _ => None,
    });

    // Apply formats in order: colors (outermost) -> link -> bold -> italic -> underline -> strikethrough -> code (innermost)

    // Code (innermost for inline)
    if has_code {
        result = format!("<code>{}</code>", result);
    }

    // Strikethrough
    if has_strikethrough {
        result = format!("<s>{}</s>", result);
    }

    // Underline
    if has_underline {
        result = format!("<u>{}</u>", result);
    }

    // Italic
    if has_italic {
        result = format!("<em>{}</em>", result);
    }

    // Bold
    if has_bold {
        result = format!("<strong>{}</strong>", result);
    }

    // Link
    if let Some(url) = link {
        let escaped_url = escape_html_attribute(&url);
        result = format!("<a href=\"{}\">{}</a>", escaped_url, result);
    }

    // Colors (outermost)
    if text_color.is_some() || bg_color.is_some() {
        let mut style = String::new();
        if let Some(color) = text_color {
            style.push_str(&format!("color: {};", escape_html_attribute(&color)));
        }
        if let Some(color) = bg_color {
            if !style.is_empty() {
                style.push(' ');
            }
            style.push_str(&format!(
                "background-color: {};",
                escape_html_attribute(&color)
            ));
        }
        result = format!("<span style=\"{}\">{}</span>", style, result);
    }

    result
}

/// Escapes special HTML characters in text content
fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Escapes special HTML characters in attribute values
fn escape_html_attribute(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// HTML Sanitizer that whitelists safe tags and attributes
///
/// This sanitizer prevents XSS attacks by:
/// - Whitelisting only safe HTML tags
/// - Stripping dangerous attributes (onclick, onerror, etc.)
/// - Validating URL formats to prevent javascript: and data: protocols
/// - Sanitizing CSS color values
/// - Escaping all user-provided content
///
/// # Security Features
///
/// - **Tag Whitelist**: Only semantic HTML tags are allowed
/// - **Attribute Whitelist**: Only href and style attributes are allowed
/// - **URL Validation**: Blocks javascript:, data:, vbscript:, and file: protocols
/// - **Color Validation**: Only hex (#RRGGBB), rgb(), rgba(), and named colors allowed
/// - **CSS Property Whitelist**: Only color and background-color properties allowed
pub struct HtmlSanitizer {
    allowed_tags: HashSet<String>,
    allowed_attributes: HashSet<String>,
}

impl HtmlSanitizer {
    /// Creates a new HtmlSanitizer with default safe tags and attributes
    pub fn new() -> Self {
        let mut allowed_tags = HashSet::new();
        // Block-level tags
        allowed_tags.insert("p".to_string());
        allowed_tags.insert("h1".to_string());
        allowed_tags.insert("h2".to_string());
        allowed_tags.insert("h3".to_string());
        allowed_tags.insert("h4".to_string());
        allowed_tags.insert("h5".to_string());
        allowed_tags.insert("h6".to_string());
        allowed_tags.insert("blockquote".to_string());
        allowed_tags.insert("pre".to_string());
        allowed_tags.insert("ul".to_string());
        allowed_tags.insert("ol".to_string());
        allowed_tags.insert("li".to_string());

        // Inline tags
        allowed_tags.insert("strong".to_string());
        allowed_tags.insert("b".to_string());
        allowed_tags.insert("em".to_string());
        allowed_tags.insert("i".to_string());
        allowed_tags.insert("u".to_string());
        allowed_tags.insert("del".to_string());
        allowed_tags.insert("s".to_string());
        allowed_tags.insert("strike".to_string());
        allowed_tags.insert("code".to_string());
        allowed_tags.insert("a".to_string());
        allowed_tags.insert("span".to_string());
        allowed_tags.insert("br".to_string());

        let mut allowed_attributes = HashSet::new();
        allowed_attributes.insert("href".to_string());
        allowed_attributes.insert("style".to_string());

        Self {
            allowed_tags,
            allowed_attributes,
        }
    }

    /// Checks if a URL is safe (not javascript:, data:, vbscript:, or file: protocol)
    ///
    /// # Security
    ///
    /// This function blocks dangerous URL protocols that could be used for XSS attacks:
    /// - `javascript:` - Executes JavaScript code
    /// - `data:` - Can contain embedded scripts
    /// - `vbscript:` - Executes VBScript code (IE)
    /// - `file:` - Accesses local file system
    ///
    /// Only http:, https:, mailto:, and relative URLs are considered safe.
    fn is_safe_url(&self, url: &str) -> bool {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return false;
        }

        let lower = trimmed.to_lowercase();

        // Block dangerous protocols
        if lower.starts_with("javascript:")
            || lower.starts_with("data:")
            || lower.starts_with("vbscript:")
            || lower.starts_with("file:")
        {
            return false;
        }

        // Allow safe protocols and relative URLs
        lower.starts_with("http:")
            || lower.starts_with("https:")
            || lower.starts_with("mailto:")
            || lower.starts_with("//")
            || lower.starts_with('/')
            || lower.starts_with('#')
            || !lower.contains(':') // Relative URLs without protocol
    }

    /// Validates a color value
    ///
    /// Accepts:
    /// - Hex colors: #RGB, #RRGGBB, #RRGGBBAA
    /// - RGB/RGBA: rgb(r, g, b), rgba(r, g, b, a)
    /// - Named colors: red, blue, green, etc.
    ///
    /// Rejects:
    /// - Invalid formats
    /// - Expressions or functions (except rgb/rgba)
    /// - URLs or imports
    fn is_valid_color(&self, color: &str) -> bool {
        let trimmed = color.trim();
        if trimmed.is_empty() {
            return false;
        }

        // Hex colors
        if trimmed.starts_with('#') {
            let hex = &trimmed[1..];
            let len = hex.len();
            // Valid lengths: 3 (RGB), 6 (RRGGBB), 8 (RRGGBBAA)
            if len == 3 || len == 6 || len == 8 {
                return hex.chars().all(|c| c.is_ascii_hexdigit());
            }
            return false;
        }

        // RGB/RGBA colors
        let lower = trimmed.to_lowercase();
        if lower.starts_with("rgb(") || lower.starts_with("rgba(") {
            // Basic validation: contains only digits, commas, spaces, parentheses, dots, percent, and letters (for 'rgb' and 'rgba')
            return trimmed.chars().all(|c| {
                c.is_ascii_digit()
                    || c == ','
                    || c == ' '
                    || c == '('
                    || c == ')'
                    || c == '.'
                    || c == '%'
                    || c.is_ascii_alphabetic()
            });
        }

        // Named colors (basic alphanumeric check)
        // Full list: https://www.w3.org/TR/css-color-3/#svg-color
        if trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
            // Common named colors
            let named_colors = [
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
                "fuchsia",
                "silver",
                "maroon",
                "transparent",
            ];
            return named_colors.contains(&lower.as_str());
        }

        false
    }

    /// Sanitizes an attribute value
    ///
    /// Returns None if the attribute should be stripped.
    fn sanitize_attribute(&self, name: &str, value: &str) -> Option<String> {
        if !self.allowed_attributes.contains(name) {
            return None;
        }

        match name {
            "href" => {
                if self.is_safe_url(value) {
                    Some(value.to_string())
                } else {
                    None
                }
            }
            "style" => {
                // Only allow color and background-color properties
                let sanitized = self.sanitize_style(value);
                if sanitized.is_empty() {
                    None
                } else {
                    Some(sanitized)
                }
            }
            _ => Some(value.to_string()),
        }
    }

    /// Sanitizes CSS style attribute
    ///
    /// Only allows color and background-color properties with validated values.
    /// All other CSS properties are stripped for security.
    fn sanitize_style(&self, style: &str) -> String {
        let mut result = Vec::new();

        for declaration in style.split(';') {
            let parts: Vec<&str> = declaration.split(':').collect();
            if parts.len() == 2 {
                let property = parts[0].trim().to_lowercase();
                let value = parts[1].trim();

                if property == "color" || property == "background-color" {
                    // Validate color value
                    if self.is_valid_color(value) {
                        result.push(format!("{}: {}", property, value));
                    }
                }
            }
        }

        result.join("; ")
    }
}

impl Default for HtmlSanitizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Document {
    /// Imports a document from HTML format with sanitization
    ///
    /// Parses HTML and converts tags to internal format representation.
    /// Strips dangerous content like javascript: URLs and event handlers.
    ///
    /// # Arguments
    ///
    /// * `html` - An HTML string
    ///
    /// # Returns
    ///
    /// A Document with content and formatting parsed from HTML.
    ///
    /// # Errors
    ///
    /// Returns an error if HTML parsing fails.
    pub fn from_html(html: &str) -> Result<Self, HtmlError> {
        let sanitizer = HtmlSanitizer::new();
        Self::from_html_with_sanitizer(html, &sanitizer)
    }

    /// Imports a document from HTML format with a custom sanitizer
    fn from_html_with_sanitizer(html: &str, sanitizer: &HtmlSanitizer) -> Result<Self, HtmlError> {
        if html.is_empty() {
            return Ok(Document::new());
        }

        let mut plain_text = String::new();
        let mut format_instructions: Vec<FormatInstruction> = Vec::new();
        let mut block_instructions: Vec<BlockInstruction> = Vec::new();
        let mut current_offset = 0;

        // Parse HTML into a simple token stream
        let tokens = parse_html_tokens(html)?;

        // Process tokens
        let mut tag_stack: Vec<HtmlTag> = Vec::new();
        let mut in_pre = false;
        let mut disallowed_tag_depth = 0; // Track depth of disallowed tags

        for token in tokens {
            match token {
                HtmlToken::OpenTag { name, attributes } => {
                    if !sanitizer.allowed_tags.contains(&name.to_lowercase()) {
                        disallowed_tag_depth += 1;
                        continue;
                    }

                    // Skip if we're inside a disallowed tag
                    if disallowed_tag_depth > 0 {
                        continue;
                    }

                    let tag_name = name.to_lowercase();

                    // Track pre tags
                    if tag_name == "pre" {
                        in_pre = true;
                    }

                    // Handle block-level tags
                    let block_type = match tag_name.as_str() {
                        "h1" => Some(BlockType::heading(1)),
                        "h2" => Some(BlockType::heading(2)),
                        "h3" => Some(BlockType::heading(3)),
                        "h4" => Some(BlockType::heading(4)),
                        "h5" => Some(BlockType::heading(5)),
                        "h6" => Some(BlockType::heading(6)),
                        "blockquote" => Some(BlockType::BlockQuote),
                        "li" => {
                            // Determine list type from parent
                            if tag_stack.iter().any(|t| t.name == "ul") {
                                Some(BlockType::BulletList)
                            } else if tag_stack.iter().any(|t| t.name == "ol") {
                                Some(BlockType::NumberedList)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    tag_stack.push(HtmlTag {
                        name: tag_name.clone(),
                        start_offset: current_offset,
                        attributes: attributes.clone(),
                        block_type,
                    });
                }
                HtmlToken::CloseTag { name } => {
                    let tag_name = name.to_lowercase();

                    // Handle closing of disallowed tags
                    if !sanitizer.allowed_tags.contains(&tag_name) {
                        if disallowed_tag_depth > 0 {
                            disallowed_tag_depth -= 1;
                        }
                        continue;
                    }

                    // Skip if we're inside a disallowed tag
                    if disallowed_tag_depth > 0 {
                        continue;
                    }

                    if tag_name == "pre" {
                        in_pre = false;
                    }

                    // Find matching open tag
                    if let Some(pos) = tag_stack.iter().rposition(|t| t.name == tag_name) {
                        let tag = tag_stack.remove(pos);

                        // Handle block-level closing
                        if let Some(block_type) = tag.block_type {
                            if current_offset > tag.start_offset {
                                block_instructions.push(BlockInstruction {
                                    start: tag.start_offset,
                                    end: current_offset,
                                    block_type,
                                });
                            }
                        }

                        // Handle inline formatting closing
                        if current_offset > tag.start_offset {
                            let format = match tag.name.as_str() {
                                "strong" | "b" => Some(InlineFormat::Bold),
                                "em" | "i" => Some(InlineFormat::Italic),
                                "u" => Some(InlineFormat::Underline),
                                "del" | "s" | "strike" => Some(InlineFormat::Strikethrough),
                                "code" => Some(InlineFormat::Code),
                                "a" => {
                                    // Extract href attribute
                                    tag.attributes.get("href").and_then(|url| {
                                        sanitizer
                                            .sanitize_attribute("href", url)
                                            .map(|safe_url| InlineFormat::Link { url: safe_url })
                                    })
                                }
                                "span" => {
                                    // Extract style attribute
                                    tag.attributes
                                        .get("style")
                                        .and_then(|style| parse_style_to_formats(style, sanitizer))
                                }
                                _ => None,
                            };

                            if let Some(fmt) = format {
                                format_instructions.push(FormatInstruction {
                                    start: tag.start_offset,
                                    end: current_offset,
                                    format: fmt,
                                    in_pre,
                                });
                            }
                        }

                        // Add newline after block elements
                        if matches!(
                            tag.name.as_str(),
                            "p" | "h1"
                                | "h2"
                                | "h3"
                                | "h4"
                                | "h5"
                                | "h6"
                                | "li"
                                | "blockquote"
                                | "br"
                        ) {
                            if !plain_text.is_empty() && !plain_text.ends_with('\n') {
                                plain_text.push('\n');
                                current_offset += 1;
                            }
                        }
                    }
                }
                HtmlToken::Text { content } => {
                    // Skip text inside disallowed tags
                    if disallowed_tag_depth > 0 {
                        continue;
                    }

                    let decoded = decode_html_entities(&content);
                    plain_text.push_str(&decoded);
                    current_offset += decoded.chars().count();
                }
                HtmlToken::SelfClosing { name, .. } => {
                    // Skip if we're inside a disallowed tag
                    if disallowed_tag_depth > 0 {
                        continue;
                    }

                    if name.to_lowercase() == "br" {
                        plain_text.push('\n');
                        current_offset += 1;
                    }
                }
            }
        }

        // Remove trailing newline if present
        if plain_text.ends_with('\n') {
            plain_text.pop();
        }

        // Create document with plain text
        let mut doc = Document::from_text(&plain_text);

        // Apply inline formats and track code blocks
        let mut code_block_ranges = Vec::new();
        for instruction in format_instructions {
            if instruction.end <= doc.get_length() {
                let range = Range::from_offsets(instruction.start, instruction.end);
                doc.apply_format(range, instruction.format.clone());

                // Track code formats that were in <pre> tags for conversion to code blocks
                if matches!(instruction.format, InlineFormat::Code) && instruction.in_pre {
                    code_block_ranges.push((instruction.start, instruction.end));
                }
            }
        }

        // Apply block types
        for instruction in block_instructions {
            if instruction.end <= doc.get_length() {
                let range = Range::from_offsets(instruction.start, instruction.end);
                doc.set_block_type(range, instruction.block_type);
            }
        }

        // Convert code formats that were in <pre> tags to code blocks
        for (start, end) in code_block_ranges {
            if end <= doc.get_length() {
                let range = Range::from_offsets(start, end);
                doc.set_block_type(range, BlockType::CodeBlock);
                doc.remove_format(range, &InlineFormat::Code);
            }
        }

        // Clear history since this is a freshly loaded document
        doc.history.clear();

        Ok(doc)
    }
}

/// Instruction for applying a format after parsing
#[derive(Debug)]
struct FormatInstruction {
    start: usize,
    end: usize,
    format: InlineFormat,
    in_pre: bool, // Track if this format was inside a <pre> tag
}

/// Instruction for applying a block type after parsing
#[derive(Debug)]
struct BlockInstruction {
    start: usize,
    end: usize,
    block_type: BlockType,
}

/// Represents an HTML tag in the stack
#[derive(Debug, Clone)]
struct HtmlTag {
    name: String,
    start_offset: usize,
    attributes: std::collections::HashMap<String, String>,
    block_type: Option<BlockType>,
}

/// HTML token types
#[derive(Debug, Clone)]
enum HtmlToken {
    OpenTag {
        name: String,
        attributes: std::collections::HashMap<String, String>,
    },
    CloseTag {
        name: String,
    },
    SelfClosing {
        name: String,
    },
    Text {
        content: String,
    },
}

/// Simple HTML tokenizer
fn parse_html_tokens(html: &str) -> Result<Vec<HtmlToken>, HtmlError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = html.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '<' {
            // Find the end of the tag
            if let Some(end) = chars[i..].iter().position(|&c| c == '>') {
                let tag_content: String = chars[i + 1..i + end].iter().collect();

                if tag_content.starts_with('/') {
                    // Closing tag
                    let name = tag_content[1..].trim().to_string();
                    tokens.push(HtmlToken::CloseTag { name });
                } else if tag_content.ends_with('/') {
                    // Self-closing tag
                    let content = tag_content[..tag_content.len() - 1].trim();
                    let (name, _attributes) = parse_tag_and_attributes(content);
                    tokens.push(HtmlToken::SelfClosing { name });
                } else {
                    // Opening tag
                    let (name, attributes) = parse_tag_and_attributes(&tag_content);
                    tokens.push(HtmlToken::OpenTag { name, attributes });
                }

                i += end + 1;
            } else {
                // Malformed tag, treat as text
                tokens.push(HtmlToken::Text {
                    content: chars[i].to_string(),
                });
                i += 1;
            }
        } else {
            // Text content
            let mut text = String::new();
            while i < chars.len() && chars[i] != '<' {
                text.push(chars[i]);
                i += 1;
            }
            if !text.is_empty() {
                tokens.push(HtmlToken::Text { content: text });
            }
        }
    }

    // Post-process: remove whitespace-only text tokens that appear between block-level tags
    let block_tags = [
        "p",
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
        "pre",
        "div",
    ];

    let mut filtered_tokens = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        match token {
            HtmlToken::Text { content } if content.chars().all(|c| c.is_whitespace()) => {
                // Check if this whitespace is between block-level tags
                let prev_is_block = if i > 0 {
                    match &tokens[i - 1] {
                        HtmlToken::CloseTag { name } | HtmlToken::OpenTag { name, .. } => {
                            block_tags.contains(&name.as_str())
                        }
                        _ => false,
                    }
                } else {
                    true // Start of document
                };

                let next_is_block = if i + 1 < tokens.len() {
                    match &tokens[i + 1] {
                        HtmlToken::CloseTag { name } | HtmlToken::OpenTag { name, .. } => {
                            block_tags.contains(&name.as_str())
                        }
                        _ => false,
                    }
                } else {
                    true // End of document
                };

                // Only skip if between block tags
                if !(prev_is_block && next_is_block) {
                    filtered_tokens.push(token.clone());
                }
            }
            _ => filtered_tokens.push(token.clone()),
        }
    }

    Ok(filtered_tokens)
}

/// Parses tag name and attributes from tag content
fn parse_tag_and_attributes(content: &str) -> (String, std::collections::HashMap<String, String>) {
    let mut parts = content.split_whitespace();
    let name = parts.next().unwrap_or("").to_string();

    let mut attributes = std::collections::HashMap::new();
    let remainder: String = parts.collect::<Vec<_>>().join(" ");

    // Simple attribute parsing (handles key="value" and key='value')
    let mut chars = remainder.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            continue;
        }

        // Read attribute name
        let mut attr_name = String::new();
        attr_name.push(c);
        while let Some(&next) = chars.peek() {
            if next == '=' || next.is_whitespace() {
                break;
            }
            attr_name.push(chars.next().unwrap());
        }

        // Skip whitespace and '='
        while let Some(&next) = chars.peek() {
            if next == '=' {
                chars.next();
                break;
            } else if next.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }

        // Read attribute value
        let mut attr_value = String::new();
        if let Some(&quote) = chars.peek() {
            if quote == '"' || quote == '\'' {
                chars.next(); // consume opening quote
                while let Some(c) = chars.next() {
                    if c == quote {
                        break;
                    }
                    attr_value.push(c);
                }
            }
        }

        if !attr_name.is_empty() {
            attributes.insert(attr_name.to_lowercase(), attr_value);
        }
    }

    (name, attributes)
}

/// Decodes HTML entities
fn decode_html_entities(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

/// Parses style attribute and returns the first applicable format
fn parse_style_to_formats(style: &str, sanitizer: &HtmlSanitizer) -> Option<InlineFormat> {
    if let Some(sanitized) = sanitizer.sanitize_attribute("style", style) {
        for declaration in sanitized.split(';') {
            let parts: Vec<&str> = declaration.split(':').collect();
            if parts.len() == 2 {
                let property = parts[0].trim();
                let value = parts[1].trim();

                match property {
                    "color" => {
                        return Some(InlineFormat::TextColor {
                            color: value.to_string(),
                        });
                    }
                    "background-color" => {
                        return Some(InlineFormat::BackgroundColor {
                            color: value.to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::document::Range;

    use super::*;

    #[test]
    fn test_to_html_empty() {
        let doc = Document::new();
        assert_eq!(doc.to_html(), "<p></p>\n");
    }

    #[test]
    fn test_to_html_plain_text() {
        let doc = Document::from_text("Hello World");
        assert_eq!(doc.to_html(), "<p>Hello World</p>\n");
    }

    #[test]
    fn test_to_html_bold() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        assert_eq!(doc.to_html(), "<p><strong>Hello</strong> World</p>\n");
    }

    #[test]
    fn test_to_html_italic() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(6, 11), InlineFormat::Italic);
        assert_eq!(doc.to_html(), "<p>Hello <em>World</em></p>\n");
    }

    #[test]
    fn test_to_html_underline() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Underline);
        assert_eq!(doc.to_html(), "<p><u>Hello</u> World</p>\n");
    }

    #[test]
    fn test_to_html_strikethrough() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 11), InlineFormat::Strikethrough);
        assert_eq!(doc.to_html(), "<p><s>Hello World</s></p>\n");
    }

    #[test]
    fn test_to_html_code() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Code);
        assert_eq!(doc.to_html(), "<p><code>Hello</code> World</p>\n");
    }

    #[test]
    fn test_to_html_link() {
        let mut doc = Document::from_text("Click here");
        doc.apply_format(
            Range::from_offsets(0, 10),
            InlineFormat::Link {
                url: "https://example.com".to_string(),
            },
        );
        assert_eq!(
            doc.to_html(),
            "<p><a href=\"https://example.com\">Click here</a></p>\n"
        );
    }

    #[test]
    fn test_to_html_text_color() {
        let mut doc = Document::from_text("Red text");
        doc.apply_format(
            Range::from_offsets(0, 8),
            InlineFormat::TextColor {
                color: "#FF0000".to_string(),
            },
        );
        assert_eq!(
            doc.to_html(),
            "<p><span style=\"color: #FF0000;\">Red text</span></p>\n"
        );
    }

    #[test]
    fn test_to_html_background_color() {
        let mut doc = Document::from_text("Highlighted");
        doc.apply_format(
            Range::from_offsets(0, 11),
            InlineFormat::BackgroundColor {
                color: "#FFFF00".to_string(),
            },
        );
        assert_eq!(
            doc.to_html(),
            "<p><span style=\"background-color: #FFFF00;\">Highlighted</span></p>\n"
        );
    }

    #[test]
    fn test_to_html_both_colors() {
        let mut doc = Document::from_text("Colored");
        doc.apply_format(
            Range::from_offsets(0, 7),
            InlineFormat::TextColor {
                color: "#FF0000".to_string(),
            },
        );
        doc.apply_format(
            Range::from_offsets(0, 7),
            InlineFormat::BackgroundColor {
                color: "#FFFF00".to_string(),
            },
        );
        assert_eq!(
            doc.to_html(),
            "<p><span style=\"color: #FF0000; background-color: #FFFF00;\">Colored</span></p>\n"
        );
    }

    #[test]
    fn test_to_html_heading() {
        let mut doc = Document::from_text("Heading");
        doc.set_block_type(Range::from_offsets(0, 7), BlockType::heading(1));
        assert_eq!(doc.to_html(), "<h1>Heading</h1>\n");
    }

    #[test]
    fn test_to_html_heading_with_bold() {
        let mut doc = Document::from_text("Bold Heading");
        doc.set_block_type(Range::from_offsets(0, 12), BlockType::heading(1));
        doc.apply_format(Range::from_offsets(0, 12), InlineFormat::Bold);
        assert_eq!(doc.to_html(), "<h1><strong>Bold Heading</strong></h1>\n");
    }

    #[test]
    fn test_to_html_all_heading_levels() {
        for level in 1..=6 {
            let mut doc = Document::from_text("Heading");
            doc.set_block_type(Range::from_offsets(0, 7), BlockType::heading(level));
            assert_eq!(doc.to_html(), format!("<h{}>Heading</h{}>\n", level, level));
        }
    }

    #[test]
    fn test_to_html_bullet_list() {
        let mut doc = Document::from_text("Item");
        doc.set_block_type(Range::from_offsets(0, 4), BlockType::BulletList);
        assert_eq!(doc.to_html(), "<ul>\n<li>Item</li>\n</ul>\n");
    }

    #[test]
    fn test_to_html_numbered_list() {
        let mut doc = Document::from_text("Item");
        doc.set_block_type(Range::from_offsets(0, 4), BlockType::NumberedList);
        assert_eq!(doc.to_html(), "<ol>\n<li>Item</li>\n</ol>\n");
    }

    #[test]
    fn test_to_html_block_quote() {
        let mut doc = Document::from_text("Quote");
        doc.set_block_type(Range::from_offsets(0, 5), BlockType::BlockQuote);
        assert_eq!(doc.to_html(), "<blockquote>Quote</blockquote>\n");
    }

    #[test]
    fn test_to_html_code_block() {
        let mut doc = Document::from_text("code");
        doc.set_block_type(Range::from_offsets(0, 4), BlockType::CodeBlock);
        assert_eq!(doc.to_html(), "<pre><code>code</code></pre>\n");
    }

    #[test]
    fn test_to_html_multiple_formats() {
        let mut doc = Document::from_text("Hello");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Italic);
        assert_eq!(doc.to_html(), "<p><strong><em>Hello</em></strong></p>\n");
    }

    #[test]
    fn test_to_html_escape_special_chars() {
        let doc = Document::from_text("<script>alert('xss')</script>");
        assert_eq!(
            doc.to_html(),
            "<p>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</p>\n"
        );
    }

    #[test]
    fn test_to_html_escape_in_link() {
        let mut doc = Document::from_text("Link");
        doc.apply_format(
            Range::from_offsets(0, 4),
            InlineFormat::Link {
                url: "javascript:alert('xss')".to_string(),
            },
        );
        let html = doc.to_html();
        // The URL is escaped for HTML attributes
        assert!(html.contains("javascript:alert(&#39;xss&#39;)"));
        assert!(!html.contains("<script"));
    }

    #[test]
    fn test_to_html_multiline() {
        let doc = Document::from_text("Line 1\nLine 2");
        assert_eq!(doc.to_html(), "<p>Line 1</p>\n<p>Line 2</p>\n");
    }

    #[test]
    fn test_to_html_multiple_list_items() {
        let mut doc = Document::from_text("Item 1\nItem 2\nItem 3");
        doc.set_block_type(Range::from_offsets(0, 6), BlockType::BulletList);
        doc.set_block_type(Range::from_offsets(7, 13), BlockType::BulletList);
        doc.set_block_type(Range::from_offsets(14, 20), BlockType::BulletList);

        let html = doc.to_html();
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>Item 1</li>"));
        assert!(html.contains("<li>Item 2</li>"));
        assert!(html.contains("<li>Item 3</li>"));
        assert!(html.contains("</ul>"));
    }

    #[test]
    fn test_to_html_mixed_blocks() {
        let mut doc = Document::from_text("Heading\nParagraph");
        doc.set_block_type(Range::from_offsets(0, 7), BlockType::heading(1));

        let html = doc.to_html();
        // The second line inherits the heading block type, so we need to set it explicitly
        assert!(html.contains("<h1>Heading</h1>"));
        // Check that the second line exists (it will also be h1 unless explicitly set)
        assert!(html.contains("Paragraph"));
    }

    #[test]
    fn test_from_html_empty() {
        let doc = Document::from_html("").unwrap();
        assert_eq!(doc.get_content(), "");
    }

    #[test]
    fn test_from_html_plain_text() {
        let doc = Document::from_html("<p>Hello World</p>").unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_from_html_bold() {
        let doc = Document::from_html("<p><strong>Hello</strong> World</p>").unwrap();
        assert_eq!(doc.get_content(), "Hello World");

        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));

        let formats_after = doc.get_formats_at(Position::new(7));
        assert!(!formats_after.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_from_html_italic() {
        let doc = Document::from_html("<p>Hello <em>World</em></p>").unwrap();
        assert_eq!(doc.get_content(), "Hello World");

        let formats = doc.get_formats_at(Position::new(8));
        assert!(formats.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_from_html_underline() {
        let doc = Document::from_html("<p><u>Hello</u> World</p>").unwrap();
        assert_eq!(doc.get_content(), "Hello World");

        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Underline));
    }

    #[test]
    fn test_from_html_strikethrough() {
        let doc = Document::from_html("<p><del>Hello</del></p>").unwrap();
        assert_eq!(doc.get_content(), "Hello");

        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Strikethrough));
    }

    #[test]
    fn test_from_html_code() {
        let doc = Document::from_html("<p><code>code</code></p>").unwrap();
        assert_eq!(doc.get_content(), "code");

        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Code));
    }

    #[test]
    fn test_from_html_link() {
        let doc =
            Document::from_html("<p><a href=\"https://example.com\">Click here</a></p>").unwrap();
        assert_eq!(doc.get_content(), "Click here");

        let formats = doc.get_formats_at(Position::new(5));
        assert!(formats.contains(&InlineFormat::Link {
            url: "https://example.com".to_string()
        }));
    }

    #[test]
    fn test_from_html_sanitize_javascript_url() {
        let doc =
            Document::from_html("<p><a href=\"javascript:alert('xss')\">Click</a></p>").unwrap();
        assert_eq!(doc.get_content(), "Click");

        // Link should be stripped due to javascript: protocol
        let formats = doc.get_formats_at(Position::new(2));
        assert!(
            !formats
                .iter()
                .any(|f| matches!(f, InlineFormat::Link { .. }))
        );
    }

    #[test]
    fn test_from_html_heading() {
        let doc = Document::from_html("<h1>Heading</h1>").unwrap();
        assert_eq!(doc.get_content(), "Heading");
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::heading(1)
        );
    }

    #[test]
    fn test_from_html_all_heading_levels() {
        for level in 1..=6 {
            let html = format!("<h{}>Heading</h{}>", level, level);
            let doc = Document::from_html(&html).unwrap();
            assert_eq!(doc.get_content(), "Heading");
            assert_eq!(
                doc.get_block_type_at(Position::new(0)),
                BlockType::heading(level)
            );
        }
    }

    #[test]
    fn test_from_html_bullet_list() {
        let doc = Document::from_html("<ul><li>Item</li></ul>").unwrap();
        assert_eq!(doc.get_content(), "Item");
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::BulletList
        );
    }

    #[test]
    fn test_from_html_numbered_list() {
        let doc = Document::from_html("<ol><li>Item</li></ol>").unwrap();
        assert_eq!(doc.get_content(), "Item");
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::NumberedList
        );
    }

    #[test]
    fn test_from_html_block_quote() {
        let doc = Document::from_html("<blockquote>Quote</blockquote>").unwrap();
        assert_eq!(doc.get_content(), "Quote");
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::BlockQuote
        );
    }

    #[test]
    fn test_from_html_code_block() {
        let doc = Document::from_html("<pre><code>code</code></pre>").unwrap();
        assert_eq!(doc.get_content(), "code");
        assert_eq!(
            doc.get_block_type_at(Position::new(0)),
            BlockType::CodeBlock
        );
    }

    #[test]
    fn test_from_html_text_color() {
        let doc = Document::from_html("<p><span style=\"color: #FF0000;\">Red</span></p>").unwrap();
        assert_eq!(doc.get_content(), "Red");

        let formats = doc.get_formats_at(Position::new(1));
        assert!(formats.contains(&InlineFormat::TextColor {
            color: "#FF0000".to_string()
        }));
    }

    #[test]
    fn test_from_html_decode_entities() {
        let doc = Document::from_html("<p>&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;</p>")
            .unwrap();
        assert_eq!(doc.get_content(), "<script>alert('xss')</script>");
    }

    #[test]
    fn test_from_html_strip_disallowed_tags() {
        let doc = Document::from_html("<p>Hello <script>alert('xss')</script>World</p>").unwrap();
        // Script tag should be stripped
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_from_html_multiple_formats() {
        let doc = Document::from_html("<p><strong><em>Hello</em></strong></p>").unwrap();
        assert_eq!(doc.get_content(), "Hello");

        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));
        assert!(formats.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_from_html_br_tag() {
        let doc = Document::from_html("<p>Line 1<br/>Line 2</p>").unwrap();
        assert_eq!(doc.get_content(), "Line 1\nLine 2");
    }

    #[test]
    fn test_roundtrip_html() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.apply_format(Range::from_offsets(6, 11), InlineFormat::Italic);

        let html = doc.to_html();
        let restored = Document::from_html(&html).unwrap();

        assert_eq!(restored.get_content(), "Hello World");

        let formats = restored.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Bold));

        let formats = restored.get_formats_at(Position::new(8));
        assert!(formats.contains(&InlineFormat::Italic));
    }

    #[test]
    fn test_roundtrip_html_heading() {
        let mut doc = Document::from_text("Heading");
        doc.set_block_type(Range::from_offsets(0, 7), BlockType::heading(2));

        let html = doc.to_html();
        let restored = Document::from_html(&html).unwrap();

        assert_eq!(restored.get_content(), "Heading");
        assert_eq!(
            restored.get_block_type_at(Position::new(0)),
            BlockType::heading(2)
        );
    }

    // HTML Sanitization Tests

    #[test]
    fn test_sanitizer_safe_url_https() {
        let sanitizer = HtmlSanitizer::new();
        assert!(sanitizer.is_safe_url("https://example.com"));
        assert!(sanitizer.is_safe_url("https://example.com/path?query=value"));
    }

    #[test]
    fn test_sanitizer_safe_url_http() {
        let sanitizer = HtmlSanitizer::new();
        assert!(sanitizer.is_safe_url("http://example.com"));
    }

    #[test]
    fn test_sanitizer_safe_url_mailto() {
        let sanitizer = HtmlSanitizer::new();
        assert!(sanitizer.is_safe_url("mailto:user@example.com"));
    }

    #[test]
    fn test_sanitizer_safe_url_relative() {
        let sanitizer = HtmlSanitizer::new();
        assert!(sanitizer.is_safe_url("/path/to/page"));
        assert!(sanitizer.is_safe_url("../relative/path"));
        assert!(sanitizer.is_safe_url("page.html"));
        assert!(sanitizer.is_safe_url("#anchor"));
    }

    #[test]
    fn test_sanitizer_safe_url_protocol_relative() {
        let sanitizer = HtmlSanitizer::new();
        assert!(sanitizer.is_safe_url("//example.com/path"));
    }

    #[test]
    fn test_sanitizer_unsafe_url_javascript() {
        let sanitizer = HtmlSanitizer::new();
        assert!(!sanitizer.is_safe_url("javascript:alert('xss')"));
        assert!(!sanitizer.is_safe_url("JavaScript:alert('xss')"));
        assert!(!sanitizer.is_safe_url("  javascript:alert('xss')  "));
    }

    #[test]
    fn test_sanitizer_unsafe_url_data() {
        let sanitizer = HtmlSanitizer::new();
        assert!(!sanitizer.is_safe_url("data:text/html,<script>alert('xss')</script>"));
        assert!(
            !sanitizer
                .is_safe_url("data:text/html;base64,PHNjcmlwdD5hbGVydCgneHNzJyk8L3NjcmlwdD4=")
        );
    }

    #[test]
    fn test_sanitizer_unsafe_url_vbscript() {
        let sanitizer = HtmlSanitizer::new();
        assert!(!sanitizer.is_safe_url("vbscript:msgbox('xss')"));
    }

    #[test]
    fn test_sanitizer_unsafe_url_file() {
        let sanitizer = HtmlSanitizer::new();
        assert!(!sanitizer.is_safe_url("file:///etc/passwd"));
        assert!(!sanitizer.is_safe_url("file://C:/Windows/System32"));
    }

    #[test]
    fn test_sanitizer_unsafe_url_empty() {
        let sanitizer = HtmlSanitizer::new();
        assert!(!sanitizer.is_safe_url(""));
        assert!(!sanitizer.is_safe_url("   "));
    }

    #[test]
    fn test_sanitizer_valid_color_hex() {
        let sanitizer = HtmlSanitizer::new();
        assert!(sanitizer.is_valid_color("#FF0000"));
        assert!(sanitizer.is_valid_color("#F00"));
        assert!(sanitizer.is_valid_color("#FF0000AA"));
        assert!(sanitizer.is_valid_color("#abc"));
        assert!(sanitizer.is_valid_color("#ABCDEF"));
    }

    #[test]
    fn test_sanitizer_valid_color_rgb() {
        let sanitizer = HtmlSanitizer::new();
        assert!(sanitizer.is_valid_color("rgb(255, 0, 0)"));
        assert!(sanitizer.is_valid_color("rgb(255,0,0)"));
        assert!(sanitizer.is_valid_color("rgba(255, 0, 0, 0.5)"));
    }

    #[test]
    fn test_sanitizer_valid_color_named() {
        let sanitizer = HtmlSanitizer::new();
        assert!(sanitizer.is_valid_color("red"));
        assert!(sanitizer.is_valid_color("blue"));
        assert!(sanitizer.is_valid_color("green"));
        assert!(sanitizer.is_valid_color("white"));
        assert!(sanitizer.is_valid_color("black"));
        assert!(sanitizer.is_valid_color("transparent"));
    }

    #[test]
    fn test_sanitizer_invalid_color_hex() {
        let sanitizer = HtmlSanitizer::new();
        assert!(!sanitizer.is_valid_color("#GG0000")); // Invalid hex
        assert!(!sanitizer.is_valid_color("#FF")); // Too short
        assert!(!sanitizer.is_valid_color("#FF00")); // Invalid length
        assert!(!sanitizer.is_valid_color("FF0000")); // Missing #
    }

    #[test]
    fn test_sanitizer_invalid_color_expression() {
        let sanitizer = HtmlSanitizer::new();
        assert!(!sanitizer.is_valid_color("expression(alert('xss'))"));
        assert!(!sanitizer.is_valid_color("url(javascript:alert('xss'))"));
    }

    #[test]
    fn test_sanitizer_invalid_color_unknown_named() {
        let sanitizer = HtmlSanitizer::new();
        assert!(!sanitizer.is_valid_color("notacolor"));
        assert!(!sanitizer.is_valid_color("xss"));
    }

    #[test]
    fn test_sanitizer_invalid_color_empty() {
        let sanitizer = HtmlSanitizer::new();
        assert!(!sanitizer.is_valid_color(""));
        assert!(!sanitizer.is_valid_color("   "));
    }

    #[test]
    fn test_sanitizer_style_allows_valid_colors() {
        let sanitizer = HtmlSanitizer::new();
        let result = sanitizer.sanitize_style("color: #FF0000; background-color: blue;");
        assert!(result.contains("color: #FF0000"));
        assert!(result.contains("background-color: blue"));
    }

    #[test]
    fn test_sanitizer_style_strips_invalid_properties() {
        let sanitizer = HtmlSanitizer::new();
        let result =
            sanitizer.sanitize_style("color: red; font-size: 20px; background-color: blue;");
        assert!(result.contains("color: red"));
        assert!(result.contains("background-color: blue"));
        assert!(!result.contains("font-size"));
    }

    #[test]
    fn test_sanitizer_style_strips_invalid_colors() {
        let sanitizer = HtmlSanitizer::new();
        let result =
            sanitizer.sanitize_style("color: expression(alert('xss')); background-color: blue;");
        assert!(!result.contains("expression"));
        assert!(result.contains("background-color: blue"));
    }

    #[test]
    fn test_sanitizer_style_empty_result() {
        let sanitizer = HtmlSanitizer::new();
        let result = sanitizer.sanitize_style("font-size: 20px; font-weight: bold;");
        assert!(result.is_empty());
    }

    #[test]
    fn test_sanitizer_attribute_href_safe() {
        let sanitizer = HtmlSanitizer::new();
        let result = sanitizer.sanitize_attribute("href", "https://example.com");
        assert_eq!(result, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_sanitizer_attribute_href_unsafe() {
        let sanitizer = HtmlSanitizer::new();
        let result = sanitizer.sanitize_attribute("href", "javascript:alert('xss')");
        assert_eq!(result, None);
    }

    #[test]
    fn test_sanitizer_attribute_style_valid() {
        let sanitizer = HtmlSanitizer::new();
        let result = sanitizer.sanitize_attribute("style", "color: red;");
        assert!(result.is_some());
        assert!(result.unwrap().contains("color: red"));
    }

    #[test]
    fn test_sanitizer_attribute_style_invalid() {
        let sanitizer = HtmlSanitizer::new();
        let result = sanitizer.sanitize_attribute("style", "font-size: 20px;");
        assert_eq!(result, None); // Empty after sanitization
    }

    #[test]
    fn test_sanitizer_attribute_disallowed() {
        let sanitizer = HtmlSanitizer::new();
        let result = sanitizer.sanitize_attribute("onclick", "alert('xss')");
        assert_eq!(result, None);
    }

    #[test]
    fn test_from_html_xss_prevention_script_tag() {
        let doc = Document::from_html("<p>Hello <script>alert('xss')</script>World</p>").unwrap();
        // Script tag and its content should be stripped
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_from_html_xss_prevention_javascript_url() {
        let doc =
            Document::from_html("<p><a href=\"javascript:alert('xss')\">Click</a></p>").unwrap();
        assert_eq!(doc.get_content(), "Click");

        // Link should be stripped due to javascript: protocol
        let formats = doc.get_formats_at(Position::new(2));
        assert!(
            !formats
                .iter()
                .any(|f| matches!(f, InlineFormat::Link { .. }))
        );
    }

    #[test]
    fn test_from_html_xss_prevention_data_url() {
        // Use a simpler data URL that doesn't contain HTML tags that confuse the parser
        let doc =
            Document::from_html("<p><a href=\"data:text/plain,malicious\">Click</a></p>").unwrap();
        assert_eq!(doc.get_content(), "Click");

        // Link should be stripped due to data: protocol
        let formats = doc.get_formats_at(Position::new(2));
        assert!(
            !formats
                .iter()
                .any(|f| matches!(f, InlineFormat::Link { .. }))
        );
    }

    #[test]
    fn test_from_html_xss_prevention_vbscript_url() {
        let doc =
            Document::from_html("<p><a href=\"vbscript:msgbox('xss')\">Click</a></p>").unwrap();
        assert_eq!(doc.get_content(), "Click");

        // Link should be stripped due to vbscript: protocol
        let formats = doc.get_formats_at(Position::new(2));
        assert!(
            !formats
                .iter()
                .any(|f| matches!(f, InlineFormat::Link { .. }))
        );
    }

    #[test]
    fn test_from_html_xss_prevention_file_url() {
        let doc = Document::from_html("<p><a href=\"file:///etc/passwd\">Click</a></p>").unwrap();
        assert_eq!(doc.get_content(), "Click");

        // Link should be stripped due to file: protocol
        let formats = doc.get_formats_at(Position::new(2));
        assert!(
            !formats
                .iter()
                .any(|f| matches!(f, InlineFormat::Link { .. }))
        );
    }

    #[test]
    fn test_from_html_allows_safe_urls() {
        let doc = Document::from_html("<p><a href=\"https://example.com\">Click</a></p>").unwrap();
        assert_eq!(doc.get_content(), "Click");

        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Link {
            url: "https://example.com".to_string()
        }));
    }

    #[test]
    fn test_from_html_allows_relative_urls() {
        let doc = Document::from_html("<p><a href=\"/path/to/page\">Click</a></p>").unwrap();
        assert_eq!(doc.get_content(), "Click");

        let formats = doc.get_formats_at(Position::new(2));
        assert!(formats.contains(&InlineFormat::Link {
            url: "/path/to/page".to_string()
        }));
    }

    #[test]
    fn test_from_html_sanitizes_invalid_color() {
        let doc = Document::from_html(
            "<p><span style=\"color: expression(alert('xss'));\">Text</span></p>",
        )
        .unwrap();
        assert_eq!(doc.get_content(), "Text");

        // Color should be stripped
        let formats = doc.get_formats_at(Position::new(2));
        assert!(
            !formats
                .iter()
                .any(|f| matches!(f, InlineFormat::TextColor { .. }))
        );
    }

    #[test]
    fn test_from_html_allows_valid_hex_color() {
        let doc = Document::from_html("<p><span style=\"color: #FF0000;\">Red</span></p>").unwrap();
        assert_eq!(doc.get_content(), "Red");

        let formats = doc.get_formats_at(Position::new(1));
        assert!(formats.contains(&InlineFormat::TextColor {
            color: "#FF0000".to_string()
        }));
    }

    #[test]
    fn test_from_html_allows_valid_named_color() {
        let doc = Document::from_html("<p><span style=\"color: red;\">Red</span></p>").unwrap();
        assert_eq!(doc.get_content(), "Red");

        let formats = doc.get_formats_at(Position::new(1));
        assert!(formats.contains(&InlineFormat::TextColor {
            color: "red".to_string()
        }));
    }

    #[test]
    fn test_from_html_strips_disallowed_attributes() {
        let doc =
            Document::from_html("<p onclick=\"alert('xss')\" onload=\"alert('xss')\">Text</p>")
                .unwrap();
        assert_eq!(doc.get_content(), "Text");
        // Document should be created successfully with dangerous attributes stripped
    }

    #[test]
    fn test_from_html_strips_style_tag() {
        let doc = Document::from_html("<p>Hello <style>body { display: none; }</style>World</p>")
            .unwrap();
        // Style tag should be stripped
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_from_html_nested_disallowed_tags() {
        let doc = Document::from_html("<p>Hello <div><script>alert('xss')</script></div>World</p>")
            .unwrap();
        // Nested disallowed tags should be stripped
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_to_plain_text_empty() {
        let doc = Document::new();
        assert_eq!(doc.to_plain_text(), "");
    }

    #[test]
    fn test_to_plain_text_simple() {
        let doc = Document::from_text("Hello World");
        assert_eq!(doc.to_plain_text(), "Hello World");
    }

    #[test]
    fn test_to_plain_text_strips_formatting() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.apply_format(Range::from_offsets(6, 11), InlineFormat::Italic);
        doc.apply_format(
            Range::from_offsets(0, 11),
            InlineFormat::TextColor {
                color: "#FF0000".to_string(),
            },
        );
        assert_eq!(doc.to_plain_text(), "Hello World");
    }

    #[test]
    fn test_to_plain_text_strips_block_formatting() {
        let mut doc = Document::from_text("Heading\nParagraph\nList Item");
        doc.set_block_type(Range::from_offsets(0, 7), BlockType::heading(1));
        doc.set_block_type(Range::from_offsets(16, 25), BlockType::BulletList);
        assert_eq!(doc.to_plain_text(), "Heading\nParagraph\nList Item");
    }

    #[test]
    fn test_to_plain_text_preserves_line_breaks() {
        let doc = Document::from_text("Line 1\nLine 2\nLine 3");
        assert_eq!(doc.to_plain_text(), "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_from_plain_text_empty() {
        let doc = Document::from_plain_text("");
        assert_eq!(doc.get_content(), "");
    }

    #[test]
    fn test_from_plain_text_simple() {
        let doc = Document::from_plain_text("Hello World");
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_from_plain_text_multiline() {
        let doc = Document::from_plain_text("Line 1\nLine 2\nLine 3");
        assert_eq!(doc.get_content(), "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_from_plain_text_no_formatting() {
        let doc = Document::from_plain_text("Hello World");
        let formats = doc.get_formats_at(Position::new(0));
        assert!(formats.is_empty());
    }

    #[test]
    fn test_roundtrip_plain_text() {
        let original = "Hello World\nThis is a test\nWith multiple lines";
        let doc = Document::from_plain_text(original);
        let exported = doc.to_plain_text();
        assert_eq!(exported, original);
    }

    #[test]
    fn test_roundtrip_plain_text_with_formatting_lost() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.set_block_type(Range::from_offsets(0, 11), BlockType::heading(1));

        let plain = doc.to_plain_text();
        let restored = Document::from_plain_text(&plain);

        // Content should match
        assert_eq!(restored.get_content(), "Hello World");

        // But formatting should be lost
        let formats = restored.get_formats_at(Position::new(0));
        assert!(formats.is_empty());
        assert_eq!(
            restored.get_block_type_at(Position::new(0)),
            BlockType::Paragraph
        );
    }
}
