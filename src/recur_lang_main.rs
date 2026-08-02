//! Stateful companion for bounded Recur Lang coordination actions.

use anyhow::{anyhow, bail, Context, Result};
use clap::{Args, Parser, Subcommand};
use recur::recur_lang_ir::{parse_warp_ir, WARP_IR_SCHEMA};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const WARP_PLAN_SCHEMA: &str = "recur-lang-warp-plan-v1";
const WARP_RECEIPT_SCHEMA: &str = "recur-lang-warp-receipt-v1";
const WARP_STATUS_SCHEMA: &str = "recur-lang-warp-status-v1";

#[derive(Debug, Clone, Serialize)]
struct WarpPlan {
    schema: &'static str,
    ir_schema: &'static str,
    source: String,
    source_hash: String,
    scope: String,
    current: String,
    slice: String,
    desired: String,
    dry_run: bool,
    confirmation_required: bool,
    required_receipt_schema: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct WarpOutcome {
    schema: &'static str,
    ir_schema: &'static str,
    id: String,
    language: &'static str,
    state: String,
    ack: String,
    nak_reason: String,
    source: String,
    source_hash: String,
    scope: String,
    warp: String,
    lane: String,
    current: String,
    slice: String,
    desired: String,
    before_evidence: String,
    after_evidence: String,
    receipt: String,
    artifact: String,
    test_receipt: String,
    attempt: u64,
    started_at: String,
    completed_at: String,
    status_receipt: String,
}

#[derive(Debug, Deserialize)]
struct ExternalReceipt {
    schema: String,
    ir_schema: String,
    scope: String,
    current: String,
    slice: String,
    desired: String,
    source_hash: String,
    ack: String,
    attempt: u64,
    artifact: String,
    test_receipt: String,
}

#[derive(Debug, Default)]
struct ReceiptEvidence {
    path: String,
    artifact: String,
    test_receipt: String,
    attempt: u64,
}

#[derive(Debug)]
struct EventnessTransition {
    before: PathBuf,
    after: PathBuf,
    before_display: String,
    after_display: String,
}

#[derive(Parser)]
#[command(
    name = "recur-lang",
    version,
    about = "Stateful companion for bounded Recur Lang coordination"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Plan or confirm one declared E0 -> dE -> Ef transition
    Warp(WarpArgs),
}

#[derive(Args)]
struct WarpArgs {
    /// Recur Lang source containing the declared Warp
    source: PathBuf,

    /// Scope whose Warp should advance
    scope: String,

    /// Project root used to bound all reads and writes
    #[arg(short = 'd', long = "dir", default_value = ".")]
    dir: PathBuf,

    /// Exact current Eventness artifact whose stem must equal E0
    #[arg(long)]
    eventness: Option<PathBuf>,

    /// Versioned external receipt proving the dE slice
    #[arg(long)]
    receipt: Option<PathBuf>,

    /// Apply the bounded transition; without this flag the command is a dry run
    #[arg(long)]
    confirm: bool,

    /// Stable status receipt id; defaults to the scope name
    #[arg(long)]
    id: Option<String>,

    /// Emit machine-readable JSON
    #[arg(long)]
    json: bool,
}

fn plan_warp(root: &Path, source: &Path, scope: &str) -> Result<WarpPlan> {
    validate_identifier("scope", scope)?;
    let root = canonical_root(root)?;
    let source = resolve_existing_within(&root, source, "source")?;
    if !source.is_file() {
        bail!("source '{}' is not a file", source.display());
    }
    let source_text = fs::read_to_string(&source)
        .with_context(|| format!("failed to read '{}'", source.display()))?;
    let source_display = relative_display_path(&root, &source);
    let ir = parse_warp_ir(&source_text, &source_display, scope).map_err(anyhow::Error::new)?;

    Ok(WarpPlan {
        schema: WARP_PLAN_SCHEMA,
        ir_schema: WARP_IR_SCHEMA,
        source: ir.source,
        source_hash: ir.source_hash,
        scope: scope.to_string(),
        current: ir.scope.warp.current,
        slice: ir.scope.warp.slice,
        desired: ir.scope.warp.desired,
        dry_run: true,
        confirmation_required: true,
        required_receipt_schema: WARP_RECEIPT_SCHEMA,
    })
}

