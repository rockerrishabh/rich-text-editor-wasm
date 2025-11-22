//! Utilities module
//!
//! This module provides shared utility functionality used across
//! the editor implementation.
//!
//! # Responsibilities
//!
//! - IME (Input Method Editor) composition state management
//! - String interning for memory efficiency
//! - Other shared utility functions
//!
//! # Key Types
//!
//! - `CompositionState`: Manages IME composition sessions
//! - `StringInterner`: Deduplicates repeated strings (URLs, colors)

pub mod ime;
pub mod interner;
