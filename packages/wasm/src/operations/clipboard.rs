// Clipboard operations module
use crate::document::{Document, Position, Range};
use crate::formatting::InlineFormat;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Represents clipboard content with text and formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardContent {
    /// Plain text content
    pub text: String,
    /// Format runs that apply to the text
    pub formats: Vec<SerializableFormatRun>,
}

/// Serializable version of FormatRun for clipboard operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableFormatRun {
    /// Start offset relative to the clipboard content
    pub start: usize,
    /// End offset relative to the clipboard content
    pub end: usize,
    /// Set of formats applied to this run
    pub formats: HashSet<InlineFormat>,
}

impl ClipboardContent {
    /// Creates a new empty ClipboardContent
    pub fn new() -> Self {
        Self {
            text: String::new(),
            formats: Vec::new(),
        }
    }

    /// Creates ClipboardContent from text and format runs
    pub fn from_text_and_formats(text: String, formats: Vec<SerializableFormatRun>) -> Self {
        Self { text, formats }
    }

    /// Converts to HTML format for clipboard
    pub fn to_html(&self) -> String {
        if self.text.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let mut pos = 0;
        let text_len = self.text.chars().count();

        while pos < text_len {
            // Find formats at this position
            let formats_at_pos: HashSet<InlineFormat> = self
                .formats
                .iter()
                .filter(|run| pos >= run.start && pos < run.end)
                .flat_map(|run| run.formats.iter().cloned())
                .collect();

            // Find the end of this format run
            let mut run_end = pos + 1;
            while run_end < text_len {
                let formats_at_next: HashSet<InlineFormat> = self
                    .formats
                    .iter()
                    .filter(|run| run_end >= run.start && run_end < run.end)
                    .flat_map(|run| run.formats.iter().cloned())
                    .collect();
                if formats_at_next != formats_at_pos {
                    break;
                }
                run_end += 1;
            }

            // Extract the text for this run
            let run_text: String = self.text.chars().skip(pos).take(run_end - pos).collect();

            // Apply formats
            let formatted = apply_html_formats(&run_text, &formats_at_pos);
            result.push_str(&formatted);

            pos = run_end;
        }

        result
    }

    /// Creates ClipboardContent from HTML
    pub fn from_html(html: &str) -> Result<Self, String> {
        // Use the Document's HTML parser to parse the content
        let doc = Document::from_html(html).map_err(|e| e.to_string())?;

        // Extract text and formats
        let text = doc.get_content();
        let formats = doc
            .formats()
            .get_runs()
            .iter()
            .map(|run| SerializableFormatRun {
                start: run.range.start_offset(),
                end: run.range.end_offset(),
                formats: run.formats.clone(),
            })
            .collect();

        Ok(Self { text, formats })
    }

