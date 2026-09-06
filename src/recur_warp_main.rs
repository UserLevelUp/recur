//! `recur-warp` - confirmation-gated writer for Warp Slice completion layers.

use anyhow::Context;
use clap::{Parser, Subcommand};
use recur::warp_bubble::{WarpBubbleMap, WarpSliceLayer, BUBBLE_MAP_SCHEMA, SLICE_LAYER_SCHEMA};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use walkdir::{DirEntry, WalkDir};
mod recur_warp_create;

#[derive(Parser)]
#[command(name = "recur-warp")]
#[command(about = "Write confirmed Warp Slice completion layers", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Project root containing the Warp bubble map
    #[arg(short = 'd', long, default_value = ".", global = true)]
    dir: PathBuf,

    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Preview a configured bubble map; --confirm creates it without overwriting
    Create {
        warp: String,
        #[arg(long)]
        goal: String,
        #[arg(long)]
        confirm: bool,
    },
    /// Preview a policy-aware lifecycle receipt; --confirm records a declaration
    Receipt {
        warp: String,
        slice: String,
        #[arg(long)]
        attempt_id: String,
        #[arg(long = "evidence", value_name = "GATE=REFERENCE")]
        evidence: Vec<String>,
        #[arg(long)]
        confirm: bool,
    },
    /// Plan or persist one accepted Slice completion layer
    Complete {
        /// Stable Warp identity, such as demo.release
        warp: String,

        /// Slice identity declared in the final Warp bubble map
        slice: String,

        /// Stable retry identity for this completion attempt
        #[arg(long)]
        attempt_id: String,

        /// Content or result hash produced by the completed Slice
        #[arg(long)]
        result_hash: String,

        /// Evidence binding in GATE=REFERENCE form; repeat for multiple references
        #[arg(long = "evidence", value_name = "GATE=REFERENCE")]
        evidence: Vec<String>,

        /// Persist the completion layer; without this flag the command is a dry run
        #[arg(long)]
        confirm: bool,
    },

    /// Plan or persist a superseding Warp bubble after an explosion
    Evolve {
        /// Exploded source Warp identity
        warp: String,

        /// Candidate target Warp bubble map JSON
        target_map: PathBuf,

        /// Persist the target map, carried layers, and supersession receipt
        #[arg(long)]
        confirm: bool,
    },

    /// Plan or execute archival of unambiguous completed eventness
    Collapse {
        /// Lane prefix whose eventness should be classified and collapsed
        lane: String,

        /// Archive known-complete evidence and write a collapse receipt
        #[arg(long)]
        confirm: bool,
    },
}

#[derive(Serialize)]
struct CompleteOutput {
    schema: &'static str,
    warp_id: String,
    slice_id: String,
    attempt_id: String,
    state: String,
    path: String,
    contract_hash: String,
    result_hash: String,
    evidence: BTreeMap<String, Vec<String>>,
}

#[derive(Serialize)]
struct NakReceipt<'a> {
    schema: &'static str,
    warp_id: &'a str,
    slice_id: &'a str,
    attempt_id: &'a str,
    result_state: &'static str,
    reason: String,
}

#[derive(Serialize)]
struct EvolveOutput {
    schema: &'static str,
    source_warp: String,
    target_warp: String,
    state: String,
    target_manifest: String,
    carried_slices: Vec<String>,
    invalidated_slices: Vec<String>,
    receipt: String,
}

#[derive(Serialize)]
struct SupersessionReceipt<'a> {
    schema: &'static str,
    source_warp: &'a str,
    target_warp: &'a str,
    result_state: &'static str,
    target_manifest: &'a str,
    carried_slices: &'a [String],
    invalidated_slices: &'a [String],
}

#[derive(Serialize)]
struct CollapseOutput {
    schema: &'static str,
    lane: String,
    state: String,
    collapse_known: Vec<String>,
    preserve_interesting: Vec<String>,
    blockers: Vec<String>,
    ambiguous: Vec<String>,
    archived: Vec<String>,
    receipt: String,
}

