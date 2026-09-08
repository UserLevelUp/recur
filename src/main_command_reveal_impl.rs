//! Implementation of the reveal command.
//!
//! This module maps to hierarchical name: main.command.reveal.impl

use anyhow::{Context, Result};
use recur::project_config::{
    self, RevealConfig, DEFAULT_REVEAL_ENTRY_SUFFIX, DEFAULT_REVEAL_MAX_THREADS,
    DEFAULT_REVEAL_MODE, DEFAULT_REVEAL_ORDER_STEPS, DEFAULT_REVEAL_SKIP_PERSONA_IF_KNOWN,
    DEFAULT_REVEAL_TRUST,
};
use recur::reveal_artifact::{valid_type, ArtifactType, TypePolicy};
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
    fields: Vec<RevealField>,
    artifact: ArtifactType,
    separators: Vec<char>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct RevealEntrySummary {
    lane: String,
    path: String,
    artifact: ArtifactType,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct RevealListOutput {
    status: String,
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
    status: String,
    artifact: ArtifactType,
    prompts: Vec<serde_json::Value>,
    eventness_policy: recur::warp_policy::WarpPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    reconciliation: Option<serde_json::Value>,
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
    TypeMismatch,
}

pub fn execute(
    lane: Option<String>,
    dir: Option<PathBuf>,
    artifact_type: Option<String>,
    sep: &[String],
    json: bool,
) -> Result<()> {
    if artifact_type
        .as_deref()
        .is_some_and(|kind| !valid_type(kind))
    {
        anyhow::bail!("Invalid --type: expected [A-Za-z][A-Za-z0-9_.-]*");
    }
    let separators: Vec<char> = sep
        .iter()
        .map(|s| {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) if !c.is_whitespace() && c != '/' && c != '\\' => Ok(c),
                _ => anyhow::bail!("--sep requires one non-path, non-whitespace character"),
            }
        })
        .collect::<Result<_>>()?;
    let explicit_root = dir.is_some();
    let requested_root = resolve_root(dir.unwrap_or_else(|| PathBuf::from(".")))?.canonicalize()?;
    if !requested_root.is_dir() {
        anyhow::bail!("Reveal root must be a directory");
    }
    let loaded = project_config::load_nearest(&requested_root)?;
    let config_root = loaded
        .as_ref()
        .map(|c| c.project_root.clone())
        .unwrap_or_else(|| requested_root.clone());
    let root = if explicit_root {
        requested_root
    } else {
        config_root.clone()
    };
    let policy = EffectiveRevealPolicy::from_config(
        loaded.as_ref().and_then(|config| config.reveal.as_ref()),
    );
    let types = loaded
        .as_ref()
        .and_then(|c| c.reveal.as_ref())
        .map(|r| r.types.clone())
        .unwrap_or_default();
    types.validate_separators(if separators.is_empty() {
        &['.']
    } else {
        &separators
    })?;
    let entries = discover_reveal_entries(
        &root,
        &policy.entry_suffix,
        &types,
        loaded.as_ref(),
        &separators,
    )?;

    match lane {
        Some(query) => match select_reveal_entry(
            &entries,
            &query,
            &policy.entry_suffix,
            artifact_type.as_deref(),
        ) {
            RevealSelection::Found(entry) => {
                let fields = &entry.fields;
                let (ordered_fields, extra_fields) = arrange_fields(&fields, &policy.order_steps);
                let field = |name: &str| {
                    fields
                        .iter()
                        .find(|f| f.key == name)
                        .map(|f| f.value.as_str())
                };
                let reconciliation = field("warp.id").map(|warp| {
                    let evidence_root = field("warp.root").map(|p| config_root.join(p)).unwrap_or_else(|| root.clone());
                    let result = (|| -> Result<serde_json::Value> {
                        let bounded = evidence_root.canonicalize()?;
                        if !bounded.starts_with(root.canonicalize()?) { anyhow::bail!("warp.root escapes project root"); }
                        crate::main_command_warp_impl::reconcile(&bounded, warp, field("observed.state"), field("readiness.slice"))
                    })();
                    result.unwrap_or_else(|error| serde_json::json!({"schema":"warp-reconciliation-v1",
                        "warnings":[format!("reconciliation unavailable: {error:#}")], "mutation":"none"}))
                });
                let eventness_root = field("warp.root")
                    .map(|relative| config_root.join(relative))
                    .unwrap_or_else(|| entry.absolute_path.parent().unwrap_or(&root).to_path_buf());
                let eventness_root = eventness_root.canonicalize()?;
                if !eventness_root.starts_with(root.canonicalize()?) {
                    anyhow::bail!("warp.root escapes project root");
                }
                let output = RevealShowOutput {
                    status: "found".into(),
                    artifact: entry.artifact.clone(),
                    prompts: if let Some(ids) = field("prompt.ids") {
                        recur::prompt::Registry::load(&root)?.references(ids)
                    } else {
                        Vec::new()
                    },
                    reconciliation,
                    eventness_policy: recur::warp_policy::WarpPolicy::load(&eventness_root)?,
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
            RevealSelection::Ambiguous(matches) => {
                print_selection_output(&root, &policy, "ambiguous", Some(&query), &matches, json)?
            }
            RevealSelection::Missing => {
                let filtered: Vec<_> = entries
                    .iter()
                    .filter(|e| e.artifact.matches(artifact_type.as_deref()))
                    .cloned()
                    .collect();
                print_selection_output(&root, &policy, "missing", Some(&query), &filtered, json)?;
            }
            RevealSelection::TypeMismatch => {
                print_selection_output(&root, &policy, "type-mismatch", Some(&query), &[], json)?
            }
        },
        None => {
            let output = RevealListOutput {
                status: "listed".into(),
                root: root.display().to_string(),
                entry_suffix: policy.entry_suffix.clone(),
                mode: policy.mode.clone(),
                trust: policy.trust.clone(),
                max_threads: policy.max_threads,
                skip_persona_if_known: policy.skip_persona_if_known,
                order_steps: policy.order_steps.clone(),
                entries: entries
                    .iter()
                    .filter(|entry| entry.artifact.matches(artifact_type.as_deref()))
                    .map(|entry| RevealEntrySummary {
                        lane: entry.lane.clone(),
                        path: entry.path.clone(),
                        artifact: entry.artifact.clone(),
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

fn discover_reveal_entries(
    root: &Path,
    entry_suffix: &str,
    types: &TypePolicy,
    config: Option<&project_config::RecurConfig>,
    explicit_separators: &[char],
) -> Result<Vec<RevealEntry>> {
    let mut entries = Vec::new();
    if entry_suffix.is_empty() {
        anyhow::bail!("reveal.entry_suffix must not be empty");
    }

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| entry.depth() == 0 || should_keep_reveal_walk_entry(entry.path()))
    {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let Some(filename) = entry.path().file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(lane) = filename.strip_suffix(entry_suffix) else {
            continue;
        };
        let absolute_path = entry.path().canonicalize()?;
        if !absolute_path.starts_with(root) {
            anyhow::bail!("Reveal artifact escapes discovery root");
        }
        let separators = if explicit_separators.is_empty() {
            vec![config
                .and_then(|c| c.separator_for_dir(entry.path().parent().unwrap_or(root)))
                .unwrap_or('.')]
        } else {
            explicit_separators.to_vec()
        };
        types.validate_separators(&separators)?;
        let fields = parse_reveal_fields(&absolute_path)?;
        let artifact = types.classify(
            lane,
            &separators,
            fields
                .iter()
                .filter(|f| f.key == "artifact.type")
                .map(|f| f.value.as_str()),
        );

        let relative = entry
            .path()
            .strip_prefix(root)
            .unwrap_or(entry.path())
            .to_path_buf();

        entries.push(RevealEntry {
            lane: lane.to_string(),
            path: normalize_relative_path(&relative),
            absolute_path,
            fields,
            artifact,
            separators,
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

fn select_reveal_entry(
    entries: &[RevealEntry],
    query: &str,
    entry_suffix: &str,
    artifact_type: Option<&str>,
) -> RevealSelection {
    let normalized = normalize_query(query, entry_suffix);
    let path_query = query.trim().replace('\\', "/");
    let path_query = path_query.strip_prefix("./").unwrap_or(&path_query);

    let exact_matches: Vec<RevealEntry> = entries
        .iter()
        .filter(|entry| {
            entry.lane == normalized
                || entry.path == path_query
                || entry.path.strip_suffix(entry_suffix) == Some(normalized.as_str())
        })
        .cloned()
        .collect();
    if !exact_matches.is_empty() {
        return select_filtered(exact_matches, artifact_type);
    }

    let suffix_matches: Vec<RevealEntry> = entries
        .iter()
        .filter(|entry| {
            entry
                .separators
                .iter()
                .any(|sep| entry.lane.ends_with(&format!("{}{}", sep, normalized)))
                || entry.path.ends_with(&normalized)
                || entry.path.ends_with(&format!("/{}", normalized))
        })
        .cloned()
        .collect();
    if !suffix_matches.is_empty() {
        return select_filtered(suffix_matches, artifact_type);
    }

    let fuzzy_matches: Vec<RevealEntry> = entries
        .iter()
        .filter(|entry| entry.lane.contains(&normalized) || entry.path.contains(&normalized))
        .cloned()
        .collect();
    if !fuzzy_matches.is_empty() {
        return select_filtered(fuzzy_matches, artifact_type);
    }

    RevealSelection::Missing
}

fn select_filtered(matches: Vec<RevealEntry>, artifact_type: Option<&str>) -> RevealSelection {
    let mut filtered: Vec<_> = matches
        .into_iter()
        .filter(|e| e.artifact.matches(artifact_type))
        .collect();
    match filtered.len() {
        0 => RevealSelection::TypeMismatch,
        1 => RevealSelection::Found(filtered.remove(0)),
        _ => RevealSelection::Ambiguous(filtered),
    }
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
        if key.is_empty() || (raw_value.is_empty() && key != "artifact.type") {
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

fn arrange_fields(
    fields: &[RevealField],
    order_steps: &[String],
) -> (Vec<RevealField>, Vec<RevealField>) {
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
        print_artifact(&entry.artifact);
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
    print_artifact(&output.artifact);
    if !output.prompts.is_empty() {
        println!("  prompts: {}", serde_json::to_string(&output.prompts)?);
    }
    println!(
        "  eventness policy: {}",
        serde_json::to_string(&output.eventness_policy)?
    );
    if let Some(reconciliation) = &output.reconciliation {
        println!(
            "  reconciliation: {}",
            serde_json::to_string(reconciliation)?
        );
    }
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

fn print_artifact(artifact: &ArtifactType) {
    println!(
        "    type: {} (status={}, source={})",
        artifact.r#type.as_deref().unwrap_or("none"),
        artifact.status,
        artifact.source
    );
    for diagnostic in &artifact.diagnostics {
        println!("    diagnostic: {}", diagnostic);
    }
}

fn print_selection_output(
    root: &Path,
    policy: &EffectiveRevealPolicy,
    status: &str,
    query: Option<&str>,
    entries: &[RevealEntry],
    json: bool,
) -> Result<()> {
    let output = RevealListOutput {
        status: status.into(),
        root: root.display().to_string(),
        entry_suffix: policy.entry_suffix.clone(),
        mode: policy.mode.clone(),
        trust: policy.trust.clone(),
        max_threads: policy.max_threads,
        skip_persona_if_known: policy.skip_persona_if_known,
        order_steps: policy.order_steps.clone(),
        entries: entries
            .iter()
            .map(|e| RevealEntrySummary {
                lane: e.lane.clone(),
                path: e.path.clone(),
                artifact: e.artifact.clone(),
            })
            .collect(),
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }
    let query = query.unwrap_or("");
    match status {
        "ambiguous" => println!("Multiple reveal files match '{}':", query),
        "type-mismatch" => println!(
            "Reveal query '{}' has no match of the requested type.",
            query
        ),
        _ => println!("No reveal file matched '{}'.", query),
    }
    for entry in &output.entries {
        println!("  - {} => {}", entry.lane, entry.path);
        print_artifact(&entry.artifact);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_does_not_follow_directory_links_or_cycles() {
        let root = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        fs::write(
            root.path().join("local.recur.md"),
            "artifact.type = skill\n",
        )
        .unwrap();
        fs::write(
            outside.path().join("external.recur.md"),
            "artifact.type = agent\n",
        )
        .unwrap();
        for (name, target) in [("escape", outside.path()), ("cycle", root.path())] {
            let link = root.path().join(name);
            #[cfg(windows)]
            {
                // Junctions exercise Windows reparse-point traversal without symlink privileges.
                let output = std::process::Command::new("powershell")
                    .args(["-NoProfile", "-NonInteractive", "-Command",
                        "$ErrorActionPreference = 'Stop'; New-Item -ItemType Junction -Path $env:RECUR_TEST_LINK -Target $env:RECUR_TEST_TARGET | Out-Null"])
                    .env("RECUR_TEST_LINK", &link).env("RECUR_TEST_TARGET", target)
                    .output().unwrap();
                assert!(
                    output.status.success(),
                    "{}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            #[cfg(unix)]
            std::os::unix::fs::symlink(target, &link).unwrap();
        }
        let canonical = root.path().canonicalize().unwrap();
        let entries =
            discover_reveal_entries(&canonical, ".recur.md", &TypePolicy::default(), None, &[])
                .unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].lane, "local");
    }

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
