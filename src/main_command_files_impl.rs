//! Implementation of the files command (standard file-list based).
//!
//! This module maps to hierarchical name: main.command.files.impl

use crate::main_command_files_stdin;
use recur::output::{JsonFormatter, TerminalFormatter};
use recur::parser::HierarchyPattern;
use recur::search::{FileSearcher, SearchOptions};
use std::path::PathBuf;
use std::process;

pub fn execute(
    pattern: String,
    dir: PathBuf,
    ext: Option<String>,
    ignore_case: bool,
    min_depth: usize,
    max_depth: Option<usize>,
    count_only: bool,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    // For now, delegate to single-separator implementation
    execute_single_separator(
        pattern,
        dir,
        ext,
        ignore_case,
        min_depth,
        max_depth,
        count_only,
        stdin,
        separator,
        json,
        color,
    )
}

/// Execute files command with multiple separators support
pub fn execute_with_separators(
    pattern: String,
    dir: PathBuf,
    ext: Option<String>,
    ignore_case: bool,
    min_depth: usize,
    max_depth: Option<usize>,
    count_only: bool,
    stdin: bool,
    separators: Vec<char>,
    replace_default: Option<char>,
    show_sep: bool,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    if let Some(max) = max_depth {
        if min_depth > max {
            anyhow::bail!(
                "--min-depth ({}) cannot be greater than --max-depth ({})",
                min_depth,
                max
            );
        }
    }

    // If only one separator, use simple path
    if separators.len() == 1 && replace_default.is_none() && !show_sep {
        return execute_single_separator(
            pattern,
            dir,
            ext,
            ignore_case,
            min_depth,
            max_depth,
            count_only,
            stdin,
            separators[0],
            json,
            color,
        );
    }

    // Multi-separator: collect files from all separators
    let mut all_files = Vec::new();
    let mut file_separators: std::collections::HashMap<PathBuf, char> =
        std::collections::HashMap::new();

    for sep in &separators {
        let files = find_files_for_separator(
            &pattern,
            &dir,
            ext.as_deref(),
            ignore_case,
            min_depth,
            max_depth,
            stdin,
            *sep,
        )?;

        for file in files {
            file_separators.insert(file.clone(), *sep);
            all_files.push(file);
        }
    }

    // TODO: Apply replace_default to normalize paths
    // TODO: Apply show_sep to show separator markers

    if count_only {
        println!("{} files", all_files.len());
    } else if json {
        let output = JsonFormatter::format_file_list(&all_files);
        println!("{}", output);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_file_list(&all_files);
    }

    if all_files.is_empty() {
        process::exit(1);
    }

    Ok(())
}

/// Find files for a specific separator
fn find_files_for_separator(
    pattern_str: &str,
    dir: &PathBuf,
    ext: Option<&str>,
    ignore_case: bool,
    min_depth: usize,
    max_depth: Option<usize>,
    stdin: bool,
    separator: char,
) -> anyhow::Result<Vec<PathBuf>> {
    let pattern = HierarchyPattern::parse_with_separator(pattern_str, separator)?;
    let pattern = if ignore_case {
        pattern.case_insensitive()
    } else {
        pattern
    };

    let all_files = if stdin {
        main_command_files_stdin::collect_files_from_stdin(dir, &pattern, ext)?
    } else {
        let mut options = SearchOptions {
            root: dir.clone(),
            case_insensitive: ignore_case,
            max_depth,
            ..Default::default()
        };

        if let Some(ext_str) = ext {
            options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
        }

        let searcher = FileSearcher::new(options);
        searcher.find(&pattern)
    };

    Ok(filter_by_min_depth(all_files, &pattern, min_depth))
}

fn execute_single_separator(
    pattern: String,
    dir: PathBuf,
    ext: Option<String>,
    ignore_case: bool,
    min_depth: usize,
    max_depth: Option<usize>,
    count_only: bool,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    if let Some(max) = max_depth {
        if min_depth > max {
            anyhow::bail!(
                "--min-depth ({}) cannot be greater than --max-depth ({})",
                min_depth,
                max
            );
        }
    }

    let pattern = HierarchyPattern::parse_with_separator(&pattern, separator)?;
    let pattern = if ignore_case {
        pattern.case_insensitive()
    } else {
        pattern
    };

    let all_files = if stdin {
        main_command_files_stdin::collect_files_from_stdin(&dir, &pattern, ext.as_deref())?
    } else {
        let mut options = SearchOptions {
            root: dir,
            case_insensitive: ignore_case,
            max_depth,
            ..Default::default()
        };

        if let Some(ext_str) = ext.as_deref() {
            options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
        }

        let searcher = FileSearcher::new(options);
        searcher.find(&pattern)
    };

    let files = filter_by_min_depth(all_files, &pattern, min_depth);

    if count_only {
        println!("{} files", files.len());
    } else if json {
        let output = JsonFormatter::format_file_list(&files);
        println!("{}", output);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_file_list(&files);
    }

    if files.is_empty() {
        process::exit(1);
    }

    Ok(())
}

fn filter_by_min_depth(
    files: Vec<PathBuf>,
    pattern: &HierarchyPattern,
    min_depth: usize,
) -> Vec<PathBuf> {
    let base_depth = pattern.raw.matches(pattern.separator).count();

    files
        .into_iter()
        .filter(|path| {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                let hier_name = filename
                    .rsplit_once('.')
                    .map(|(name, _)| name)
                    .unwrap_or(filename);
                let file_depth = hier_name.matches(pattern.separator).count();
                let relative_depth = file_depth.saturating_sub(base_depth);
                relative_depth >= min_depth
            } else {
                false
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use recur::parser::HierarchyPattern;

    #[test]
    fn filter_by_min_depth_respects_relative_depth() {
        let pattern = HierarchyPattern::parse("main.**").unwrap();
        let files = vec![
            PathBuf::from("main.rs"),
            PathBuf::from("main.command.rs"),
            PathBuf::from("main.command.files.rs"),
        ];

        let filtered = filter_by_min_depth(files, &pattern, 1);
        assert_eq!(filtered.len(), 1);
        assert!(filtered
            .iter()
            .any(|p| p.ends_with("main.command.files.rs")));
    }
}