#[derive(Serialize)]
struct CollapseReceipt<'a> {
    schema: &'static str,
    lane: &'a str,
    result_state: &'static str,
    archived: &'a [String],
    preserved: &'a [String],
}

fn main() {
    let cli = Cli::parse();
    let root = if cli.dir.is_absolute() {
        cli.dir
    } else {
        match std::env::current_dir() {
            Ok(current) => current.join(cli.dir),
            Err(error) => {
                eprintln!("Error: {error}");
                process::exit(2);
            }
        }
    };
    let result = match cli.command {
        Commands::Create {
            warp,
            goal,
            confirm,
        } => recur_warp_create::create(&root, &warp, &goal, confirm).and_then(|output| {
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!(
                    "{}: {}",
                    output["state"].as_str().unwrap_or(""),
                    output["path"].as_str().unwrap_or("")
                );
            }
            Ok(())
        }),
        Commands::Receipt {
            warp,
            slice,
            attempt_id,
            evidence,
            confirm,
        } => receipt(&root, &warp, &slice, &attempt_id, &evidence, confirm).and_then(|output| {
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }),
        Commands::Complete {
            warp,
            slice,
            attempt_id,
            result_hash,
            evidence,
            confirm,
        } => complete_with_nak(
            &root,
            &warp,
            &slice,
            &attempt_id,
            &result_hash,
            &evidence,
            confirm,
        )
        .and_then(|output| emit(&output, cli.json)),
        Commands::Evolve {
            warp,
            target_map,
            confirm,
        } => evolve(&root, &warp, &target_map, confirm)
            .and_then(|output| emit_evolve(&output, cli.json)),
        Commands::Collapse { lane, confirm } => {
            collapse(&root, &lane, confirm).and_then(|output| emit_collapse(&output, cli.json))
        }
    };
    if let Err(error) = result {
        eprintln!("Error: {error:#}");
        process::exit(2);
    }
}

fn receipt(
    root: &Path,
    warp: &str,
    slice: &str,
    attempt: &str,
    values: &[String],
    confirm: bool,
) -> anyhow::Result<serde_json::Value> {
    validate_identity("Warp", warp, true)?;
    validate_identity("Slice", slice, true)?;
    validate_identity("attempt", attempt, false)?;
    let (manifest, map) = find_map(root, warp)?;
    let required = map
        .required_slices
        .iter()
        .find(|s| s.slice_id == slice)
        .ok_or_else(|| anyhow::anyhow!("unknown Slice '{}'", slice))?;
    let policy = recur::warp_policy::WarpPolicy::load(root)?;
    let suffix = policy
        .complete
        .first()
        .context("no completion suffix configured")?;
    let parent = manifest.parent().context("map has no parent")?;
    let stem = format!("{warp}.{slice}.{attempt}.receipt");
    let target = parent.join(format!("{stem}.{suffix}.md"));
    // An existing attempt in any lifecycle state must be reconciled explicitly.
    for entry in fs::read_dir(parent)? {
        let entry = entry?;
        if entry
            .file_name()
            .to_string_lossy()
            .starts_with(&format!("{stem}."))
        {
            anyhow::bail!("receipt attempt already exists or conflicts at '{}'; use a new attempt or reconcile state", entry.path().display());
        }
    }
    let evidence = parse_evidence(values)?;
    let data = serde_json::json!({"schema":"warp-lifecycle-receipt-v1", "warp_id":warp,
        "slice_id":slice, "attempt_id":attempt, "contract_hash":required.contract_hash,
        "depends_on":required.depends_on, "evidence_gates":required.evidence_gates,
        "evidence_mode":required.evidence_mode, "gate_rules":required.gate_rules,
        "evidence":evidence, "evidence_status":"declared"});
    let text = format!("# Recorded Slice completion\n\nThis receipt records a declaration. Required evidence must be validated separately.\n\nwarp.receipt = {}\n", serde_json::to_string(&data)?);
    if confirm {
        write_bytes_atomically(&target, text.as_bytes())?;
    }
    Ok(
        serde_json::json!({"schema":"recur-warp-receipt-v1", "state":if confirm {"recorded"} else {"planned"},
        "path":normalize_path(target.strip_prefix(root).unwrap_or(&target)), "policy":policy, "receipt":data, "template":text}),
    )
}

