//! Implementation of the watch command.
//!
//! This module maps to hierarchical name: main.command.watch.impl

use anyhow::Context;
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Watcher};
use recur::parser::{HierarchicalName, HierarchyPattern};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

enum WatchFormat {
    Oneline,
    Json,
}

struct WatchConfig {
    filter: HierarchyPattern,
    dir: PathBuf,
    format: WatchFormat,
    poll_framing: Option<u64>,
}

#[derive(Serialize)]
struct WatchEventRecord<'a> {
    path: String,
    event_type: &'a str,
}

pub fn execute(
    filter: String,
    dir: PathBuf,
    format: String,
    poll_framing: Option<String>,
    separator: char,
) -> anyhow::Result<()> {
    let filter = HierarchyPattern::parse_with_separator(&filter, separator)
        .with_context(|| "failed to parse watch filter pattern")?;

    let format = parse_format(&format)?;
    let poll_framing = parse_poll_framing(poll_framing)?;
    let config = WatchConfig {
        filter,
        dir,
        format,
        poll_framing,
    };

    validate_dir(&config.dir)?;

    if let Some(seconds) = config.poll_framing {
        run_poll_loop(&config, seconds)
    } else {
        run_stream_loop(&config)
    }
}

fn parse_format(raw: &str) -> anyhow::Result<WatchFormat> {
    match raw {
        "oneline" => Ok(WatchFormat::Oneline),
        "json" => Ok(WatchFormat::Json),
        _ => anyhow::bail!("invalid --format '{}': expected 'oneline' or 'json'", raw),
    }
}

fn parse_poll_framing(raw: Option<String>) -> anyhow::Result<Option<u64>> {
    match raw {
        Some(raw) => {
            let seconds = raw.parse::<i64>().map_err(|_| {
                anyhow::anyhow!(
                    "invalid --poll-framing value '{}': expected positive integer seconds",
                    raw
                )
            })?;

            if seconds <= 0 {
                anyhow::bail!(
                    "invalid --poll-framing value '{}': expected positive integer seconds",
                    raw
                );
            }

            Ok(Some(seconds as u64))
        }
        None => Ok(None),
    }
}

fn validate_dir(dir: &Path) -> anyhow::Result<()> {
    if !dir.exists() {
        anyhow::bail!("invalid --dir '{}': directory not found", dir.display());
    }

    if !dir.is_dir() {
        anyhow::bail!("invalid --dir '{}': path is not a directory", dir.display());
    }

    Ok(())
}

fn run_stream_loop(config: &WatchConfig) -> anyhow::Result<()> {
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(move |result| {
        let _ = tx.send(result);
    })?;

    watcher.watch(&config.dir, RecursiveMode::Recursive)?;
    eprintln!("recur watch: ready (stream mode, dir={})", config.dir.display());

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                emit_notify_event(config, event)?;
            }
            Ok(Err(_)) => {}
            Err(_) => break,
        }
    }

    Ok(())
}

fn run_poll_loop(config: &WatchConfig, seconds: u64) -> anyhow::Result<()> {
    let mut previous = collect_matching_mtimes(config)?;
    let interval = Duration::from_secs(seconds);
    eprintln!(
        "recur watch: ready (poll mode, interval={}s, dir={})",
        seconds,
        config.dir.display()
    );

    loop {
        thread::sleep(interval);

        let current = collect_matching_mtimes(config)?;

        for (path, modified) in &current {
            match previous.get(path) {
                None => emit_event(config, path, "created")?,
                Some(previous_modified) if modified > previous_modified => {
                    emit_event(config, path, "modified")?
                }
                _ => {}
            }
        }

        for path in previous.keys() {
            if !current.contains_key(path) {
                emit_event(config, path, "deleted")?;
            }
        }

        previous = current;
    }
}

fn emit_notify_event(config: &WatchConfig, event: Event) -> anyhow::Result<()> {
    let Some(event_type) = classify_event_kind(&event.kind) else {
        return Ok(());
    };

    for path in event.paths {
        if matches_filter(&path, &config.filter) {
            emit_event(config, &path, event_type)?;
        }
    }

    Ok(())
}

fn collect_matching_mtimes(config: &WatchConfig) -> anyhow::Result<HashMap<PathBuf, SystemTime>> {
    let mut mtimes = HashMap::new();

    for entry in WalkDir::new(&config.dir).into_iter() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path().to_path_buf();
        if !matches_filter(&path, &config.filter) {
            continue;
        }

        let modified = fs::metadata(&path)
            .and_then(|metadata| metadata.modified())
            .with_context(|| format!("failed to read modified time for '{}'", path.display()))?;

        mtimes.insert(path, modified);
    }

    Ok(mtimes)
}

fn matches_filter(path: &Path, filter: &HierarchyPattern) -> bool {
    let Some(filename) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };

    let hierarchical_name = HierarchicalName::with_separator(filename, filter.separator);
    filter.matches(&hierarchical_name)
}

fn classify_event_kind(kind: &EventKind) -> Option<&'static str> {
    match kind {
        EventKind::Create(_) => Some("created"),
        EventKind::Modify(_) => Some("modified"),
        EventKind::Remove(_) => Some("deleted"),
        _ => None,
    }
}

fn emit_event(config: &WatchConfig, path: &Path, event_type: &'static str) -> anyhow::Result<()> {
    let path_text = path.to_string_lossy().to_string();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match config.format {
        WatchFormat::Oneline => {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            writeln!(handle, "{} {}\t{}", timestamp, event_type, path_text)?;
        }
        WatchFormat::Json => {
            let record = WatchEventRecord {
                path: path_text,
                event_type,
            };
            writeln!(handle, "{}", serde_json::to_string(&record)?)?;
        }
    }

    handle.flush()?;
    Ok(())
}
