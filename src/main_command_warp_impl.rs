//! Read-only eventness warp status queries.

use anyhow::Context;
use clap::Subcommand;
use recur::warp_bubble::{
    WarpBubbleMap, WarpRequiredSlice, WarpSliceLayer, BUBBLE_MAP_SCHEMA, MAP_VIEW_SCHEMA,
    MERGE_SCHEMA, SLICE_LAYER_SCHEMA,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
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

    /// Show one declared final Warp bubble map
    Map {
        /// Stable Warp identity such as demo.release
        warp: String,
    },

    /// Compose accepted Slice layers over one final Warp bubble map
    Merge {
        /// Stable Warp identity such as demo.release
        warp: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    bubble: Option<WarpMergeOutput>,
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

#[derive(Clone)]
struct LocatedWarpLayer {
    path: String,
    layer: WarpSliceLayer,
}

#[derive(Serialize)]
struct WarpMapOutput {
    schema: &'static str,
    warp_id: String,
    root: String,
    manifest: String,
    manifest_schema: String,
    required_slices: Vec<WarpRequiredSlice>,
}

#[derive(Clone, Serialize)]
struct WarpProjectionIssue {
    slice_id: String,
    reason: String,
    evidence: Vec<String>,
}

#[derive(Clone, Serialize)]
struct WarpMergeOutput {
    schema: &'static str,
    warp_id: String,
    root: String,
    manifest: String,
    state: String,
    counts: BTreeMap<String, usize>,
    covered: Vec<String>,
    pending: Vec<String>,
    blocked: Vec<WarpProjectionIssue>,
    stale_contract: Vec<WarpProjectionIssue>,
    conflicts: Vec<WarpProjectionIssue>,
    layers: Vec<String>,
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
        WarpSubcommand::Map { warp } => {
            let output = bubble_map(&root, &warp)?;
            emit_map(&output, json)
        }
        WarpSubcommand::Merge { warp } => {
            let output = merge_bubble(&root, &warp)?;
            emit_merge(&output, json)
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

fn bubble_map(root: &Path, raw_warp: &str) -> anyhow::Result<WarpMapOutput> {
    let warp = checked_warp_id(raw_warp)?;
    let (manifest, map) = load_bubble_map(root, warp)?.ok_or_else(|| {
        anyhow::anyhow!(
            "no Warp bubble map found for '{}'; expected '{}.warp-map.json'",
            warp,
            warp
        )
    })?;
    Ok(WarpMapOutput {
        schema: MAP_VIEW_SCHEMA,
        warp_id: map.warp_id,
        root: root.display().to_string(),
        manifest,
        manifest_schema: map.schema,
        required_slices: map.required_slices,
    })
}

fn merge_bubble(root: &Path, raw_warp: &str) -> anyhow::Result<WarpMergeOutput> {
    let warp = checked_warp_id(raw_warp)?;
    let (manifest, map) = load_bubble_map(root, warp)?.ok_or_else(|| {
        anyhow::anyhow!(
            "no Warp bubble map found for '{}'; expected '{}.warp-map.json'",
            warp,
            warp
        )
    })?;
    let layers = load_warp_layers(root, warp)?;
    Ok(compose_bubble(root, manifest, map, layers))
}

fn checked_warp_id(raw_warp: &str) -> anyhow::Result<&str> {
    let warp = raw_warp.trim();
    if warp.is_empty() {
        anyhow::bail!("Warp identity must not be blank");
    }
    Ok(warp)
}

fn load_bubble_map(root: &Path, warp: &str) -> anyhow::Result<Option<(String, WarpBubbleMap)>> {
    let expected_name = format!("{warp}.warp-map.json");
    let mut matches = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(keep_entry)
        .filter_map(Result::ok)
    {
        if entry.file_type().is_file() && entry.file_name().to_str() == Some(&expected_name) {
            matches.push(entry.path().to_path_buf());
        }
    }
    matches.sort();
    if matches.len() > 1 {
        anyhow::bail!(
            "multiple Warp bubble maps found for '{}': {}",
            warp,
            matches
                .iter()
                .map(|path| normalize_path(path.strip_prefix(root).unwrap_or(path)))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let Some(path) = matches.pop() else {
        return Ok(None);
    };
    let text = fs::read_to_string(&path)
        .with_context(|| format!("failed to read '{}'", path.display()))?;
    let map: WarpBubbleMap = serde_json::from_str(&text)
        .with_context(|| format!("failed to parse Warp bubble map '{}'", path.display()))?;
    validate_bubble_map(&map, warp, &path)?;
    let relative = normalize_path(path.strip_prefix(root).unwrap_or(&path));
    Ok(Some((relative, map)))
}

fn validate_bubble_map(map: &WarpBubbleMap, warp: &str, path: &Path) -> anyhow::Result<()> {
    if map.schema != BUBBLE_MAP_SCHEMA {
        anyhow::bail!(
            "unsupported Warp bubble map schema '{}' in '{}'; expected '{}'",
            map.schema,
            path.display(),
            BUBBLE_MAP_SCHEMA
        );
    }
    if map.warp_id != warp {
        anyhow::bail!(
            "Warp identity '{}' in '{}' does not match requested '{}'",
            map.warp_id,
            path.display(),
            warp
        );
    }
    let mut ids = BTreeSet::new();
    for required in &map.required_slices {
        if required.slice_id.trim().is_empty() || required.contract_hash.trim().is_empty() {
            anyhow::bail!(
                "Warp bubble map '{}' contains a blank Slice identity or contract hash",
                path.display()
            );
        }
        if !ids.insert(required.slice_id.clone()) {
            anyhow::bail!(
                "Warp bubble map '{}' contains duplicate Slice '{}'",
                path.display(),
                required.slice_id
            );
        }
        let gates = required
            .evidence_gates
            .iter()
            .map(|gate| gate.trim())
            .collect::<BTreeSet<_>>();
        if gates.len() != required.evidence_gates.len() || gates.contains("") {
            anyhow::bail!(
                "Warp Slice '{}' in '{}' has blank or duplicate evidence gates",
                required.slice_id,
                path.display()
            );
        }
    }
    for required in &map.required_slices {
        let dependencies = required.depends_on.iter().collect::<BTreeSet<_>>();
        if dependencies.len() != required.depends_on.len() {
            anyhow::bail!(
                "Warp Slice '{}' in '{}' has duplicate dependencies",
                required.slice_id,
                path.display()
            );
        }
        for dependency in &required.depends_on {
            if dependency == &required.slice_id || !ids.contains(dependency) {
                anyhow::bail!(
                    "Warp Slice '{}' in '{}' has invalid dependency '{}'",
                    required.slice_id,
                    path.display(),
                    dependency
                );
            }
        }
    }
    let mut remaining = map
        .required_slices
        .iter()
        .map(|required| {
            (
                required.slice_id.clone(),
                required.depends_on.iter().cloned().collect::<BTreeSet<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    while !remaining.is_empty() {
        let ready = remaining
            .iter()
            .filter(|(_, dependencies)| dependencies.is_empty())
            .map(|(slice, _)| slice.clone())
            .collect::<Vec<_>>();
        if ready.is_empty() {
            anyhow::bail!(
                "Warp bubble map '{}' contains a dependency cycle involving {}",
                path.display(),
                remaining.keys().cloned().collect::<Vec<_>>().join(", ")
            );
        }
        for slice in &ready {
            remaining.remove(slice);
        }
        for dependencies in remaining.values_mut() {
            for slice in &ready {
                dependencies.remove(slice);
            }
        }
    }
    Ok(())
}

fn load_warp_layers(root: &Path, warp: &str) -> anyhow::Result<Vec<LocatedWarpLayer>> {
    let prefix = format!("{warp}.");
    let mut paths = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(keep_entry)
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let Some(name) = entry.file_name().to_str() else {
            continue;
        };
        if name.starts_with(&prefix) && name.ends_with(".warp-layer.json") {
            paths.push(entry.path().to_path_buf());
        }
    }
    paths.sort();
    let mut layers = Vec::new();
    for path in paths {
        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read '{}'", path.display()))?;
        let layer: WarpSliceLayer = serde_json::from_str(&text)
            .with_context(|| format!("failed to parse Warp Slice layer '{}'", path.display()))?;
        if layer.schema != SLICE_LAYER_SCHEMA {
            anyhow::bail!(
                "unsupported Warp Slice layer schema '{}' in '{}'; expected '{}'",
                layer.schema,
                path.display(),
                SLICE_LAYER_SCHEMA
            );
        }
        if layer.warp_id != warp {
            anyhow::bail!(
                "Warp identity '{}' in '{}' does not match requested '{}'",
                layer.warp_id,
                path.display(),
                warp
            );
        }
        if layer.slice_id.trim().is_empty()
            || layer.contract_hash.trim().is_empty()
            || layer.attempt_id.trim().is_empty()
        {
            anyhow::bail!(
                "Warp Slice layer '{}' contains a blank Slice, contract, or attempt identity",
                path.display()
            );
        }
        layers.push(LocatedWarpLayer {
            path: normalize_path(path.strip_prefix(root).unwrap_or(&path)),
            layer,
        });
    }
    Ok(layers)
}

fn compose_bubble(
    root: &Path,
    manifest: String,
    map: WarpBubbleMap,
    layers: Vec<LocatedWarpLayer>,
) -> WarpMergeOutput {
    let required_by_id = map
        .required_slices
        .iter()
        .map(|required| (required.slice_id.clone(), required))
        .collect::<BTreeMap<_, _>>();
    let mut candidates = BTreeSet::new();
    let mut pending = Vec::new();
    let mut blocked = Vec::new();
    let mut stale_contract = Vec::new();
    let mut conflicts = Vec::new();
    let mut by_attempt: BTreeMap<(String, String), Vec<&LocatedWarpLayer>> = BTreeMap::new();
    for layer in &layers {
        by_attempt
            .entry((layer.layer.slice_id.clone(), layer.layer.attempt_id.clone()))
            .or_default()
            .push(layer);
    }
    let mut normalized_layers = Vec::new();
    let mut layer_ids = Vec::new();
    for ((slice_id, attempt_id), attempts) in by_attempt {
        let first = attempts[0];
        if attempts.iter().all(|item| item.layer == first.layer) {
            normalized_layers.push(first);
            layer_ids.push(format!("{}@{}", slice_id, attempt_id));
        } else {
            conflicts.push(WarpProjectionIssue {
                slice_id,
                reason: format!(
                    "attempt '{}' was replayed with incompatible layer content",
                    attempt_id
                ),
                evidence: attempts.iter().map(|item| item.path.clone()).collect(),
            });
        }
    }
    let mut by_slice: BTreeMap<String, Vec<&LocatedWarpLayer>> = BTreeMap::new();
    for layer in normalized_layers {
        by_slice
            .entry(layer.layer.slice_id.clone())
            .or_default()
            .push(layer);
    }

    for (slice_id, located) in &by_slice {
        if !required_by_id.contains_key(slice_id) {
            conflicts.push(WarpProjectionIssue {
                slice_id: slice_id.clone(),
                reason: "completion layer targets a Slice absent from the final bubble map"
                    .to_string(),
                evidence: located.iter().map(|item| item.path.clone()).collect(),
            });
        }
    }

    for required in &map.required_slices {
        let located = by_slice
            .get(&required.slice_id)
            .cloned()
            .unwrap_or_default();
        if located.is_empty() {
            pending.push(required.slice_id.clone());
            continue;
        }
        let accepted = located
            .iter()
            .filter(|item| item.layer.result_state.eq_ignore_ascii_case("accepted"))
            .copied()
            .collect::<Vec<_>>();
        let stale = accepted
            .iter()
            .filter(|item| item.layer.contract_hash != required.contract_hash)
            .copied()
            .collect::<Vec<_>>();
        if !stale.is_empty() {
            stale_contract.push(WarpProjectionIssue {
                slice_id: required.slice_id.clone(),
                reason: format!(
                    "accepted layer contract does not match required '{}'",
                    required.contract_hash
                ),
                evidence: stale.iter().map(|item| item.path.clone()).collect(),
            });
        }
        let current = accepted
            .iter()
            .filter(|item| item.layer.contract_hash == required.contract_hash)
            .copied()
            .collect::<Vec<_>>();
        let valid = current
            .iter()
            .filter(|item| {
                !item.layer.result_hash.trim().is_empty()
                    && required.evidence_gates.iter().all(|gate| {
                        item.layer
                            .evidence
                            .get(gate)
                            .is_some_and(|references| !references.is_empty())
                    })
            })
            .copied()
            .collect::<Vec<_>>();
        let result_hashes = valid
            .iter()
            .map(|item| item.layer.result_hash.clone())
            .collect::<BTreeSet<_>>();
        if result_hashes.len() > 1 {
            conflicts.push(WarpProjectionIssue {
                slice_id: required.slice_id.clone(),
                reason: format!(
                    "accepted layers disagree on result hashes: {}",
                    result_hashes.into_iter().collect::<Vec<_>>().join(", ")
                ),
                evidence: valid.iter().map(|item| item.path.clone()).collect(),
            });
        } else if stale.is_empty() && result_hashes.len() == 1 {
            candidates.insert(required.slice_id.clone());
        } else if current.len() > valid.len() {
            let missing = required
                .evidence_gates
                .iter()
                .filter(|gate| {
                    !current.iter().any(|item| {
                        item.layer
                            .evidence
                            .get(*gate)
                            .is_some_and(|references| !references.is_empty())
                    })
                })
                .cloned()
                .collect::<Vec<_>>();
            blocked.push(WarpProjectionIssue {
                slice_id: required.slice_id.clone(),
                reason: if missing.is_empty() {
                    "accepted layer is missing a result hash or a complete evidence set".to_string()
                } else {
                    format!(
                        "accepted layer is missing evidence gates: {}",
                        missing.join(", ")
                    )
                },
                evidence: current.iter().map(|item| item.path.clone()).collect(),
            });
        } else if accepted.is_empty() {
            blocked.push(WarpProjectionIssue {
                slice_id: required.slice_id.clone(),
                reason: located
                    .iter()
                    .filter_map(|item| item.layer.reason.clone())
                    .next()
                    .unwrap_or_else(|| "no accepted completion layer is available".to_string()),
                evidence: located.iter().map(|item| item.path.clone()).collect(),
            });
        }
    }

    loop {
        let dependency_blocked = map
            .required_slices
            .iter()
            .filter(|required| candidates.contains(&required.slice_id))
            .filter_map(|required| {
                let missing = required
                    .depends_on
                    .iter()
                    .filter(|dependency| !candidates.contains(*dependency))
                    .cloned()
                    .collect::<Vec<_>>();
                (!missing.is_empty()).then(|| (required.slice_id.clone(), missing))
            })
            .collect::<Vec<_>>();
        if dependency_blocked.is_empty() {
            break;
        }
        for (slice_id, missing) in dependency_blocked {
            candidates.remove(&slice_id);
            blocked.push(WarpProjectionIssue {
                slice_id,
                reason: format!(
                    "required dependencies are not covered: {}",
                    missing.join(", ")
                ),
                evidence: Vec::new(),
            });
        }
    }

    let covered = candidates.into_iter().collect::<Vec<_>>();
    pending.sort();
    blocked.sort_by(|left, right| left.slice_id.cmp(&right.slice_id));
    stale_contract.sort_by(|left, right| left.slice_id.cmp(&right.slice_id));
    conflicts.sort_by(|left, right| left.slice_id.cmp(&right.slice_id));
    let state = if !conflicts.is_empty() || !stale_contract.is_empty() {
        "exploded"
    } else if !blocked.is_empty() {
        "blocked"
    } else if covered.len() == map.required_slices.len() && pending.is_empty() {
        "complete"
    } else {
        "incomplete"
    };
    let counts = BTreeMap::from([
        ("required".to_string(), map.required_slices.len()),
        ("covered".to_string(), covered.len()),
        ("pending".to_string(), pending.len()),
        ("blocked".to_string(), blocked.len()),
        ("stale_contract".to_string(), stale_contract.len()),
        ("conflicting".to_string(), conflicts.len()),
    ]);
    layer_ids.sort();
    WarpMergeOutput {
        schema: MERGE_SCHEMA,
        warp_id: map.warp_id,
        root: root.display().to_string(),
        manifest,
        state: state.to_string(),
        counts,
        covered,
        pending,
        blocked,
        stale_contract,
        conflicts,
        layers: layer_ids,
    }
}

fn status(root: &Path, raw_lane: &str) -> anyhow::Result<WarpStatusOutput> {
    let lane = raw_lane.trim();
    if lane.is_empty() {
        anyhow::bail!("lane must not be blank");
    }
    let policy = load_suffix_policy(root)?;
    let mut files = collect_lane_files(root, lane, &policy)?;
    let bubble = match load_bubble_map(root, lane)? {
        Some((manifest, map)) => Some(compose_bubble(
            root,
            manifest,
            map,
            load_warp_layers(root, lane)?,
        )),
        None => None,
    };
    if files.is_empty() && bubble.is_none() {
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
        if complete == 0 && bubble.is_none() {
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
    if let Some(projection) = &bubble {
        match projection.state.as_str() {
            "complete" => signals.push(WarpSignal {
                name: "warp-bubble-complete".to_string(),
                weight: -1.0,
                evidence: projection.layers.clone(),
            }),
            "exploded" => {
                residuals.push(WarpResidual {
                    name: "warp-bubble-exploded".to_string(),
                    weight: 5.0,
                    evidence: projection.layers.clone(),
                    blocker: true,
                });
                actions.push(WarpNextAction {
                    kind: "evolve-warp".to_string(),
                    lane: lane.to_string(),
                    reason: "the composed bubble contains stale or conflicting accepted layers"
                        .to_string(),
                });
            }
            "blocked" => {
                residuals.push(WarpResidual {
                    name: "warp-bubble-blocked".to_string(),
                    weight: 5.0,
                    evidence: projection.layers.clone(),
                    blocker: true,
                });
                actions.push(WarpNextAction {
                    kind: "resolve-slice-blocker".to_string(),
                    lane: lane.to_string(),
                    reason: "one or more required Slices cannot contribute accepted coverage"
                        .to_string(),
                });
            }
            _ => {
                residuals.push(WarpResidual {
                    name: "incomplete-warp-coverage".to_string(),
                    weight: 1.0,
                    evidence: projection.layers.clone(),
                    blocker: false,
                });
                actions.push(WarpNextAction {
                    kind: "complete-slice".to_string(),
                    lane: lane.to_string(),
                    reason: format!(
                        "{} required Slice layer(s) remain pending",
                        projection.pending.len()
                    ),
                });
            }
        }
    }
    let objective = residuals.iter().map(|residual| residual.weight).sum();
    let verdict = if residuals.iter().any(|residual| residual.blocker) {
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
        bubble,
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

fn emit_map(output: &WarpMapOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
        return Ok(());
    }
    println!("Warp bubble map for {}", output.warp_id);
    println!("  manifest: {}", output.manifest);
    println!("  required slices: {}", output.required_slices.len());
    for required in &output.required_slices {
        println!("  - {} [{}]", required.slice_id, required.contract_hash);
    }
    Ok(())
}

fn emit_merge(output: &WarpMergeOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
        return Ok(());
    }
    println!("Warp bubble merge for {}", output.warp_id);
    println!("  state: {}", output.state);
    println!(
        "  coverage: {}/{}",
        output.counts["covered"], output.counts["required"]
    );
    println!("  pending: {}", output.counts["pending"]);
    println!("  blocked: {}", output.counts["blocked"]);
    println!("  stale contracts: {}", output.counts["stale_contract"]);
    println!("  conflicts: {}", output.counts["conflicting"]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn required(slice_id: &str, depends_on: &[&str]) -> WarpRequiredSlice {
        WarpRequiredSlice {
            slice_id: slice_id.to_string(),
            contract_hash: format!("contract-{slice_id}"),
            depends_on: depends_on.iter().map(|value| value.to_string()).collect(),
            evidence_gates: vec!["tests".to_string()],
        }
    }

    fn located(slice_id: &str, attempt_id: &str, result_hash: &str) -> LocatedWarpLayer {
        LocatedWarpLayer {
            path: format!("{slice_id}.{attempt_id}.warp-layer.json"),
            layer: WarpSliceLayer {
                schema: SLICE_LAYER_SCHEMA.to_string(),
                warp_id: "demo.release".to_string(),
                slice_id: slice_id.to_string(),
                contract_hash: format!("contract-{slice_id}"),
                attempt_id: attempt_id.to_string(),
                result_state: "accepted".to_string(),
                result_hash: result_hash.to_string(),
                evidence: BTreeMap::from([(
                    "tests".to_string(),
                    vec![format!("receipt-{slice_id}")],
                )]),
                reason: None,
            },
        }
    }

    fn map() -> WarpBubbleMap {
        WarpBubbleMap {
            schema: BUBBLE_MAP_SCHEMA.to_string(),
            warp_id: "demo.release".to_string(),
            required_slices: vec![required("alpha", &[]), required("beta", &["alpha"])],
        }
    }

    #[test]
    fn bubble_merge_is_completion_order_independent_and_idempotent() {
        let alpha = located("alpha", "attempt-a", "result-a");
        let beta = located("beta", "attempt-b", "result-b");
        let forward = compose_bubble(
            Path::new("fixture"),
            "demo.release.warp-map.json".to_string(),
            map(),
            vec![alpha.clone(), beta.clone()],
        );
        let reverse_with_retry = compose_bubble(
            Path::new("fixture"),
            "demo.release.warp-map.json".to_string(),
            map(),
            vec![beta, alpha.clone(), alpha],
        );

        assert_eq!(forward.state, "complete");
        assert_eq!(
            serde_json::to_value(&forward).unwrap(),
            serde_json::to_value(&reverse_with_retry).unwrap()
        );
    }

    #[test]
    fn incompatible_accepted_results_explode_the_bubble() {
        let output = compose_bubble(
            Path::new("fixture"),
            "demo.release.warp-map.json".to_string(),
            WarpBubbleMap {
                schema: BUBBLE_MAP_SCHEMA.to_string(),
                warp_id: "demo.release".to_string(),
                required_slices: vec![required("alpha", &[])],
            },
            vec![
                located("alpha", "attempt-a", "result-one"),
                located("alpha", "attempt-b", "result-two"),
            ],
        );

        assert_eq!(output.state, "exploded");
        assert!(output.covered.is_empty());
        assert_eq!(output.conflicts.len(), 1);
    }

    #[test]
    fn dependency_cycles_are_rejected_before_composition() {
        let cyclic = WarpBubbleMap {
            schema: BUBBLE_MAP_SCHEMA.to_string(),
            warp_id: "demo.release".to_string(),
            required_slices: vec![required("alpha", &["beta"]), required("beta", &["alpha"])],
        };

        let error = validate_bubble_map(&cyclic, "demo.release", Path::new("map.json"))
            .unwrap_err()
            .to_string();
        assert!(error.contains("dependency cycle"));
    }
}
