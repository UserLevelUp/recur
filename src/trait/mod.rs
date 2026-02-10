//! Trait modules for command capabilities.
//!
//! This module contains traits that define common capabilities for commands,
//! enabling code reuse and consistent behavior across the codebase.

pub mod content_search;
pub mod separator_merge;
pub mod stdin;

// Re-export commonly used items
pub use content_search::ContentSearchCapable;
pub use separator_merge::MultiSeparatorCapable;
pub use stdin::{read_paths_from_stdin, StdinCapable};
