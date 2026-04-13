//! Implementation of the reveal command.
//!
//! This module maps to hierarchical name: main.command.reveal.impl

use anyhow::{Context, Result};
use recur::project_config::{
    self, RevealConfig, DEFAULT_REVEAL_ENTRY_SUFFIX, DEFAULT_REVEAL_MAX_THREADS,
    DEFAULT_REVEAL_MODE, DEFAULT_REVEAL_ORDER_STEPS, DEFAULT_REVEAL_SKIP_PERSONA_IF_KNOWN,
    DEFAULT_REVEAL_TRUST,
};
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct EffectiveRevealPolicy {
    mode: String,
    entry_suffix: String,
    trust: String,
    max_threads: usize,
    skip_persona_if_known: bool,
    order_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct RevealField {
    key: String,
    value: String,
}

#[derive(Debug, Clone)]
struct RevealEntry {
    lane: String,
    path: String,
    absolute_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct RevealEntrySummary {
    lane: String,
    path: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct RevealListOutput {
    root: String,
    entry_suffix: String,
    mode: String,
    trust: String,
    max_threads: usize,
    skip_persona_if_known: bool,
    order_steps: Vec<String>,
    entries: Vec<RevealEntrySummary>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct RevealShowOutput {
    root: String,
    lane: String,
    path: String,
    entry_suffix: String,
    mode: String,
    trust: String,
    max_threads: usize,
    skip_persona_if_known: bool,
    order_steps: Vec<String>,
    ordered_fields: Vec<RevealField>,
    extra_fields: Vec<RevealField>,
}

#[derive(Debug, Clone)]
enum RevealSelection {
    Found(RevealEntry),
    Ambiguous(Vec<RevealEntry>),
    Missing,
}

pub fn execute(lane: Option<String>, dir: PathBuf, json: bool) -> Result<()> {
    let requested_root = resolve_root(dir)?;
    let loaded = project_config::load_nearest(&requested_root)?;
    let root = loaded
        .as_ref()
        .map(|config| config.project_root.clone())
        .unwrap_or(requested_root);
    let policy = EffectiveRevealPolicy::from_config(loaded.as_ref().and_then(|config| {
        config.reveal.as_ref()
    }));
    let entries = discover_reveal_entries(&root, &policy.entry_suffix)?;

    match lane {
        Some(query) => match select_reveal_entry(&entries, &query, &policy.entry_suffix) {
            RevealSelection::Found(entry) => {
                let fields = parse_reveal_fields(&entry.absolute_path)?;
                let (ordered_fields, extra_fields) = arrange_fields(&fields, &policy.order_steps);
                let output = RevealShowOutput {
                    root: root.display().to_string(),
                    lane: entry.lane.clone(),
                    path: entry.path.clone(),
                    entry_suffix: policy.entry_suffix.clone(),
                    mode: policy.mode.clone(),
                    trust: policy.trust.clone(),
                    max_threads: policy.max_threads,
                    skip_persona_if_known: policy.skip_persona_if_known,
                    order_steps: policy.order_steps.clone(),
                    ordered_fields,
                    extra_fields,
                };
                print_show_output(&output, json)?;
            }
            RevealSelection::Ambiguous(matches) => print_ambiguous_matches(&root, &query, &matches, json)?,
            RevealSelection::Missing => print_missing_match(&root, &query, &entries, json)?,
        },
        None => {
            let output = RevealListOutput {
                root: root.display().to_string(),
                entry_suffix: policy.entry_suffix.clone(),
                mode: policy.mode.clone(),
                trust: policy.trust.clone(),
                max_threads: policy.max_threads,
                skip_persona_if_known: policy.skip_persona_if_known,
                order_steps: policy.order_steps.clone(),
                entries: entries
                    .iter()
                    .map(|entry| RevealEntrySummary {
                        lane: entry.lane.clone(),
                        path: entry.path.clone(),
                    })
                    .collect(),
            };
            print_list_output(&output, json)?;
        }
    }

    Ok(())
}

impl EffectiveRevealPolicy {
    fn from_config(config: Option<&RevealConfig>) -> Self {
        let order_steps = config
            .and_then(|reveal| reveal.order.as_ref())
            .map(|order| order.steps.clone())
            .filter(|steps| !steps.is_empty())
            .unwrap_or_else(|| {
                DEFAULT_REVEAL_ORDER_STEPS
                    .iter()
                    .map(|step| step.to_string())
                    .collect()
            });

        Self {
            mode: config
                .and_then(|reveal| reveal.mode.clone())
                .unwrap_or_else(|| DEFAULT_REVEAL_MODE.to_string()),
            entry_suffix: config
                .and_then(|reveal| reveal.entry_suffix.clone())
                .unwrap_or_else(|| DEFAULT_REVEAL_ENTRY_SUFFIX.to_string()),
            trust: config
                .and_then(|reveal| reveal.trust.clone())
                .unwrap_or_else(|| DEFAULT_REVEAL_TRUST.to_string()),
            max_threads: config
                .and_then(|reveal| reveal.max_threads)
                .unwrap_or(DEFAULT_REVEAL_MAX_THREADS),
            skip_persona_if_known: config
                .and_then(|reveal| reveal.skip_persona_if_known)
                .unwrap_or(DEFAULT_REVEAL_SKIP_PERSONA_IF_KNOWN),
            order_steps,
        }
    }
}

fn resolve_root(dir: PathBuf) -> Result<PathBuf> {
    if dir.is_absolute() {
        return Ok(dir);
    }

    Ok(std::env::current_dir()?.join(dir))
}

fn discover_reveal_entries(root: &Path, entry_suffix: &str) -> Result<Vec<RevealEntry>> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| should_keep_reveal_walk_entry(entry.path()))
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let Some(filename) = entry.path().file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(lane) = filename.strip_suffix(entry_suffix) else {
            continue;
        };

        let relative = entry
            .path()
            .strip_prefix(root)
            .unwrap_or(entry.path())
            .to_path_buf();

        entries.push(RevealEntry {
            lane: lane.to_string(),
            path: normalize_relative_path(&relative),
            absolute_path: entry.path().to_path_buf(),
        });
    }

    entries.sort_by(|a, b| a.lane.cmp(&b.lane).then_with(|| a.path.cmp(&b.path)));
    Ok(entries)
}

fn should_keep_reveal_walk_entry(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return true;
    };

    !matches!(
        name,
        ".git" | "target" | "node_modules" | "vendor" | "dist" | "build"
    )
}