fn complete_with_nak(
    root: &Path,
    warp: &str,
    slice: &str,
    attempt_id: &str,
    result_hash: &str,
    evidence_values: &[String],
    confirm: bool,
) -> anyhow::Result<CompleteOutput> {
    match complete(
        root,
        warp,
        slice,
        attempt_id,
        result_hash,
        evidence_values,
        confirm,
    ) {
        Ok(output) => Ok(output),
        Err(error) => {
            if confirm
                && root.is_dir()
                && validate_identity("Warp", warp, true).is_ok()
                && validate_identity("Slice", slice, true).is_ok()
                && validate_identity("attempt", attempt_id, false).is_ok()
            {
                if let Err(receipt_error) =
                    write_nak_receipt(root, warp, slice, attempt_id, format!("{error:#}"))
                {
                    return Err(error.context(format!(
                        "also failed to write NAK receipt: {receipt_error:#}"
                    )));
                }
            }
            Err(error)
        }
    }
}

fn complete(
    root: &Path,
    warp: &str,
    slice: &str,
    attempt_id: &str,
    result_hash: &str,
    evidence_values: &[String],
    confirm: bool,
) -> anyhow::Result<CompleteOutput> {
    if !root.is_dir() {
        anyhow::bail!("invalid --dir '{}': directory not found", root.display());
    }
    validate_identity("Warp", warp, true)?;
    validate_identity("Slice", slice, true)?;
    validate_identity("attempt", attempt_id, false)?;
    if result_hash.trim().is_empty() {
        anyhow::bail!("result hash must not be blank");
    }
    let (manifest_path, map) = find_map(root, warp)?;
    let required = map
        .required_slices
        .iter()
        .find(|required| required.slice_id == slice)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Slice '{}' is not required by Warp bubble map '{}'",
                slice,
                manifest_path.display()
            )
        })?;
    let evidence = parse_evidence(evidence_values)?;
    validate_evidence_gates(&evidence, &required.evidence_gates)?;
    let assessments = recur::warp_evidence::gates(root, required, &evidence);
    if let Some(gate) = assessments.iter().find(|g| !g.satisfied) {
        anyhow::bail!(
            "evidence gate '{}' is {}: {}",
            gate.gate,
            gate.status,
            serde_json::to_string(gate)?
        );
    }
    reject_conflicting_result(root, warp, slice, &required.contract_hash, result_hash)?;

    let layer = WarpSliceLayer {
        schema: SLICE_LAYER_SCHEMA.to_string(),
        warp_id: warp.to_string(),
        slice_id: slice.to_string(),
        contract_hash: required.contract_hash.clone(),
        attempt_id: attempt_id.to_string(),
        result_state: "accepted".to_string(),
        result_hash: result_hash.to_string(),
        evidence: evidence.clone(),
        reason: None,
    };
    let parent = manifest_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Warp map has no parent directory"))?;
    let target = parent.join(format!("{warp}.{slice}.{attempt_id}.warp-layer.json"));
    let relative = normalize_path(target.strip_prefix(root).unwrap_or(&target));
    let state = if !confirm {
        "planned"
    } else if target.is_file() {
        let existing: WarpSliceLayer = serde_json::from_str(
            &fs::read_to_string(&target)
                .with_context(|| format!("failed to read '{}'", target.display()))?,
        )
        .with_context(|| format!("failed to parse '{}'", target.display()))?;
        if existing != layer {
            anyhow::bail!(
                "completion attempt '{}' already exists with incompatible content at '{}'",
                attempt_id,
                target.display()
            );
        }
        "idempotent"
    } else {
        write_layer_atomically(&target, &layer)?;
        "written"
    };
    Ok(CompleteOutput {
        schema: "recur-warp-complete-v1",
        warp_id: warp.to_string(),
        slice_id: slice.to_string(),
        attempt_id: attempt_id.to_string(),
        state: state.to_string(),
        path: relative,
        contract_hash: required.contract_hash.clone(),
        result_hash: result_hash.to_string(),
        evidence,
    })
}

