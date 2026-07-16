//! Read-only eventness warp status queries.

use anyhow::Context;
use clap::Subcommand;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

const SCHEMA: &str = "warp-status-v1";

#[derive(Subcommand)]
pub enum WarpSubcommand {
    /// Score one lane from its file eventness and trace-id role evidence
    Status {
        /// Lane prefix such as demo.project.good
        lane: String,
    },

    /// Explain the evidence and residuals behind a lane verdict
    Explain {
        /// Lane prefix such as demo.project.good
        lane: String,
    },

    /// Suggest the next read-only management action for a lane
    Next {
        /// Lane prefix such as demo.project.good
        lane: String,
    },

    /// Classify one lane's evidence as collapsible, interesting, blocked, or active
    CollapsePlan {
        /// Lane prefix such as demo.project.good
        lane: String,
    },

    /// Show the active read-only Warp suffix policy
    Config,
}

#[derive(Clone)]
struct SuffixPolicy {
    complete: Vec<String>,
    interesting: Vec<String>,
    blocked: Vec<String>,
}

#[derive(Clone, Serialize)]
struct WarpFile {
    path: String,
    state: String,
    age_days: u64,
}

#[derive(Serialize)]
struct TraceIdRoles {
    define: u64,
    consume: u64,
    produce: u64,
    trigger: u64,
}

#[derive(Serialize)]
struct WarpSignal {
    name: String,
    weight: f64,
    evidence: Vec<String>,
}

#[derive(Serialize)]
struct WarpResidual {
    name: String,
    weight: f64,
    evidence: Vec<String>,
    blocker: bool,
}

#[derive(Serialize)]
struct WarpNextAction {
    kind: String,
    lane: String,
    reason: String,
}

#[derive(Serialize)]
struct WarpStatusOutput {
    schema: &'static str,
    lane: String,
    scope: String,
    root: String,
    verdict: String,
    objective: f64,
    files: Vec<WarpFile>,
    state_suffixes: BTreeMap<String, u64>,
    state_groups: BTreeMap<String, u64>,
    trace_id_roles: TraceIdRoles,
    signals: Vec<WarpSignal>,
    residuals: Vec<WarpResidual>,
    next_actions: Vec<WarpNextAction>,
}

#[derive(Serialize)]
struct WarpNextOutput {
    schema: &'static str,
    lane: String,
    verdict: String,
    objective: f64,
    next_actions: Vec<WarpNextAction>,
}

#[derive(Serialize)]
struct WarpCollapsePlanOutput {
    schema: &'static str,
    lane: String,
    scope: String,
    verdict: String,
    objective: f64,
    collapse_known: Vec<WarpFile>,
    preserve_interesting: Vec<WarpFile>,
    blockers: Vec<WarpFile>,
    ambiguous: Vec<WarpFile>,
}

#[derive(Serialize)]
struct WarpConfigOutput {
    schema: &'static str,
    root: String,
    active_suffixes: Vec<String>,
    complete_suffixes: Vec<String>,
    interesting_suffixes: Vec<String>,
    blocked_suffixes: Vec<String>,
}

pub fn execute(command: WarpSubcommand, dir: PathBuf, json: bool) -> anyhow::Result<()> {
    let root = resolve_root(dir)?;
    match command {
        WarpSubcommand::Status { lane } => {
            let output = status(&root, &lane)?;
            emit(&output, json)
        }
        WarpSubcommand::Explain { lane } => {
            let output = status(&root, &lane)?;
            emit_explain(&output, json)
        }
        WarpSubcommand::Next { lane } => {
            let output = status(&root, &lane)?;
            emit_next(&output, json)
        }
        WarpSubcommand::CollapsePlan { lane } => {
            let output = collapse_plan(&root, &lane)?;
            emit_collapse_plan(&output, json)
        }
        WarpSubcommand::Config => {
            let output = config(&root)?;
            emit_config(&output, json)
        }
    }
}

