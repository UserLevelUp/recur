//! Implementation of the children command (standard file-list based).
//!
//! This module maps to hierarchical name: main.command.children.impl

use recur::output::{JsonFormatter, TerminalFormatter};
use recur::parser::HierarchyPattern;
use recur::search::{read_paths_from_stdin, FileSearcher, SearchOptions};
use std::path::{Path, PathBuf};
use std::process;

pub fn execute(
    parent: String,
    dir: PathBuf,
    count_only: bool,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let pattern =
        HierarchyPattern::parse_with_separator(&format!("{}{}**", parent, separator), separator)?;

    let mut options = SearchOptions {
        root: dir.clone(),
        ..Default::default()
    };
    if stdin {
        options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    let searcher = FileSearcher::new(options);
    let files = searcher.find(&pattern);

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