fn evolve(
    root: &Path,
    source_warp: &str,
    target_map_arg: &Path,
    confirm: bool,
) -> anyhow::Result<EvolveOutput> {
    if !root.is_dir() {
        anyhow::bail!("invalid --dir '{}': directory not found", root.display());
    }
    validate_identity("Warp", source_warp, true)?;
    let canonical_root = fs::canonicalize(root)?;
    let candidate = if target_map_arg.is_absolute() {
        target_map_arg.to_path_buf()
    } else {
        root.join(target_map_arg)
    };
    let canonical_candidate = fs::canonicalize(&candidate)
        .with_context(|| format!("failed to resolve target map '{}'", candidate.display()))?;
    if !canonical_candidate.starts_with(&canonical_root) {
        anyhow::bail!(
            "target map '{}' escapes Warp root '{}'",
            candidate.display(),
            root.display()
        );
    }

    let (source_manifest, source_map) = find_map(root, source_warp)?;
    let source_layers = load_layers(root, source_warp)?;
    let target_bytes = fs::read(&canonical_candidate)
        .with_context(|| format!("failed to read '{}'", canonical_candidate.display()))?;
    let target_map: WarpBubbleMap = serde_json::from_slice(&target_bytes)
        .with_context(|| format!("failed to parse '{}'", canonical_candidate.display()))?;
    validate_identity("target Warp", &target_map.warp_id, true)?;
    if target_map.schema != BUBBLE_MAP_SCHEMA {
        anyhow::bail!(
            "target map schema '{}' is unsupported; expected '{}'",
            target_map.schema,
            BUBBLE_MAP_SCHEMA
        );
    }
    if target_map.warp_id == source_warp {
        anyhow::bail!("target Warp must supersede the source with a new identity");
    }

    let (exploded, source_invalidated) = explosion_state(&source_map, &source_layers);
    if !exploded {
        anyhow::bail!(
            "Warp '{}' is not exploded; refusing to supersede a converging bubble",
            source_warp
        );
    }

    let target_parent = source_manifest
        .parent()
        .ok_or_else(|| anyhow::anyhow!("source Warp map has no parent directory"))?;
    let target_manifest = target_parent.join(format!("{}.warp-map.json", target_map.warp_id));
    let mut carried = Vec::new();
    let mut carried_layers = Vec::new();
    for required in &target_map.required_slices {
        let Some(source_required) = source_map
            .required_slices
            .iter()
            .find(|source| source.slice_id == required.slice_id)
        else {
            continue;
        };
        if source_required.contract_hash != required.contract_hash {
            continue;
        }
        let candidates = source_layers
            .iter()
            .filter(|layer| {
                layer.slice_id == required.slice_id
                    && layer.contract_hash == required.contract_hash
                    && layer.result_state.eq_ignore_ascii_case("accepted")
                    && !layer.result_hash.trim().is_empty()
                    && required.evidence_gates.iter().all(|gate| {
                        layer
                            .evidence
                            .get(gate)
                            .is_some_and(|references| !references.is_empty())
                    })
            })
            .collect::<Vec<_>>();
        let hashes = candidates
            .iter()
            .map(|layer| layer.result_hash.as_str())
            .collect::<BTreeSet<_>>();
        if hashes.len() != 1 {
            continue;
        }
        let source = candidates[0];
        let attempt_id = format!("evolved-{}", source.attempt_id);
        carried.push(required.slice_id.clone());
        carried_layers.push(WarpSliceLayer {
            schema: SLICE_LAYER_SCHEMA.to_string(),
            warp_id: target_map.warp_id.clone(),
            slice_id: required.slice_id.clone(),
            contract_hash: required.contract_hash.clone(),
            attempt_id,
            result_state: "accepted".to_string(),
            result_hash: source.result_hash.clone(),
            evidence: source.evidence.clone(),
            reason: Some(format!("carried forward from Warp '{source_warp}'")),
        });
    }
    carried.sort();
    let mut invalidated = source_invalidated;
    invalidated.extend(
        source_map
            .required_slices
            .iter()
            .filter(|required| !carried.contains(&required.slice_id))
            .map(|required| required.slice_id.clone()),
    );
    invalidated.sort();
    invalidated.dedup();

    let receipt_path = root.join(".recur").join("warp").join(format!(
        "recur-warp.{source_warp}.to.{}.supersession.ack.json",
        target_map.warp_id
    ));
    if confirm {
        fs::create_dir_all(target_parent)?;
        write_if_absent_or_equal(&target_manifest, &target_bytes)?;
        for layer in &carried_layers {
            let path = target_parent.join(format!(
                "{}.{}.{}.warp-layer.json",
                layer.warp_id, layer.slice_id, layer.attempt_id
            ));
            let bytes = format!("{}\n", serde_json::to_string_pretty(layer)?);
            write_if_absent_or_equal(&path, bytes.as_bytes())?;
        }
        fs::create_dir_all(receipt_path.parent().expect("receipt has parent"))?;
        let target_manifest_text = normalize_path(
            target_manifest
                .strip_prefix(root)
                .unwrap_or(&target_manifest),
        );
        let receipt = SupersessionReceipt {
            schema: "recur-warp-supersession-v1",
            source_warp,
            target_warp: &target_map.warp_id,
            result_state: "accepted",
            target_manifest: &target_manifest_text,
            carried_slices: &carried,
            invalidated_slices: &invalidated,
        };
        let bytes = format!("{}\n", serde_json::to_string_pretty(&receipt)?);
        write_if_absent_or_equal(&receipt_path, bytes.as_bytes())?;
    }

    Ok(EvolveOutput {
        schema: "recur-warp-evolve-v1",
        source_warp: source_warp.to_string(),
        target_warp: target_map.warp_id,
        state: if confirm { "written" } else { "planned" }.to_string(),
        target_manifest: normalize_path(
            target_manifest
                .strip_prefix(root)
                .unwrap_or(&target_manifest),
        ),
        carried_slices: carried,
        invalidated_slices: invalidated,
        receipt: normalize_path(receipt_path.strip_prefix(root).unwrap_or(&receipt_path)),
    })
}

