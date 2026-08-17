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
    };
    if let Err(error) = result {
        eprintln!("Error: {error:#}");
        process::exit(2);
    }
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
