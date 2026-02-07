//! Implementation of the stats command (standard file-list based).
//!
//! This module maps to hierarchical name: main.command.stats.impl

use crate::main_command_stats_stdin;
use recur::parser::HierarchyPattern;
use recur::search::{FileSearcher, SearchOptions};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn execute(
    pattern: String,
    dir: PathBuf,
    level: Option<usize>,
    ext: Option<String>,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let pattern_parsed = HierarchyPattern::parse_with_separator(&pattern, separator)?;

    let files = if stdin {
        main_command_stats_stdin::collect_files_from_stdin(&dir, &pattern_parsed, ext.as_deref())?
    } else {
        let mut options = SearchOptions {
            root: dir,
            ..Default::default()
        };

        if let Some(ext_str) = ext.as_deref() {
            options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
        }

        let searcher = FileSearcher::new(options);
        searcher.find(&pattern_parsed)
    };

    if files.is_empty() {
        if !json {
            eprintln!("No files found matching pattern: {}", pattern);
        }
        process::exit(1);
    }

    let base_depth = pattern.matches(separator).count();
    let mut depth_map: HashMap<usize, Vec<(PathBuf, usize)>> = HashMap::new();
    let mut total_lines = 0;

    for file_path in &files {
        if let Some(filename) = file_path.file_name().and_then(|n| n.to_str()) {
            let hier_name = filename
                .rsplit_once('.')
                .map(|(name, _)| name)
                .unwrap_or(filename);

            let file_depth = hier_name.matches(separator).count();
            let relative_depth = file_depth.saturating_sub(base_depth);

            let line_count = if let Ok(file) = fs::File::open(file_path) {
                let reader = std::io::BufReader::new(file);
                let count = reader.lines().count();
                total_lines += count;
                count
            } else {
                0
            };

            depth_map
                .entry(relative_depth)
                .or_default()
                .push((file_path.clone(), line_count));
        }
    }

    let max_depth = *depth_map.keys().max().unwrap_or(&0);

    if json {
        let output = serde_json::json!({
            "pattern": pattern,
            "total_files": files.len(),
            "total_lines": total_lines,
            "max_depth": max_depth,
            "depth_breakdown": (0..=max_depth).map(|d| {
                let files_at_depth = depth_map.get(&d).map(|v| v.len()).unwrap_or(0);
                serde_json::json!({
                    "depth": d,
                    "file_count": files_at_depth
                })
            }).collect::<Vec<_>>(),
            "files": if let Some(lvl) = level {
                depth_map.get(&lvl).map(|files_at_level| {
                    files_at_level.iter().map(|(path, lines)| {
                        serde_json::json!({
                            "path": path.display().to_string(),
                            "lines": lines
                        })
                    }).collect::<Vec<_>>()
                })
            } else {
                None
            }
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    let mut stdout = StandardStream::stdout(if color {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    });

    if color {
        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true));
    }
    let _ = writeln!(stdout, "\nStatistics for: {}", pattern);
    if color {
        let _ = stdout.reset();
    }

    let _ = writeln!(stdout, "  Total files: {}", files.len());
    let _ = writeln!(stdout, "  Total lines: {}", total_lines);
    let _ = writeln!(stdout, "  Max depth:   {}", max_depth);

    if level.is_none() {
        let _ = writeln!(stdout, "\n  Depth breakdown:");
        for d in 0..=max_depth {
            let count = depth_map.get(&d).map(|v| v.len()).unwrap_or(0);
            if count > 0 {
                if color {
                    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
                }
                let _ = write!(stdout, "    Level {}", d);
                if color {
                    let _ = stdout.reset();
                }
                let _ = writeln!(stdout, ": {} files", count);
            }
        }
        let _ = writeln!(
            stdout,
            "\n  Use -l <level> to see files at a specific depth"
        );
        let _ = writeln!(stdout);
        return Ok(());
    }

    if let Some(lvl) = level {
        let terminal_height =
            if let Some((_, terminal_size::Height(h))) = terminal_size::terminal_size() {
                h as usize
            } else {
                24
            };

        let available_lines = terminal_height.saturating_sub(10);

        if let Some(files_at_level) = depth_map.get(&lvl) {
            let _ = writeln!(stdout, "\n  Files at depth level {}:", lvl);

            let mut sorted_files = files_at_level.clone();
            sorted_files.sort_by_key(|(_, lines)| std::cmp::Reverse(*lines));

            let files_to_show = sorted_files.len().min(available_lines);
            let remaining = sorted_files.len().saturating_sub(available_lines);

            for (path, lines) in sorted_files.iter().take(files_to_show) {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if color {
                        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
                    }
                    let _ = write!(stdout, "    {:6}", lines);
                    if color {
                        let _ = stdout.reset();
                    }
                    let _ = write!(stdout, " lines  ");
                    if color {
                        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                    }
                    let _ = writeln!(stdout, "{}", filename);
                    if color {
                        let _ = stdout.reset();
                    }
                }
            }

            if remaining > 0 {
                if color {
                    let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
                }
                let _ = writeln!(
                    stdout,
                    "\n    ... and {} more files (terminal shows {} of {})",
                    remaining,
                    files_to_show,
                    sorted_files.len()
                );
                if color {
                    let _ = stdout.reset();
                }
            }
        } else {
            let _ = writeln!(stdout, "\n  No files at depth level {}", lvl);
        }
    }

    let _ = writeln!(stdout);
    Ok(())
}
