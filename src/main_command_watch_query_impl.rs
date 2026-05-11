//! Pure watcher-state query surface for `recur watch`.
//!
//! This module maps to hierarchical name: main.command.watch.query.impl

use anyhow::Context;
use clap::Subcommand;
use recur::parser::{HierarchicalName, HierarchyPattern};
use recur::project_config;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

const WATCH_DIR: &str = ".recur/watch";
const STATUS_PREFIX: &str = "recur-watch.";
const STATUS_SUFFIX: &str = ".status.current.md";

#[derive(Subcommand)]
pub enum WatchQuerySubcommand {
    /// List known watcher state records
    List {
        /// Filter virtual watcher names such as docs-monkey.active
        #[arg(long, value_name = "PATTERN")]
        filter: Option<String>,
    },

    /// Show one watcher state record
    Status {
        /// Watch id, for example docs-monkey
        watch_id: String,
    },

    /// Explain the pure query / active runner split
    Explain,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct WatchState {
    id: String,
    path: String,
    state: String,
    ack: Option<String>,
    nak_reason: Option<String>,
    filter: Option<String>,
    dir: Option<String>,
    mode: Option<String>,
    poll_framing: Option<String>,
    format: Option<String>,
    pid: Option<String>,
    started_at: Option<String>,
    last_event_at: Option<String>,
    events_seen: Option<String>,
    filtered_out: Option<String>,
}

pub fn execute(
    command: Option<WatchQuerySubcommand>,
    dir: PathBuf,
    json: bool,
) -> anyhow::Result<()> {
    match command.unwrap_or(WatchQuerySubcommand::List { filter: None }) {
        WatchQuerySubcommand::List { filter } => {
            let states = collect_states(&dir, filter.as_deref())?;
            emit_states(&states, json)
        }
        WatchQuerySubcommand::Status { watch_id } => {
            let states = collect_states(&dir, None)?;
            let Some(state) = states.into_iter().find(|state| state.id == watch_id) else {
                anyhow::bail!("watch '{}' not found", watch_id);
            };
            emit_states(&[state], json)
        }
        WatchQuerySubcommand::Explain => emit_explain(json),
    }
}

fn collect_states(dir: &Path, filter: Option<&str>) -> anyhow::Result<Vec<WatchState>> {
    let root = if dir.join(".recur").exists() {
        dir.to_path_buf()
    } else {
        project_config::load_nearest(dir)?
            .map(|config| config.project_root)
            .unwrap_or_else(|| dir.to_path_buf())
    };
    let watch_dir = root.join(WATCH_DIR);

    if !watch_dir.exists() {
        return Ok(Vec::new());
    }

    let filter = match filter {
        Some(raw) => Some(
            HierarchyPattern::parse_with_separator(raw, '.')
                .with_context(|| "failed to parse watch filter pattern")?,
        ),
        None => None,
    };

    let mut states = Vec::new();
    for entry in fs::read_dir(&watch_dir)
        .with_context(|| format!("failed to read '{}'", watch_dir.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let path = entry.path();
        let Some(id) = watch_id_from_path(&path) else {
            continue;
        };

        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read '{}'", path.display()))?;
        let fields = parse_state_fields(&text);
        let state = state_from_fields(&root, &path, id, &fields);

        if matches_filter(&state, filter.as_ref()) {
            states.push(state);
        }
    }

    states.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(states)
}

fn watch_id_from_path(path: &Path) -> Option<String> {
    let filename = path.file_name()?.to_str()?;
    let id = filename
        .strip_prefix(STATUS_PREFIX)?
        .strip_suffix(STATUS_SUFFIX)?;
    Some(id.to_string())
}

fn parse_state_fields(text: &str) -> BTreeMap<String, String> {
    let mut fields = BTreeMap::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let Some((key, value)) = split_field(trimmed) else {
            continue;
        };

        fields.insert(key.to_ascii_lowercase(), unquote(value));
    }

    fields
}

fn split_field(line: &str) -> Option<(&str, &str)> {
    if let Some((key, value)) = line.split_once('=') {
        return Some((key.trim(), value.trim()));
    }

    if let Some((key, value)) = line.split_once(':') {
        return Some((key.trim(), value.trim()));
    }

    None
}

fn unquote(value: &str) -> String {
    value
        .trim_matches('"')
        .trim_matches('\'')
        .trim()
        .to_string()
}

fn state_from_fields(
    root: &Path,
    path: &Path,
    id: String,
    fields: &BTreeMap<String, String>,
) -> WatchState {
    WatchState {
        id,
        path: relative_display_path(root, path),
        state: field_or(fields, "state", "unknown"),
        ack: optional_field(fields, "ack"),
        nak_reason: optional_field(fields, "nak_reason"),
        filter: optional_field(fields, "filter"),
        dir: optional_field(fields, "dir"),
        mode: optional_field(fields, "mode"),
        poll_framing: optional_field(fields, "poll_framing"),
        format: optional_field(fields, "format"),
        pid: optional_field(fields, "pid"),
        started_at: optional_field(fields, "started_at"),
        last_event_at: optional_field(fields, "last_event_at"),
        events_seen: optional_field(fields, "events_seen"),
        filtered_out: optional_field(fields, "filtered_out"),
    }
}

fn field_or(fields: &BTreeMap<String, String>, key: &str, fallback: &str) -> String {
    fields
        .get(key)
        .filter(|value| !value.is_empty())
        .cloned()
        .unwrap_or_else(|| fallback.to_string())
}

fn optional_field(fields: &BTreeMap<String, String>, key: &str) -> Option<String> {
    fields.get(key).filter(|value| !value.is_empty()).cloned()
}

fn matches_filter(state: &WatchState, filter: Option<&HierarchyPattern>) -> bool {
    let Some(filter) = filter else {
        return true;
    };

    let virtual_name = format!("{}.{}", state.id, state.state);
    filter.matches(&HierarchicalName::with_separator(&virtual_name, '.'))
        || filter.matches(&HierarchicalName::with_separator(&state.id, '.'))
}

fn relative_display_path(base_dir: &Path, path: &Path) -> String {
    path.strip_prefix(base_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn emit_states(states: &[WatchState], json: bool) -> anyhow::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if json {
        serde_json::to_writer(&mut handle, states)?;
        writeln!(handle)?;
        return Ok(());
    }

    for state in states {
        writeln!(
            handle,
            "{}\t{}\t{}\t{}",
            state.id,
            state.state,
            state.ack.as_deref().unwrap_or("unknown"),
            state.path
        )?;
    }

    handle.flush()?;
    Ok(())
}

fn emit_explain(json: bool) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct WatchExplain<'a> {
        query_surface: &'a str,
        runner: &'a str,
        state_dir: &'a str,
    }

