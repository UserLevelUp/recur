//! Explicit, read-only external evidence validation. No runner is invoked.
//! defines: recur.warp.evidence.integrity.external structured result and source checks
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
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

/// Resolve an explicitly declared evidence root without widening the caller's scope.
/// Omitted metadata preserves the caller-specific legacy default.
pub fn resolve_root(requested: &Path, manifest: &Path, default: &Path) -> Result<PathBuf> {
    let requested = requested.canonicalize()?;
    let manifest = manifest.canonicalize()?;
    anyhow::ensure!(manifest.starts_with(&requested), "Warp manifest escapes requested root");
    let file = fs::File::open(&manifest)?;
    const MAP_LIMIT: u64 = 2 * 1024 * 1024;
    anyhow::ensure!(file.metadata()?.len() <= MAP_LIMIT, "Warp map exceeds 2 MiB JSON limit");
    let mut bytes = Vec::new();
    file.take(MAP_LIMIT + 1).read_to_end(&mut bytes)?;
    anyhow::ensure!(bytes.len() as u64 <= MAP_LIMIT, "Warp map exceeds 2 MiB JSON limit");
    let raw: serde_json::Value = serde_json::from_slice(&bytes)?;
    let Some(value) = raw.get("evidence_root") else { return Ok(default.canonicalize()?); };
    let name = value.as_str().context("evidence_root must be a relative directory string")?;
    anyhow::ensure!(
        !name.is_empty() && name.len() <= 4096 && !name.contains(['\\', ':'])
            && !name.chars().any(char::is_control)
            && name.split('/').all(|part| !part.is_empty())
            && Path::new(name).components().all(|part| matches!(part, Component::Normal(_) | Component::CurDir | Component::ParentDir)),
        "evidence_root must be a nonempty relative directory path"
    );
    let selected = manifest.parent().context("manifest has no parent")?.join(name).canonicalize()
        .with_context(|| format!("missing evidence_root '{name}'"))?;
    anyhow::ensure!(selected.is_dir() && selected.starts_with(&requested), "evidence_root escapes requested -d root or is not a directory");
    anyhow::ensure!(manifest.starts_with(&selected), "evidence_root must contain its Warp manifest");
    Ok(selected)
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

/// Source digests for a single read-only query. Do not cache validation decisions:
/// each manifest and result is still parsed and checked against its own gate rule.
#[derive(Default)]
pub(crate) struct SourceFingerprints {
    by_root: BTreeMap<PathBuf, BTreeMap<String, String>>,
    count: usize,
    bounded: BTreeMap<PathBuf, BTreeMap<String, BoundedFingerprint>>,
}

#[derive(Clone)]
pub(crate) struct BoundedFingerprint {
    pub path: PathBuf,
    pub len: usize,
    pub hash: String,
}

impl SourceFingerprints {
    pub(crate) fn bounded(&self, root: &Path, name: &str) -> Option<BoundedFingerprint> {
        self.bounded.get(root)?.get(name).cloned()
    }

    pub(crate) fn insert_bounded(&mut self, root: &Path, name: &str, input: BoundedFingerprint) {
        if self.count < 4096 {
            self.bounded
                .entry(root.to_path_buf())
                .or_default()
                .insert(name.into(), input);
            self.count += 1;
        }
    }

    fn get(&mut self, root: &Path, file: &str) -> Result<String> {
        if let Some(hash) = self.by_root.get(root).and_then(|files| files.get(file)) {
            return Ok(hash.clone());
        }
        let hash = fingerprint(&fs::read(contained_file(root, file)?)?);
        if self.count < 4096 {
            self.by_root
                .entry(root.to_path_buf())
                .or_default()
                .insert(file.into(), hash.clone());
            self.count += 1;
        }
        Ok(hash)
    }
}

pub fn assess(root: &Path, reference: &str, rule: &GateRule) -> Assessment {
    assess_with_reader(reference, rule, |path, _json| {
        Ok(fs::read(contained_file(root, path)?)?)
    })
}

/// The same result rules with caller-owned bounded reads. The reader must
/// enforce its canonical root; no producer is invoked by this function.
pub fn assess_with_reader(
    reference: &str,
    rule: &GateRule,
    mut read: impl FnMut(&str, bool) -> Result<Vec<u8>>,
) -> Assessment {
    assess_with_inputs(
        reference,
        rule,
        &mut read,
        |read, path| read(path, true),
        |read, path| read(path, false).map(|bytes| fingerprint(&bytes)),
    )
}

pub(crate) fn assess_with_inputs<T>(
    reference: &str,
    rule: &GateRule,
    inputs: &mut T,
    mut read: impl FnMut(&mut T, &str) -> Result<Vec<u8>>,
    mut hash: impl FnMut(&mut T, &str) -> Result<String>,
) -> Assessment {
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
    match check(path, rule, inputs, &mut read, &mut hash) {
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

fn check<T>(
    path: &str,
    rule: &GateRule,
    inputs: &mut T,
    read: &mut impl FnMut(&mut T, &str) -> Result<Vec<u8>>,
    hash: &mut impl FnMut(&mut T, &str) -> Result<String>,
) -> Result<(&'static str, Vec<String>)> {
    let evidence: Evidence = serde_json::from_slice(&read(inputs, path)?)?;
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
        match hash(inputs, file) {
            Ok(actual) if actual == *expected => {}
            Ok(_) => stale.push(format!("source changed: {file}")),
            Err(error) => stale.push(format!("source unavailable: {file}: {error}")),
        }
    }
    let bytes = read(inputs, &evidence.result_artifact)?;
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
    gate_assessments(slice, references, |reference, rule| {
        assess(root, reference, rule)
    })
}

pub(crate) fn query_gates(
    root: &Path,
    slice: &crate::warp_bubble::WarpRequiredSlice,
    references: &BTreeMap<String, Vec<String>>,
    source: &mut SourceFingerprints,
) -> Vec<GateAssessment> {
    gate_assessments(slice, references, |reference, rule| {
        assess_with_inputs(
            reference,
            rule,
            source,
            |_, path| Ok(fs::read(contained_file(root, path)?)?),
            |source, path| source.get(root, path),
        )
    })
}

fn gate_assessments(
    slice: &crate::warp_bubble::WarpRequiredSlice,
    references: &BTreeMap<String, Vec<String>>,
    mut assess: impl FnMut(&str, &GateRule) -> Assessment,
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
                .map(|r| assess(r, &rule))
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

    #[test]
    fn query_fingerprints_reuse_reads_within_one_root_and_query() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let file = "board.kicad_pcb";
        fs::write(first.path().join(file), "first").unwrap();
        fs::write(second.path().join(file), "second").unwrap();
        let mut query = SourceFingerprints::default();
        assert_eq!(
            query.get(first.path(), file).unwrap(),
            fingerprint(b"first")
        );
        fs::write(first.path().join(file), "updated").unwrap();
        assert_eq!(
            query.get(first.path(), file).unwrap(),
            fingerprint(b"first")
        );
        assert_eq!(
            query.get(second.path(), file).unwrap(),
            fingerprint(b"second")
        );
        assert!(query.get(first.path(), "../board.kicad_pcb").is_err());
        let mut next = SourceFingerprints::default();
        assert_eq!(
            next.get(first.path(), file).unwrap(),
            fingerprint(b"updated")
        );
    }

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
