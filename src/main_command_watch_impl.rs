//! Implementation of the watch command.
//!
//! This module maps to hierarchical name: main.command.watch.impl

use anyhow::Context;
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Watcher};
use recur::parser::{HierarchicalName, HierarchyPattern};
use recur::project_config;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

const WATCH_STATE_DIR: &str = ".recur/watch";
const STATUS_PREFIX: &str = "recur-watch.";
const STATUS_SUFFIX: &str = ".status.current.md";

enum WatchFormat {
    Oneline,
    Json,
}

struct WatchConfig {
    filter: HierarchyPattern,
    filter_raw: String,
    dir: PathBuf,
    format: WatchFormat,
    format_raw: String,
    poll_framing: Option<u64>,
    poll_framing_raw: Option<String>,
    mode: &'static str,
    state_writer: Option<WatchStateWriter>,
    runtime: RefCell<WatchRuntime>,
}

#[derive(Default)]
struct WatchRuntime {
    events_seen: u64,
    filtered_out: u64,
    last_event_at: Option<String>,
}

struct WatchStateWriter {
    id: String,
    path: PathBuf,
    started_at: String,
}

#[derive(Serialize)]
struct WatchEventRecord<'a> {
    path: String,
    event_type: &'a str,
}

pub fn execute(
    watch_id: Option<String>,
    filter: String,
    dir: PathBuf,
    format: String,
    poll_framing: Option<String>,
    separator: char,
) -> anyhow::Result<()> {
    let state_writer = watch_id
        .as_ref()
        .map(|id| WatchStateWriter::new(id.clone(), &dir));
    let result = execute_inner(
        filter.clone(),
        dir.clone(),
        format.clone(),
        poll_framing.clone(),
        separator,
        state_writer,
    );

    if let Err(error) = &result {
        if let Some(writer) = WatchStateWriter::from_id_for_rejection(&watch_id, &dir) {
            let request = WatchStateRequest {
                filter: &filter,
                dir: &dir,
                format: &format,
                poll_framing: poll_framing.as_deref(),
                mode: mode_for_poll_framing(poll_framing.as_deref()),
            };
            let _ = writer.write_rejected(&request, &error.to_string());
        }
    }

    result
}

fn execute_inner(
    filter: String,
    dir: PathBuf,
    format: String,
    poll_framing: Option<String>,
    separator: char,
    state_writer: Option<WatchStateWriter>,
) -> anyhow::Result<()> {
    let parsed_filter = HierarchyPattern::parse_with_separator(&filter, separator)
        .with_context(|| "failed to parse watch filter pattern")?;
    let format = parse_format(&format)?;
    let poll_framing_raw = poll_framing.clone();
    let poll_framing = parse_poll_framing(poll_framing)?;
    let mode = if poll_framing.is_some() {
        "poll"
    } else {
        "stream"
    };
    let config = WatchConfig {
        filter: parsed_filter,
        filter_raw: filter,
        dir,
        format_raw: format_to_text(&format).to_string(),
        format,
        poll_framing,
        poll_framing_raw,
        mode,
        state_writer,
        runtime: RefCell::new(WatchRuntime::default()),
    };

    validate_dir(&config.dir)?;
    write_active_state(&config)?;

    if let Some(seconds) = config.poll_framing {
        run_poll_loop(&config, seconds)
    } else {
        run_stream_loop(&config)
    }
}