fn collapse_plan(root: &Path, lane: &str) -> anyhow::Result<WarpCollapsePlanOutput> {
    let status = status(root, lane)?;
    let policy = load_suffix_policy(root)?;
    let mut collapse_known = Vec::new();
    let mut preserve_interesting = Vec::new();
    let mut blockers = Vec::new();
    let mut ambiguous = Vec::new();

    for file in &status.files {
        let absolute = root.join(&file.path);
        let text = fs::read_to_string(&absolute)
            .with_context(|| format!("failed to read '{}'", absolute.display()))?;
        let group = group_for_state(&file.state, &policy);
        if group == "blocked" || file_contains_blocker(&text) {
            blockers.push(file.clone());
        } else if group == "complete" {
            collapse_known.push(file.clone());
        } else if group == "interesting" {
            preserve_interesting.push(file.clone());
        } else {
            ambiguous.push(file.clone());
        }
    }

    Ok(WarpCollapsePlanOutput {
        schema: "warp-collapse-plan-v1",
        lane: status.lane,
        scope: status.scope,
        verdict: status.verdict,
        objective: status.objective,
        collapse_known,
        preserve_interesting,
        blockers,
        ambiguous,
    })
}

fn config(root: &Path) -> anyhow::Result<WarpConfigOutput> {
    let policy = load_suffix_policy(root)?;
    Ok(WarpConfigOutput {
        schema: "warp-config-v1",
        root: root.display().to_string(),
        active_suffixes: vec!["current".to_string()],
        complete_suffixes: policy.complete,
        interesting_suffixes: policy.interesting,
        blocked_suffixes: policy.blocked,
    })
}

fn resolve_root(dir: PathBuf) -> anyhow::Result<PathBuf> {
    let root = if dir.is_absolute() {
        dir
    } else {
        std::env::current_dir()?.join(dir)
    };
    if !root.is_dir() {
        anyhow::bail!("invalid --dir '{}': directory not found", root.display());
    }
    Ok(root)
}

fn status(root: &Path, raw_lane: &str) -> anyhow::Result<WarpStatusOutput> {
    let lane = raw_lane.trim();
    if lane.is_empty() {
        anyhow::bail!("lane must not be blank");
    }
    let policy = load_suffix_policy(root)?;
    let mut files = collect_lane_files(root, lane, &policy)?;
    if files.is_empty() {
        anyhow::bail!("no eventness files found for lane '{}'", lane);
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));

    let mut state_suffixes = BTreeMap::new();
    let mut state_groups = BTreeMap::from([
        ("active".to_string(), 0),
        ("complete".to_string(), 0),
        ("interesting".to_string(), 0),
        ("blocked".to_string(), 0),
        ("other".to_string(), 0),
    ]);
    let mut roles = TraceIdRoles {
        define: 0,
        consume: 0,
        produce: 0,
        trigger: 0,
    };
    let mut blocked_evidence = Vec::new();

    for file in &files {
        *state_suffixes.entry(file.state.clone()).or_insert(0) += 1;
        let group = group_for_state(&file.state, &policy);
        *state_groups.entry(group.to_string()).or_insert(0) += 1;
        let absolute = root.join(&file.path);
        let text = fs::read_to_string(&absolute)
            .with_context(|| format!("failed to read '{}'", absolute.display()))?;
        count_roles(&text, &mut roles);
        if file_contains_blocker(&text) {
            *state_groups.entry("blocked".to_string()).or_insert(0) += 1;
            blocked_evidence.push(file.path.clone());
        }
    }

    let complete = state_groups["complete"];
    let interesting = state_groups["interesting"];
    let blocked = state_groups["blocked"];
    let mut signals = Vec::new();
    if complete > 0 {
        signals.push(WarpSignal {
            name: "complete-state-present".to_string(),
            weight: -1.0,
            evidence: files
                .iter()
                .filter(|file| group_for_state(&file.state, &policy) == "complete")
                .map(|file| file.path.clone())
                .collect(),
        });
    }
    if roles.define + roles.consume + roles.produce + roles.trigger > 0 {
        signals.push(WarpSignal {
            name: "trace-id-roles-present".to_string(),
            weight: -0.5,
            evidence: files.iter().map(|file| file.path.clone()).collect(),
        });
    }

    let mut residuals = Vec::new();
    let mut actions = Vec::new();
    if blocked > 0 {
        residuals.push(WarpResidual {
            name: "external-blocker".to_string(),
            weight: 5.0,
            evidence: blocked_evidence,
            blocker: true,
        });
        actions.push(WarpNextAction {
            kind: "await-approval".to_string(),
            lane: lane.to_string(),
            reason: "a blocker marker requires an external operator or event".to_string(),
        });
    } else {
        if complete == 0 {
            residuals.push(WarpResidual {
                name: "missing-verification".to_string(),
                weight: 1.0,
                evidence: Vec::new(),
                blocker: false,
            });
        }
        if interesting > 0 {
            residuals.push(WarpResidual {
                name: "unresolved-interest".to_string(),
                weight: 1.0,
                evidence: files
                    .iter()
                    .filter(|file| group_for_state(&file.state, &policy) == "interesting")
                    .map(|file| file.path.clone())
                    .collect(),
                blocker: false,
            });
        }
        if !residuals.is_empty() {
            actions.push(WarpNextAction {
                kind: "verify".to_string(),
                lane: lane.to_string(),
                reason: "record verification or resolve the remaining eventness pressure"
                    .to_string(),
            });
        }
    }
    let objective = residuals.iter().map(|residual| residual.weight).sum();
    let verdict = if blocked > 0 {
        "blocked"
    } else if residuals.is_empty() {
        "optimum"
    } else {
        "sub_optimum"
    };

    Ok(WarpStatusOutput {
        schema: SCHEMA,
        lane: lane.to_string(),
        scope: format!("{lane}.**"),
        root: root.display().to_string(),
        verdict: verdict.to_string(),
        objective,
        files,
        state_suffixes,
        state_groups,
        trace_id_roles: roles,
        signals,
        residuals,
        next_actions: actions,
    })
}