fn apply_warp(
    root: &Path,
    source: &Path,
    scope: &str,
    eventness: &Path,
    receipt: &Path,
    id: &str,
) -> Result<WarpOutcome> {
    validate_identifier("id", id)?;
    let root = canonical_root(root)?;
    let started_at = now_stamp();
    let plan = match plan_warp(&root, source, scope) {
        Ok(plan) => plan,
        Err(error) => {
            let outcome = minimal_rejection(id, source, scope, &started_at, &error.to_string());
            let _ = write_status(&root, &outcome);
            return Err(error);
        }
    };

    let mut evidence = ReceiptEvidence {
        path: relative_unresolved_display_path(&root, receipt),
        ..ReceiptEvidence::default()
    };
    let result: Result<EventnessTransition> = (|| {
        let transition = resolve_eventness_transition(&root, eventness, &plan)?;
        let loaded = load_external_receipt(&root, receipt, &plan)?;
        evidence = loaded;
        Ok(transition)
    })();

    let transition = match result {
        Ok(transition) => transition,
        Err(error) => {
            let outcome = rejection_outcome(
                id,
                &plan,
                eventness,
                &evidence,
                &root,
                &started_at,
                &error.to_string(),
            );
            write_status(&root, &outcome)?;
            return Err(error);
        }
    };

    let status_receipt = status_relative_path(id);
    let mut outcome = WarpOutcome {
        schema: WARP_STATUS_SCHEMA,
        ir_schema: WARP_IR_SCHEMA,
        id: id.to_string(),
        language: "main.lang",
        state: "complete".to_string(),
        ack: "accepted".to_string(),
        nak_reason: String::new(),
        source: plan.source.clone(),
        source_hash: plan.source_hash.clone(),
        scope: plan.scope.clone(),
        warp: plan.scope.clone(),
        lane: plan.scope.clone(),
        current: plan.current.clone(),
        slice: plan.slice.clone(),
        desired: plan.desired.clone(),
        before_evidence: transition.before_display.clone(),
        after_evidence: transition.after_display.clone(),
        receipt: evidence.path.clone(),
        artifact: evidence.artifact.clone(),
        test_receipt: evidence.test_receipt.clone(),
        attempt: evidence.attempt,
        started_at,
        completed_at: String::new(),
        status_receipt,
    };

    if let Err(rename_error) = fs::rename(&transition.before, &transition.after) {
        let error = anyhow!(
            "failed to move E0 '{}' to Ef '{}': {}",
            transition.before.display(),
            transition.after.display(),
            rename_error
        );
        let rejected = rejection_outcome(
            id,
            &plan,
            eventness,
            &evidence,
            &root,
            &outcome.started_at,
            &error.to_string(),
        );
        write_status(&root, &rejected)?;
        return Err(error);
    }
    outcome.completed_at = now_stamp();
    if let Err(error) = write_status(&root, &outcome) {
        let rollback = fs::rename(&transition.after, &transition.before);
        return match rollback {
            Ok(()) => {
                let reason =
                    format!("status write failed; Eventness move was rolled back: {error}");
                let rejected = rejection_outcome(
                    id,
                    &plan,
                    eventness,
                    &evidence,
                    &root,
                    &outcome.started_at,
                    &reason,
                );
                let _ = write_status(&root, &rejected);
                Err(anyhow!(reason))
            }
            Err(rollback_error) => Err(error.context(format!(
                "status write failed and Eventness rollback also failed: {rollback_error}"
            ))),
        };
    }
    Ok(outcome)
}

fn canonical_root(root: &Path) -> Result<PathBuf> {
    let absolute = if root.is_absolute() {
        root.to_path_buf()
    } else {
        std::env::current_dir()?.join(root)
    };
    let canonical = absolute
        .canonicalize()
        .with_context(|| format!("invalid --dir '{}'", absolute.display()))?;
    if !canonical.is_dir() {
        bail!(
            "invalid --dir '{}': directory not found",
            canonical.display()
        );
    }
    Ok(canonical)
}

fn resolve_existing_within(root: &Path, path: &Path, label: &str) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    let canonical = absolute
        .canonicalize()
        .with_context(|| format!("{label} '{}' does not exist", absolute.display()))?;
    if !canonical.starts_with(root) {
        bail!(
            "{label} '{}' is outside the bounded root '{}'",
            canonical.display(),
            root.display()
        );
    }
    Ok(canonical)
}

