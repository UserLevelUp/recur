//! Stdin capability trait for reading and filtering file paths from stdin.
//!
//! This trait provides the core functionality for commands that can read file paths
//! from stdin (e.g., from git commands) and filter them by hierarchical patterns.

use crate::parser::{HierarchicalName, HierarchyPattern};
use anyhow::Context;
use std::io::{stdin, BufRead};
use std::path::PathBuf;

/// Read file paths from stdin (one per line)
///
/// Used for Git integration and Unix pipelines:
/// ```bash
/// git diff --name-only | recur files "**" --stdin
/// git ls-files | recur tree "Module" --stdin
/// ```
///
/// This function reads paths from stdin, one per line, trimming whitespace
/// and filtering out empty lines.
pub fn read_paths_from_stdin() -> anyhow::Result<Vec<PathBuf>> {
    let stdin = stdin();
    let mut paths = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.context("Failed to read line from stdin")?;
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            paths.push(PathBuf::from(trimmed));
        }
    }

    Ok(paths)
}

/// Trait for commands that can read from stdin and filter by hierarchical patterns.
///
/// Commands implementing this trait can process file paths from stdin instead of
/// scanning the filesystem, enabling integration with Git and other Unix tools.
///
/// # Example
/// ```rust,ignore
/// struct FilesCommand;
///
/// impl StdinCapable for FilesCommand {}
///
/// // Use the trait's filter_stdin_paths method
/// let stdin_paths = read_paths_from_stdin()?;
/// let filtered = FilesCommand::filter_stdin_paths(
///     stdin_paths,
///     &pattern,
///     Some(&extensions),
/// );
/// ```
pub trait StdinCapable {
    /// Filter stdin paths by a hierarchical pattern and optional extensions.
    ///
    /// This method takes a list of paths (typically from stdin), extracts the
    /// hierarchical name from each filename (removing the extension), and matches
    /// it against the provided pattern.
    ///
    /// # Arguments
    /// * `paths` - List of file paths to filter
    /// * `pattern` - Hierarchical pattern to match against
    /// * `extensions` - Optional list of file extensions to filter by (e.g., ["rs", "toml"])
    ///
    /// # Returns
    /// Filtered list of paths that match the pattern and extension criteria
    ///
    /// # Example
    /// ```rust,ignore
    /// let pattern = HierarchyPattern::parse("Module.**")?;
    /// let extensions = vec!["cs".to_string(), "json".to_string()];
    /// let filtered = MyCommand::filter_stdin_paths(
    ///     paths,
    ///     &pattern,
    ///     Some(&extensions),
    /// );
    /// ```
    fn filter_stdin_paths(
        paths: Vec<PathBuf>,
        pattern: &HierarchyPattern,
        extensions: Option<&[String]>,
    ) -> Vec<PathBuf> {
        paths
            .into_iter()
            .filter(|p| {
                // Extract hierarchical name from filename (remove extension)
                if let Some(filename) = p.file_name().and_then(|n| n.to_str()) {
                    let name_without_ext = filename
                        .rsplit_once('.')
                        .map(|(name, _)| name)
                        .unwrap_or(filename);
                    let hier_name = HierarchicalName::new(name_without_ext);
                    pattern.matches(&hier_name)
                } else {
                    false
                }
            })
            .filter(|p| {
                // Apply extension filter if specified
                if let Some(exts) = extensions {
                    if let Some(file_ext) = p.extension().and_then(|e| e.to_str()) {
                        exts.iter().any(|e| {
                            let e = e.trim_start_matches('.');
                            file_ext == e
                        })
                    } else {
                        false
                    }
                } else {
                    true
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct TestCommand;
    impl StdinCapable for TestCommand {}

    #[test]
    fn test_filter_stdin_paths_basic() {
        let paths = vec![
            PathBuf::from("Module.Feature.cs"),
            PathBuf::from("Module.Other.cs"),
            PathBuf::from("Different.cs"),
        ];

        let pattern = HierarchyPattern::parse("Module.*").unwrap();
        let filtered = TestCommand::filter_stdin_paths(paths, &pattern, None);

        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_stdin_paths_with_extensions() {
        let paths = vec![
            PathBuf::from("Module.Feature.cs"),
            PathBuf::from("Module.Feature.json"),
            PathBuf::from("Module.Feature.txt"),
        ];

        let pattern = HierarchyPattern::parse("Module.*").unwrap();
        let extensions = vec!["cs".to_string(), "json".to_string()];
        let filtered = TestCommand::filter_stdin_paths(paths, &pattern, Some(&extensions));

        assert_eq!(filtered.len(), 2);
        assert!(filtered
            .iter()
            .any(|p| p.to_str().unwrap().ends_with(".cs")));
        assert!(filtered
            .iter()
            .any(|p| p.to_str().unwrap().ends_with(".json")));
        assert!(!filtered
            .iter()
            .any(|p| p.to_str().unwrap().ends_with(".txt")));
    }

    #[test]
    fn test_filter_stdin_paths_recursive_pattern() {
        let paths = vec![
            PathBuf::from("Module.cs"),
            PathBuf::from("Module.Sub.cs"),
            PathBuf::from("Module.Sub.Deep.cs"),
            PathBuf::from("Other.cs"),
        ];

        let pattern = HierarchyPattern::parse("Module.**").unwrap();
        let filtered = TestCommand::filter_stdin_paths(paths, &pattern, None);

        assert_eq!(filtered.len(), 3);
    }
}
