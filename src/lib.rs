//! Library crate for `recur`.
//!
//! The binary (`src/main.rs`) calls into this crate.

pub mod capability_traits;
pub mod output;
pub mod parser;
pub mod project_config;
pub mod prompt;
mod prompt_context;
pub mod recur_lang_concurrent_ir;
pub mod recur_lang_graph;
pub mod recur_lang_ir;
pub mod recur_lang_query;
pub mod reveal_artifact;
pub mod search;
pub mod tree;
pub mod warp_bubble;
pub mod warp_discovery;
pub mod warp_evidence;
pub mod warp_policy;
#[path = "main_command_warp_impl.rs"]
pub mod warp_query;

// Traits for dogfooding hierarchical organization
#[path = "trait/mod.rs"]
pub mod r#trait;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
