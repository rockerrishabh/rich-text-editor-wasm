//! Formatting module
//!
//! This module manages inline and block-level text formatting.
//! It provides efficient storage and querying of format information
//! using a run-based approach.
//!
//! # Responsibilities
//!
//! - Define inline format types (bold, italic, links, colors, etc.)
//! - Define block types (paragraphs, headings, lists, quotes, code blocks)
//! - Store and query format information efficiently
//! - Adjust formats when text is inserted or deleted
//! - Merge adjacent format runs with identical formatting
//!
//! # Key Types
//!
//! - `InlineFormat`: Enum representing inline formatting options
//! - `BlockType`: Enum representing block-level structure types
//! - `FormatStorage`: Run-based storage for efficient format queries
//! - `FormatRun`: A contiguous range of text with the same formatting

pub mod block;
pub mod inline;
pub mod storage;

// Re-export commonly used types
pub use block::BlockType;
pub use inline::InlineFormat;
pub use storage::{FormatRun, FormatStorage};
