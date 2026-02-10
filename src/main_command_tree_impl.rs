//! Implementation of the tree command (hierarchy visualization).
//!
//! This module maps to hierarchical name: main.command.tree.impl

use recur::parser::{HierarchicalName, HierarchyPattern};
use recur::search::{read_paths_from_stdin, FileSearcher, SearchOptions};
use recur::tree::HierarchyTree;
use std::path::{Path, PathBuf};
use std::process;

pub fn execute(
    base: String,
    dir: PathBuf,
    max_depth: Option<usize>,
    show_count: bool,
    unicode: bool,
    stdin: bool,
    separator: char,
    json: bool,
) -> anyhow::Result<()> {
    // For now, delegate to single-separator implementation
    execute_single_separator(
        base,
        dir,
        max_depth,
        show_count,
        unicode,
        stdin,
        separator,
        json,
    )
}

/// Execute tree command with multiple separators support
pub fn execute_with_separators(
    base: String,
    dir: PathBuf,
    max_depth: Option<usize>,
    show_count: bool,
    unicode: bool,
    stdin: bool,
    separators: Vec<char>,
    replace_default: Option<char>,
    show_sep: bool,
    json: bool,
) -> anyhow::Result<()> {
    // If only one separator, use simple path
    if separators.len() == 1 && replace_default.is_none() && !show_sep {
        return execute_single_separator(
            base,
            dir,
            max_depth,
            show_count,
            unicode,
            stdin,
            separators[0],
            json,
        );
    }

    // Multi-separator: collect files from all separators
    let mut all_files = Vec::new();
    let mut file_separators: std::collections::HashMap<PathBuf, char> =
        std::collections::HashMap::new();

    for sep in &separators {
        let files = find_files_for_separator(&base, &dir, max_depth, stdin, *sep)?;

        for file in files {
            file_separators.insert(file.clone(), *sep);
            all_files.push(file);
        }
    }

    if all_files.is_empty() {
        eprintln!("No files found starting with '{}'", base);
        process::exit(1);
    }

    // Normalize paths to tree separator for consistent hierarchy
    let tree_separator = replace_default.unwrap_or(separators[0]);
    let normalized_files: Vec<PathBuf> = all_files
        .iter()
        .map(|path| {
            let original_sep = file_separators.get(path).copied().unwrap_or(separators[0]);
            normalize_path_separator(path, original_sep, tree_separator)
        })
        .collect();

    // Apply separator markers if requested (only for multi-separator queries)
    let show_markers = show_sep && separators.len() > 1;
    let tree_files: Vec<PathBuf> = if show_markers {
        normalized_files
            .iter()
            .map(|path| {
                let original_path = all_files
                    .iter()
                    .find(|p| {
                        normalize_path_separator(
                            p,
                            file_separators.get(&**p).copied().unwrap_or(separators[0]),
                            tree_separator,
                        ) == *path
                    })
                    .unwrap_or(path);

                let sep = file_separators.get(&**original_path).copied().unwrap_or(separators[0]);

                if let Some(filename) = path.file_name() {
                    let marked_filename = format!("{} [{}]", filename.to_string_lossy(), sep);
                    let mut marked_path = path.clone();
                    marked_path.set_file_name(marked_filename);
                    marked_path
                } else {
                    path.clone()
                }
            })
            .collect()
    } else {
        normalized_files.clone()
    };

    let tree = HierarchyTree::from_paths_with_separator(base, &tree_files, tree_separator);

    if json {
        println!("{}", tree.to_json());
    } else {
        print!("{}", tree.to_string(unicode));

        if show_count {
            let stats = tree.stats();
            println!(
                "\n{} files, {} directories (recursive)",
                stats.total_files, stats.total_dirs
            );
        }
    }

    Ok(())
}