    /// Returns true if the clipboard content is empty
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl Default for ClipboardContent {
    fn default() -> Self {
        Self::new()
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
        result = format!("<del>{}</del>", result);
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

impl Document {
    /// Copies the current selection to clipboard content
    ///
    /// Serializes the selected text along with all formatting information.
    /// Returns ClipboardContent that can be converted to HTML or plain text.
    ///
    /// # Returns
    ///
    /// ClipboardContent containing the selected text and formats.
    /// Returns empty content if selection is collapsed.
    pub fn copy(&self) -> ClipboardContent {
        if self.selection.is_collapsed() {
            return ClipboardContent::new();
        }

        let range = self.selection.range().normalize();
        let text = self.get_text_in_range(range);

        // Get all format runs that overlap with the selection
        let formats: Vec<SerializableFormatRun> = self
            .formats()
            .get_runs()
            .iter()
            .filter_map(|run| {
                let run_range = run.range.normalize();
                if run_range.overlaps(&range) {
                    // Calculate the intersection of the run with the selection
                    let start = run_range.start_offset().max(range.start_offset());
                    let end = run_range.end_offset().min(range.end_offset());

                    // Adjust offsets to be relative to the copied text
                    let relative_start = start - range.start_offset();
                    let relative_end = end - range.start_offset();

                    Some(SerializableFormatRun {
                        start: relative_start,
                        end: relative_end,
                        formats: run.formats.clone(),
                    })
                } else {
                    None
                }
            })
            .collect();

        ClipboardContent::from_text_and_formats(text, formats)
    }

    /// Cuts the current selection to clipboard content
    ///
    /// Copies the selected text and formats, then deletes the selection.
    /// This operation is undoable.
    ///
    /// # Returns
    ///
    /// ClipboardContent containing the cut text and formats.
    /// Returns empty content if selection is collapsed.
    ///
    /// # Errors
    ///
    /// Returns an error if the delete operation fails.
    pub fn cut(&mut self) -> Result<ClipboardContent, crate::operations::CommandError> {
        if self.selection.is_collapsed() {
            return Ok(ClipboardContent::new());
        }

        // Copy the content first
        let content = self.copy();

        // Delete the selection
        let range = self.selection.range();
        self.delete_range(range)?;

        // Collapse selection to the start of the deleted range
        self.collapse_to_start();

        Ok(content)
    }

    /// Pastes clipboard content at the current cursor position
    ///
    /// If there is a selection, it will be replaced with the pasted content.
    /// The pasted content retains all formatting from the clipboard.
    /// This operation is undoable.
    ///
    /// # Arguments
    ///
    /// * `content` - The ClipboardContent to paste
    ///
    /// # Errors
    ///
    /// Returns an error if the paste operation fails.
    pub fn paste(
        &mut self,
        content: &ClipboardContent,
    ) -> Result<(), crate::operations::CommandError> {
        if content.is_empty() {
            return Ok(());
        }

        let insert_pos = if self.selection.is_collapsed() {
            self.selection.anchor
        } else {
            // Delete the current selection first
            let range = self.selection.range();
            self.delete_range(range)?;
            self.selection.anchor
        };

        // Insert the text
        self.insert_text(insert_pos, &content.text)?;

        // Apply the formats
        let base_offset = insert_pos.offset();
        for format_run in &content.formats {
            let start = Position::new(base_offset + format_run.start);
            let end = Position::new(base_offset + format_run.end);
            let range = Range::new(start, end);

            for format in &format_run.formats {
                self.apply_format(range, format.clone());
            }
        }

        // Move cursor to the end of pasted content
        let end_pos = Position::new(base_offset + content.text.chars().count());
        self.selection = crate::selection::Selection::collapsed(end_pos);

        Ok(())
    }

    /// Pastes HTML content from the clipboard
    ///
    /// Parses the HTML, sanitizes it, and pastes the resulting content.
    /// This is a convenience method for pasting from external applications.
    ///
    /// # Arguments
    ///
    /// * `html` - The HTML string to paste
    ///
    /// # Errors
    ///
    /// Returns an error if HTML parsing or paste operation fails.
    pub fn paste_html(&mut self, html: &str) -> Result<(), String> {
        let content = ClipboardContent::from_html(html)?;
        self.paste(&content)
            .map_err(|e| format!("Paste failed: {}", e))
    }

    /// Pastes plain text at the current cursor position
    ///
    /// This is a convenience method for pasting text without formatting.
    ///
    /// # Arguments
    ///
    /// * `text` - The plain text to paste
    ///
    /// # Errors
    ///
    /// Returns an error if the paste operation fails.
    pub fn paste_plain_text(&mut self, text: &str) -> Result<(), crate::operations::CommandError> {
        let content = ClipboardContent::from_text_and_formats(text.to_string(), Vec::new());
        self.paste(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_content_new() {
        let content = ClipboardContent::new();
        assert!(content.is_empty());
        assert_eq!(content.text, "");
        assert_eq!(content.formats.len(), 0);
    }

    #[test]
    fn test_clipboard_content_from_text_and_formats() {
        let text = "Hello".to_string();
        let mut formats_set = HashSet::new();
        formats_set.insert(InlineFormat::Bold);

        let formats = vec![SerializableFormatRun {
            start: 0,
            end: 5,
            formats: formats_set,
        }];

        let content = ClipboardContent::from_text_and_formats(text, formats);
        assert_eq!(content.text, "Hello");
        assert_eq!(content.formats.len(), 1);
    }

    #[test]
    fn test_clipboard_content_to_html_plain() {
        let content = ClipboardContent::from_text_and_formats("Hello".to_string(), Vec::new());
        assert_eq!(content.to_html(), "Hello");
    }

    #[test]
    fn test_clipboard_content_to_html_bold() {
        let mut formats_set = HashSet::new();
        formats_set.insert(InlineFormat::Bold);

        let formats = vec![SerializableFormatRun {
            start: 0,
            end: 5,
            formats: formats_set,
        }];

        let content = ClipboardContent::from_text_and_formats("Hello".to_string(), formats);
        assert_eq!(content.to_html(), "<strong>Hello</strong>");
    }

    #[test]
    fn test_clipboard_content_to_html_multiple_formats() {
        let mut formats_set = HashSet::new();
        formats_set.insert(InlineFormat::Bold);
        formats_set.insert(InlineFormat::Italic);

        let formats = vec![SerializableFormatRun {
            start: 0,
            end: 5,
            formats: formats_set,
        }];

        let content = ClipboardContent::from_text_and_formats("Hello".to_string(), formats);
        let html = content.to_html();
        assert!(html.contains("<strong>"));
        assert!(html.contains("<em>"));
    }

    #[test]
    fn test_clipboard_content_to_html_partial_format() {
        let mut formats_set = HashSet::new();
        formats_set.insert(InlineFormat::Bold);

        let formats = vec![SerializableFormatRun {
            start: 0,
            end: 5,
            formats: formats_set,
        }];

        let content = ClipboardContent::from_text_and_formats("Hello World".to_string(), formats);
        assert_eq!(content.to_html(), "<strong>Hello</strong> World");
    }

    #[test]
    fn test_clipboard_content_from_html_plain() {
        let content = ClipboardContent::from_html("<p>Hello</p>").unwrap();
        assert_eq!(content.text, "Hello");
    }

    #[test]
    fn test_clipboard_content_from_html_bold() {
        let content = ClipboardContent::from_html("<p><strong>Hello</strong></p>").unwrap();
        assert_eq!(content.text, "Hello");
        assert!(
            content
                .formats
                .iter()
                .any(|run| run.formats.contains(&InlineFormat::Bold))
        );
    }

    #[test]
    fn test_document_copy_empty_selection() {
        let doc = Document::from_text("Hello World");
        let content = doc.copy();
        assert!(content.is_empty());
    }

    #[test]
    fn test_document_copy_with_selection() {
        let mut doc = Document::from_text("Hello World");
        doc.set_selection(crate::selection::Selection::new(
            Position::new(0),
            Position::new(5),
        ));

        let content = doc.copy();
        assert_eq!(content.text, "Hello");
    }

    #[test]
    fn test_document_copy_with_formats() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.set_selection(crate::selection::Selection::new(
            Position::new(0),
            Position::new(5),
        ));

        let content = doc.copy();
        assert_eq!(content.text, "Hello");
        assert_eq!(content.formats.len(), 1);
        assert!(content.formats[0].formats.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_document_cut_empty_selection() {
        let mut doc = Document::from_text("Hello World");
        let content = doc.cut().unwrap();
        assert!(content.is_empty());
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_document_cut_with_selection() {
        let mut doc = Document::from_text("Hello World");
        doc.set_selection(crate::selection::Selection::new(
            Position::new(0),
            Position::new(6),
        ));

        let content = doc.cut().unwrap();
        assert_eq!(content.text, "Hello ");
        assert_eq!(doc.get_content(), "World");
    }

    #[test]
    fn test_document_cut_with_formats() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.set_selection(crate::selection::Selection::new(
            Position::new(0),
            Position::new(5),
        ));

        let content = doc.cut().unwrap();
        assert_eq!(content.text, "Hello");
        assert!(content.formats[0].formats.contains(&InlineFormat::Bold));
        assert_eq!(doc.get_content(), " World");
    }

    #[test]
    fn test_document_paste_empty_content() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(crate::selection::Selection::collapsed(Position::new(5)));

        let content = ClipboardContent::new();
        doc.paste(&content).unwrap();
        assert_eq!(doc.get_content(), "Hello");
    }

    #[test]
    fn test_document_paste_plain_text() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(crate::selection::Selection::collapsed(Position::new(5)));

        let content = ClipboardContent::from_text_and_formats(" World".to_string(), Vec::new());
        doc.paste(&content).unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_document_paste_with_formats() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(crate::selection::Selection::collapsed(Position::new(5)));

        let mut formats_set = HashSet::new();
        formats_set.insert(InlineFormat::Bold);
        let formats = vec![SerializableFormatRun {
            start: 0,
            end: 6,
            formats: formats_set,
        }];

        let content = ClipboardContent::from_text_and_formats(" World".to_string(), formats);
        doc.paste(&content).unwrap();
        assert_eq!(doc.get_content(), "Hello World");

        let formats_at_7 = doc.get_formats_at(Position::new(7));
        assert!(formats_at_7.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_document_paste_replace_selection() {
        let mut doc = Document::from_text("Hello World");
        doc.set_selection(crate::selection::Selection::new(
            Position::new(6),
            Position::new(11),
        ));

        let content = ClipboardContent::from_text_and_formats("Rust".to_string(), Vec::new());
        doc.paste(&content).unwrap();
        assert_eq!(doc.get_content(), "Hello Rust");
    }

    #[test]
    fn test_document_paste_html() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(crate::selection::Selection::collapsed(Position::new(5)));

        doc.paste_html("<p><strong> World</strong></p>").unwrap();
        assert_eq!(doc.get_content(), "Hello World");

        let formats = doc.get_formats_at(Position::new(7));
        assert!(formats.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_document_paste_plain_text_method() {
        let mut doc = Document::from_text("Hello");
        doc.set_selection(crate::selection::Selection::collapsed(Position::new(5)));

        doc.paste_plain_text(" World").unwrap();
        assert_eq!(doc.get_content(), "Hello World");
    }

    #[test]
    fn test_copy_paste_roundtrip() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);
        doc.apply_format(Range::from_offsets(6, 11), InlineFormat::Italic);

        // Copy "Hello "
        doc.set_selection(crate::selection::Selection::new(
            Position::new(0),
            Position::new(6),
        ));
        let content = doc.copy();

        // Paste at the end
        doc.set_selection(crate::selection::Selection::collapsed(Position::new(11)));
        doc.paste(&content).unwrap();

        assert_eq!(doc.get_content(), "Hello WorldHello ");

        // Check formats were preserved
        let formats_at_13 = doc.get_formats_at(Position::new(13));
        assert!(formats_at_13.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_cut_paste_roundtrip() {
        let mut doc = Document::from_text("Hello World");
        doc.apply_format(Range::from_offsets(0, 5), InlineFormat::Bold);

        // Cut "Hello"
        doc.set_selection(crate::selection::Selection::new(
            Position::new(0),
            Position::new(5),
        ));
        let content = doc.cut().unwrap();
        assert_eq!(doc.get_content(), " World");

        // Paste at the end
        doc.set_selection(crate::selection::Selection::collapsed(Position::new(6)));
        doc.paste(&content).unwrap();

        assert_eq!(doc.get_content(), " WorldHello");

        // Check formats were preserved
        let formats_at_8 = doc.get_formats_at(Position::new(8));
        assert!(formats_at_8.contains(&InlineFormat::Bold));
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
        assert_eq!(escape_html("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_clipboard_content_to_html_escape() {
        let content = ClipboardContent::from_text_and_formats(
            "<script>alert('xss')</script>".to_string(),
            Vec::new(),
        );
        let html = content.to_html();
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }
}
