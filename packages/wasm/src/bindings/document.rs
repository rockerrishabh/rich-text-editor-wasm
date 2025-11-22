// WasmDocument wrapper for JavaScript bindings

use crate::bindings::events::EventCallbacks;
use crate::document::{Document, Position, Range};
use crate::formatting::{BlockType, InlineFormat};
use crate::operations::search::SearchQuery;
use crate::selection::Selection;
use wasm_bindgen::prelude::*;

/// WASM-exposed wrapper around the Document struct
///
/// # Memory Management
///
/// WasmDocument instances are managed by JavaScript's garbage collector.
/// However, for optimal memory usage, especially with large documents or
/// many document instances, you can explicitly free the memory by calling
/// the `free()` method when the document is no longer needed.
///
/// ## Automatic Cleanup
///
/// When a WasmDocument instance is garbage collected by JavaScript, Rust's
/// Drop trait will automatically clean up internal resources including:
/// - Text storage buffers
/// - Format storage structures
/// - Command history stacks
/// - Event callback references
///
/// ## Manual Cleanup
///
/// For immediate memory reclamation, call `free()`:
///
/// ```javascript
/// const doc = new WasmDocument();
/// // ... use document ...
/// doc.free(); // Explicitly free memory
/// // doc is now invalid and should not be used
/// ```
///
/// ## Best Practices
///
/// 1. **Long-lived documents**: Let JavaScript GC handle cleanup
/// 2. **Temporary documents**: Call `free()` when done to reclaim memory immediately
/// 3. **Large documents**: Consider calling `free()` to avoid memory pressure
/// 4. **Multiple instances**: Free unused instances to reduce memory footprint
///
/// ## Memory Characteristics
///
/// - Base overhead: ~1KB per document instance
/// - Text storage: ~4 bytes per character (UTF-32 internal representation)
/// - Format storage: ~32 bytes per format run
/// - Command history: ~64 bytes per command (configurable, default 100 commands)
/// - Event callbacks: ~8 bytes per registered callback
///
/// Example memory usage for a 10,000 character document with 50 format runs:
/// - Text: 40KB
/// - Formats: 1.6KB
/// - History: 6.4KB (100 commands)
/// - Total: ~48KB
#[wasm_bindgen]
pub struct WasmDocument {
    inner: Document,
    callbacks: EventCallbacks,
}