fn collapse(root: &Path, lane: &str, confirm: bool) -> anyhow::Result<CollapseOutput> {
    if !root.is_dir() {
        anyhow::bail!("invalid --dir '{}': directory not found", root.display());
    }
    validate_identity("lane", lane, true)?;
    let policy = collapse_suffix_policy(root)?;
    let lane_prefix = format!("{lane}.");
    let mut collapse_known = Vec::new();
    let mut preserve_interesting = Vec::new();
    let mut blockers = Vec::new();
    let mut ambiguous = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(keep_entry)
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy();
        if !name.starts_with(&lane_prefix) {
            continue;
        }
        let relative = normalize_path(entry.path().strip_prefix(root).unwrap_or(entry.path()));
        // Unreadable evidence must not be silently treated as completed content.
        let text = fs::read_to_string(entry.path())
            .with_context(|| format!("failed to read '{}'", entry.path().display()))?;
        let state = policy.state(&name);
        let group = state.as_deref().map(|s| policy.group(s)).unwrap_or("other");
        if group == "blocked"
            || text.to_ascii_lowercase().contains("operator approval")
            || text.to_ascii_lowercase().contains("blocker")
        {
            blockers.push(relative);
        } else if group == "complete" {
            collapse_known.push(relative);
        } else if group == "interesting" {
            preserve_interesting.push(relative);
        } else {
            ambiguous.push(relative);
        }
    }
    collapse_known.sort();
    preserve_interesting.sort();
    blockers.sort();
    ambiguous.sort();

    if confirm && (!blockers.is_empty() || !ambiguous.is_empty()) {
        anyhow::bail!(
            "collapse requires operator resolution; blockers [{}], ambiguous [{}]",
            blockers.join(", "),
            ambiguous.join(", ")
        );
    }

    let archive_root = root.join(".recur").join("warp").join("archive").join(lane);
    let receipt_path = root
        .join(".recur")
        .join("warp")
        .join(format!("recur-warp.{lane}.collapse.ack.json"));
    let mut archived = Vec::new();
    if confirm {
        let moves = collapse_known
            .iter()
            .map(|relative| {
                let source = root.join(relative);
                let target = archive_root.join(relative);
                (source, target)
            })
            .collect::<Vec<_>>();
        for (_, target) in &moves {
            if target.exists() {
                anyhow::bail!(
                    "collapse archive target already exists: '{}'",
                    target.display()
                );
            }
        }
        let mut completed_moves = Vec::new();
        for (source, target) in &moves {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            if let Err(error) = fs::rename(source, target) {
                for (moved_source, moved_target) in completed_moves.iter().rev() {
                    let _ = fs::rename(moved_target, moved_source);
                }
                return Err(error)
                    .with_context(|| format!("failed to archive '{}'", source.display()));
            }
            completed_moves.push((source.clone(), target.clone()));
            archived.push(normalize_path(target.strip_prefix(root).unwrap_or(target)));
        }
        let mut preserved = preserve_interesting.clone();
        preserved.extend(blockers.clone());
        preserved.extend(ambiguous.clone());
        let receipt = CollapseReceipt {
            schema: "recur-warp-collapse-receipt-v1",
            lane,
            result_state: "accepted",
            archived: &archived,
            preserved: &preserved,
        };
        if let Some(parent) = receipt_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let bytes = format!("{}\n", serde_json::to_string_pretty(&receipt)?);
        write_if_absent_or_equal(&receipt_path, bytes.as_bytes())?;
    }

    Ok(CollapseOutput {
        schema: "recur-warp-collapse-v1",
        lane: lane.to_string(),
        state: if confirm { "written" } else { "planned" }.to_string(),
        collapse_known,
        preserve_interesting,
        blockers,
        ambiguous,
        archived,
        receipt: normalize_path(receipt_path.strip_prefix(root).unwrap_or(&receipt_path)),
    })
}

