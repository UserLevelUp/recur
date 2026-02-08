//! Implementation of the id command (content search by hierarchical identifier).
//!
//! This module maps to hierarchical name: main.command.id.impl

use recur::output::{JsonFormatter, TerminalFormatter};
use recur::parser::HierarchyPattern;
use recur::search::{read_paths_from_stdin, IdentifierSearcher, SearchOptions};
use std::path::{Path, PathBuf};
use std::process;

pub fn execute(
    pattern: String,
    dir: PathBuf,
    ext: Option<String>,
    context: usize,
    ignore_case: bool,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
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

    let searcher = IdentifierSearcher::new(options);
    let pattern_parsed = HierarchyPattern::parse_with_separator(&pattern, separator)?;
    let results = searcher.search(&pattern_parsed);

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
