///! merge command implementation
///!
///! Merges hierarchical results from multiple pattern/separator pairs into unified view.
///! Follows Unix philosophy: explicit composition over automatic conversion.

use anyhow::Result;
use recur::parser::HierarchyPattern;
use recur::search::{FileSearcher, SearchOptions};
use recur::tree::HierarchyTree;
use std::collections::HashSet;
use std::path::PathBuf;

/// Execute merge command
pub fn execute(
    patterns: Vec<String>,
    separators: Vec<char>,
    dir: PathBuf,
    max_depth: Option<usize>,
    replace_default: Option<char>,
    show_sep: bool,
    unicode: bool,
    show_count: bool,
    json: bool,
) -> Result<()> {
    // Step 1: Collect files from all pattern/separator pairs
    let mut all_files: Vec<PathBuf> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut file_separators: std::collections::HashMap<PathBuf, char> =
        std::collections::HashMap::new();

    for (pattern, separator) in patterns.iter().zip(separators.iter()) {
        let files = find_files_for_pattern(pattern, *separator, &dir, max_depth)?;
        let count = files.len();
        let _ = count;
        for file in files {
            // Deduplicate: only add if not seen before
            if seen.insert(file.clone()) {
                all_files.push(file.clone());
                file_separators.insert(file, *separator);
            }
        }
    }

    // Step 2: Check if we found anything
    if all_files.is_empty() {
        println!("No files found");
        return Ok(());
    }

    // Step 3: Normalize paths (if requested) and display unified tree
    let tree_separator = replace_default.unwrap_or(separators[0]);
    let base_pattern = normalize_pattern_for_separator(&patterns[0], tree_separator);
    let show_markers = show_sep && separators.len() > 1;

    let tree_files: Vec<PathBuf> = all_files
        .iter()
        .map(|path| {
            let original_sep = file_separators.get(path).copied().unwrap_or(separators[0]);
            let mut display_path = if let Some(replace_sep) = replace_default {
                normalize_path_separator(path, original_sep, replace_sep)
            } else {
                path.clone()
            };

            if show_markers {
                if let Some(filename) = display_path.file_name() {
                    let marked_filename =
                        format!("{} [{}]", filename.to_string_lossy(), original_sep);
                    display_path.set_file_name(marked_filename);
                }
            }

            display_path
        })
        .collect();

    display_tree(
        &tree_files,
        &base_pattern,
        tree_separator,
        unicode,
        show_count,
        json,
    )?;

    Ok(())
}

/// Find files matching a specific pattern with specific separator
fn find_files_for_pattern(
    pattern: &str,
    separator: char,
    dir: &PathBuf,
    max_depth: Option<usize>,
) -> Result<Vec<PathBuf>> {
    // Normalize pattern to use the specified separator
    // E.g., "main.command.tree" with sep='_' → "main_command_tree"
    let normalized_pattern = normalize_pattern_for_separator(pattern, separator);

    // Create hierarchical pattern for searching
    // Add ".**" to match all descendants
    let pattern_str = format!("{}{}**", normalized_pattern, separator);
    let hier_pattern = HierarchyPattern::parse_with_separator(&pattern_str, separator)?;

    // Search for files
    let options = SearchOptions {
        root: dir.clone(),
        max_depth,
        ..Default::default()
    };

    let searcher = FileSearcher::new(options);
    let files = searcher.find(&hier_pattern);

    Ok(files)
}

/// Normalize pattern to use specific separator
fn normalize_pattern_for_separator(pattern: &str, target_separator: char) -> String {
    // Replace common separators with target separator
    let mut normalized = pattern.to_string();

    // Replace dots, underscores, dashes, slashes with target
    for source_sep in ['.', '_', '-', '/'] {
        if source_sep != target_separator {
            normalized = normalized.replace(source_sep, &target_separator.to_string());
        }
    }

    normalized
}

/// Normalize a file path's separator to a different character
///
/// Replaces the hierarchy separator in the filename (not directory path)
/// while preserving file extensions.
fn normalize_path_separator(path: &PathBuf, from_sep: char, to_sep: char) -> PathBuf {
    if from_sep == to_sep {
        return path.clone();
    }

    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
        let (base, ext) = filename
            .rsplit_once('.')
            .map(|(b, e)| (b, Some(e)))
            .unwrap_or((filename, None));

        let normalized_base = base.replace(from_sep, &to_sep.to_string());
        let normalized_filename = if let Some(e) = ext {
            format!("{}.{}", normalized_base, e)
        } else {
            normalized_base
        };

        let mut normalized_path = path.clone();
        normalized_path.set_file_name(normalized_filename);
        normalized_path
    } else {
        path.clone()
    }
}

/// Display merged tree
fn display_tree(
    files: &[PathBuf],
    base_pattern: &str,
    separator: char,
    unicode: bool,
    show_count: bool,
    json: bool,
) -> Result<()> {
    // Build hierarchical tree structure
    let tree = HierarchyTree::from_paths_with_separator(base_pattern, files, separator);

    // Display
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