fn collapse_suffix_policy(root: &Path) -> anyhow::Result<recur::warp_policy::WarpPolicy> {
    recur::warp_policy::WarpPolicy::load(root)
}

fn load_layers(root: &Path, warp: &str) -> anyhow::Result<Vec<WarpSliceLayer>> {
    let prefix = format!("{warp}.");
    let mut layers = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(keep_entry)
        .filter_map(Result::ok)
    {
        let name = entry.file_name().to_string_lossy();
        if entry.file_type().is_file()
            && name.starts_with(&prefix)
            && name.ends_with(".warp-layer.json")
        {
            let layer: WarpSliceLayer = serde_json::from_str(&fs::read_to_string(entry.path())?)
                .with_context(|| format!("failed to parse '{}'", entry.path().display()))?;
            if layer.warp_id == warp && layer.schema == SLICE_LAYER_SCHEMA {
                layers.push(layer);
            }
        }
    }
    Ok(layers)
}

fn explosion_state(map: &WarpBubbleMap, layers: &[WarpSliceLayer]) -> (bool, Vec<String>) {
    let mut exploded = false;
    let mut invalidated = Vec::new();
    for required in &map.required_slices {
        let accepted = layers
            .iter()
            .filter(|layer| {
                layer.slice_id == required.slice_id
                    && layer.result_state.eq_ignore_ascii_case("accepted")
            })
            .collect::<Vec<_>>();
        let stale = accepted
            .iter()
            .any(|layer| layer.contract_hash != required.contract_hash);
        let hashes = accepted
            .iter()
            .filter(|layer| layer.contract_hash == required.contract_hash)
            .map(|layer| layer.result_hash.as_str())
            .collect::<BTreeSet<_>>();
        if stale || hashes.len() > 1 {
            exploded = true;
            invalidated.push(required.slice_id.clone());
        }
    }
    (exploded, invalidated)
}

fn write_if_absent_or_equal(target: &Path, bytes: &[u8]) -> anyhow::Result<()> {
    if target.is_file() {
        let existing = fs::read(target)?;
        if existing == bytes {
            return Ok(());
        }
        anyhow::bail!(
            "refusing to replace incompatible file '{}'",
            target.display()
        );
    }
    write_bytes_atomically(target, bytes)
}

