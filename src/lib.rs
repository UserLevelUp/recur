//! Library crate for `recur`.
//!
//! The binary (`src/main.rs`) calls into this crate.

pub mod output;
pub mod parser;
pub mod project_config;
pub mod recur_lang_concurrent_ir;
pub mod recur_lang_ir;
pub mod search;
pub mod tree;
pub mod warp_bubble;
pub mod warp_evidence;
pub mod warp_policy;

// Traits for dogfooding hierarchical organization
#[path = "trait/mod.rs"]
pub mod r#trait;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
