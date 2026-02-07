//! Stdin helpers for the stats command.
//!
//! This module maps to hierarchical name: main.command.stats.stdin

use recur::parser::HierarchyPattern;
use recur::r#trait::{read_paths_from_stdin, StdinCapable};
use std::path::{Path, PathBuf};

struct StatsStdin;

impl StdinCapable for StatsStdin {}

/// Collect and filter stdin file paths for the stats command.
pub fn collect_files_from_stdin(
    root: &Path,
    pattern: &HierarchyPattern,
    ext: Option<&str>,
) -> anyhow::Result<Vec<PathBuf>> {
    let stdin_paths = read_resolved_paths_from_stdin(root)?;
    let extensions = ext.map(parse_extensions);

    Ok(StatsStdin::filter_stdin_paths(
        stdin_paths,
        pattern,
        extensions.as_deref(),
    ))
}

fn parse_extensions(ext: &str) -> Vec<String> {
    ext.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn read_resolved_paths_from_stdin(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut resolved = Vec::new();

    for path in read_paths_from_stdin()? {
        if path.is_absolute() || path.exists() {
            resolved.push(path);
            continue;
        }

        if path.is_relative() {
            let candidate = root.join(&path);
            if candidate.exists() {
                resolved.push(candidate);
                continue;
            }
        }

        resolved.push(path);
    }

    Ok(resolved)
}
