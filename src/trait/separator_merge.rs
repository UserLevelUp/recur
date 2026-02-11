//! Multi-separator capability trait for merging results across separator domains.
//!
//! This trait provides the core functionality for commands that can accept multiple
//! --sep flags and merge hierarchical results from different separator domains
//! (e.g., docs with `.` + source with `_`) into a unified view.

use anyhow::Result;

/// Trait for commands that can merge results from multiple separator domains.
///
/// Commands implementing this trait can:
/// 1. Accept multiple --sep flags to query different hierarchies
/// 2. Merge results by logical path structure
/// 3. Normalize separator display with --sep-replace-default
/// 4. Show which separator was used with --show-sep
///
/// ## Example Usage
/// ```bash
/// recur tree "main" --sep "." --sep "_"
/// recur files "main.command.**" --sep "." --sep "_" --sep-replace-default "." --show-sep
/// ```
///
/// ## Merging Strategy
/// Results are merged by logical hierarchical path:
/// 1. Query with separator `.` finds `main.command.files.readme.md`
/// 2. Query with separator `_` finds `main_command_files_impl.rs`
/// 3. Both map to logical path `main → command → files`
/// 4. Merged output shows both files under the same logical node
pub trait MultiSeparatorCapable {
    /// Execute command with multiple separators and merge results.
    ///
    /// # Arguments
    /// * `separators` - List of separator characters to query (e.g., ['.', '_'])
    /// * `replace_default` - Optional separator to normalize all output paths
    /// * `show_sep` - Whether to append separator markers (e.g., "[.]", "[_]")
    ///
    /// # Returns
    /// Result indicating success or error
    ///
    /// # Behavior
    /// - Executes query with each separator independently
    /// - Merges results by logical hierarchical structure
    /// - If `replace_default` is Some, replaces actual separator with this in output
    /// - If `show_sep` is true, appends "[X]" marker showing actual separator used
    fn execute_with_separators(
        &self,
        separators: Vec<char>,
        replace_default: Option<char>,
        show_sep: bool,
    ) -> Result<()>;
}

// TODO: Implement for TreeCommand
// TODO: Implement for FilesCommand
// TODO: Add helper functions for:
//   - Merging hierarchical results by logical path
//   - Replacing separators in output
//   - Appending separator markers

#[cfg(test)]
mod tests {
    // Placeholder tests for when trait is implemented
    #[test]
    fn test_separator_merge_trait_exists() {
        // Trait compiles successfully
        assert!(true);
    }
}