fn validate_identifier(label: &str, value: &str) -> Result<()> {
    let pattern = Regex::new(r"^[A-Za-z0-9][A-Za-z0-9_.-]*$")?;
    if !pattern.is_match(value) {
        bail!("{label} '{value}' must use only letters, numbers, '.', '-', or '_'");
    }
    Ok(())
}

fn resolve_eventness_transition(
    root: &Path,
    eventness: &Path,
    plan: &WarpPlan,
) -> Result<EventnessTransition> {
    let before = resolve_existing_within(root, eventness, "Eventness evidence")?;
    if !before.is_file() {
        bail!("Eventness evidence '{}' is not a file", before.display());
    }
    let stem = before
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            anyhow!(
                "Eventness evidence '{}' has no UTF-8 stem",
                before.display()
            )
        })?;
    if stem != plan.current {
        bail!(
            "Eventness evidence stem '{stem}' does not match declared E0({})",
            plan.current
        );
    }
    let parent = before
        .parent()
        .ok_or_else(|| anyhow!("Eventness evidence '{}' has no parent", before.display()))?;
    let after_name = match before.extension().and_then(|value| value.to_str()) {
        Some(extension) => format!("{}.{}", plan.desired, extension),
        None => plan.desired.clone(),
    };
    let after = parent.join(after_name);
    if after.exists() {
        bail!("Ef destination '{}' already exists", after.display());
    }
    Ok(EventnessTransition {
        before_display: relative_display_path(root, &before),
        after_display: relative_display_path(root, &after),
        before,
        after,
    })
}

fn load_external_receipt(
    root: &Path,
    receipt_path: &Path,
    plan: &WarpPlan,
) -> Result<ReceiptEvidence> {
    let path = resolve_existing_within(root, receipt_path, "external receipt")?;
    if !path.is_file() {
        bail!("external receipt '{}' is not a file", path.display());
    }
    let text = fs::read_to_string(&path)
        .with_context(|| format!("failed to read '{}'", path.display()))?;
    let receipt: ExternalReceipt = toml::from_str(&text)
        .with_context(|| format!("invalid external receipt '{}'", path.display()))?;

    require_receipt_field("schema", &receipt.schema, WARP_RECEIPT_SCHEMA)?;
    require_receipt_field("ir_schema", &receipt.ir_schema, WARP_IR_SCHEMA)?;
    require_receipt_field("scope", &receipt.scope, &plan.scope)?;
    require_receipt_field("current", &receipt.current, &plan.current)?;
    require_receipt_field("slice", &receipt.slice, &plan.slice)?;
    require_receipt_field("desired", &receipt.desired, &plan.desired)?;
    require_receipt_field("source_hash", &receipt.source_hash, &plan.source_hash)?;
    require_receipt_field("ack", &receipt.ack, "accepted")?;
    if receipt.attempt == 0 {
        bail!("external receipt attempt must be greater than zero");
    }
    if receipt.artifact.trim().is_empty() {
        bail!("external receipt artifact cannot be empty");
    }
    if receipt.test_receipt.trim().is_empty() {
        bail!("external receipt test_receipt cannot be empty");
    }

    Ok(ReceiptEvidence {
        path: relative_display_path(root, &path),
        artifact: receipt.artifact,
        test_receipt: receipt.test_receipt,
        attempt: receipt.attempt,
    })
}

fn require_receipt_field(field: &str, actual: &str, expected: &str) -> Result<()> {
    if actual != expected {
        bail!("external receipt {field} is '{actual}'; expected '{expected}'");
    }
    Ok(())
}

