//! Stdin helpers for the files command.
//!
//! This module maps to hierarchical name: main.command.files.stdin

use recur::parser::HierarchyPattern;
use recur::r#trait::{read_paths_from_stdin, StdinCapable};
use std::path::{Path, PathBuf};

struct FilesStdin;

impl StdinCapable for FilesStdin {}

/// Collect and filter stdin file paths for the files command.
pub fn collect_files_from_stdin(
    root: &Path,
    pattern: &HierarchyPattern,
    ext: Option<&str>,
) -> anyhow::Result<Vec<PathBuf>> {
    let stdin_paths = read_resolved_paths_from_stdin(root)?;
    let extensions = ext.map(parse_extensions);

    Ok(FilesStdin::filter_stdin_paths(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_extensions_trims_and_skips_empty() {
        let parsed = parse_extensions("rs, md, ,toml");
        assert_eq!(parsed, vec!["rs", "md", "toml"]);
    }
}
