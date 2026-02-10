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

    // Build tree using first separator as default (or replace_default if specified)
    let tree_separator = replace_default.unwrap_or(separators[0]);
    let tree = HierarchyTree::from_paths_with_separator(base, &all_files, tree_separator);

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

    // TODO: Implement show_sep to display separator markers

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
    let pattern =
        HierarchyPattern::parse_with_separator(&format!("{}{}**", base, separator), separator)?;

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