#[wasm_bindgen]
impl WasmDocument {
    /// Creates a new empty document
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Document::new(),
            callbacks: EventCallbacks::new(),
        }
    }

    /// Creates a document from existing text
    #[wasm_bindgen(js_name = fromText)]
    pub fn from_text(text: &str) -> Self {
        Self {
            inner: Document::from_text(text),
            callbacks: EventCallbacks::new(),
        }
    }

    /// Inserts text at the specified position
    ///
    /// # Arguments
    /// * `text` - The text to insert
    /// * `position` - The character offset where text should be inserted
    ///
    /// # Errors
    /// Returns a JsValue error if the position is invalid
    #[wasm_bindgen(js_name = insertText)]
    pub fn insert_text(&mut self, text: &str, position: usize) -> Result<(), JsValue> {
        let pos = Position::new(position);
        self.inner
            .insert_text(pos, text)
            .map_err(|e| JsValue::from_str(&format!("Insert failed: {}", e)))?;
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Deletes text in the specified range
    ///
    /// # Arguments
    /// * `start` - The start position of the range to delete
    /// * `end` - The end position of the range to delete
    ///
    /// # Errors
    /// Returns a JsValue error if the range is invalid
    #[wasm_bindgen(js_name = deleteRange)]
    pub fn delete_range(&mut self, start: usize, end: usize) -> Result<(), JsValue> {
        let range = Range::from_offsets(start, end);
        self.inner
            .delete_range(range)
            .map_err(|e| JsValue::from_str(&format!("Delete failed: {}", e)))?;
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Replaces text in the specified range with new text
    ///
    /// # Arguments
    /// * `start` - The start position of the range to replace
    /// * `end` - The end position of the range to replace
    /// * `text` - The new text to insert
    ///
    /// # Errors
    /// Returns a JsValue error if the range is invalid
    #[wasm_bindgen(js_name = replaceRange)]
    pub fn replace_range(&mut self, start: usize, end: usize, text: &str) -> Result<(), JsValue> {
        let range = Range::from_offsets(start, end);
        self.inner
            .replace_range(range, text)
            .map_err(|e| JsValue::from_str(&format!("Replace failed: {}", e)))?;
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Returns the entire content of the document
    #[wasm_bindgen(js_name = getContent)]
    pub fn get_content(&self) -> String {
        self.inner.get_content()
    }

    /// Returns the length of the document in characters
    #[wasm_bindgen(js_name = getLength)]
    pub fn get_length(&self) -> usize {
        self.inner.get_length()
    }

    /// Returns the number of words in the document
    ///
    /// Words are defined as sequences of non-whitespace characters
    /// separated by whitespace. This matches the behavior of most
    /// word processors.
    ///
    /// # Returns
    /// The number of words in the document
    ///
    /// # Example
    /// ```javascript
    /// const doc = new WasmDocument();
    /// doc.insertText("Hello world", 0);
    /// console.log(doc.getWordCount()); // 2
    /// ```
    #[wasm_bindgen(js_name = getWordCount)]
    pub fn get_word_count(&self) -> usize {
        let content = self.inner.get_content();
        if content.trim().is_empty() {
            return 0;
        }
        content
            .split_whitespace()
            .filter(|word| !word.is_empty())
            .count()
    }

    /// Returns the number of lines in the document
    ///
    /// Lines are separated by newline characters (\n).
    /// An empty document has 1 line.
    ///
    /// # Returns
    /// The number of lines in the document
    ///
    /// # Example
    /// ```javascript
    /// const doc = new WasmDocument();
    /// doc.insertText("Line 1\nLine 2\nLine 3", 0);
    /// console.log(doc.getLineCount()); // 3
    /// ```
    #[wasm_bindgen(js_name = getLineCount)]
    pub fn get_line_count(&self) -> usize {
        let content = self.inner.get_content();
        if content.is_empty() {
            return 1;
        }
        content.chars().filter(|&c| c == '\n').count() + 1
    }

    /// Returns the text within the specified range
    ///
    /// # Arguments
    /// * `start` - The start position of the range
    /// * `end` - The end position of the range
    ///
    /// # Errors
    /// Returns a JsValue error if the range is invalid
    #[wasm_bindgen(js_name = getTextInRange)]
    pub fn get_text_in_range(&self, start: usize, end: usize) -> Result<String, JsValue> {
        if end > self.inner.get_length() {
            return Err(JsValue::from_str(&format!(
                "Range end {} exceeds document length {}",
                end,
                self.inner.get_length()
            )));
        }
        let range = Range::from_offsets(start, end);
        Ok(self.inner.get_text_in_range(range))
    }

    /// Returns the current version of the document
    #[wasm_bindgen(js_name = getVersion)]
    pub fn get_version(&self) -> u64 {
        self.inner.version()
    }

    /// Returns true if the document is empty
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Undoes the last operation
    ///
    /// # Errors
    /// Returns a JsValue error if there is nothing to undo
    pub fn undo(&mut self) -> Result<(), JsValue> {
        self.inner
            .undo()
            .map_err(|e| JsValue::from_str(&format!("Undo failed: {}", e)))?;
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Redoes the last undone operation
    ///
    /// # Errors
    /// Returns a JsValue error if there is nothing to redo
    pub fn redo(&mut self) -> Result<(), JsValue> {
        self.inner
            .redo()
            .map_err(|e| JsValue::from_str(&format!("Redo failed: {}", e)))?;
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Returns true if there are operations that can be undone
    #[wasm_bindgen(js_name = canUndo)]
    pub fn can_undo(&self) -> bool {
        self.inner.can_undo()
    }

    /// Returns true if there are operations that can be redone
    #[wasm_bindgen(js_name = canRedo)]
    pub fn can_redo(&self) -> bool {
        self.inner.can_redo()
    }

    /// Applies a format to the specified range
    ///
    /// # Arguments
    /// * `format_type` - The type of format ("bold", "italic", "underline", "strikethrough", "code")
    /// * `start` - The start position of the range
    /// * `end` - The end position of the range
    ///
    /// # Errors
    /// Returns a JsValue error if the format type is invalid or range is invalid
    #[wasm_bindgen(js_name = applyFormat)]
    pub fn apply_format(
        &mut self,
        format_type: &str,
        start: usize,
        end: usize,
    ) -> Result<(), JsValue> {
        let format = parse_inline_format(format_type)?;
        let range = Range::from_offsets(start, end);
        self.inner.apply_format(range, format);
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Applies a format with a parameter (link, text color, background color)
    ///
    /// # Arguments
    /// * `format_type` - The type of format ("link", "textColor", "text-color", "backgroundColor", "background-color")
    /// * `value` - The value for the format (URL for link, color for colors)
    /// * `start` - The start position of the range
    /// * `end` - The end position of the range
    ///
    /// # Errors
    /// Returns a JsValue error if the format type is invalid or range is invalid
    #[wasm_bindgen(js_name = applyFormatWithValue)]
    pub fn apply_format_with_value(
        &mut self,
        format_type: &str,
        value: &str,
        start: usize,
        end: usize,
    ) -> Result<(), JsValue> {
        let format = match format_type {
            "link" => InlineFormat::Link {
                url: value.to_string(),
            },
            "textColor" | "text-color" => InlineFormat::TextColor {
                color: value.to_string(),
            },
            "backgroundColor" | "background-color" => InlineFormat::BackgroundColor {
                color: value.to_string(),
            },
            _ => {
                return Err(JsValue::from_str(&format!(
                    "Unknown format type with value: {}",
                    format_type
                )));
            }
        };
        let range = Range::from_offsets(start, end);
        self.inner.apply_format(range, format);
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Removes a format from the specified range
    ///
    /// # Arguments
    /// * `format_type` - The type of format to remove
    /// * `start` - The start position of the range
    /// * `end` - The end position of the range
    ///
    /// # Errors
    /// Returns a JsValue error if the format type is invalid or range is invalid
    #[wasm_bindgen(js_name = removeFormat)]
    pub fn remove_format(
        &mut self,
        format_type: &str,
        start: usize,
        end: usize,
    ) -> Result<(), JsValue> {
        let format = parse_inline_format(format_type)?;
        let range = Range::from_offsets(start, end);
        self.inner.remove_format(range, &format);
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Toggles a format on the specified range
    ///
    /// # Arguments
    /// * `format_type` - The type of format to toggle
    /// * `start` - The start position of the range
    /// * `end` - The end position of the range
    ///
    /// # Errors
    /// Returns a JsValue error if the format type is invalid or range is invalid
    #[wasm_bindgen(js_name = toggleFormat)]
    pub fn toggle_format(
        &mut self,
        format_type: &str,
        start: usize,
        end: usize,
    ) -> Result<(), JsValue> {
        let format = parse_inline_format(format_type)?;
        let range = Range::from_offsets(start, end);
        self.inner.toggle_format(range, format);
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Gets all formats at the specified position
    ///
    /// Returns a JsValue array of format type strings
    ///
    /// # Arguments
    /// * `position` - The character offset to query
    #[wasm_bindgen(js_name = getFormatsAt)]
    pub fn get_formats_at(&self, position: usize) -> js_sys::Array {
        let pos = Position::new(position);
        let formats = self.inner.get_formats_at(pos);

        let format_values: Vec<JsValue> = formats.iter().map(|f| format_to_js_value(f)).collect();
        js_sys::Array::from_iter(format_values)
    }

    /// Returns formats at position as a plain array of strings (debug-friendly)
    #[wasm_bindgen(js_name = getFormatsAtStrings)]
    pub fn get_formats_at_strings(&self, position: usize) -> js_sys::Array {
        let pos = Position::new(position);
        let formats = self.inner.get_formats_at(pos);
        let strings: Vec<JsValue> = formats
            .iter()
            .map(|f| match f {
                InlineFormat::Bold => JsValue::from_str("bold"),
                InlineFormat::Italic => JsValue::from_str("italic"),
                InlineFormat::Underline => JsValue::from_str("underline"),
                InlineFormat::Strikethrough => JsValue::from_str("strikethrough"),
                InlineFormat::Code => JsValue::from_str("code"),
                InlineFormat::Link { .. } => JsValue::from_str("link"),
                InlineFormat::TextColor { .. } => JsValue::from_str("textColor"),
                InlineFormat::BackgroundColor { .. } => JsValue::from_str("backgroundColor"),
            })
            .collect();
        js_sys::Array::from_iter(strings)
    }

    /// Debug helper: returns all format runs with start/end and formats
    /// Returned shape: Array of objects { start: number, end: number, formats: Array<string|object> }
    #[wasm_bindgen(js_name = getFormatRunsDebug)]
    pub fn get_format_runs_debug(&self) -> JsValue {
        let runs = self.inner.formats().get_runs();
        let out = js_sys::Array::new();

        for run in runs.iter() {
            let obj = js_sys::Object::new();
            let start = run.range.start_offset() as u32;
            let end = run.range.end_offset() as u32;

            // Build formats array
            let fmts = js_sys::Array::new();
            for f in run.formats.iter() {
                fmts.push(&format_to_js_value(f));
            }

            js_sys::Reflect::set(&obj, &"start".into(), &JsValue::from_f64(start as f64)).unwrap();
            js_sys::Reflect::set(&obj, &"end".into(), &JsValue::from_f64(end as f64)).unwrap();
            js_sys::Reflect::set(&obj, &"formats".into(), &fmts.into()).unwrap();

            out.push(&obj);
        }

        out.into()
    }

    /// Sets the block type for the specified range
    ///
    /// # Arguments
    /// * `block_type` - The type of block ("paragraph", "heading1"-"heading6", "bulletList", "numberedList", "blockQuote", "codeBlock")
    /// * `start` - The start position of the range
    /// * `end` - The end position of the range
    ///
    /// # Errors
    /// Returns a JsValue error if the block type is invalid or range is invalid
    #[wasm_bindgen(js_name = setBlockType)]
    pub fn set_block_type(
        &mut self,
        block_type: &str,
        start: usize,
        end: usize,
    ) -> Result<(), JsValue> {
        let block = parse_block_type(block_type)?;
        let range = Range::from_offsets(start, end);
        self.inner.set_block_type(range, block);
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Gets the block type at the specified position
    ///
    /// Returns a string representing the block type
    ///
    /// # Arguments
    /// * `position` - The character offset to query
    #[wasm_bindgen(js_name = getBlockTypeAt)]
    pub fn get_block_type_at(&self, position: usize) -> String {
        let pos = Position::new(position);
        let block_type = self.inner.get_block_type_at(pos);
        block_type_to_string(&block_type)
    }

    /// Sets the selection to the specified anchor and focus positions
    ///
    /// # Arguments
    /// * `anchor` - The anchor position (where selection started)
    /// * `focus` - The focus position (where selection ends)
    #[wasm_bindgen(js_name = setSelection)]
    pub fn set_selection(&mut self, anchor: usize, focus: usize) {
        let selection = Selection::new(Position::new(anchor), Position::new(focus));
        self.inner.set_selection(selection);
        self.callbacks.trigger_selection_callbacks();
    }

    /// Gets the current selection
    ///
    /// Returns a JsValue object with anchor and focus properties
    #[wasm_bindgen(js_name = getSelection)]
    pub fn get_selection(&self) -> JsValue {
        let selection = self.inner.get_selection();
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"anchor".into(), &selection.anchor.offset().into()).unwrap();
        js_sys::Reflect::set(&obj, &"focus".into(), &selection.focus.offset().into()).unwrap();
        obj.into()
    }

    /// Gets the text within the current selection
    #[wasm_bindgen(js_name = getSelectedText)]
    pub fn get_selected_text(&self) -> String {
        self.inner.get_selected_text()
    }

    /// Selects all content in the document
    #[wasm_bindgen(js_name = selectAll)]
    pub fn select_all(&mut self) {
        self.inner.select_all();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Collapses the selection to the start position
    #[wasm_bindgen(js_name = collapseToStart)]
    pub fn collapse_to_start(&mut self) {
        self.inner.collapse_to_start();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Collapses the selection to the end position
    #[wasm_bindgen(js_name = collapseToEnd)]
    pub fn collapse_to_end(&mut self) {
        self.inner.collapse_to_end();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Moves the cursor left by one character
    #[wasm_bindgen(js_name = moveCursorLeft)]
    pub fn move_cursor_left(&mut self) {
        self.inner.move_cursor_left();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Moves the cursor right by one character
    #[wasm_bindgen(js_name = moveCursorRight)]
    pub fn move_cursor_right(&mut self) {
        self.inner.move_cursor_right();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Moves the cursor up by one line
    #[wasm_bindgen(js_name = moveCursorUp)]
    pub fn move_cursor_up(&mut self) {
        self.inner.move_cursor_up();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Moves the cursor down by one line
    #[wasm_bindgen(js_name = moveCursorDown)]
    pub fn move_cursor_down(&mut self) {
        self.inner.move_cursor_down();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Moves the cursor to the start of the current line
    #[wasm_bindgen(js_name = moveToLineStart)]
    pub fn move_to_line_start(&mut self) {
        self.inner.move_to_line_start();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Moves the cursor to the end of the current line
    #[wasm_bindgen(js_name = moveToLineEnd)]
    pub fn move_to_line_end(&mut self) {
        self.inner.move_to_line_end();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Moves the cursor to the start of the document
    #[wasm_bindgen(js_name = moveToDocumentStart)]
    pub fn move_to_document_start(&mut self) {
        self.inner.move_to_document_start();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Moves the cursor to the end of the document
    #[wasm_bindgen(js_name = moveToDocumentEnd)]
    pub fn move_to_document_end(&mut self) {
        self.inner.move_to_document_end();
        self.callbacks.trigger_selection_callbacks();
    }

    /// Moves the cursor by word boundaries
    ///
    /// # Arguments
    /// * `forward` - If true, moves to the next word; otherwise, moves to the previous word
    #[wasm_bindgen(js_name = moveByWord)]
    pub fn move_by_word(&mut self, forward: bool) {
        self.inner.move_by_word(forward);
        self.callbacks.trigger_selection_callbacks();
    }

    /// Serializes the document to JSON format
    ///
    /// # Returns
    /// A JSON string representation of the document
    ///
    /// # Errors
    /// Returns a JsValue error if serialization fails
    #[wasm_bindgen(js_name = toJSON)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        self.inner
            .to_json()
            .map_err(|e| JsValue::from_str(&format!("JSON serialization failed: {}", e)))
    }

    /// Serializes the document to pretty-printed JSON format
    ///
    /// # Returns
    /// A formatted JSON string representation of the document
    ///
    /// # Errors
    /// Returns a JsValue error if serialization fails
    #[wasm_bindgen(js_name = toJSONPretty)]
    pub fn to_json_pretty(&self) -> Result<String, JsValue> {
        self.inner
            .to_json_pretty()
            .map_err(|e| JsValue::from_str(&format!("JSON serialization failed: {}", e)))
    }

    /// Deserializes a document from JSON format
    ///
    /// # Arguments
    /// * `json` - A JSON string representation of a document
    ///
    /// # Returns
    /// A new WasmDocument instance
    ///
    /// # Errors
    /// Returns a JsValue error if deserialization fails
    #[wasm_bindgen(js_name = fromJSON)]
    pub fn from_json(json: &str) -> Result<WasmDocument, JsValue> {
        let doc = Document::from_json(json)
            .map_err(|e| JsValue::from_str(&format!("JSON deserialization failed: {}", e)))?;
        Ok(WasmDocument {
            inner: doc,
            callbacks: EventCallbacks::new(),
        })
    }

    /// Exports the document to Markdown format
    ///
    /// # Returns
    /// A Markdown string representation of the document
    #[wasm_bindgen(js_name = toMarkdown)]
    pub fn to_markdown(&self) -> String {
        self.inner.to_markdown()
    }

    /// Exports the document to HTML format
    ///
    /// Maps inline formats to HTML tags and block types to appropriate HTML elements.
    /// All content is properly escaped to prevent XSS attacks.
    ///
    /// # Returns
    /// An HTML string representation of the document
    #[wasm_bindgen(js_name = toHTML)]
    pub fn to_html(&self) -> String {
        self.inner.to_html()
    }

    /// Exports a specific range of the document to HTML format
    ///
    /// This method supports incremental rendering by allowing you to generate
    /// HTML for only specific regions of the document.
    ///
    /// # Arguments
    /// * `start` - The start position of the range
    /// * `end` - The end position of the range
    ///
    /// # Returns
    /// An HTML string representation of the specified range
    ///
    /// # Example
    /// ```javascript
    /// // Render only the first 100 characters
    /// const html = doc.toHTMLRange(0, 100);
    /// ```
    #[wasm_bindgen(js_name = toHTMLRange)]
    pub fn to_html_range(&self, start: usize, end: usize) -> String {
        let range = Range::from_offsets(start, end);
        self.inner.to_html_range(Some(range))
    }

    /// Exports HTML for all dirty regions in the document
    ///
    /// This method is useful for incremental rendering. It returns an array
    /// of objects containing the range and HTML for each dirty region.
    ///
    /// # Returns
    /// A JavaScript array of objects with `start`, `end`, and `html` properties
    ///
    /// # Example
    /// ```javascript
    /// const dirtyRegions = doc.getDirtyHTML();
    /// dirtyRegions.forEach(region => {
    ///   console.log(`Update range ${region.start}-${region.end}`);
    ///   updateDOM(region.start, region.end, region.html);
    /// });
    /// ```
    #[wasm_bindgen(js_name = getDirtyHTML)]
    pub fn get_dirty_html(&self) -> JsValue {
        let dirty_regions = self.inner.to_html_dirty_regions();

        let result: Vec<JsValue> = dirty_regions
            .into_iter()
            .map(|(range, html)| {
                let obj = js_sys::Object::new();
                let normalized = range.normalize();
                js_sys::Reflect::set(&obj, &"start".into(), &normalized.start_offset().into())
                    .unwrap();
                js_sys::Reflect::set(&obj, &"end".into(), &normalized.end_offset().into()).unwrap();
                js_sys::Reflect::set(&obj, &"html".into(), &html.into()).unwrap();
                obj.into()
            })
            .collect();

        js_sys::Array::from_iter(result).into()
    }

    /// Clears all dirty region flags
    ///
    /// This should be called after rendering dirty regions to reset the
    /// dirty tracking state.
    ///
    /// # Example
    /// ```javascript
    /// // Get and render dirty regions
    /// const dirtyRegions = doc.getDirtyHTML();
    /// renderDirtyRegions(dirtyRegions);
    ///
    /// // Clear dirty flags after rendering
    /// doc.clearDirtyFlags();
    /// ```
    #[wasm_bindgen(js_name = clearDirtyFlags)]
    pub fn clear_dirty_flags(&mut self) {
        self.inner.clear_dirty_flags();
    }

    /// Returns true if there are any dirty regions
    ///
    /// This can be used to check if rendering is needed.
    ///
    /// # Returns
    /// true if there are dirty regions, false otherwise
    ///
    /// # Example
    /// ```javascript
    /// if (doc.hasDirtyRegions()) {
    ///   const html = doc.toHTML();
    ///   updateEditor(html);
    ///   doc.clearDirtyFlags();
    /// }
    /// ```
    #[wasm_bindgen(js_name = hasDirtyRegions)]
    pub fn has_dirty_regions(&self) -> bool {
        self.inner.has_dirty_regions()
    }

    /// Imports a document from HTML format
    ///
    /// Parses HTML and converts tags to internal format representation.
    /// Dangerous content like javascript: URLs and event handlers are stripped.
    ///
    /// # Arguments
    /// * `html` - An HTML string
    ///
    /// # Returns
    /// A new WasmDocument instance
    ///
    /// # Errors
    /// Returns a JsValue error if HTML parsing fails
    #[wasm_bindgen(js_name = fromHTML)]
    pub fn from_html(html: &str) -> Result<WasmDocument, JsValue> {
        let doc = Document::from_html(html)
            .map_err(|e| JsValue::from_str(&format!("HTML parsing failed: {}", e)))?;
        Ok(WasmDocument {
            inner: doc,
            callbacks: EventCallbacks::new(),
        })
    }

    /// Imports a document from Markdown format (static method)
    ///
    /// # Arguments
    /// * `markdown` - A Markdown string
    ///
    /// # Returns
    /// A new WasmDocument instance
    ///
    /// # Errors
    /// Returns a JsValue error if parsing fails
    #[wasm_bindgen(js_name = fromMarkdown)]
    pub fn from_markdown(markdown: &str) -> Result<WasmDocument, JsValue> {
        let doc = Document::from_markdown(markdown)
            .map_err(|e| JsValue::from_str(&format!("Markdown parsing failed: {}", e)))?;
        Ok(WasmDocument {
            inner: doc,
            callbacks: EventCallbacks::new(),
        })
    }

    /// Imports markdown content into the current document (instance method)
    ///
    /// Replaces the current document content with the parsed markdown.
    /// This is useful for loading markdown into an existing document instance.
    ///
    /// # Arguments
    /// * `markdown` - A Markdown string
    ///
    /// # Errors
    /// Returns a JsValue error if parsing fails
    #[wasm_bindgen(js_name = loadFromMarkdown)]
    pub fn load_from_markdown(&mut self, markdown: &str) -> Result<(), JsValue> {
        let doc = Document::from_markdown(markdown)
            .map_err(|e| JsValue::from_str(&format!("Markdown parsing failed: {}", e)))?;
        self.inner = doc;
        self.callbacks.trigger_change_callbacks();
        Ok(())
    }

    /// Exports the document to plain text format (strips all formatting)
    ///
    /// # Returns
    /// A plain text string representation of the document
    #[wasm_bindgen(js_name = toPlainText)]
    pub fn to_plain_text(&self) -> String {
        // Simply return the content without formatting
        self.inner.get_content()
    }

    /// Finds all occurrences of the search pattern in the document
    ///
    /// # Arguments
    /// * `pattern` - The text pattern to search for
    /// * `case_sensitive` - Whether the search should be case-sensitive
    /// * `use_regex` - Whether to interpret the pattern as a regular expression
    ///
    /// # Returns
    /// A JsValue array of match objects with start and end properties
    ///
    /// # Errors
    /// Returns a JsValue error if the regex pattern is invalid
    #[wasm_bindgen(js_name = find)]
    pub fn find(
        &self,
        pattern: &str,
        case_sensitive: bool,
        use_regex: bool,
    ) -> Result<JsValue, JsValue> {
        let query = SearchQuery::new(pattern.to_string())
            .case_sensitive(case_sensitive)
            .use_regex(use_regex);

        let result = self
            .inner
            .find(&query)
            .map_err(|e| JsValue::from_str(&format!("Search failed: {}", e)))?;

        let matches: Vec<JsValue> = result
            .matches
            .iter()
            .map(|range| {
                let obj = js_sys::Object::new();
                let normalized = range.normalize();
                js_sys::Reflect::set(&obj, &"start".into(), &normalized.start_offset().into())
                    .unwrap();
                js_sys::Reflect::set(&obj, &"end".into(), &normalized.end_offset().into()).unwrap();
                obj.into()
            })
            .collect();

        Ok(js_sys::Array::from_iter(matches).into())
    }

    /// Finds and replaces all occurrences of the search pattern with replacement text
    ///
    /// # Arguments
    /// * `pattern` - The text pattern to search for
    /// * `replacement` - The text to replace matches with
    /// * `case_sensitive` - Whether the search should be case-sensitive
    /// * `use_regex` - Whether to interpret the pattern as a regular expression
    ///
    /// # Returns
    /// The number of replacements made
    ///
    /// # Errors
    /// Returns a JsValue error if the regex pattern is invalid or replacement fails
    #[wasm_bindgen(js_name = findAndReplace)]
    pub fn find_and_replace(
        &mut self,
        pattern: &str,
        replacement: &str,
        case_sensitive: bool,
        use_regex: bool,
    ) -> Result<usize, JsValue> {
        let query = SearchQuery::new(pattern.to_string())
            .case_sensitive(case_sensitive)
            .use_regex(use_regex);

        let count = self
            .inner
            .find_and_replace(&query, replacement)
            .map_err(|e| JsValue::from_str(&format!("Find and replace failed: {}", e)))?;

        if count > 0 {
            self.callbacks.trigger_change_callbacks();
        }

        Ok(count)
    }

    /// Registers a callback to be called when the document content changes
    ///
    /// # Arguments
    /// * `callback` - JavaScript function to call on document changes
    ///
    /// # Example
    /// ```javascript
    /// document.onChange(() => {
    ///     console.log("Document changed!");
    /// });
    /// ```
    #[wasm_bindgen(js_name = onChange)]
    pub fn on_change(&mut self, callback: js_sys::Function) {
        self.callbacks.add_change_callback(callback);
    }

    /// Registers a callback to be called when the selection changes
    ///
    /// # Arguments
    /// * `callback` - JavaScript function to call on selection changes
    ///
    /// # Example
    /// ```javascript
    /// document.onSelectionChange(() => {
    ///     console.log("Selection changed!");
    /// });
    /// ```
    #[wasm_bindgen(js_name = onSelectionChange)]
    pub fn on_selection_change(&mut self, callback: js_sys::Function) {
        self.callbacks.add_selection_callback(callback);
    }

    /// Unregisters a change callback
    ///
    /// # Arguments
    /// * `callback` - JavaScript function to remove from change callbacks
    ///
    /// # Returns
    /// true if the callback was found and removed, false otherwise
    #[wasm_bindgen(js_name = offChange)]
    pub fn off_change(&mut self, callback: js_sys::Function) -> bool {
        self.callbacks.remove_change_callback(&callback)
    }

    /// Unregisters a selection change callback
    ///
    /// # Arguments
    /// * `callback` - JavaScript function to remove from selection callbacks
    ///
    /// # Returns
    /// true if the callback was found and removed, false otherwise
    #[wasm_bindgen(js_name = offSelectionChange)]
    pub fn off_selection_change(&mut self, callback: js_sys::Function) -> bool {
        self.callbacks.remove_selection_callback(&callback)
    }

    /// Copies the current selection to clipboard format
    ///
    /// Returns an object with `text` and `html` properties containing the
    /// selected content in both plain text and HTML formats. The HTML format
    /// preserves all inline formatting.
    ///
    /// # Returns
    /// An object with:
    /// - `text`: Plain text content
    /// - `html`: HTML formatted content
    ///
    /// Returns an object with empty strings if selection is collapsed.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const clipboardData = document.copy();
    /// console.log(clipboardData.text); // "Hello"
    /// console.log(clipboardData.html); // "<strong>Hello</strong>"
    /// ```
    #[wasm_bindgen(js_name = copy)]
    pub fn copy(&self) -> JsValue {
        let content = self.inner.copy();
        let html = content.to_html();
        let text = content.text;

        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"text".into(), &text.into()).unwrap();
        js_sys::Reflect::set(&obj, &"html".into(), &html.into()).unwrap();
        obj.into()
    }

    /// Cuts the current selection to clipboard format
    ///
    /// Copies the selected text and formats, then deletes the selection.
    /// This operation is undoable.
    ///
    /// # Returns
    /// An object with:
    /// - `text`: Plain text content
    /// - `html`: HTML formatted content
    ///
    /// Returns an object with empty strings if selection is collapsed.
    ///
    /// # Errors
    /// Returns a JsValue error if the delete operation fails.
    ///
    /// # Example
    ///
    /// ```javascript
    /// const clipboardData = document.cut();
    /// console.log(clipboardData.text); // "Hello"
    /// // Selection is now deleted from document
    /// ```
    #[wasm_bindgen(js_name = cut)]
    pub fn cut(&mut self) -> Result<JsValue, JsValue> {
        let content = self
            .inner
            .cut()
            .map_err(|e| JsValue::from_str(&format!("Cut failed: {}", e)))?;

        let is_empty = content.is_empty();
        let html = content.to_html();
        let text = content.text;

        if !is_empty {
            self.callbacks.trigger_change_callbacks();
            self.callbacks.trigger_selection_callbacks();
        }

        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"text".into(), &text.into()).unwrap();
        js_sys::Reflect::set(&obj, &"html".into(), &html.into()).unwrap();
        Ok(obj.into())
    }

    /// Pastes HTML content at the current cursor position
    ///
    /// If there is a selection, it will be replaced with the pasted content.
    /// The HTML is parsed and sanitized to prevent XSS attacks. All formatting
    /// is preserved from the HTML.
    ///
    /// This operation is undoable.
    ///
    /// # Arguments
    /// * `html` - The HTML string to paste
    ///
    /// # Errors
    /// Returns a JsValue error if HTML parsing or paste operation fails.
    ///
    /// # Example
    ///
    /// ```javascript
    /// document.pasteHtml("<strong>Hello</strong> World");
    /// // Document now contains "Hello World" with "Hello" in bold
    /// ```
    #[wasm_bindgen(js_name = pasteHtml)]
    pub fn paste_html(&mut self, html: &str) -> Result<(), JsValue> {
        self.inner
            .paste_html(html)
            .map_err(|e| JsValue::from_str(&format!("Paste HTML failed: {}", e)))?;
        self.callbacks.trigger_change_callbacks();
        self.callbacks.trigger_selection_callbacks();
        Ok(())
    }

    /// Pastes plain text at the current cursor position
    ///
    /// If there is a selection, it will be replaced with the pasted text.
    /// No formatting is applied to the pasted text.
    ///
    /// This operation is undoable.
    ///
    /// # Arguments
    /// * `text` - The plain text to paste
    ///
    /// # Errors
    /// Returns a JsValue error if the paste operation fails.
    ///
    /// # Example
    ///
    /// ```javascript
    /// document.pastePlainText("Hello World");
    /// // Document now contains "Hello World" with no formatting
    /// ```
    #[wasm_bindgen(js_name = pastePlainText)]
    pub fn paste_plain_text(&mut self, text: &str) -> Result<(), JsValue> {
        self.inner
            .paste_plain_text(text)
            .map_err(|e| JsValue::from_str(&format!("Paste plain text failed: {}", e)))?;
        self.callbacks.trigger_change_callbacks();
        self.callbacks.trigger_selection_callbacks();
        Ok(())
    }

    /// Starts an IME composition at the current cursor position
    ///
    /// This should be called when the browser fires a `compositionstart` event.
    /// It marks the beginning of an IME composition session for languages like
    /// Chinese, Japanese, or Korean.
    ///
    /// # Example
    ///
    /// ```javascript
    /// element.addEventListener('compositionstart', () => {
    ///     document.startComposition();
    /// });
    /// ```
    #[wasm_bindgen(js_name = startComposition)]
    pub fn start_composition(&mut self) {
        self.inner.start_composition();
    }

    /// Updates the IME composition with new text
    ///
    /// This should be called when the browser fires a `compositionupdate` event.
    /// It replaces the current composition text with the new text.
    ///
    /// # Arguments
    /// * `text` - The new composition text
    ///
    /// # Example
    ///
    /// ```javascript
    /// element.addEventListener('compositionupdate', (e) => {
    ///     document.updateComposition(e.data);
    /// });
    /// ```
    #[wasm_bindgen(js_name = updateComposition)]
    pub fn update_composition(&mut self, text: &str) {
        self.inner.update_composition(text);
    }

    /// Ends the IME composition and commits the final text
    ///
    /// This should be called when the browser fires a `compositionend` event.
    /// It finalizes the composition and commits the text to the document.
    ///
    /// # Example
    ///
    /// ```javascript
    /// element.addEventListener('compositionend', () => {
    ///     document.endComposition();
    /// });
    /// ```
    #[wasm_bindgen(js_name = endComposition)]
    pub fn end_composition(&mut self) {
        self.inner.end_composition();
        self.callbacks.trigger_change_callbacks();
    }

    /// Cancels the IME composition without committing
    ///
    /// This removes any composition text and returns the document to its
    /// state before composition started. This is useful if the user cancels
    /// the composition (e.g., by pressing Escape).
    ///
    /// # Example
    ///
    /// ```javascript
    /// element.addEventListener('keydown', (e) => {
    ///     if (e.key === 'Escape' && document.isComposing()) {
    ///         document.cancelComposition();
    ///     }
    /// });
    /// ```
    #[wasm_bindgen(js_name = cancelComposition)]
    pub fn cancel_composition(&mut self) {
        self.inner.cancel_composition();
        self.callbacks.trigger_change_callbacks();
    }

    /// Returns true if IME composition is currently active
    ///
    /// This can be used to check whether the user is currently composing text
    /// with an IME, which may affect how keyboard events are handled.
    ///
    /// # Returns
    /// true if composition is active, false otherwise
    ///
    /// # Example
    ///
    /// ```javascript
    /// if (document.isComposing()) {
    ///     console.log("User is composing text");
    /// }
    /// ```
    #[wasm_bindgen(js_name = isComposing)]
    pub fn is_composing(&self) -> bool {
        self.inner.is_composing()
    }

    /// Returns the current composition range, if composition is active
    ///
    /// Returns a JsValue object with start and end properties, or null if
    /// composition is not active.
    ///
    /// # Returns
    /// An object with start and end properties, or null
    ///
    /// # Example
    ///
    /// ```javascript
    /// const range = document.getCompositionRange();
    /// if (range) {
    ///     console.log(`Composing from ${range.start} to ${range.end}`);
    /// }
    /// ```
    #[wasm_bindgen(js_name = getCompositionRange)]
    pub fn get_composition_range(&self) -> JsValue {
        if let Some(range) = self.inner.composition_range() {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"start".into(), &range.start.offset().into()).unwrap();
            js_sys::Reflect::set(&obj, &"end".into(), &range.end.offset().into()).unwrap();
            obj.into()
        } else {
            JsValue::NULL
        }
    }

    /// Returns the current composition text, if composition is active
    ///
    /// Returns the text being composed, or null if composition is not active.
    ///
    /// # Returns
    /// The composition text as a string, or null
    ///
    /// # Example
    ///
    /// ```javascript
    /// const text = document.getCompositionText();
    /// if (text) {
    ///     console.log(`Composing: ${text}`);
    /// }
    /// ```
    #[wasm_bindgen(js_name = getCompositionText)]
    pub fn get_composition_text(&self) -> JsValue {
        if let Some(text) = self.inner.composition_text() {
            JsValue::from_str(text)
        } else {
            JsValue::NULL
        }
    }

    /// Gets the current maximum history size
    ///
    /// Returns the maximum number of commands that can be stored in the
    /// undo/redo history. The default is 100 commands.
    ///
    /// # Returns
    /// The maximum number of commands in the history
    ///
    /// # Example
    ///
    /// ```javascript
    /// const limit = doc.getHistoryLimit();
    /// console.log(`History limit: ${limit}`); // "History limit: 100"
    /// ```
    #[wasm_bindgen(js_name = getHistoryLimit)]
    pub fn get_history_limit(&self) -> usize {
        self.inner.get_history_limit()
    }

    /// Sets the maximum history size
    ///
    /// Configures the maximum number of commands that can be stored in the
    /// undo/redo history. If the new size is smaller than the current number
    /// of commands, the oldest commands will be removed to fit the new limit.
    ///
    /// # Arguments
    /// * `max_size` - The new maximum number of commands to store
    ///
    /// # Memory Impact
    ///
    /// Reducing the history limit immediately frees memory from removed commands.
    /// Each command stores approximately 64 bytes plus the size of affected text.
    ///
    /// Examples:
    /// - Reducing from 100 to 50 commands: Saves ~3.2KB + text data
    /// - Reducing from 100 to 20 commands: Saves ~5.1KB + text data
    ///
    /// # Use Cases
    ///
    /// - **Mobile devices**: Use 20-50 commands to reduce memory usage
    /// - **Large documents**: Use smaller history to prevent memory pressure
    /// - **Memory-constrained environments**: Reduce to minimum needed
    /// - **Desktop applications**: Default 100 is usually fine
    ///
    /// # Example
    ///
    /// ```javascript
    /// // Reduce history for mobile devices
    /// doc.setHistoryLimit(50);
    ///
    /// // Minimal history for memory-constrained environments
    /// doc.setHistoryLimit(20);
    ///
    /// // Restore default
    /// doc.setHistoryLimit(100);
    /// ```
    #[wasm_bindgen(js_name = setHistoryLimit)]
    pub fn set_history_limit(&mut self, max_size: usize) {
        self.inner.set_history_limit(max_size);
    }

    /// Clears all undo and redo history
    ///
    /// This immediately frees all memory used by the command history.
    /// Useful after save operations or when memory is constrained.
    ///
    /// # Memory Impact
    ///
    /// Frees approximately 6.4KB plus text data for a full history of 100 commands.
    /// The actual amount depends on the size of operations in the history.
    ///
    /// # Use Cases
    ///
    /// - After saving the document (user can reload if needed)
    /// - When memory usage is high
    /// - After major operations that you don't want to undo
    /// - When switching between documents
    ///
    /// # Warning
    ///
    /// After calling this method, users will not be able to undo or redo
    /// any previous operations. Use with caution.
    ///
    /// # Example
    ///
    /// ```javascript
    /// // Clear history after save
    /// function saveDocument(doc) {
    ///   const json = doc.toJSON();
    ///   localStorage.setItem('document', json);
    ///   doc.clearHistory(); // Free memory
    /// }
    ///
    /// // Clear history when memory is low
    /// if (performance.memory.usedJSHeapSize > threshold) {
    ///   doc.clearHistory();
    /// }
    /// ```
    #[wasm_bindgen(js_name = clearHistory)]
    pub fn clear_history(&mut self) {
        self.inner.clear_history();
    }

    /// Returns memory usage statistics for the document
    ///
    /// Provides detailed information about memory usage across all components
    /// of the document. Useful for monitoring and optimizing memory usage.
    ///
    /// # Returns
    ///
    /// A JavaScript object with the following properties:
    /// - `textLength`: Number of characters in the document
    /// - `formatRuns`: Number of format runs (fewer is better)
    /// - `blocks`: Number of block info entries
    /// - `undoCommands`: Number of commands in undo stack
    /// - `redoCommands`: Number of commands in redo stack
    /// - `estimatedBytes`: Estimated total memory usage in bytes
    /// - `estimatedKB`: Estimated total memory usage in kilobytes
    ///
    /// # Memory Breakdown
    ///
    /// The estimated memory includes:
    /// - Base overhead: ~1KB
    /// - Text storage: ~4 bytes per character + gap overhead
    /// - Format storage: ~32 bytes per format run + ~16 bytes per block
    /// - Command history: ~64 bytes per command + affected text
    ///
    /// # Example
    ///
    /// ```javascript
    /// const stats = doc.getMemoryStats();
    /// console.log(`Document: ${stats.textLength} chars`);
    /// console.log(`Format runs: ${stats.formatRuns}`);
    /// console.log(`Memory: ${stats.estimatedKB.toFixed(2)} KB`);
    ///
    /// // Monitor memory usage
    /// if (stats.estimatedKB > 500) {
    ///   console.warn('Large document, consider reducing history');
    ///   doc.setHistoryLimit(50);
    /// }
    ///
    /// // Check format efficiency
    /// if (stats.formatRuns > stats.textLength / 10) {
    ///   console.warn('Many format runs, document may be fragmented');
    /// }
    /// ```
    #[wasm_bindgen(js_name = getMemoryStats)]
    pub fn get_memory_stats(&self) -> JsValue {
        let (text_length, run_count, block_count, undo_count, redo_count, total_bytes) =
            self.inner.memory_stats();

        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"textLength".into(), &text_length.into()).unwrap();
        js_sys::Reflect::set(&obj, &"formatRuns".into(), &run_count.into()).unwrap();
        js_sys::Reflect::set(&obj, &"blocks".into(), &block_count.into()).unwrap();
        js_sys::Reflect::set(&obj, &"undoCommands".into(), &undo_count.into()).unwrap();
        js_sys::Reflect::set(&obj, &"redoCommands".into(), &redo_count.into()).unwrap();
        js_sys::Reflect::set(&obj, &"estimatedBytes".into(), &total_bytes.into()).unwrap();
        js_sys::Reflect::set(
            &obj,
            &"estimatedKB".into(),
            &((total_bytes as f64) / 1024.0).into(),
        )
        .unwrap();

        obj.into()
    }


}

// Implement Drop trait for automatic cleanup
impl Drop for WasmDocument {
    /// Automatically cleans up resources when the document is dropped
    ///
    /// This is called automatically by Rust when:
    /// 1. The `free()` method is called explicitly
    /// 2. The JavaScript garbage collector collects the object
    /// 3. The document goes out of scope in Rust code
    ///
    /// The Drop implementation ensures that all resources are properly
    /// released, including:
    /// - Clearing the command history to free command objects
    /// - Clearing event callbacks to release JavaScript function references
    /// - Allowing the Document and its internal structures to be deallocated
    fn drop(&mut self) {
        // Clear event callbacks to release JavaScript function references
        // This is important to avoid memory leaks in JavaScript
        self.callbacks.clear_all();

        // The Document's Drop implementation (if any) will be called automatically
        // The inner Document will be dropped, which will:
        // - Drop the TextStorage (releasing the gap buffer)
        // - Drop the FormatStorage (releasing format runs and blocks)
        // - Drop the CommandHistory (releasing all commands in undo/redo stacks)
        // - Drop the Selection
    }
}

/// Helper function to parse inline format from string
fn parse_inline_format(format_type: &str) -> Result<InlineFormat, JsValue> {
    match format_type {
        "bold" => Ok(InlineFormat::Bold),
        "italic" => Ok(InlineFormat::Italic),
        "underline" => Ok(InlineFormat::Underline),
        "strikethrough" => Ok(InlineFormat::Strikethrough),
        "code" => Ok(InlineFormat::Code),
        // For removeFormat, we need to support link/textColor/backgroundColor
        // We'll use empty values as placeholders since we're removing them anyway
        "link" => Ok(InlineFormat::Link { url: String::new() }),
        "textColor" | "text-color" => Ok(InlineFormat::TextColor {
            color: String::new(),
        }),
        "backgroundColor" | "background-color" => Ok(InlineFormat::BackgroundColor {
            color: String::new(),
        }),
        _ => Err(JsValue::from_str(&format!(
            "Unknown format type: {}",
            format_type
        ))),
    }
}

/// Helper function to convert InlineFormat to JsValue
fn format_to_js_value(format: &InlineFormat) -> JsValue {
    match format {
        InlineFormat::Bold => JsValue::from_str("bold"),
        InlineFormat::Italic => JsValue::from_str("italic"),
        InlineFormat::Underline => JsValue::from_str("underline"),
        InlineFormat::Strikethrough => JsValue::from_str("strikethrough"),
        InlineFormat::Code => JsValue::from_str("code"),
        InlineFormat::Link { .. } => JsValue::from_str("link"),
        InlineFormat::TextColor { .. } => JsValue::from_str("textColor"),
        InlineFormat::BackgroundColor { .. } => JsValue::from_str("backgroundColor"),
    }
}

/// Helper function to parse block type from string
fn parse_block_type(block_type: &str) -> Result<BlockType, JsValue> {
    match block_type {
        "paragraph" => Ok(BlockType::Paragraph),
        // Support both old and new naming conventions
        "heading1" | "h1" => Ok(BlockType::heading(1)),
        "heading2" | "h2" => Ok(BlockType::heading(2)),
        "heading3" | "h3" => Ok(BlockType::heading(3)),
        "heading4" | "h4" => Ok(BlockType::heading(4)),
        "heading5" | "h5" => Ok(BlockType::heading(5)),
        "heading6" | "h6" => Ok(BlockType::heading(6)),
        "bulletList" | "unordered-list" => Ok(BlockType::BulletList),
        "numberedList" | "ordered-list" => Ok(BlockType::NumberedList),
        "blockQuote" | "blockquote" => Ok(BlockType::BlockQuote),
        "codeBlock" | "code-block" => Ok(BlockType::CodeBlock),
        _ => Err(JsValue::from_str(&format!(
            "Unknown block type: {}",
            block_type
        ))),
    }
}

/// Helper function to convert BlockType to string
fn block_type_to_string(block_type: &BlockType) -> String {
    match block_type {
        BlockType::Paragraph => "paragraph".to_string(),
        // Use short format for consistency with tests
        BlockType::Heading { level } => format!("h{}", level),
        BlockType::BulletList => "unordered-list".to_string(),
        BlockType::NumberedList => "ordered-list".to_string(),
        BlockType::BlockQuote => "blockquote".to_string(),
        BlockType::CodeBlock => "code-block".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;



    #[wasm_bindgen_test]
    fn test_wasm_document_creation() {
        let doc = WasmDocument::new();
        assert_eq!(doc.get_content(), "");
        assert_eq!(doc.get_length(), 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_document_from_text() {
        let doc = WasmDocument::from_text("Hello World");
        assert_eq!(doc.get_content(), "Hello World");
        assert_eq!(doc.get_length(), 11);
    }

    #[wasm_bindgen_test]
    fn test_wasm_insert_text() {
        let mut doc = WasmDocument::new();
        assert!(doc.insert_text("Hello", 0).is_ok());
        assert_eq!(doc.get_content(), "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_delete_range() {
        let mut doc = WasmDocument::from_text("Hello World");
        assert!(doc.delete_range(5, 11).is_ok());
        assert_eq!(doc.get_content(), "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_replace_range() {
        let mut doc = WasmDocument::from_text("Hello World");
        assert!(doc.replace_range(6, 11, "Rust").is_ok());
        assert_eq!(doc.get_content(), "Hello Rust");
    }

    #[wasm_bindgen_test]
    fn test_wasm_undo_redo() {
        let mut doc = WasmDocument::new();
        doc.insert_text("Hello", 0).unwrap();
        assert_eq!(doc.get_content(), "Hello");

        assert!(doc.can_undo());
        assert!(!doc.can_redo());

        doc.undo().unwrap();
        assert_eq!(doc.get_content(), "");
        assert!(doc.can_redo());

        doc.redo().unwrap();
        assert_eq!(doc.get_content(), "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_apply_format() {
        let mut doc = WasmDocument::from_text("Hello");
        assert!(doc.apply_format("bold", 0, 5).is_ok());
    }

    #[wasm_bindgen_test]
    fn test_wasm_toggle_format() {
        let mut doc = WasmDocument::from_text("Hello");
        // Toggle on
        assert!(doc.toggle_format("bold", 0, 5).is_ok());
        let formats = doc.get_formats_at(2);
        let arr = js_sys::Array::from(&formats);
        let contains_bold = arr
            .iter()
            .any(|v| v.as_string().map(|s| s == "bold").unwrap_or(false));
        assert!(contains_bold);

        // Toggle off
        assert!(doc.toggle_format("bold", 0, 5).is_ok());
        let formats2 = doc.get_formats_at(2);
        let arr2 = js_sys::Array::from(&formats2);
        let contains_bold2 = arr2
            .iter()
            .any(|v| v.as_string().map(|s| s == "bold").unwrap_or(false));
        assert!(!contains_bold2);
    }

    #[wasm_bindgen_test]
    fn test_wasm_set_block_type() {
        let mut doc = WasmDocument::from_text("Heading");
        assert!(doc.set_block_type("heading1", 0, 7).is_ok());
    }

    #[wasm_bindgen_test]
    fn test_wasm_selection() {
        let mut doc = WasmDocument::from_text("Hello World");
        doc.set_selection(0, 5);
        assert_eq!(doc.get_selected_text(), "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_select_all() {
        let mut doc = WasmDocument::from_text("Hello World");
        doc.select_all();
        assert_eq!(doc.get_selected_text(), "Hello World");
    }

    #[wasm_bindgen_test]
    fn test_wasm_to_json() {
        let doc = WasmDocument::from_text("Hello");
        let json = doc.to_json();
        assert!(json.is_ok());
        let json_str = json.unwrap();
        assert!(json_str.contains("Hello"));
    }

    #[wasm_bindgen_test]
    fn test_wasm_from_json() {
        let json = r#"{"version":"1.0","content":"Hello","formats":[],"blocks":[]}"#;
        let doc = WasmDocument::from_json(json);
        assert!(doc.is_ok());
        let doc = doc.unwrap();
        assert_eq!(doc.get_content(), "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_to_markdown() {
        let doc = WasmDocument::from_text("Hello");
        let md = doc.to_markdown();
        assert_eq!(md, "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_from_markdown() {
        let md = "**Hello**";
        let doc = WasmDocument::from_markdown(md);
        assert!(doc.is_ok());
        let doc = doc.unwrap();
        assert_eq!(doc.get_content(), "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_paste_html() {
        let mut doc = WasmDocument::new();
        let html = "<p>Hello</p>";
        assert!(doc.paste_html(html).is_ok());
        assert_eq!(doc.get_content(), "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_to_plain_text() {
        let mut doc = WasmDocument::from_text("Hello");
        doc.apply_format("bold", 0, 5).unwrap();
        assert_eq!(doc.to_plain_text(), "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_find() {
        let doc = WasmDocument::from_text("Hello World Hello");
        let result = doc.find("Hello", false, false);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_wasm_error_invalid_position() {
        let mut doc = WasmDocument::from_text("Hello");
        let result = doc.insert_text("World", 100);
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_wasm_error_invalid_range() {
        let mut doc = WasmDocument::from_text("Hello");
        let result = doc.delete_range(10, 20);
        assert!(result.is_err());
    }

    // Memory management tests
    #[wasm_bindgen_test]
    fn test_wasm_get_history_limit() {
        let doc = WasmDocument::new();
        assert_eq!(doc.get_history_limit(), 100);
    }

    #[wasm_bindgen_test]
    fn test_wasm_set_history_limit() {
        let mut doc = WasmDocument::new();
        doc.set_history_limit(50);
        assert_eq!(doc.get_history_limit(), 50);
    }

    #[wasm_bindgen_test]
    fn test_wasm_clear_history() {
        let mut doc = WasmDocument::new();
        doc.insert_text("Hello", 0).unwrap();
        doc.insert_text(" World", 5).unwrap();

        assert!(doc.can_undo());

        doc.clear_history();

        assert!(!doc.can_undo());
        assert!(!doc.can_redo());
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_memory_stats() {
        let mut doc = WasmDocument::from_text("Hello World");
        doc.apply_format("bold", 0, 5).unwrap();

        let stats = doc.get_memory_stats();
        assert!(stats.is_object());

        // Verify stats object has expected properties
        let obj = js_sys::Object::from(stats);
        assert!(js_sys::Reflect::has(&obj, &"textLength".into()).unwrap());
        assert!(js_sys::Reflect::has(&obj, &"formatRuns".into()).unwrap());
        assert!(js_sys::Reflect::has(&obj, &"estimatedBytes".into()).unwrap());
    }

    // Data marshalling tests
    #[wasm_bindgen_test]
    fn test_wasm_string_marshalling() {
        let mut doc = WasmDocument::new();

        // Test with various string types
        doc.insert_text("ASCII", 0).unwrap();
        assert_eq!(doc.get_content(), "ASCII");

        doc.insert_text(" Unicode: 你好", 5).unwrap();
        assert_eq!(doc.get_content(), "ASCII Unicode: 你好");

        doc.insert_text(" Emoji: 🎉", 17).unwrap();
        assert_eq!(doc.get_content(), "ASCII Unicode: 你好 Emoji: 🎉");
    }

    #[wasm_bindgen_test]
    fn test_wasm_number_marshalling() {
        let mut doc = WasmDocument::from_text("Hello World");

        // Test position and range parameters
        assert!(doc.insert_text("!", 11).is_ok());
        assert!(doc.delete_range(0, 5).is_ok());
        assert_eq!(doc.get_content(), " World!");
    }

    #[wasm_bindgen_test]
    fn test_wasm_array_marshalling() {
        let doc = WasmDocument::from_text("Hello World Hello");

        // Test find returns array
        let result = doc.find("Hello", false, false);
        assert!(result.is_ok());

        let matches = result.unwrap();
        assert!(matches.is_array());
    }

    #[wasm_bindgen_test]
    fn test_wasm_object_marshalling() {
        let mut doc = WasmDocument::from_text("Hello World");
        doc.set_selection(0, 5);

        // Test getSelection returns object
        let selection = doc.get_selection();
        assert!(selection.is_object());

        let obj = js_sys::Object::from(selection);
        assert!(js_sys::Reflect::has(&obj, &"anchor".into()).unwrap());
        assert!(js_sys::Reflect::has(&obj, &"focus".into()).unwrap());
    }

    // Format with value tests
    #[wasm_bindgen_test]
    fn test_wasm_apply_format_with_value_link() {
        let mut doc = WasmDocument::from_text("Click here");
        let result = doc.apply_format_with_value("link", "https://example.com", 0, 10);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_wasm_apply_format_with_value_color() {
        let mut doc = WasmDocument::from_text("Colored text");
        let result = doc.apply_format_with_value("textColor", "#FF0000", 0, 12);
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    fn test_wasm_apply_format_with_value_background() {
        let mut doc = WasmDocument::from_text("Highlighted");
        let result = doc.apply_format_with_value("backgroundColor", "#FFFF00", 0, 11);
        assert!(result.is_ok());
    }

    // Clipboard tests
    #[wasm_bindgen_test]
    fn test_wasm_copy() {
        let mut doc = WasmDocument::from_text("Hello World");
        doc.set_selection(0, 5);

        let clipboard = doc.copy();
        assert!(clipboard.is_object());

        let obj = js_sys::Object::from(clipboard);
        assert!(js_sys::Reflect::has(&obj, &"text".into()).unwrap());
        assert!(js_sys::Reflect::has(&obj, &"html".into()).unwrap());
    }

    #[wasm_bindgen_test]
    fn test_wasm_cut() {
        let mut doc = WasmDocument::from_text("Hello World");
        doc.set_selection(0, 5);

        let result = doc.cut();
        assert!(result.is_ok());

        let clipboard = result.unwrap();
        assert!(clipboard.is_object());

        // Text should be removed
        assert_eq!(doc.get_content(), " World");
    }

    #[wasm_bindgen_test]
    fn test_wasm_paste_plain_text() {
        let mut doc = WasmDocument::new();
        let result = doc.paste_plain_text("Hello World");
        assert!(result.is_ok());
        assert_eq!(doc.get_content(), "Hello World");
    }

    // IME composition tests
    #[wasm_bindgen_test]
    fn test_wasm_ime_composition() {
        let mut doc = WasmDocument::from_text("Hello");

        doc.set_selection(5, 5);
        doc.start_composition();
        assert!(doc.is_composing());

        doc.update_composition("世");
        assert_eq!(doc.get_content(), "Hello世");

        doc.update_composition("世界");
        assert_eq!(doc.get_content(), "Hello世界");

        doc.end_composition();
        assert!(!doc.is_composing());
        assert_eq!(doc.get_content(), "Hello世界");
    }

    #[wasm_bindgen_test]
    fn test_wasm_ime_cancel_composition() {
        let mut doc = WasmDocument::from_text("Hello");

        doc.set_selection(5, 5);
        doc.start_composition();
        doc.update_composition("世界");

        doc.cancel_composition();
        assert!(!doc.is_composing());
        assert_eq!(doc.get_content(), "Hello");
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_composition_range() {
        let mut doc = WasmDocument::from_text("Hello");

        doc.set_selection(5, 5);
        doc.start_composition();
        doc.update_composition("世界");

        let range = doc.get_composition_range();
        assert!(!range.is_null());
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_composition_text() {
        let mut doc = WasmDocument::from_text("Hello");

        doc.set_selection(5, 5);
        doc.start_composition();
        doc.update_composition("世界");

        let text = doc.get_composition_text();
        assert!(!text.is_null());
    }

    // Cursor movement tests
    #[wasm_bindgen_test]
    fn test_wasm_cursor_movement() {
        let mut doc = WasmDocument::from_text("Hello World");

        doc.set_selection(0, 0);
        doc.move_cursor_right();

        let sel = doc.get_selection();
        let obj = js_sys::Object::from(sel);
        let anchor = js_sys::Reflect::get(&obj, &"anchor".into()).unwrap();
        assert_eq!(anchor.as_f64().unwrap() as usize, 1);
    }

    #[wasm_bindgen_test]
    fn test_wasm_move_by_word() {
        let mut doc = WasmDocument::from_text("Hello World Test");

        doc.set_selection(0, 0);
        doc.move_by_word(true); // Move forward

        let sel = doc.get_selection();
        let obj = js_sys::Object::from(sel);
        let anchor = js_sys::Reflect::get(&obj, &"anchor".into()).unwrap();
        assert!(anchor.as_f64().unwrap() as usize > 0);
    }

    // Search and replace tests
    #[wasm_bindgen_test]
    fn test_wasm_find_and_replace() {
        let mut doc = WasmDocument::from_text("Hello World Hello");
        let result = doc.find_and_replace("Hello", "Hi", false, false);
        assert!(result.is_ok());

        let count = result.unwrap();
        assert_eq!(count, 2);
        assert_eq!(doc.get_content(), "Hi World Hi");
    }

    // Block type tests
    #[wasm_bindgen_test]
    fn test_wasm_get_block_type_at() {
        let mut doc = WasmDocument::from_text("Heading");
        doc.set_block_type("heading1", 0, 7).unwrap();

        let block_type = doc.get_block_type_at(0);
        assert_eq!(block_type, "h1");
    }

    // Format query tests
    #[wasm_bindgen_test]
    fn test_wasm_get_formats_at() {
        let mut doc = WasmDocument::from_text("Hello");
        doc.apply_format("bold", 0, 5).unwrap();
        doc.apply_format("italic", 0, 5).unwrap();

        let formats = doc.get_formats_at(2);
        assert!(formats.is_array());

        let arr = js_sys::Array::from(&formats);
        assert_eq!(arr.length(), 2);
    }

    // Error handling tests
    #[wasm_bindgen_test]
    fn test_wasm_error_messages() {
        let mut doc = WasmDocument::from_text("Hello");

        // Test invalid position error
        let result = doc.insert_text("World", 100);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let err_str = err.as_string().unwrap();
        assert!(err_str.contains("Insert failed"));
    }

    // Serialization roundtrip tests
    #[wasm_bindgen_test]
    fn test_wasm_json_roundtrip() {
        let mut doc = WasmDocument::from_text("Hello World");
        doc.apply_format("bold", 0, 5).unwrap();

        let json = doc.to_json().unwrap();
        let doc2 = WasmDocument::from_json(&json).unwrap();

        assert_eq!(doc2.get_content(), "Hello World");
    }

    #[wasm_bindgen_test]
    fn test_wasm_html_export() {
        let mut doc = WasmDocument::from_text("Hello");
        doc.apply_format("bold", 0, 5).unwrap();

        let html = doc.to_html();
        assert!(html.contains("<strong>"));
        assert!(html.contains("Hello"));
    }

    #[wasm_bindgen_test]
    fn test_wasm_markdown_roundtrip() {
        let md = "**Bold** and *italic*";
        let doc = WasmDocument::from_markdown(md).unwrap();

        let content = doc.get_content();
        assert!(content.contains("Bold"));
        assert!(content.contains("italic"));
    }

    // Edge case tests
    #[wasm_bindgen_test]
    fn test_wasm_empty_document() {
        let doc = WasmDocument::new();
        assert_eq!(doc.get_length(), 0);
        assert!(doc.is_empty());
        assert_eq!(doc.get_content(), "");
    }

    #[wasm_bindgen_test]
    fn test_wasm_large_text() {
        let mut doc = WasmDocument::new();
        let large_text = "a".repeat(10000);

        let result = doc.insert_text(&large_text, 0);
        assert!(result.is_ok());
        assert_eq!(doc.get_length(), 10000);
    }

    #[wasm_bindgen_test]
    fn test_wasm_unicode_handling() {
        let mut doc = WasmDocument::new();

        // Test various unicode characters
        doc.insert_text("Hello 世界 🌍 مرحبا", 0).unwrap();
        assert!(doc.get_length() > 0);

        let content = doc.get_content();
        assert!(content.contains("世界"));
        assert!(content.contains("🌍"));
        assert!(content.contains("مرحبا"));
    }

    // Document statistics tests
    #[wasm_bindgen_test]
    fn test_wasm_get_word_count_empty() {
        let doc = WasmDocument::new();
        assert_eq!(doc.get_word_count(), 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_word_count_simple() {
        let doc = WasmDocument::from_text("Hello world");
        assert_eq!(doc.get_word_count(), 2);
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_word_count_multiple_spaces() {
        let doc = WasmDocument::from_text("Hello   world   test");
        assert_eq!(doc.get_word_count(), 3);
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_word_count_with_newlines() {
        let doc = WasmDocument::from_text("Hello\nworld\ntest");
        assert_eq!(doc.get_word_count(), 3);
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_word_count_whitespace_only() {
        let doc = WasmDocument::from_text("   \n\t  ");
        assert_eq!(doc.get_word_count(), 0);
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_line_count_empty() {
        let doc = WasmDocument::new();
        assert_eq!(doc.get_line_count(), 1);
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_line_count_single_line() {
        let doc = WasmDocument::from_text("Hello world");
        assert_eq!(doc.get_line_count(), 1);
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_line_count_multiple_lines() {
        let doc = WasmDocument::from_text("Line 1\nLine 2\nLine 3");
        assert_eq!(doc.get_line_count(), 3);
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_line_count_trailing_newline() {
        let doc = WasmDocument::from_text("Line 1\nLine 2\n");
        assert_eq!(doc.get_line_count(), 3);
    }

    #[wasm_bindgen_test]
    fn test_wasm_get_line_count_only_newlines() {
        let doc = WasmDocument::from_text("\n\n\n");
        assert_eq!(doc.get_line_count(), 4);
    }
}
