//! Rich Text Editor WASM
//!
//! A high-performance rich text editor built with Rust and WebAssembly.
//! This library provides a complete document model with formatting, undo/redo,
//! and serialization capabilities, all compiled to WebAssembly for use in web browsers.
//!
//! # Architecture
//!
//! The editor is organized into several modules with clear responsibilities:
//!
//! - **bindings**: WASM/JavaScript interface layer
//! - **document**: Core document model and text storage
//! - **formatting**: Inline and block-level formatting
//! - **operations**: Command pattern and undo/redo
//! - **selection**: Text selection and cursor management
//! - **serialization**: Import/export (JSON, Markdown, HTML)
//! - **utils**: Shared utilities (IME, string interning)
//!
//! # Memory Management
//!
//! ## Overview
//!
//! The rich text editor uses WebAssembly's linear memory model, which is managed
//! by both Rust and JavaScript. Understanding memory characteristics is important
//! for optimal performance, especially with large documents or many instances.
//!
//! ## Memory Characteristics by Component
//!
//! ### Document Instance
//! - **Base overhead**: ~1KB per document instance
//! - **Includes**: Document struct, selection state, version tracking, dirty tracker
//!
//! ### Text Storage (Gap Buffer)
//! - **Memory per character**: ~4 bytes (UTF-32 internal representation)
//! - **Gap overhead**: 16-32 characters of unused space (64-128 bytes)
//! - **Example**: 10,000 character document = ~40KB + 128 bytes gap
//!
//! ### Format Storage
//! - **Memory per format run**: ~32 bytes
//! - **Memory per block info**: ~16 bytes
//! - **String interning**: Shared storage for repeated URLs/colors
//! - **Example**: 50 format runs = ~1.6KB
//!
//! ### Command History
//! - **Memory per command**: ~64 bytes + size of affected text
//! - **Default limit**: 100 commands (configurable)
//! - **Total overhead**: ~6.4KB + text data for 100 commands
//! - **Note**: Large operations (e.g., paste 10KB text) store full text for undo
//!
//! ### Event Callbacks
//! - **Memory per callback**: ~8 bytes
//! - **Typical usage**: 2-5 callbacks per document
//!
//! ## Total Memory Example
//!
//! For a typical 10,000 character document with 50 format runs and 100 commands:
//! - Text storage: 40KB
//! - Format storage: 1.6KB
//! - Command history: 6.4KB
//! - Base overhead: 1KB
//! - **Total**: ~49KB
//!
//! ## Memory Management Best Practices
//!
//! ### Automatic Cleanup
//!
//! WasmDocument instances are automatically cleaned up by JavaScript's garbage
//! collector. When a document is no longer referenced, Rust's Drop trait will
//! free all internal resources including text buffers, format storage, and
//! command history.
//!
//! ### Manual Cleanup with free()
//!
//! For immediate memory reclamation, call `free()` on the document:
//!
//! ```javascript
//! const doc = new WasmDocument();
//! // ... use document ...
//! doc.free(); // Explicitly free memory
//! // doc is now invalid and should not be used
//! ```
//!
//! ### When to Call free()
//!
//! 1. **Long-lived documents**: Let JavaScript GC handle cleanup automatically
//! 2. **Temporary documents**: Call `free()` when done to reclaim memory immediately
//! 3. **Large documents (>100KB)**: Consider calling `free()` to avoid memory pressure
//! 4. **Multiple instances**: Free unused instances to reduce memory footprint
//! 5. **Memory-constrained environments**: Call `free()` proactively on mobile devices
//!
//! ### Reducing Memory Usage
//!
//! 1. **Limit command history**: Configure smaller history size for memory-constrained environments
//! 2. **Clear history**: Call `clearHistory()` to free undo/redo memory
//! 3. **Batch operations**: Group multiple edits to reduce command history entries
//! 4. **String interning**: Repeated URLs and colors are automatically deduplicated
//! 5. **Format merging**: Adjacent format runs with identical formats are automatically merged
//!
//! ## Memory Leak Prevention
//!
//! The editor is designed to prevent memory leaks:
//! - All allocations are tracked by Rust's ownership system
//! - No circular references between Rust and JavaScript
//! - Event callbacks are properly cleaned up on document drop
//! - String interning prevents unbounded growth of repeated strings
//!
//! # Usage
//!
//! ```javascript
//! import init, { WasmDocument } from 'rich-text-editor-wasm';
//!
//! // Initialize WASM module
//! await init();
//!
//! // Create a new document
//! const doc = new WasmDocument();
//!
//! // Insert text
//! doc.insertText("Hello World", 0);
//!
//! // Apply formatting
//! doc.applyFormat("bold", 0, 5);
//!
//! // Export to HTML
//! const html = doc.toHTML();
//!
//! // Clean up when done (optional but recommended for large documents)
//! doc.free();
//! ```

use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages in the browser console
///
/// This should be called once when the WASM module is loaded to ensure
/// that Rust panics are properly displayed in the browser console with
/// full stack traces.
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// Module declarations
pub mod bindings;
pub mod document;
pub mod formatting;
pub mod operations;
pub mod selection;
pub mod serialization;
pub mod utils;

// Re-export WasmDocument for JavaScript
pub use bindings::WasmDocument;