/// Find files for a specific separator
fn find_files_for_separator(
    base: &str,
    dir: &PathBuf,
    max_depth: Option<usize>,
    stdin: bool,
    separator: char,
) -> anyhow::Result<Vec<PathBuf>> {
    let normalized_base = normalize_pattern_for_separator(base, separator);
    let pattern = HierarchyPattern::parse_with_separator(
        &format!("{}{}**", normalized_base, separator),
        separator,
    )?;

    if stdin {
        Ok(read_resolved_paths_from_stdin(dir)?
            .into_iter()
            .filter(|p| {
                if let Some(filename) = p.file_name().and_then(|n| n.to_str()) {
                    let name_without_ext = filename
                        .rsplit_once('.')
                        .map(|(name, _)| name)
                        .unwrap_or(filename);
                    let hier_name = HierarchicalName::with_separator(name_without_ext, separator);
                    pattern.matches(&hier_name)
                } else {
                    false
                }
            })
            .collect())
    } else {
        let options = SearchOptions {
            root: dir.clone(),
            max_depth,
            ..Default::default()
        };

        let searcher = FileSearcher::new(options);
        Ok(searcher.find(&pattern))
    }
}

fn normalize_pattern_for_separator(pattern: &str, target_separator: char) -> String {
    let mut normalized = pattern.to_string();

    for source_sep in ['.', '_', '-', '/'] {
        if source_sep != target_separator {
            normalized = normalized.replace(source_sep, &target_separator.to_string());
        }
    }

    normalized
}

fn execute_single_separator(
    base: String,
    dir: PathBuf,
    max_depth: Option<usize>,
    show_count: bool,
    unicode: bool,
    stdin: bool,
    separator: char,
    json: bool,
) -> anyhow::Result<()> {
    // Find all files starting with base (recursive).
    let pattern =
        HierarchyPattern::parse_with_separator(&format!("{}{}**", base, separator), separator)?;

    let files = if stdin {
        // Read paths from stdin and filter by pattern.
        read_resolved_paths_from_stdin(&dir)?
            .into_iter()
            .filter(|p| {
                if let Some(filename) = p.file_name().and_then(|n| n.to_str()) {
                    let name_without_ext = filename
                        .rsplit_once('.')
                        .map(|(name, _)| name)
                        .unwrap_or(filename);
                    let hier_name = HierarchicalName::with_separator(name_without_ext, separator);
                    pattern.matches(&hier_name)
                } else {
                    false
                }
            })
            .collect()
    } else {
        let options = SearchOptions {
            root: dir,
            max_depth,
            ..Default::default()
        };

        let searcher = FileSearcher::new(options);
        searcher.find(&pattern)
    };

    if files.is_empty() {
        eprintln!("No files found starting with '{}'", base);
        process::exit(1);
    }

    let tree = HierarchyTree::from_paths_with_separator(base, &files, separator);

    if json {
        println!("{}", tree.to_json());
    } else {
        print!("{}", tree.to_string(unicode));

        if show_count {
            let stats = tree.stats();
            println!(
                "\n{} files, {} directories (recursive)",
                stats.total_files, stats.total_dirs
            );
        }
    }

    Ok(())
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

/// Normalize a file path's separator to a different character
///
/// Replaces the hierarchy separator in the filename (not directory path)
/// while preserving file extensions.
///
/// Example:
///   main_command_files_impl.rs -> main.command.files.impl.rs
///   (when normalizing '_' to '.')
fn normalize_path_separator(path: &PathBuf, from_sep: char, to_sep: char) -> PathBuf {
    // If separators are the same, no normalization needed
    if from_sep == to_sep {
        return path.clone();
    }

    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
        // Split into base name and extension
        let (base, ext) = filename
            .rsplit_once('.')
            .map(|(b, e)| (b, Some(e)))
            .unwrap_or((filename, None));

        // Replace separator in base name
        let normalized_base = base.replace(from_sep, &to_sep.to_string());

        // Reconstruct filename with extension
        let normalized_filename = if let Some(e) = ext {
            format!("{}.{}", normalized_base, e)
        } else {
            normalized_base
        };

        // Reconstruct path with normalized filename
        let mut normalized_path = path.clone();
        normalized_path.set_file_name(normalized_filename);
        normalized_path
    } else {
        path.clone()
    }
}
