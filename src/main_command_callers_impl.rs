//! Implementation of the callers command (function caller search).
//!
//! This module maps to hierarchical name: main.command.callers.impl

use recur::output::{JsonFormatter, TerminalFormatter};
use recur::parser::HierarchyPattern;
use recur::r#trait::{apply_content_search_policy, read_resolved_paths_from_stdin};
use recur::search::{CallerSearcher, SearchOptions};
use std::path::PathBuf;
use std::process;

pub fn execute(
    function: String,
    scope: String,
    dir: PathBuf,
    context: usize,
    ignore_case: bool,
    ext: Option<String>,
    count_only: bool,
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

    apply_content_search_policy(&mut options, &dir)?;

    if stdin {
        options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    let searcher = CallerSearcher::new(options);
    let results = searcher.find_callers(&function, &scope_pattern)?;

    if count_only {
        println!("{}", results.len());
        if results.is_empty() {
            process::exit(1);
        }
        return Ok(());
    }

    if json {
        let output = JsonFormatter::format_caller_results(&results);
        println!("{}", output);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_caller_results(&results);
    }

    if results.is_empty() {
        process::exit(1);
    }

    Ok(())
}
