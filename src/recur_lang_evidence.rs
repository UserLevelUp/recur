//! Bounded source/scope relevance layered over the existing external checker.
//! defines: main.lang.checked-transition.assessment shared pure assessment
use crate::warp_evidence::{self as external, fingerprint};
use anyhow::{bail, Context, Result};
use clap::Args;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Args)]
pub struct EvidenceArgs {
    pub source: PathBuf,
    #[arg(long)]
    pub scope: String,
    #[arg(long)]
    pub contract: PathBuf,
    #[arg(long)]
    pub receipt: Option<PathBuf>,
    #[arg(long)]
    pub status: Option<PathBuf>,
    #[arg(long)]
    pub expand: bool,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Requirement {
    pub id: String,
    pub cases: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Transition {
    pub current: String,
    pub slice: String,
    pub desired: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub schema: String,
    pub contract_id: String,
    pub source: String,
    pub source_hash: String,
    pub scope: String,
    pub aliases: BTreeMap<String, String>,
    pub transition: Transition,
    pub inputs: BTreeMap<String, Vec<String>>,
    pub requirements: Vec<Requirement>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub id: String,
    pub outcome: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Attempt {
    pub schema: String,
    pub attempt_id: String,
    pub contract_hash: String,
    pub source_hash: String,
    pub scope: String,
    pub phase: String,
    pub producer: String,
    pub runtime: String,
    pub evidence: String,
    pub cases: Vec<Case>,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CheckedStatus {
    pub schema: String,
    pub state: String,
    pub source: String,
    pub source_hash: String,
    pub scope: String,
    pub contract: String,
    pub receipt: String,
    pub attempt_id: String,
    pub contract_hash: String,
    pub attempt_hash: String,
    pub before: String,
    pub after: String,
    pub artifact_hash: String,
    pub checked_inputs: BTreeMap<String, String>,
}
const ORDER: &[&str] = &[
    "malformed",
    "ambiguous",
    "mismatched",
    "failed",
    "stale",
    "declared",
    "absent",
    "checked",
];
fn nonempty(s: &str) -> bool {
    !s.trim().is_empty()
}
pub fn valid_id(s: &str) -> bool {
    nonempty(s)
        && s.len() <= 80
        && s != "."
        && s != ".."
        && s.bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"._-".contains(&c))
}
pub fn relative(path: &Path) -> Result<&str> {
    let s = path.to_str().context("non UTF-8 path")?;
    if s.is_empty()
        || path.is_absolute()
        || s.contains(['\\', ':', '\0'])
        || s.split('/').any(|c| c.is_empty() || c == "." || c == "..")
    {
        bail!("unsafe root-relative path: {s}");
    }
    Ok(s)
}
#[derive(Default)]
pub struct ReadSet {
    cache: BTreeMap<PathBuf, Vec<u8>>,
    pub hashes: BTreeMap<String, String>,
    total: usize,
}
impl ReadSet {
    pub fn read(&mut self, root: &Path, path: &str, json_file: bool) -> Result<Vec<u8>> {
        relative(Path::new(path))?;
        let resolved = external::contained_file(root, path)?;
        let max = if json_file { 1048576 } else { 8 * 1024 * 1024 };
        let bytes = if let Some(bytes) = self.cache.get(&resolved) {
            if bytes.len() > max {
                bail!("file byte limit: {path}");
            }
            bytes.clone()
        } else {
            if self.cache.len() >= 64 {
                bail!("unique file limit");
            }
            let file = File::open(&resolved)?;
            let n = usize::try_from(file.metadata()?.len())?;
            if n > max
                || self
                    .total
                    .checked_add(n)
                    .map_or(true, |n| n > 16 * 1024 * 1024)
            {
                bail!("file or aggregate byte limit: {path}");
            }
            let mut bytes = Vec::with_capacity(n);
            file.take((max + 1) as u64).read_to_end(&mut bytes)?;
            if bytes.len() != n {
                bail!("file changed during bounded read: {path}");
            }
            self.total += n;
            self.cache.insert(resolved, bytes.clone());
            bytes
        };
        self.hashes.insert(path.into(), fingerprint(&bytes));
        Ok(bytes)
    }
}
fn push(report: &mut Value, status: &str, reason: impl Into<String>) {
    let current = report["assessment"]["status"].as_str().unwrap();
    if ORDER.iter().position(|s| *s == status) < ORDER.iter().position(|s| *s == current) {
        report["assessment"]["status"] = json!(status);
    }
    let code = match status {
        "malformed" => "CE001",
        "ambiguous" => "CE002",
        "mismatched" => "CE003",
        "failed" => "CE004",
        "stale" => "CE005",
        "absent" => "CE006",
        _ => "CE007",
    };
    report["assessment"]["reasons"]
        .as_array_mut()
        .unwrap()
        .push(json!({"code":code,"message":reason.into()}));
}
fn check(report: &mut Value, ok: bool, status: &str, reason: &str) -> bool {
    if !ok {
        push(report, status, reason);
    }
    ok
}
fn vocabulary(reads: &mut ReadSet, root: &Path) -> Result<Vec<String>> {
    let mut v = vec!["todo.current".into(), "todo".into(), "complete".into()];
    if root.join(".recur/config.toml").try_exists()? {
        let b = reads.read(root, ".recur/config.toml", true)?;
        let config: toml::Value = toml::from_str(std::str::from_utf8(&b)?)?;
        for (i, k) in ["current_suffix", "todo_suffix", "complete_suffix"]
            .iter()
            .enumerate()
        {
            if let Some(value) = config.get("status").and_then(|s| s.get(k)) {
                v[i] = value
                    .as_str()
                    .context("status suffix must be a string")?
                    .trim_start_matches('.')
                    .into();
            }
        }
    }
    Ok(v)
}
fn examine(
    root: &Path,
    args: &EvidenceArgs,
    reads: &mut ReadSet,
    report: &mut Value,
) -> Result<()> {
    let source = relative(&args.source)?;
    let policy_path = relative(&args.contract)?;
    let source_bytes = reads.read(root, source, true)?;
    let source_text = std::str::from_utf8(&source_bytes)?;
    if !check(
        report,
        args.scope.contains('.'),
        "ambiguous",
        "select a qualified function identity",
    ) {
        return Ok(());
    }
    let names = crate::recur_lang_ir::query_declarations(source_text)?.2;
    if !check(
        report,
        names
            .iter()
            .any(|name| args.scope.starts_with(&format!("{name}."))),
        "mismatched",
        "unknown qualified scope",
    ) {
        return Ok(());
    }
    let selected_scope = args.scope.rsplit_once('.').unwrap().0;
    let ir = crate::recur_lang_ir::parse_warp_ir(source_text, source, selected_scope)?;
    if !check(
        report,
        ir.scope.function.identity == args.scope,
        "mismatched",
        "unknown qualified function",
    ) {
        return Ok(());
    }
    let packet = crate::recur_lang_query::evidence_packet(
        source_text,
        source,
        &args.scope,
        args.expand,
        vocabulary(reads, root)?,
    )
    .map_err(anyhow::Error::msg)?;
    report["packet"] = packet;
    let pb = reads.read(root, policy_path, true)?;
    let policy: Policy = serde_json::from_slice(&pb)?;
    if policy.schema != "recur-lang-checked-contract-v1"
        || !nonempty(&policy.contract_id)
        || !nonempty(&policy.source_hash)
    {
        bail!("invalid policy schema/identity");
    }
    let roles: BTreeSet<_> = [
        "specification",
        "implementation",
        "tests",
        "configuration",
        "runner",
        "behavior",
    ]
    .into_iter()
    .collect();
    if policy
        .inputs
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>()
        != roles
        || policy
            .inputs
            .values()
            .any(|v| v.is_empty() || v.iter().any(|s| !nonempty(s)))
    {
        bail!("invalid required input roles");
    }
    if policy.requirements.is_empty() || policy.requirements.len() > 64 {
        bail!("requirement count limit");
    }
    let mut ids = BTreeSet::new();
    let mut required = BTreeSet::new();
    for r in &policy.requirements {
        if !nonempty(&r.id) || r.cases.is_empty() || r.cases.iter().any(|s| !nonempty(s)) {
            bail!("invalid requirement");
        }
        check(
            report,
            ids.insert(&r.id),
            "ambiguous",
            "duplicate requirement id",
        );
        check(
            report,
            r.cases.iter().collect::<BTreeSet<_>>().len() == r.cases.len(),
            "ambiguous",
            "duplicate required case id",
        );
        required.extend(r.cases.iter().map(String::as_str));
    }
    if required.len() > 128 {
        bail!("required case count limit");
    }
    let h = &report["packet"]["header"][0];
    let aliases: BTreeMap<String, String> = ["input", "output"]
        .iter()
        .map(|k| {
            (
                h[k]["local_identity"].as_str().unwrap().into(),
                h[k]["canonical_identity"].as_str().unwrap().into(),
            )
        })
        .collect();
    let transition = &report["packet"]["footer"]["events"][0]["requested_transition"];
    let expected = Transition {
        current: transition["current"].as_str().unwrap().into(),
        slice: transition["slice"].as_str().unwrap().into(),
        desired: transition["desired"].as_str().unwrap().into(),
    };
    check(
        report,
        policy.source == source && policy.scope == args.scope,
        "mismatched",
        "policy source/scope mismatch",
    );
    check(
        report,
        policy.aliases == aliases,
        "mismatched",
        "canonical alias mismatch",
    );
    check(
        report,
        policy.transition == expected,
        "mismatched",
        "transition mismatch",
    );
    check(
        report,
        policy.source_hash == fingerprint(&source_bytes),
        "stale",
        "policy source changed",
    );
    check(
        report,
        policy.inputs["specification"].contains(&source.to_string()),
        "mismatched",
        "source not in specification inputs",
    );
    report["requirements"] = json!(policy.requirements);
    report["transition"] = json!(expected);
    // Always bound explicit policy inputs even when no receipt was supplied.
    for p in policy.inputs.values().flatten() {
        reads.read(root, p, false)?;
    }
    let Some(receipt) = &args.receipt else {
        push(report, "absent", "no attempt supplied");
        return Ok(());
    };
    let receipt_path = relative(receipt)?;
    let ab = reads.read(root, receipt_path, true)?;
    let attempt: Attempt = serde_json::from_slice(&ab)?;
    if attempt.schema != "recur-lang-checked-receipt-v1"
        || !valid_id(&attempt.attempt_id)
        || !nonempty(&attempt.producer)
        || !nonempty(&attempt.runtime)
        || !matches!(attempt.phase.as_str(), "red" | "green")
        || attempt.cases.len() > 128
    {
        bail!("invalid attempt schema/identity/limits");
    }
    let mut case_ids = BTreeSet::new();
    let mut passed = 0u64;
    let mut failed = 0u64;
    for c in &attempt.cases {
        if !nonempty(&c.id) {
            bail!("empty case id");
        }
        check(
            report,
            case_ids.insert(c.id.as_str()),
            "ambiguous",
            "duplicate observed case id",
        );
        match c.outcome.as_str() {
            "passed" => passed += 1,
            "failed" => failed += 1,
            _ => bail!("invalid case outcome"),
        }
    }
    report["observation"] = json!(attempt);
    check(
        report,
        required.is_subset(&case_ids),
        "mismatched",
        "missing required cases",
    );
    check(
        report,
        attempt.scope == args.scope,
        "mismatched",
        "attempt scope mismatch",
    );
    check(
        report,
        attempt.source_hash == fingerprint(&source_bytes),
        "stale",
        "attempt source changed",
    );
    check(
        report,
        attempt.contract_hash == fingerprint(&pb),
        "stale",
        "policy content changed",
    );
    check(
        report,
        attempt.phase == "green" && failed == 0,
        "failed",
        "attempt is red or cases failed",
    );
    let Some(evidence_path) = attempt.evidence.strip_prefix("evidence:") else {
        push(
            report,
            "declared",
            "manual/native reference; test result not checked",
        );
        return Ok(());
    };
    let eb = reads.read(root, evidence_path, true)?;
    let evidence: external::Evidence = serde_json::from_slice(&eb)?;
    if evidence.schema != "warp-external-evidence-v1" {
        bail!("unsupported external schema");
    }
    check(
        report,
        evidence.producer == attempt.producer && evidence.kind == "test",
        "mismatched",
        "external producer/kind mismatch",
    );
    check(
        report,
        policy
            .inputs
            .values()
            .flatten()
            .chain(std::iter::once(&policy_path.to_string()))
            .all(|p| evidence.source.files.contains_key(p)),
        "mismatched",
        "missing policy or required live input in evidence",
    );
    let result_bytes = reads.read(root, &evidence.result_artifact, true)?;
    let result: external::Outcome = serde_json::from_slice(&result_bytes)?;
    if let Some(counts) = &result.tests {
        check(
            report,
            counts.passed == passed
                && counts.failed == failed
                && counts.executed == attempt.cases.len() as u64,
            "failed",
            "case labels disagree with observed aggregate",
        );
    }
    let mut read_error = None;
    let verdict = external::assess_with_reader(
        &attempt.evidence,
        &external::GateRule {
            kind: "test".into(),
            allow_skipped: false,
        },
        |p, j| {
            let result = reads.read(root, p, j);
            if let Err(e) = &result {
                read_error = Some(e.to_string());
            }
            result
        },
    );
    if verdict.status != "checked" {
        for reason in &verdict.reasons {
            push(report, &verdict.status, reason);
        }
    }
    if let Some(e) = read_error {
        push(report, "malformed", e);
    }
    report["external_assessment"] = json!(verdict);
    if let Some(status) = &args.status {
        let status_bytes = reads.read(root, relative(status)?, true)?;
        let record: CheckedStatus = serde_json::from_slice(&status_bytes)?;
        if record.schema != "recur-lang-checked-status-v1"
            || !matches!(record.state.as_str(), "prepared" | "accepted")
            || record.checked_inputs.is_empty()
            || !valid_id(&record.attempt_id)
        {
            bail!("unsupported status schema/shape");
        }
        relative(Path::new(&record.before))?;
        relative(Path::new(&record.after))?;
        let matches = record.source == source
            && record.scope == args.scope
            && record.attempt_hash == fingerprint(&ab)
            && record.contract_hash == fingerprint(&pb)
            && record.receipt == receipt_path
            && record.contract == policy_path
            && record.attempt_id == attempt.attempt_id;
        check(
            report,
            matches,
            "mismatched",
            "status request identity mismatch",
        );
        if record.state == "accepted" {
            let unchanged = if root.join(&record.after).try_exists()? {
                fingerprint(&reads.read(root, &record.after, false)?) == record.artifact_hash
            } else {
                false
            };
            check(
                report,
                unchanged && !root.join(&record.before).try_exists()?,
                "stale",
                "accepted artifact changed or transition no longer holds",
            );
            for (p, h) in &record.checked_inputs {
                let b = reads.read(
                    root,
                    p,
                    p.ends_with(".json") || p.ends_with(".recur") || p.ends_with(".toml"),
                )?;
                check(
                    report,
                    fingerprint(&b) == *h,
                    "stale",
                    "historically accepted input changed",
                );
            }
        }
        report["transition_status"] =
            json!({"accepted":matches && record.state=="accepted","record":record});
    }
    Ok(())
}
pub fn assess(root: &Path, args: &EvidenceArgs) -> Value {
    let mut report = json!({"schema":"recur-lang-evidence-report-v1","packet":null,"assessment":{"status":"checked","reasons":[]},"observation":null,
        "transition_status":{"accepted":false},"recorded_inventory":"not-scanned","checked_inputs":{},"execution":"not-run",
        "qualification":"producer not rerun; case labels are producer claims; fingerprints detect changes, not authenticity or dependency closure"});
    let mut reads = ReadSet::default();
    if let Err(e) = examine(root, args, &mut reads, &mut report) {
        push(&mut report, "malformed", format!("{e:#}"));
    }
    report["checked_inputs"] = json!(reads.hashes);
    report["transition_status"]["current_accepted"] = json!(
        report["transition_status"]["accepted"] == true
            && report["assessment"]["status"] == "checked"
    );
    report
}
pub fn exit_code(report: &Value) -> i32 {
    match report["assessment"]["status"]
        .as_str()
        .unwrap_or("malformed")
    {
        "checked" | "absent" | "declared" => 0,
        "malformed" => 2,
        _ => 1,
    }
}
pub fn text(report: &Value) -> String {
    let mut out = if report["packet"].is_null() {
        String::new()
    } else {
        crate::recur_lang_query::evidence_text(&report["packet"])
    };
    out.push_str(&format!("\nEvidence assessment: {}\nRecorded inventory: not-scanned\n{}\nRequirements: {}\nObserved attempt: {}\nTransition status: {}\nReasons: {}\n",
        report["assessment"]["status"],report["qualification"],report["requirements"],report["observation"],report["transition_status"],report["assessment"]["reasons"]));
    out
}