fn validate_identity(label: &str, value: &str, allow_dot: bool) -> anyhow::Result<()> {
    let valid = !value.is_empty()
        && !value.starts_with('.')
        && !value.ends_with('.')
        && !value.contains("..")
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric()
                || character == '-'
                || character == '_'
                || (allow_dot && character == '.')
        });
    if !valid {
        anyhow::bail!("invalid {label} identity '{value}'");
    }
    Ok(())
}

fn find_map(root: &Path, warp: &str) -> anyhow::Result<(PathBuf, WarpBubbleMap)> {
    let expected = format!("{warp}.warp-map.json");
    let mut paths = WalkDir::new(root)
        .into_iter()
        .filter_entry(keep_entry)
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().is_file() && entry.file_name().to_str() == Some(&expected)
        })
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();
    paths.sort();
    if paths.len() != 1 {
        anyhow::bail!("expected exactly one '{}', found {}", expected, paths.len());
    }
    let path = paths.remove(0);
    let map: WarpBubbleMap = serde_json::from_str(
        &fs::read_to_string(&path)
            .with_context(|| format!("failed to read '{}'", path.display()))?,
    )
    .with_context(|| format!("failed to parse '{}'", path.display()))?;
    if map.schema != BUBBLE_MAP_SCHEMA || map.warp_id != warp {
        anyhow::bail!(
            "Warp map '{}' has schema '{}' and identity '{}'; expected '{}' and '{}'",
            path.display(),
            map.schema,
            map.warp_id,
            BUBBLE_MAP_SCHEMA,
            warp
        );
    }
    recur::warp_bubble::validate_bubble_map(&map, warp, &path)?;
    Ok((path, map))
}

fn parse_evidence(values: &[String]) -> anyhow::Result<BTreeMap<String, Vec<String>>> {
    let mut evidence: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for value in values {
        let Some((gate, reference)) = value.split_once('=') else {
            anyhow::bail!("invalid evidence '{}'; expected GATE=REFERENCE", value);
        };
        let gate = gate.trim();
        let reference = reference.trim();
        if gate.is_empty() || reference.is_empty() {
            anyhow::bail!(
                "invalid evidence '{}'; gate and reference are required",
                value
            );
        }
        evidence
            .entry(gate.to_string())
            .or_default()
            .push(reference.to_string());
    }
    for references in evidence.values_mut() {
        references.sort();
        references.dedup();
    }
    Ok(evidence)
}

fn validate_evidence_gates(
    evidence: &BTreeMap<String, Vec<String>>,
    required_gates: &[String],
) -> anyhow::Result<()> {
    let required = required_gates.iter().cloned().collect::<BTreeSet<_>>();
    let supplied = evidence.keys().cloned().collect::<BTreeSet<_>>();
    let missing = required.difference(&supplied).cloned().collect::<Vec<_>>();
    let extra = supplied.difference(&required).cloned().collect::<Vec<_>>();
    if !missing.is_empty() || !extra.is_empty() {
        anyhow::bail!(
            "evidence gates do not match the Slice contract; missing [{}], unexpected [{}]",
            missing.join(", "),
            extra.join(", ")
        );
    }
    Ok(())
}

fn reject_conflicting_result(
    root: &Path,
    warp: &str,
    slice: &str,
    contract_hash: &str,
    result_hash: &str,
) -> anyhow::Result<()> {
    let prefix = format!("{warp}.");
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
        if !name.starts_with(&prefix) || !name.ends_with(".warp-layer.json") {
            continue;
        }
        let existing: WarpSliceLayer = serde_json::from_str(
            &fs::read_to_string(entry.path())
                .with_context(|| format!("failed to read '{}'", entry.path().display()))?,
        )
        .with_context(|| format!("failed to parse '{}'", entry.path().display()))?;
        if existing.schema == SLICE_LAYER_SCHEMA
            && existing.warp_id == warp
            && existing.slice_id == slice
            && existing.contract_hash == contract_hash
            && existing.result_state.eq_ignore_ascii_case("accepted")
            && existing.result_hash != result_hash
        {
            anyhow::bail!(
                "accepted Slice '{}' already has conflicting result hash '{}' in '{}'",
                slice,
                existing.result_hash,
                entry.path().display()
            );
        }
    }
    Ok(())
}