fn normalize_relative_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn select_reveal_entry(entries: &[RevealEntry], query: &str, entry_suffix: &str) -> RevealSelection {
    let normalized = normalize_query(query, entry_suffix);

    let exact_matches: Vec<RevealEntry> = entries
        .iter()
        .filter(|entry| entry.lane == normalized || entry.path == normalized)
        .cloned()
        .collect();
    if exact_matches.len() == 1 {
        return RevealSelection::Found(exact_matches[0].clone());
    }
    if !exact_matches.is_empty() {
        return RevealSelection::Ambiguous(exact_matches);
    }

    let suffix_matches: Vec<RevealEntry> = entries
        .iter()
        .filter(|entry| {
            entry.lane.ends_with(&format!(".{}", normalized))
                || entry.path.ends_with(&normalized)
                || entry.path.ends_with(&format!("/{}", normalized))
        })
        .cloned()
        .collect();
    if suffix_matches.len() == 1 {
        return RevealSelection::Found(suffix_matches[0].clone());
    }
    if !suffix_matches.is_empty() {
        return RevealSelection::Ambiguous(suffix_matches);
    }

    let fuzzy_matches: Vec<RevealEntry> = entries
        .iter()
        .filter(|entry| entry.lane.contains(&normalized) || entry.path.contains(&normalized))
        .cloned()
        .collect();
    if fuzzy_matches.len() == 1 {
        return RevealSelection::Found(fuzzy_matches[0].clone());
    }
    if !fuzzy_matches.is_empty() {
        return RevealSelection::Ambiguous(fuzzy_matches);
    }

    RevealSelection::Missing
}

fn normalize_query(query: &str, entry_suffix: &str) -> String {
    let normalized = query.trim().replace('\\', "/");
    let without_prefix = normalized.trim_start_matches("./");
    without_prefix
        .strip_suffix(entry_suffix)
        .unwrap_or(without_prefix)
        .trim_matches('/')
        .to_string()
}

fn parse_reveal_fields(path: &Path) -> Result<Vec<RevealField>> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("Failed to read reveal file {}", path.display()))?;

    let mut fields = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some(index) = trimmed.find('=') else {
            continue;
        };

        let key = trimmed[..index].trim();
        let raw_value = trimmed[index + 1..].trim();
        if key.is_empty() || raw_value.is_empty() {
            continue;
        }

        fields.push(RevealField {
            key: key.to_string(),
            value: strip_matching_quotes(raw_value),
        });
    }

    Ok(fields)
}

fn strip_matching_quotes(value: &str) -> String {
    let bytes = value.as_bytes();
    if bytes.len() >= 2
        && ((bytes[0] == b'"' && bytes[bytes.len() - 1] == b'"')
            || (bytes[0] == b'\'' && bytes[bytes.len() - 1] == b'\''))
    {
        return value[1..value.len() - 1].to_string();
    }
    value.to_string()
}

fn arrange_fields(fields: &[RevealField], order_steps: &[String]) -> (Vec<RevealField>, Vec<RevealField>) {
    let mut ordered = Vec::new();
    let mut seen = HashSet::new();

    if let Some(field) = fields.iter().find(|field| field.key == "recur.gift") {
        ordered.push(field.clone());
        seen.insert(field.key.clone());
    }

    for key in order_steps {
        if let Some(field) = fields.iter().find(|field| field.key == *key) {
            if seen.insert(field.key.clone()) {
                ordered.push(field.clone());
            }
        }
    }

    let extras = fields
        .iter()
        .filter(|field| !seen.contains(&field.key))
        .cloned()
        .collect();

    (ordered, extras)
}