fn rejection_outcome(
    id: &str,
    plan: &WarpPlan,
    eventness: &Path,
    evidence: &ReceiptEvidence,
    root: &Path,
    started_at: &str,
    reason: &str,
) -> WarpOutcome {
    WarpOutcome {
        schema: WARP_STATUS_SCHEMA,
        ir_schema: WARP_IR_SCHEMA,
        id: id.to_string(),
        language: "main.lang",
        state: "stopped".to_string(),
        ack: "rejected".to_string(),
        nak_reason: reason.to_string(),
        source: plan.source.clone(),
        source_hash: plan.source_hash.clone(),
        scope: plan.scope.clone(),
        warp: plan.scope.clone(),
        lane: plan.scope.clone(),
        current: plan.current.clone(),
        slice: plan.slice.clone(),
        desired: plan.desired.clone(),
        before_evidence: relative_unresolved_display_path(root, eventness),
        after_evidence: String::new(),
        receipt: evidence.path.clone(),
        artifact: evidence.artifact.clone(),
        test_receipt: evidence.test_receipt.clone(),
        attempt: evidence.attempt,
        started_at: started_at.to_string(),
        completed_at: now_stamp(),
        status_receipt: status_relative_path(id),
    }
}

fn minimal_rejection(
    id: &str,
    source: &Path,
    scope: &str,
    started_at: &str,
    reason: &str,
) -> WarpOutcome {
    WarpOutcome {
        schema: WARP_STATUS_SCHEMA,
        ir_schema: WARP_IR_SCHEMA,
        id: id.to_string(),
        language: "main.lang",
        state: "stopped".to_string(),
        ack: "rejected".to_string(),
        nak_reason: reason.to_string(),
        source: source.display().to_string(),
        source_hash: String::new(),
        scope: scope.to_string(),
        warp: scope.to_string(),
        lane: scope.to_string(),
        current: String::new(),
        slice: String::new(),
        desired: String::new(),
        before_evidence: String::new(),
        after_evidence: String::new(),
        receipt: String::new(),
        artifact: String::new(),
        test_receipt: String::new(),
        attempt: 0,
        started_at: started_at.to_string(),
        completed_at: now_stamp(),
        status_receipt: status_relative_path(id),
    }
}

fn write_status(root: &Path, outcome: &WarpOutcome) -> Result<PathBuf> {
    let path = root.join(&outcome.status_receipt);
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("status path '{}' has no parent", path.display()))?;
    fs::create_dir_all(parent)
        .with_context(|| format!("failed to create status directory '{}'", parent.display()))?;
    let text = toml::to_string(outcome).context("failed to serialize Warp status")?;
    fs::write(&path, text).with_context(|| format!("failed to write '{}'", path.display()))?;
    Ok(path)
}

fn status_relative_path(id: &str) -> String {
    format!(".recur/lang/recur-lang.{id}.status.current.md")
}

fn relative_display_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn relative_unresolved_display_path(root: &Path, path: &Path) -> String {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    relative_display_path(root, &absolute)
}

fn now_stamp() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("unix:{seconds}")
}

fn emit_json<T: Serialize>(value: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}

fn print_plan(plan: &WarpPlan) {
    println!("recur-lang Warp plan (dry-run)");
    println!("source: {} ({})", plan.source, plan.source_hash);
    println!(
        "warp: E0({}) -> dE({}) -> Ef({})",
        plan.current, plan.slice, plan.desired
    );
    println!("required receipt: {}", plan.required_receipt_schema);
    println!("no files changed; pass --eventness, --receipt, and --confirm to apply");
}

