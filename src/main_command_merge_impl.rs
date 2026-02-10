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
    unicode: bool,
    show_count: bool,
    json: bool,
) -> Result<()> {
    // Step 1: Collect files from all pattern/separator pairs
    let mut all_files: Vec<PathBuf> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut pattern_counts: Vec<(String, usize)> = Vec::new();

    for (pattern, separator) in patterns.iter().zip(separators.iter()) {
        let files = find_files_for_pattern(pattern, *separator, &dir, max_depth)?;
        let count = files.len();
        pattern_counts.push((format!("{} [{}]", pattern, separator), count));

        eprintln!("Pattern '{}' with separator '{}': {} files found", pattern, separator, count);
        if count > 0 && count < 20 {
            eprintln!("  Sample files:");
            for (i, file) in files.iter().enumerate().take(5) {
                eprintln!("    {}: {}", i+1, file.display());
            }
        }

        for file in files {
            // Deduplicate: only add if not seen before
            if seen.insert(file.clone()) {
                all_files.push(file);
            }
        }
    }

    eprintln!("Total unique files after merge: {}", all_files.len());
    eprintln!("");

    // Step 2: Check if we found anything
    if all_files.is_empty() {
        println!("No files found");
        return Ok(());
    }

    // Step 3: Build and display unified tree
    // Use first pattern as base name for display
    let base_pattern = &patterns[0];
    // Use first separator as canonical form
    let canonical_separator = separators[0];

    display_tree(
        &all_files,
        base_pattern,
        canonical_separator,
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
