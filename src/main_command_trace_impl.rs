//! Implementation of the trace command (multi-level call graph).
//!
//! This module maps to hierarchical name: main.command.trace.impl

use recur::output::{JsonFormatter, TerminalFormatter, TraceFormat};
use recur::parser::HierarchyPattern;
use recur::search::{
    read_paths_from_stdin, SearchOptions, TraceDirection, TraceOptions, TraceSearcher,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process;

pub fn execute(
    function: String,
    scope: String,
    dir: PathBuf,
    depth: usize,
    direction_str: String,
    ignore_case: bool,
    ext: Option<String>,
    max_width: usize,
    verbose: bool,
    format_str: String,
    pick: Option<usize>,
    scope_alias: Vec<String>,
    stdin: bool,
    force: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    if depth > 5 && !force {
        anyhow::bail!("Maximum depth is 5 (to prevent exponential explosion)");
    }

    let direction = match direction_str.to_lowercase().as_str() {
        "callees" => TraceDirection::Callees,
        "callers" => TraceDirection::Callers,
        "both" => TraceDirection::Both,
        _ => anyhow::bail!(
            "Invalid direction '{}'. Must be 'callees', 'callers', or 'both'",
            direction_str
        ),
    };

    let output_format = match format_str.to_lowercase().as_str() {
        "tree" => TraceFormat::Tree,
        "flat" => TraceFormat::Flat,
        "graph" => TraceFormat::Graph,
        _ => anyhow::bail!(
            "Invalid format '{}'. Must be 'tree', 'flat', or 'graph'",
            format_str
        ),
    };

    let resolved_scope = apply_scope_alias(&scope, &scope_alias)?;

    let scope_pattern = HierarchyPattern::parse_with_separator(&resolved_scope, separator)?;
    let scope_pattern = if ignore_case {
        scope_pattern.case_insensitive()
    } else {
        scope_pattern
    };

    let mut search_options = SearchOptions {
        root: dir.clone(),
        case_insensitive: ignore_case,
        ..Default::default()
    };

    if let Some(ext_str) = ext.as_deref() {
        search_options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    if stdin {
        search_options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    let trace_options = TraceOptions {
        max_width,
        verbose,
        pick,
    };

    if direction == TraceDirection::Both {
        let mut caller_searcher = TraceSearcher::new(search_options.clone(), trace_options.clone());
        let callers_result =
            caller_searcher.trace(&function, &scope_pattern, TraceDirection::Callers, depth)?;

        let mut callee_searcher = TraceSearcher::new(search_options, trace_options);
        let callees_result =
            callee_searcher.trace(&function, &scope_pattern, TraceDirection::Callees, depth)?;

        if json {
            let output = JsonFormatter::format_trace_result_both(&callers_result, &callees_result);
            println!("{}", output);
            if callees_result.root.path.as_os_str().is_empty() {
                process::exit(1);
            }
            return Ok(());
        } else {
            if callees_result.root.path.as_os_str().is_empty() {
                print_trace_not_found(&function, &resolved_scope, ext.as_deref());
                process::exit(1);
            }

            let mut formatter = TerminalFormatter::new(color);
            formatter.print_trace_both(&callers_result, &callees_result, verbose)?;
        }

        return Ok(());
    }

    let mut searcher = TraceSearcher::new(search_options, trace_options);
    let trace_result = searcher.trace(&function, &scope_pattern, direction, depth)?;

    if json {
        let output = JsonFormatter::format_trace_result(&trace_result);
        println!("{}", output);
        if trace_result.root.path.as_os_str().is_empty() {
            process::exit(1);
        }
        return Ok(());
    }

    if trace_result.root.path.as_os_str().is_empty() {
        print_trace_not_found(&function, &resolved_scope, ext.as_deref());
        process::exit(1);
    }

    let mut formatter = TerminalFormatter::new(color);
    formatter.print_trace_result(&trace_result, output_format, verbose)?;

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

fn apply_scope_alias(scope: &str, aliases: &[String]) -> anyhow::Result<String> {
    if aliases.is_empty() {
        return Ok(scope.to_string());
    }

    let mut map = HashMap::new();
    for alias in aliases {
        let Some((key, value)) = alias.split_once('=') else {
            anyhow::bail!(
                "Invalid --scope-alias '{}'. Expected format name=pattern",
                alias
            );
        };
        map.insert(key.trim(), value.trim());
    }

    if let Some(replacement) = map.get(scope) {
        Ok((*replacement).to_string())
    } else {
        Ok(scope.to_string())
    }
}

fn print_trace_not_found(function: &str, scope: &str, ext: Option<&str>) {
    println!("No symbols found for '{}'.", function);
    if let Some(ext) = ext {
        println!(
            "Hint: if this is a string reference, try: recur find \"{}\" --scope \"{}\" --ext {}",
            function, scope, ext
        );
    } else {
        println!(
            "Hint: if this is a string reference, try: recur find \"{}\" --scope \"{}\"",
            function, scope
        );
    }
}
