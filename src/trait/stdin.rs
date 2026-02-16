//! Stdin capability trait for reading and filtering file paths from stdin.
//!
//! This trait provides the core functionality for commands that can read file paths
//! from stdin (e.g., from git commands) and filter them by hierarchical patterns.

use crate::parser::{HierarchicalName, HierarchyPattern};
use crate::project_config;
use anyhow::Context;
use std::io::{stdin, BufRead};
use std::path::{Path, PathBuf};

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

/// Stdin path resolution policy from config.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StdinPathPolicy {
    pub exclude_missing: bool,
    pub resolve_relative_to_root: bool,
}

impl Default for StdinPathPolicy {
    fn default() -> Self {
        Self {
            exclude_missing: false,
            resolve_relative_to_root: true,
        }
    }
}

/// Resolve stdin path policy from `.recur/config.toml` (trait-aware).
pub fn resolve_stdin_path_policy(root: &Path) -> anyhow::Result<StdinPathPolicy> {
    let lookup_root = if root.is_absolute() {
        root.to_path_buf()
    } else if let Ok(cwd) = std::env::current_dir() {
        cwd.join(root)
    } else {
        root.to_path_buf()
    };

    let config = project_config::load_nearest(&lookup_root)?;
    let trait_stdin = config
        .as_ref()
        .and_then(|cfg| cfg.traits.as_ref())
        .and_then(|traits| traits.stdin.as_ref())
        .filter(|cfg| cfg.enabled.unwrap_or(true));

    let mut policy = StdinPathPolicy::default();
    if let Some(cfg) = trait_stdin {
        if let Some(exclude_missing) = cfg.exclude_missing {
            policy.exclude_missing = exclude_missing;
        }
        if let Some(resolve_relative) = cfg.resolve_relative_to_root {
            policy.resolve_relative_to_root = resolve_relative;
        }
    }

    Ok(policy)
}

/// Resolve stdin paths against root and policy.
pub fn resolve_stdin_paths(
    paths: Vec<PathBuf>,
    root: &Path,
    policy: StdinPathPolicy,
) -> Vec<PathBuf> {
    let mut resolved = Vec::new();

    for path in paths {
        if path.is_absolute() || path.exists() {
            resolved.push(path);
            continue;
        }

        if policy.resolve_relative_to_root && path.is_relative() {
            let candidate = root.join(&path);
            if candidate.exists() {
                resolved.push(candidate);
                continue;
            }
        }

        if !policy.exclude_missing {
            resolved.push(path);
        }
    }

    resolved
}

/// Read stdin file paths and resolve using project policy.
pub fn read_resolved_paths_from_stdin(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let policy = resolve_stdin_path_policy(root)?;
    let paths = read_paths_from_stdin()?;
    Ok(resolve_stdin_paths(paths, root, policy))
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
                    let hier_name =
                        HierarchicalName::with_separator(name_without_ext, pattern.separator);
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
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

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

    #[test]
    fn test_filter_stdin_paths_with_custom_separator() {
        let paths = vec![
            PathBuf::from("main_command_stats_impl.rs"),
            PathBuf::from("main_command_stats_stdin.rs"),
            PathBuf::from("main.command.stats.impl.rs"),
        ];

        let pattern = HierarchyPattern::parse_with_separator("main_command_*_impl", '_').unwrap();
        let filtered = TestCommand::filter_stdin_paths(paths, &pattern, None);

        assert_eq!(filtered.len(), 1);
        assert!(filtered[0]
            .to_str()
            .unwrap()
            .ends_with("main_command_stats_impl.rs"));
    }

    #[test]
    fn resolve_stdin_paths_keeps_missing_by_default() {
        let root = Path::new(".");
        let paths = vec![PathBuf::from("missing.file")];

        let resolved = resolve_stdin_paths(paths, root, StdinPathPolicy::default());
        assert_eq!(resolved, vec![PathBuf::from("missing.file")]);
    }

    #[test]
    fn resolve_stdin_paths_excludes_missing_when_enabled() {
        let root = Path::new(".");
        let paths = vec![PathBuf::from("missing.file")];
        let policy = StdinPathPolicy {
            exclude_missing: true,
            resolve_relative_to_root: true,
        };

        let resolved = resolve_stdin_paths(paths, root, policy);
        assert!(resolved.is_empty());
    }

    #[test]
    fn resolve_stdin_path_policy_reads_trait_settings() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".recur")).unwrap();
        fs::write(
            temp.path().join(".recur/config.toml"),
            r#"
[traits.stdin]
enabled = true
exclude_missing = true
resolve_relative_to_root = false
"#,
        )
        .unwrap();

        let policy = resolve_stdin_path_policy(temp.path()).unwrap();
        assert!(policy.exclude_missing);
        assert!(!policy.resolve_relative_to_root);
    }

    #[test]
    fn resolve_stdin_path_policy_uses_defaults_when_trait_disabled() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".recur")).unwrap();
        fs::write(
            temp.path().join(".recur/config.toml"),
            r#"
[traits.stdin]
enabled = false
exclude_missing = true
resolve_relative_to_root = false
"#,
        )
        .unwrap();

        let policy = resolve_stdin_path_policy(temp.path()).unwrap();
        assert_eq!(policy, StdinPathPolicy::default());
    }
}