fn print_list_output(output: &RevealListOutput, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
        return Ok(());
    }

    if output.entries.is_empty() {
        println!(
            "No reveal files matching '{}' found under {}",
            output.entry_suffix, output.root
        );
        return Ok(());
    }

    println!("Reveal entries under {}:", output.root);
    for entry in &output.entries {
        println!("  - {} => {}", entry.lane, entry.path);
    }
    println!();
    println!(
        "Policy: mode={}, trust={}, max_threads={}, skip_persona_if_known={}",
        output.mode, output.trust, output.max_threads, output.skip_persona_if_known
    );
    println!("Use `recur reveal <lane>` to open one reveal capsule.");

    Ok(())
}

fn print_show_output(output: &RevealShowOutput, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
        return Ok(());
    }

    println!("Reveal for {}", output.lane);
    println!("  path: {}", output.path);
    println!(
        "  policy: mode={}, trust={}, max_threads={}, skip_persona_if_known={}",
        output.mode, output.trust, output.max_threads, output.skip_persona_if_known
    );

    for field in &output.ordered_fields {
        println!("{} = {}", field.key, field.value);
    }

    if !output.extra_fields.is_empty() {
        println!();
        println!("extra:");
        for field in &output.extra_fields {
            println!("{} = {}", field.key, field.value);
        }
    }

    Ok(())
}

fn print_ambiguous_matches(root: &Path, query: &str, matches: &[RevealEntry], json: bool) -> Result<()> {
    let payload = RevealListOutput {
        root: root.display().to_string(),
        entry_suffix: DEFAULT_REVEAL_ENTRY_SUFFIX.to_string(),
        mode: DEFAULT_REVEAL_MODE.to_string(),
        trust: DEFAULT_REVEAL_TRUST.to_string(),
        max_threads: DEFAULT_REVEAL_MAX_THREADS,
        skip_persona_if_known: DEFAULT_REVEAL_SKIP_PERSONA_IF_KNOWN,
        order_steps: DEFAULT_REVEAL_ORDER_STEPS
            .iter()
            .map(|step| step.to_string())
            .collect(),
        entries: matches
            .iter()
            .map(|entry| RevealEntrySummary {
                lane: entry.lane.clone(),
                path: entry.path.clone(),
            })
            .collect(),
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    println!("Multiple reveal files match '{}':", query);
    for entry in &payload.entries {
        println!("  - {} => {}", entry.lane, entry.path);
    }
    println!("Be more specific and try again.");

    Ok(())
}

fn print_missing_match(root: &Path, query: &str, entries: &[RevealEntry], json: bool) -> Result<()> {
    let payload = RevealListOutput {
        root: root.display().to_string(),
        entry_suffix: DEFAULT_REVEAL_ENTRY_SUFFIX.to_string(),
        mode: DEFAULT_REVEAL_MODE.to_string(),
        trust: DEFAULT_REVEAL_TRUST.to_string(),
        max_threads: DEFAULT_REVEAL_MAX_THREADS,
        skip_persona_if_known: DEFAULT_REVEAL_SKIP_PERSONA_IF_KNOWN,
        order_steps: DEFAULT_REVEAL_ORDER_STEPS
            .iter()
            .map(|step| step.to_string())
            .collect(),
        entries: entries
            .iter()
            .map(|entry| RevealEntrySummary {
                lane: entry.lane.clone(),
                path: entry.path.clone(),
            })
            .collect(),
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    println!("No reveal file matched '{}'.", query);
    if payload.entries.is_empty() {
        println!("No reveal files are present under {}.", payload.root);
    } else {
        println!("Known reveal entries:");
        for entry in &payload.entries {
            println!("  - {} => {}", entry.lane, entry.path);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrange_fields_prefers_gift_then_configured_order() {
        let fields = vec![
            RevealField {
                key: "verify".to_string(),
                value: "cargo test".to_string(),
            },
            RevealField {
                key: "recur.gift".to_string(),
                value: "follow the lane".to_string(),
            },
            RevealField {
                key: "persona".to_string(),
                value: "recur expert".to_string(),
            },
        ];

        let (ordered, extra) =
            arrange_fields(&fields, &["persona".to_string(), "verify".to_string()]);

        assert_eq!(ordered[0].key, "recur.gift");
        assert_eq!(ordered[1].key, "persona");
        assert_eq!(ordered[2].key, "verify");
        assert!(extra.is_empty());
    }

    #[test]
    fn normalize_query_trims_suffix_and_relative_prefix() {
        assert_eq!(
            normalize_query("./docs/main.command.trace-id.recur.md", ".recur.md"),
            "docs/main.command.trace-id"
        );
        assert_eq!(
            normalize_query("main.command.trace-id.recur.md", ".recur.md"),
            "main.command.trace-id"
        );
    }
}
