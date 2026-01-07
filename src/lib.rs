//! Library crate for `recur`.
//!
//! The binary (`src/main.rs`) calls into this crate.

pub mod parser;
pub mod search;
pub mod tree;
pub mod output;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}