fn load_suffix_policy(root: &Path) -> anyhow::Result<SuffixPolicy> {
    let defaults = SuffixPolicy {
        complete: vec!["complete".to_string()],
        interesting: vec!["strange".to_string()],
        blocked: vec!["blocked".to_string()],
    };
    let config_path = root.join(".recur").join("config.toml");
    if !config_path.is_file() {
        return Ok(defaults);
    }
    let text = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read '{}'", config_path.display()))?;
    let value: toml::Value = toml::from_str(&text)
        .with_context(|| format!("failed to parse '{}'", config_path.display()))?;
    let Some(suffixes) = value
        .get("warp")
        .and_then(toml::Value::as_table)
        .and_then(|warp| warp.get("suffixes"))
        .and_then(toml::Value::as_table)
    else {
        return Ok(defaults);
    };
    Ok(SuffixPolicy {
        complete: strings_at(suffixes, "complete").unwrap_or(defaults.complete),
        interesting: strings_at(suffixes, "interesting").unwrap_or(defaults.interesting),
        blocked: strings_at(suffixes, "blocked").unwrap_or(defaults.blocked),
    })
}

fn strings_at(table: &toml::value::Table, key: &str) -> Option<Vec<String>> {
    table.get(key).and_then(toml::Value::as_array).map(|items| {
        items
            .iter()
            .filter_map(toml::Value::as_str)
            .map(|value| value.to_ascii_lowercase())
            .collect()
    })
}

fn collect_lane_files(
    root: &Path,
    lane: &str,
    policy: &SuffixPolicy,
) -> anyhow::Result<Vec<WarpFile>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(keep_entry)
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        let Some(state) = state_from_name(name, policy) else {
            continue;
        };
        let stem = name.strip_suffix(".md").unwrap_or(name);
        if !stem.starts_with(&format!("{lane}.")) {
            continue;
        }
        let relative = path.strip_prefix(root).unwrap_or(path);
        let age_days = fs::metadata(path)
            .and_then(|metadata| metadata.modified())
            .ok()
            .and_then(|modified| SystemTime::now().duration_since(modified).ok())
            .map(|age| age.as_secs() / 86_400)
            .unwrap_or(0);
        files.push(WarpFile {
            path: normalize_path(relative),
            state,
            age_days,
        });
    }
    Ok(files)
}

fn keep_entry(entry: &DirEntry) -> bool {
    entry.file_name().to_str() != Some(".recur")
}

