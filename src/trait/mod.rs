//! Trait modules for command capabilities.
//!
//! This module contains traits that define common capabilities for commands,
//! enabling code reuse and consistent behavior across the codebase.

pub mod content_search;
pub mod separator_merge;
pub mod stdin;
pub mod traversal_budget;

// Re-export commonly used items
pub use content_search::ContentSearchCapable;
pub use separator_merge::MultiSeparatorCapable;
pub use stdin::{
    read_paths_from_stdin, read_resolved_paths_from_stdin, resolve_stdin_path_policy,
    resolve_stdin_paths, StdinCapable, StdinPathPolicy,
};
pub use traversal_budget::{
    parse_depth_budget_mode, resolve_depth_budget_policy, DepthBudgetMode, DepthBudgetPolicy,
    DepthBudgetResult, TraversalBudgetCapable,
};