    let explanation = WatchExplain {
        query_surface: "recur watch reads watcher eventness and exits",
        runner: "recur-watch runs the active subscription loop",
        state_dir: WATCH_DIR,
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if json {
        serde_json::to_writer(&mut handle, &explanation)?;
        writeln!(handle)?;
        return Ok(());
    }

    writeln!(handle, "recur watch: pure watcher-state query")?;
    writeln!(handle, "recur-watch: active subscription runner")?;
    writeln!(handle, "state: {}", WATCH_DIR)?;
    handle.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_key_value_state_fields() {
        let fields = parse_state_fields(
            r#"
            state = active
            ack = "accepted"
            nak_reason = ""
            "#,
        );

        assert_eq!(fields.get("state"), Some(&"active".to_string()));
        assert_eq!(fields.get("ack"), Some(&"accepted".to_string()));
        assert_eq!(fields.get("nak_reason"), Some(&"".to_string()));
    }

    #[test]
    fn extracts_watch_id_from_status_filename() {
        let id = watch_id_from_path(Path::new(
            ".recur/watch/recur-watch.docs-monkey.status.current.md",
        ));

        assert_eq!(id, Some("docs-monkey".to_string()));
    }

    #[test]
    fn filter_matches_virtual_state_name() {
        let state = WatchState {
            id: "docs-monkey".to_string(),
            path: "ignored".to_string(),
            state: "active".to_string(),
            ack: None,
            nak_reason: None,
            filter: None,
            dir: None,
            mode: None,
            poll_framing: None,
            format: None,
            pid: None,
            started_at: None,
            last_event_at: None,
            events_seen: None,
            filtered_out: None,
        };
        let filter = HierarchyPattern::parse_with_separator("**.active", '.').unwrap();

        assert!(matches_filter(&state, Some(&filter)));
    }
}