fn format_to_text(format: &WatchFormat) -> &'static str {
    match format {
        WatchFormat::Oneline => "oneline",
        WatchFormat::Json => "json",
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
    eprintln!(
        "recur watch: ready (stream mode, dir={})",
        config.dir.display()
    );

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
        } else {
            record_filtered(config)?;
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
    // Persist the accepted event before publishing it. Consumers commonly use
    // the stdout event as the signal that the query-side status receipt is
    // ready, so publishing first creates a race where `recur watch status`
    // can still observe the initial record with no ACK.
    record_event_seen(config)?;

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

fn record_event_seen(config: &WatchConfig) -> anyhow::Result<()> {
    {
        let mut runtime = config.runtime.borrow_mut();
        runtime.events_seen += 1;
        runtime.last_event_at = Some(now_stamp());
    }

    write_active_state(config)
}

fn record_filtered(config: &WatchConfig) -> anyhow::Result<()> {
    {
        let mut runtime = config.runtime.borrow_mut();
        runtime.filtered_out += 1;
    }

    write_active_state(config)
}

fn write_active_state(config: &WatchConfig) -> anyhow::Result<()> {
    let Some(writer) = &config.state_writer else {
        return Ok(());
    };

    let runtime = config.runtime.borrow();
    let request = WatchStateRequest {
        filter: &config.filter_raw,
        dir: &config.dir,
        format: &config.format_raw,
        poll_framing: config.poll_framing_raw.as_deref(),
        mode: config.mode,
    };

    writer.write_active(&request, &runtime)
}

struct WatchStateRequest<'a> {
    filter: &'a str,
    dir: &'a Path,
    format: &'a str,
    poll_framing: Option<&'a str>,
    mode: &'a str,
}

impl WatchStateWriter {
    fn new(id: String, dir: &Path) -> Self {
        let root = resolve_state_root(dir);
        let safe_id = sanitize_watch_id(&id);
        let path = root
            .join(WATCH_STATE_DIR)
            .join(format!("{STATUS_PREFIX}{safe_id}{STATUS_SUFFIX}"));

        Self {
            id: safe_id,
            path,
            started_at: now_stamp(),
        }
    }

    fn from_id_for_rejection(id: &Option<String>, dir: &Path) -> Option<Self> {
        id.as_ref().map(|id| Self::new(id.clone(), dir))
    }

    fn write_active(
        &self,
        request: &WatchStateRequest<'_>,
        runtime: &WatchRuntime,
    ) -> anyhow::Result<()> {
        self.write_state("active", "accepted", "", request, runtime)
    }

    fn write_rejected(&self, request: &WatchStateRequest<'_>, reason: &str) -> anyhow::Result<()> {
        self.write_state(
            "stopped",
            "rejected",
            reason,
            request,
            &WatchRuntime::default(),
        )
    }

    fn write_state(
        &self,
        state: &str,
        ack: &str,
        nak_reason: &str,
        request: &WatchStateRequest<'_>,
        runtime: &WatchRuntime,
    ) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create '{}'", parent.display()))?;
        }

        let poll_framing = request.poll_framing.unwrap_or("");
        let last_event_at = runtime.last_event_at.as_deref().unwrap_or("");
        let body = format!(
            "id = \"{}\"\nstate = \"{}\"\nack = \"{}\"\nnak_reason = \"{}\"\nfilter = \"{}\"\ndir = \"{}\"\nmode = \"{}\"\npoll_framing = \"{}\"\nformat = \"{}\"\npid = \"{}\"\nstarted_at = \"{}\"\nlast_event_at = \"{}\"\nevents_seen = \"{}\"\nfiltered_out = \"{}\"\n",
            escape_value(&self.id),
            escape_value(state),
            escape_value(ack),
            escape_value(nak_reason),
            escape_value(request.filter),
            escape_value(&request.dir.display().to_string()),
            escape_value(request.mode),
            escape_value(poll_framing),
            escape_value(request.format),
            std::process::id(),
            escape_value(&self.started_at),
            escape_value(last_event_at),
            runtime.events_seen,
            runtime.filtered_out,
        );

        fs::write(&self.path, body)
            .with_context(|| format!("failed to write '{}'", self.path.display()))
    }
}

fn resolve_state_root(dir: &Path) -> PathBuf {
    if dir.join(".recur").exists() {
        return dir.to_path_buf();
    }

    if let Ok(Some(config)) = project_config::load_nearest(dir) {
        return config.project_root;
    }

    if dir.is_dir() {
        return dir.to_path_buf();
    }

    dir.parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn sanitize_watch_id(id: &str) -> String {
    id.chars()
        .map(|ch| match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '-',
            _ => ch,
        })
        .collect()
}

fn escape_value(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn now_stamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("unix:{seconds}")
}

fn mode_for_poll_framing(poll_framing: Option<&str>) -> &'static str {
    if poll_framing.is_some() {
        "poll"
    } else {
        "stream"
    }
}
