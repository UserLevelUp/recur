//! Explicit, read-only external evidence validation. No runner is invoked.
//! defines: recur.warp.evidence.integrity.external structured result and source checks
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub schema: String,
    pub kind: String,
    pub producer: String,
    pub project: String,
    pub configuration: String,
    pub platform: String,
    pub executed_at_unix: u64,
    pub result_artifact: String,
    pub result_fingerprint: String,
    pub source: Source,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub revision: Option<String>,
    pub dirty: bool,
    /// Exact verification scope; include build/configuration inputs explicitly.
    pub files: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    pub schema: String,
    pub kind: String,
    pub outcome: String,
    pub exit_code: i32,
    pub tests: Option<TestCounts>,
    pub matches: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TestCounts {
    pub discovered: u64,
    pub executed: u64,
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct GateRule {
    /// test, build, or scan; empty accepts any supported kind.
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub allow_skipped: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct Assessment {
    pub reference: String,
    pub status: String,
    pub method: String,
    pub reasons: Vec<String>,
}

pub fn fingerprint(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

pub fn contained_file(root: &Path, relative: &str) -> Result<PathBuf> {
    let path = Path::new(relative);
    if path.is_absolute()
        || path
            .components()
            .any(|c| !matches!(c, Component::Normal(_) | Component::CurDir))
    {
        bail!(
            "artifact must be a relative path within the evidence root: {}",
            relative
        );
    }
    let canonical_root = root.canonicalize()?;
    let resolved = canonical_root
        .join(path)
        .canonicalize()
        .with_context(|| format!("missing artifact '{}'", relative))?;
    if !resolved.starts_with(&canonical_root) || !resolved.is_file() {
        bail!(
            "artifact escapes the evidence root or is not a file: {}",
            relative
        );
    }
    Ok(resolved)
}

pub fn assess(root: &Path, reference: &str, rule: &GateRule) -> Assessment {
    let mut result = Assessment {
        reference: reference.into(),
        status: "declared".into(),
        method: "manual-reference; producer not rerun".into(),
        reasons: vec![],
    };
    let Some(path) = reference.strip_prefix("evidence:") else {
        return result;
    };
    result.method =
        "external-result-artifact-and-scoped-content-fingerprints; producer not rerun".into();
    match check(root, path, rule) {
        Ok((state, reasons)) => {
            result.status = state.into();
            result.reasons = reasons;
        }
        Err(error) => {
            result.status = "failed".into();
            result.reasons.push(format!("{error:#}"));
        }
    }
    result
}

fn check(root: &Path, path: &str, rule: &GateRule) -> Result<(&'static str, Vec<String>)> {
    let evidence: Evidence = serde_json::from_slice(&fs::read(contained_file(root, path)?)?)?;
    if evidence.schema != "warp-external-evidence-v1" {
        bail!("unsupported external evidence schema");
    }
    if !matches!(evidence.kind.as_str(), "test" | "build" | "scan") {
        bail!("unsupported evidence kind");
    }
    if !rule.kind.is_empty() && evidence.kind != rule.kind {
        bail!("evidence kind does not match gate kind '{}'", rule.kind);
    }
    if [
        &evidence.producer,
        &evidence.project,
        &evidence.configuration,
        &evidence.platform,
    ]
    .iter()
    .any(|v| v.trim().is_empty())
    {
        bail!("evidence requires producer, project, configuration and platform");
    }
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    if evidence.executed_at_unix == 0 || evidence.executed_at_unix > now {
        bail!("execution timestamp is missing or in the future");
    }
    if evidence.source.files.is_empty() {
        bail!("source fingerprint scope is empty");
    }
    if evidence
        .source
        .revision
        .as_ref()
        .is_some_and(|s| s.trim().is_empty())
    {
        bail!("source revision must not be blank");
    }
    let mut stale = Vec::new();
    for (file, expected) in &evidence.source.files {
        match contained_file(root, file).and_then(|p| Ok(fingerprint(&fs::read(p)?))) {
            Ok(actual) if actual == *expected => {}
            Ok(_) => stale.push(format!("source changed: {file}")),
            Err(error) => stale.push(format!("source unavailable: {file}: {error}")),
        }
    }
    let bytes = fs::read(contained_file(root, &evidence.result_artifact)?)?;
    if fingerprint(&bytes) != evidence.result_fingerprint {
        stale.push("result artifact changed".into());
    }
    let outcome: Outcome =
        serde_json::from_slice(&bytes).context("invalid structured result artifact")?;
    if outcome.schema != "warp-external-result-v1" || outcome.kind != evidence.kind {
        bail!("result schema/kind mismatch");
    }
    if outcome.outcome != "passed" || outcome.exit_code != 0 {
        bail!(
            "external result failed: outcome={}, exit_code={}",
            outcome.outcome,
            outcome.exit_code
        );
    }
    if evidence.kind == "test" {
        let tests = outcome.tests.context("test counts are missing")?;
        if tests.executed == 0 {
            bail!("zero executed tests cannot satisfy a passing gate");
        }
        if tests.passed.checked_add(tests.failed) != Some(tests.executed)
            || tests.executed.checked_add(tests.skipped) != Some(tests.discovered)
        {
            bail!("test totals are inconsistent or undispatched tests remain");
        }
        if tests.failed > 0 || (!rule.allow_skipped && tests.skipped > 0) {
            bail!(
                "failed or disallowed skipped tests: executed={}, passed={}, failed={}, skipped={}",
                tests.executed,
                tests.passed,
                tests.failed,
                tests.skipped
            );
        }
    } else if evidence.kind == "scan" && outcome.matches != Some(0) {
        bail!("zero-matches scan gate failed or match count missing");
    }
    if stale.is_empty() {
        Ok(("checked", vec![format!("scope: {} explicitly named source files; dirty={}; revision is recorded provenance", evidence.source.files.len(), evidence.source.dirty)]))
    } else {
        Ok(("stale", stale))
    }
}

pub fn combined_status<'a>(states: impl IntoIterator<Item = &'a str>) -> &'static str {
    let states: Vec<_> = states.into_iter().collect();
    if states.contains(&"failed") {
        "failed"
    } else if states.contains(&"stale") {
        "stale"
    } else if states.is_empty() || states.iter().all(|s| *s == "absent") {
        "absent"
    } else if states.iter().all(|s| *s == "checked") {
        "checked"
    } else {
        "declared"
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GateAssessment {
    pub slice_id: String,
    pub gate: String,
    pub status: String,
    pub satisfied: bool,
    pub evidence: Vec<Assessment>,
}

pub fn gates(
    root: &Path,
    slice: &crate::warp_bubble::WarpRequiredSlice,
    references: &BTreeMap<String, Vec<String>>,
) -> Vec<GateAssessment> {
    slice
        .evidence_gates
        .iter()
        .map(|gate| {
            let rule = slice.gate_rules.get(gate).cloned().unwrap_or_default();
            let evidence = references
                .get(gate)
                .into_iter()
                .flatten()
                .filter(|r| !r.trim().is_empty())
                .map(|r| assess(root, r, &rule))
                .collect::<Vec<_>>();
            let status = combined_status(evidence.iter().map(|e| e.status.as_str()));
            GateAssessment {
                slice_id: slice.slice_id.clone(),
                gate: gate.clone(),
                status: status.into(),
                satisfied: status == "checked"
                    || (slice.evidence_mode == "declared" && status == "declared"),
                evidence,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(root: &Path) -> (Evidence, Outcome) {
        fs::write(root.join("source.txt"), "source").unwrap();
        let outcome = Outcome {
            schema: "warp-external-result-v1".into(),
            kind: "test".into(),
            outcome: "passed".into(),
            exit_code: 0,
            tests: Some(TestCounts {
                discovered: 2,
                executed: 2,
                passed: 2,
                failed: 0,
                skipped: 0,
            }),
            matches: None,
        };
        let evidence = Evidence {
            schema: "warp-external-evidence-v1".into(),
            kind: "test".into(),
            producer: "external fixture".into(),
            project: "demo".into(),
            configuration: "Debug".into(),
            platform: "x64".into(),
            executed_at_unix: 1,
            result_artifact: "result.json".into(),
            result_fingerprint: String::new(),
            source: Source {
                revision: None,
                dirty: true,
                files: BTreeMap::from([("source.txt".into(), fingerprint(b"source"))]),
            },
        };
        (evidence, outcome)
    }
    fn publish(root: &Path, e: &mut Evidence, o: &Outcome) {
        let bytes = serde_json::to_vec(o).unwrap();
        e.result_fingerprint = fingerprint(&bytes);
        fs::write(root.join("result.json"), bytes).unwrap();
        fs::write(root.join("evidence.json"), serde_json::to_vec(e).unwrap()).unwrap();
    }
    #[test]
    fn outcomes_freshness_and_scope() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let (mut e, mut o) = fixture(root);
        let rule = GateRule::default();
        publish(root, &mut e, &o);
        assert_eq!(assess(root, "manual:50 tests", &rule).status, "declared");
        assert_eq!(
            assess(root, "evidence:evidence.json", &rule).status,
            "checked"
        );
        fs::write(root.join("source.txt"), "changed").unwrap();
        assert_eq!(
            assess(root, "evidence:evidence.json", &rule).status,
            "stale"
        );
        fs::write(root.join("source.txt"), "source").unwrap();
        o.tests.as_mut().unwrap().executed = 0;
        publish(root, &mut e, &o);
        assert_eq!(
            assess(root, "evidence:evidence.json", &rule).status,
            "failed"
        );
        o.tests = Some(TestCounts {
            discovered: 3,
            executed: 2,
            passed: 2,
            failed: 0,
            skipped: 1,
        });
        publish(root, &mut e, &o);
        assert_eq!(
            assess(root, "evidence:evidence.json", &rule).status,
            "failed"
        );
        assert_eq!(
            assess(
                root,
                "evidence:evidence.json",
                &GateRule {
                    kind: "test".into(),
                    allow_skipped: true
                }
            )
            .status,
            "checked"
        );
        o.exit_code = 1;
        publish(root, &mut e, &o);
        assert_eq!(
            assess(root, "evidence:evidence.json", &rule).status,
            "failed"
        );
        e.result_artifact = "../outside.json".into();
        publish(root, &mut e, &o);
        assert_eq!(
            assess(root, "evidence:evidence.json", &rule).status,
            "failed"
        );
        assert_eq!(
            assess(root, "evidence:missing.json", &rule).status,
            "failed"
        );
    }
}
