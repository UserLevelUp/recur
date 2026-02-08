//! Implementation of the related command (standard file-list based).
//!
//! This module maps to hierarchical name: main.command.related.impl

use recur::output::{JsonFormatter, TerminalFormatter};
use recur::parser::HierarchyPattern;
use recur::search::{read_paths_from_stdin, FileSearcher, SearchOptions};
use std::path::{Path, PathBuf};
use std::process;

pub fn execute(
    filename: String,
    dir: PathBuf,
    exclude_self: bool,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let stem = filename
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(&filename);
    let base = stem
        .rsplit_once(separator)
        .map(|(parent, _)| parent)
        .unwrap_or(stem);
    let pattern =
        HierarchyPattern::parse_with_separator(&format!("{}{}*", base, separator), separator)?;

    let mut options = SearchOptions {
        root: dir.clone(),
        ..Default::default()
    };
    if stdin {
        options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    let searcher = FileSearcher::new(options);
    let mut files = searcher.find(&pattern);

    if exclude_self {
        let input_name = Path::new(&filename)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&filename);
        files.retain(|path| {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                file_name != input_name
            } else {
                true
            }
        });
    }

    if json {
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
