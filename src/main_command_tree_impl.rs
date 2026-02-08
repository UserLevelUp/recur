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