fn print_outcome(outcome: &WarpOutcome) {
    println!(
        "ACK {}: E0({}) -> dE({}) -> Ef({})",
        outcome.id, outcome.current, outcome.slice, outcome.desired
    );
    println!(
        "eventness: {} -> {}",
        outcome.before_evidence, outcome.after_evidence
    );
    println!("evidence: {} / {}", outcome.artifact, outcome.test_receipt);
    println!("status: {}", outcome.status_receipt);
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Warp(arguments) => {
            if !arguments.confirm {
                let plan = plan_warp(&arguments.dir, &arguments.source, &arguments.scope)?;
                if arguments.json {
                    emit_json(&plan)?;
                } else {
                    print_plan(&plan);
                }
                return Ok(());
            }

            let eventness = arguments
                .eventness
                .as_deref()
                .ok_or_else(|| anyhow!("--confirm requires --eventness <exact-E0-file>"))?;
            let receipt = arguments
                .receipt
                .as_deref()
                .ok_or_else(|| anyhow!("--confirm requires --receipt <external-receipt>"))?;
            let id = arguments.id.as_deref().unwrap_or(&arguments.scope);
            let outcome = apply_warp(
                &arguments.dir,
                &arguments.source,
                &arguments.scope,
                eventness,
                receipt,
                id,
            )?;
            if arguments.json {
                emit_json(&outcome)?;
            } else {
                print_outcome(&outcome);
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("recur-lang: {error:#}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    const SOURCE: &str = r#"
recur 0.1 class Demo

header {
  scope build {
    i(a) := (request: Text)
    o(b) := (artifact: Text)
    f : i(a) -> o(b) ~ "Build one artifact" by external.build
  }
}

body {
  build sync : i(a) -> f(a) -> o(b)
}

footer {
  event build {
    consume demo.build.request
    trigger demo.build.run
    produce demo.build.artifact
    state demo.build.complete
  }
  warp build : E0(demo.build.todo.current) -> dE(build.f) -> Ef(demo.build.complete)
}
"#;

    fn fixture() -> (tempfile::TempDir, PathBuf, PathBuf) {
        let temp = tempdir().unwrap();
        let source = temp.path().join("demo.recur");
        let eventness = temp.path().join("demo.build.todo.current.md");
        fs::write(&source, SOURCE).unwrap();
        fs::write(&eventness, "# Build\n\nCurrent work and evidence.\n").unwrap();
        (temp, source, eventness)
    }

    fn write_receipt(path: &Path, plan: &WarpPlan, source_hash: &str, ack: &str) {
        fs::write(
            path,
            format!(
                concat!(
                    "schema = \"{}\"\n",
                    "ir_schema = \"{}\"\n",
                    "scope = \"{}\"\n",
                    "current = \"{}\"\n",
                    "slice = \"{}\"\n",
                    "desired = \"{}\"\n",
                    "source_hash = \"{}\"\n",
                    "ack = \"{}\"\n",
                    "attempt = 1\n",
                    "artifact = \"commit:abc123\"\n",
                    "test_receipt = \"ci:test-42\"\n",
                ),
                WARP_RECEIPT_SCHEMA,
                WARP_IR_SCHEMA,
                plan.scope,
                plan.current,
                plan.slice,
                plan.desired,
                source_hash,
                ack,
            ),
        )
        .unwrap();
    }

    #[test]
    fn plan_reads_exact_declared_warp_and_does_not_mutate() {
        let (temp, source, eventness) = fixture();

        let plan = plan_warp(temp.path(), &source, "build").unwrap();

        assert_eq!(plan.schema, WARP_PLAN_SCHEMA);
        assert_eq!(plan.ir_schema, WARP_IR_SCHEMA);
        assert_eq!(plan.current, "demo.build.todo.current");
        assert_eq!(plan.slice, "build.f");
        assert_eq!(plan.desired, "demo.build.complete");
        assert!(plan.source_hash.starts_with("fnv1a64:"));
        assert!(plan.dry_run);
        assert!(eventness.exists());
        assert!(!temp.path().join("demo.build.complete.md").exists());
        assert!(!temp.path().join(".recur").exists());
    }

    #[test]
    fn confirmed_warp_moves_exact_e0_to_ef_and_writes_ack() {
        let (temp, source, eventness) = fixture();
        let plan = plan_warp(temp.path(), &source, "build").unwrap();
        let receipt = temp.path().join("worker.receipt.md");
        write_receipt(&receipt, &plan, &plan.source_hash, "accepted");

        let outcome = apply_warp(
            temp.path(),
            &source,
            "build",
            &eventness,
            &receipt,
            "build-001",
        )
        .unwrap();

        let final_eventness = temp.path().join("demo.build.complete.md");
        let status = temp
            .path()
            .join(".recur/lang/recur-lang.build-001.status.current.md");
        assert_eq!(outcome.ack, "accepted");
        assert_eq!(outcome.state, "complete");
        assert!(!eventness.exists());
        assert_eq!(
            fs::read_to_string(final_eventness).unwrap(),
            "# Build\n\nCurrent work and evidence.\n",
        );
        let status_text = fs::read_to_string(status).unwrap();
        assert!(status_text.contains("schema = \"recur-lang-warp-status-v1\""));
        assert!(status_text.contains("ir_schema = \"recur-lang-warp-ir-v1\""));
        assert!(status_text.contains("ack = \"accepted\""));
        assert!(status_text.contains("current = \"demo.build.todo.current\""));
        assert!(status_text.contains("desired = \"demo.build.complete\""));
    }

    #[test]
    fn stale_receipt_writes_nak_without_moving_eventness() {
        let (temp, source, eventness) = fixture();
        let plan = plan_warp(temp.path(), &source, "build").unwrap();
        let receipt = temp.path().join("worker.receipt.md");
        write_receipt(&receipt, &plan, "fnv1a64:stale", "accepted");

        let error = apply_warp(
            temp.path(),
            &source,
            "build",
            &eventness,
            &receipt,
            "build-002",
        )
        .unwrap_err();

        assert!(error.to_string().contains("source_hash"));
        assert!(eventness.exists());
        assert!(!temp.path().join("demo.build.complete.md").exists());
        let status = temp
            .path()
            .join(".recur/lang/recur-lang.build-002.status.current.md");
        let status_text = fs::read_to_string(status).unwrap();
        assert!(status_text.contains("ack = \"rejected\""));
        assert!(status_text.contains("state = \"stopped\""));
        assert!(status_text.contains("nak_reason = "));
    }

    #[test]
    fn receipt_from_an_unknown_ir_schema_writes_nak_without_moving_eventness() {
        let (temp, source, eventness) = fixture();
        let plan = plan_warp(temp.path(), &source, "build").unwrap();
        let receipt = temp.path().join("worker.receipt.md");
        write_receipt(&receipt, &plan, &plan.source_hash, "accepted");
        let incompatible = fs::read_to_string(&receipt)
            .unwrap()
            .replace(WARP_IR_SCHEMA, "recur-lang-warp-ir-v0");
        fs::write(&receipt, incompatible).unwrap();

        let error = apply_warp(
            temp.path(),
            &source,
            "build",
            &eventness,
            &receipt,
            "build-ir-mismatch",
        )
        .unwrap_err();

        assert!(error.to_string().contains("ir_schema"));
        assert!(eventness.exists());
        assert!(!temp.path().join("demo.build.complete.md").exists());
        let status = temp
            .path()
            .join(".recur/lang/recur-lang.build-ir-mismatch.status.current.md");
        assert!(fs::read_to_string(status)
            .unwrap()
            .contains("ack = \"rejected\""));
    }

    #[test]
    fn warp_requires_ef_to_be_a_declared_state_event() {
        let (temp, source, _) = fixture();
        let invalid = fs::read_to_string(&source)
            .unwrap()
            .replace("state demo.build.complete", "state demo.build.reviewed");
        fs::write(&source, invalid).unwrap();

        let error = plan_warp(temp.path(), &source, "build").unwrap_err();

        assert!(error.to_string().contains("declared state event"));
    }

    #[test]
    fn warp_requires_data_flow_to_match_the_declared_slice() {
        let (temp, source, _) = fixture();
        let invalid = fs::read_to_string(&source)
            .unwrap()
            .replace("i(a) -> f(a) -> o(b)", "i(a) -> f(a) -> o(c)");
        fs::write(&source, invalid).unwrap();

        let error = plan_warp(temp.path(), &source, "build").unwrap_err();

        assert!(error.to_string().contains("flow does not match"));
    }

    #[test]
    fn confirmed_warp_rejects_eventness_outside_the_bounded_root() {
        let (temp, source, _) = fixture();
        let outside = tempdir().unwrap();
        let outside_eventness = outside.path().join("demo.build.todo.current.md");
        fs::write(&outside_eventness, "outside root").unwrap();
        let plan = plan_warp(temp.path(), &source, "build").unwrap();
        let receipt = temp.path().join("worker.receipt.md");
        write_receipt(&receipt, &plan, &plan.source_hash, "accepted");

        let error = apply_warp(
            temp.path(),
            &source,
            "build",
            &outside_eventness,
            &receipt,
            "build-003",
        )
        .unwrap_err();

        assert!(error.to_string().contains("outside the bounded root"));
        assert!(outside_eventness.exists());
        let status = temp
            .path()
            .join(".recur/lang/recur-lang.build-003.status.current.md");
        assert!(fs::read_to_string(status)
            .unwrap()
            .contains("ack = \"rejected\""));
    }
}
