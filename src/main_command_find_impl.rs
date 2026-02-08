//! Implementation of the find command (content search in hierarchical scope).
//!
//! This module maps to hierarchical name: main.command.find.impl

use recur::output::{JsonFormatter, TerminalFormatter};
use recur::parser::HierarchyPattern;
use recur::search::{read_paths_from_stdin, ContentSearcher, SearchOptions};
use std::path::{Path, PathBuf};
use std::process;

pub fn execute(
    query: String,
    scope: String,
    dir: PathBuf,
    context: usize,
    ignore_case: bool,
    use_regex: bool,
    ext: Option<String>,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let scope_pattern = HierarchyPattern::parse_with_separator(&scope, separator)?;
    let scope_pattern = if ignore_case {
        scope_pattern.case_insensitive()
    } else {
        scope_pattern
    };

    let mut options = SearchOptions {
        root: dir.clone(),
        case_insensitive: ignore_case,
        context_lines: context,
        ..Default::default()
    };

    if let Some(ext_str) = ext {
        options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    if stdin {
        options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    let searcher = ContentSearcher::new(options);

    let results = if use_regex {
        let regex = regex::Regex::new(&query)?;
        searcher.search_regex(&regex, &scope_pattern)
    } else {
        searcher.search(&query, &scope_pattern)
    };

    if json {
        let output = JsonFormatter::format_search_results(&results);
        println!("{}", output);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_search_results(&results);
    }

    if results.is_empty() {
        process::exit(1);
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