fn write_layer_atomically(target: &Path, layer: &WarpSliceLayer) -> anyhow::Result<()> {
    let bytes = format!("{}\n", serde_json::to_string_pretty(layer)?);
    write_bytes_atomically(target, bytes.as_bytes())
}

fn write_nak_receipt(
    root: &Path,
    warp: &str,
    slice: &str,
    attempt_id: &str,
    reason: String,
) -> anyhow::Result<()> {
    let directory = root.join(".recur").join("warp");
    fs::create_dir_all(&directory)
        .with_context(|| format!("failed to create '{}'", directory.display()))?;
    let target = directory.join(format!(
        "recur-warp.{warp}.{slice}.{attempt_id}.status.nak.json"
    ));
    let receipt = NakReceipt {
        schema: "recur-warp-nak-v1",
        warp_id: warp,
        slice_id: slice,
        attempt_id,
        result_state: "nak",
        reason,
    };
    let bytes = format!("{}\n", serde_json::to_string_pretty(&receipt)?);
    if target.is_file() {
        let existing =
            fs::read(&target).with_context(|| format!("failed to read '{}'", target.display()))?;
        if existing == bytes.as_bytes() {
            return Ok(());
        }
        anyhow::bail!(
            "NAK receipt already exists with different content at '{}'",
            target.display()
        );
    }
    write_bytes_atomically(&target, bytes.as_bytes())
}

fn write_bytes_atomically(target: &Path, bytes: &[u8]) -> anyhow::Result<()> {
    let parent = target
        .parent()
        .ok_or_else(|| anyhow::anyhow!("output has no parent directory"))?;
    let file_name = target
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| anyhow::anyhow!("completion layer path is not valid UTF-8"))?;
    let temporary = parent.join(format!(".{file_name}.{}.tmp", process::id()));
    let write_result = (|| -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .with_context(|| format!("failed to create '{}'", temporary.display()))?;
        file.write_all(bytes)
            .with_context(|| format!("failed to write '{}'", temporary.display()))?;
        file.sync_all()
            .with_context(|| format!("failed to sync '{}'", temporary.display()))?;
        fs::rename(&temporary, target).with_context(|| {
            format!("failed to publish completion layer '{}'", target.display())
        })?;
        Ok(())
    })();
    if write_result.is_err() && temporary.is_file() {
        let _ = fs::remove_file(&temporary);
    }
    write_result
}

fn keep_entry(entry: &DirEntry) -> bool {
    entry.file_name().to_str() != Some(".recur")
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn emit(output: &CompleteOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
    } else {
        println!("Warp Slice completion for {}", output.slice_id);
        println!("  Warp: {}", output.warp_id);
        println!("  state: {}", output.state);
        println!("  layer: {}", output.path);
    }
    Ok(())
}

fn emit_evolve(output: &EvolveOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
    } else {
        println!("Warp evolution from {}", output.source_warp);
        println!("  target: {}", output.target_warp);
        println!("  state: {}", output.state);
        println!("  carried: {}", output.carried_slices.join(", "));
        println!("  invalidated: {}", output.invalidated_slices.join(", "));
        println!("  receipt: {}", output.receipt);
    }
    Ok(())
}

fn emit_collapse(output: &CollapseOutput, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(output)?);
    } else {
        println!("Warp collapse for {}", output.lane);
        println!("  state: {}", output.state);
        println!("  known: {}", output.collapse_known.len());
        println!("  interesting: {}", output.preserve_interesting.len());
        println!("  blockers: {}", output.blockers.len());
        println!("  ambiguous: {}", output.ambiguous.len());
        println!("  archived: {}", output.archived.len());
        println!("  receipt: {}", output.receipt);
    }
    Ok(())
}