fn state_from_name(name: &str, policy: &SuffixPolicy) -> Option<String> {
    let stem = name.strip_suffix(".md")?;
    let state = stem.rsplit('.').next()?.to_ascii_lowercase();
    if state == "current"
        || policy.complete.contains(&state)
        || policy.interesting.contains(&state)
        || policy.blocked.contains(&state)
    {
        Some(state)
    } else {
        None
    }
}

fn group_for_state(state: &str, policy: &SuffixPolicy) -> &'static str {
    if state == "current" {
        "active"
    } else if policy.complete.iter().any(|value| value == state) {
        "complete"
    } else if policy.interesting.iter().any(|value| value == state) {
        "interesting"
    } else if policy.blocked.iter().any(|value| value == state) {
        "blocked"
    } else {
        "other"
    }
}

fn count_roles(text: &str, roles: &mut TraceIdRoles) {
    for line in text.lines().map(str::to_ascii_lowercase) {
        if line.contains("define") {
            roles.define += 1;
        }
        if line.contains("publish") || line.contains("produce") {
            roles.produce += 1;
        }
        if line.contains("subscribe") || line.contains("consume") {
            roles.consume += 1;
        }
        if line.contains("trigger") {
            roles.trigger += 1;
        }
    }
}

fn file_contains_blocker(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("blocker") || lower.contains("operator approval")
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn emit(output: &WarpStatusOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
        return Ok(());
    }
    println!("Warp status for {}", output.lane);
    println!("  verdict: {}", output.verdict);
    println!("  objective: {}", output.objective);
    println!("  files: {}", output.files.len());
    if output.residuals.is_empty() {
        println!("  residuals: none");
    } else {
        println!("  residuals:");
        for residual in &output.residuals {
            println!("    - {} ({})", residual.name, residual.weight);
        }
    }
    Ok(())
}

fn emit_explain(output: &WarpStatusOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
        return Ok(());
    }
    println!("Warp explanation for {}", output.lane);
    println!("  verdict: {}", output.verdict);
    println!("  objective: {}", output.objective);
    println!("  evidence:");
    for signal in &output.signals {
        println!("    + {} ({})", signal.name, signal.weight);
        for path in &signal.evidence {
            println!("      - {}", path);
        }
    }
    if output.residuals.is_empty() {
        println!("  residuals: none");
    } else {
        println!("  residuals:");
        for residual in &output.residuals {
            println!("    - {} ({})", residual.name, residual.weight);
            for path in &residual.evidence {
                println!("      - {}", path);
            }
        }
    }
    Ok(())
}

fn emit_next(output: &WarpStatusOutput, json: bool) -> anyhow::Result<()> {
    let next = WarpNextOutput {
        schema: "warp-next-v1",
        lane: output.lane.clone(),
        verdict: output.verdict.clone(),
        objective: output.objective,
        next_actions: output
            .next_actions
            .iter()
            .map(|action| WarpNextAction {
                kind: action.kind.clone(),
                lane: action.lane.clone(),
                reason: action.reason.clone(),
            })
            .collect(),
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&next)?);
        return Ok(());
    }
    println!("Warp next for {}", next.lane);
    if next.next_actions.is_empty() {
        println!("  no action suggested");
    } else {
        for action in &next.next_actions {
            println!("  - {}: {}", action.kind, action.reason);
        }
    }
    Ok(())
}

fn emit_collapse_plan(output: &WarpCollapsePlanOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
        return Ok(());
    }
    println!("Warp collapse plan for {}", output.lane);
    println!("  verdict: {}", output.verdict);
    println!("  collapse known: {}", output.collapse_known.len());
    println!(
        "  preserve interesting: {}",
        output.preserve_interesting.len()
    );
    println!("  blockers: {}", output.blockers.len());
    println!("  ambiguous: {}", output.ambiguous.len());
    Ok(())
}

fn emit_config(output: &WarpConfigOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
        return Ok(());
    }
    println!("Warp config for {}", output.root);
    println!("  active: {}", output.active_suffixes.join(", "));
    println!("  complete: {}", output.complete_suffixes.join(", "));
    println!("  interesting: {}", output.interesting_suffixes.join(", "));
    println!("  blocked: {}", output.blocked_suffixes.join(", "));
    Ok(())
}
