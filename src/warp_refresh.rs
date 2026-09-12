//! Pure bounded resolution of immutable same-contract evidence refresh chains.
//! defines: recur.warp.evidence.refresh.resolve current evidence with historical lineage
use crate::warp_bubble::{WarpBubbleMap, WarpRequiredSlice, WarpSliceLayer, SLICE_LAYER_SCHEMA};
use crate::warp_evidence::{
    self as evidence, fingerprint, Assessment, Evidence, GateAssessment, GateRule,
};
use anyhow::{bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
};
#[derive(Default)]
pub struct Reads {
    pub hashes: BTreeMap<String, String>,
    cache: BTreeMap<PathBuf, Vec<u8>>,
    total: usize,
}
impl Reads {
    pub fn read(&mut self, root: &Path, path: &str, json: bool) -> Result<Vec<u8>> {
        let actual = safe(root, path)?.canonicalize()?;
        ensure!(
            actual.starts_with(root.canonicalize()?) && actual.is_file(),
            "contained file required: {path}"
        );
        let limit = if json {
            2 * 1024 * 1024
        } else {
            32 * 1024 * 1024
        };
        let bytes = if let Some(bytes) = self.cache.get(&actual) {
            ensure!(bytes.len() <= limit, "file byte limit: {path}");
            bytes.clone()
        } else {
            ensure!(self.cache.len() < 128, "unique file limit");
            let file = File::open(&actual)?;
            let n = usize::try_from(file.metadata()?.len())?;
            ensure!(
                n <= limit
                    && self
                        .total
                        .checked_add(n)
                        .is_some_and(|n| n <= 64 * 1024 * 1024),
                "file or aggregate byte limit: {path}"
            );
            let mut bytes = Vec::with_capacity(n);
            file.take((limit + 1) as u64).read_to_end(&mut bytes)?;
            ensure!(bytes.len() == n, "file changed while reading: {path}");
            self.total += n;
            self.cache.insert(actual, bytes.clone());
            bytes
        };
        self.hashes.insert(path.into(), fingerprint(&bytes));
        Ok(bytes)
    }
    pub fn verify(&self, root: &Path) -> Result<()> {
        let mut now = Reads::default();
        for (p, h) in &self.hashes {
            ensure!(
                fingerprint(&now.read(root, p, false)?) == *h,
                "input changed before publication: {p}"
            );
        }
        Ok(())
    }
}
pub fn relative(p: &Path) -> Result<&str> {
    crate::recur_lang_evidence::relative(p)
}
pub fn safe(root: &Path, name: &str) -> Result<PathBuf> {
    relative(Path::new(name))?;
    let mut p = root.to_path_buf();
    for part in name.split('/') {
        p.push(part);
        match fs::symlink_metadata(&p) {
            Ok(m) => {
                #[cfg(windows)]
                let reparse = {
                    use std::os::windows::fs::MetadataExt;
                    m.file_attributes() & 0x400 != 0
                };
                #[cfg(not(windows))]
                let reparse = false;
                ensure!(
                    !m.file_type().is_symlink() && !reparse,
                    "unsafe symlink/reparse path: {name}"
                );
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(p)
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub schema: String,
    pub refresh_id: String,
    pub map: String,
    pub warp_id: String,
    pub bubble_uuid: Option<String>,
    pub slice_id: String,
    pub contract_hash: String,
    pub policy_hash: String,
    pub layer: String,
    pub layer_hash: String,
    pub gate: String,
    pub from_reference: String,
    pub from_hash: String,
    pub to_reference: String,
    pub to_hash: String,
    pub reason: String,
}
pub struct Plan {
    pub record: Record,
    pub target: String,
    pub state: &'static str,
    pub reads: Reads,
    pub inventory: Vec<String>,
}
fn directory(map: &str, warp: &str) -> Result<String> {
    ensure!(
        crate::recur_lang_evidence::valid_id(warp),
        "invalid Warp path identity"
    );
    Ok(Path::new(map)
        .with_file_name(format!("{warp}.evidence-refresh"))
        .to_str()
        .context("non UTF8 path")?
        .replace('\\', "/"))
}
pub fn inventory(root: &Path, dir: &str) -> Result<Vec<String>> {
    let path = safe(root, dir)?;
    if !path.try_exists()? {
        return Ok(vec![]);
    }
    let mut names = Vec::new();
    for entry in fs::read_dir(path)?.take(65) {
        let entry = entry?;
        names.push(
            entry
                .file_name()
                .into_string()
                .map_err(|_| anyhow::anyhow!("non UTF8 receipt name"))?,
        );
    }
    ensure!(names.len() <= 64, "refresh history entry limit");
    names.sort();
    Ok(names)
}
fn policy_hash(s: &WarpRequiredSlice) -> Result<String> {
    Ok(fingerprint(&serde_json::to_vec(s)?))
}
fn anchor(map: &WarpBubbleMap, layer: &WarpSliceLayer) -> Result<WarpRequiredSlice> {
    ensure!(
        layer.schema == SLICE_LAYER_SCHEMA
            && layer.warp_id == map.warp_id
            && layer.result_state.eq_ignore_ascii_case("accepted")
            && !layer.result_hash.trim().is_empty(),
        "refresh requires accepted layer with result hash"
    );
    let s = map
        .required_slices
        .iter()
        .find(|s| s.slice_id == layer.slice_id)
        .context("layer slice not in map")?;
    ensure!(
        s.contract_hash == layer.contract_hash,
        "accepted contract mismatch"
    );
    Ok(s.clone())
}
fn manifest(root: &Path, reference: &str, reads: &mut Reads) -> Result<Evidence> {
    let p = reference
        .strip_prefix("evidence:")
        .context("refresh requires external evidence reference")?;
    let e: Evidence = serde_json::from_slice(&reads.read(root, p, true)?)?;
    ensure!(
        fingerprint(&reads.read(root, &e.result_artifact, true)?) == e.result_fingerprint,
        "historical/result artifact changed: {reference}"
    );
    Ok(e)
}
fn assess(root: &Path, r: &str, rule: &GateRule, reads: &mut Reads) -> Result<Assessment> {
    let mut failure = None;
    let a = evidence::assess_with_reader(r, rule, |p, j| {
        let b = reads.read(root, p, j);
        if let Err(e) = &b {
            failure = Some(e.to_string())
        }
        b
    });
    if let Some(e) = failure {
        bail!("bounded evidence read: {e}")
    }
    Ok(a)
}
fn relation(a: &Evidence, b: &Evidence) -> Result<()> {
    ensure!(
        a.kind == b.kind && a.project == b.project,
        "replacement project/kind mismatch"
    );
    ensure!(
        a.source
            .files
            .keys()
            .all(|p| b.source.files.contains_key(p)),
        "replacement must preserve every predecessor input path"
    );
    Ok(())
}
fn records(
    root: &Path,
    map_path: &str,
    map: &WarpBubbleMap,
    reads: &mut Reads,
    allow_tmp: Option<&str>,
) -> Result<(Vec<Record>, Vec<String>)> {
    let dir = directory(map_path, &map.warp_id)?;
    let names = inventory(root, &dir)?;
    let mut records = Vec::new();
    for name in &names {
        let path = format!("{dir}/{name}");
        let bytes = reads.read(root, &path, true)?;
        if Some(name.as_str()) == allow_tmp {
            continue;
        }
        ensure!(name.ends_with(".json"), "unexpected refresh file: {path}");
        let record: Record = serde_json::from_slice(&bytes)?;
        ensure!(
            record.schema == "warp-evidence-refresh-v1"
                && crate::recur_lang_evidence::valid_id(&record.refresh_id)
                && *name == format!("{}.json", record.refresh_id)
                && !record.reason.trim().is_empty(),
            "invalid refresh schema/identity: {path}"
        );
        ensure!(
            record.map == map_path
                && record.warp_id == map.warp_id
                && record.bubble_uuid == map.bubble_uuid,
            "refresh map identity mismatch"
        );
        let lb = reads.read(root, &record.layer, true)?;
        let layer: WarpSliceLayer = serde_json::from_slice(&lb)?;
        let policy = anchor(map, &layer)?;
        ensure!(
            record.layer_hash == fingerprint(&lb)
                && record.policy_hash == policy_hash(&policy)?
                && record.slice_id == policy.slice_id
                && record.contract_hash == policy.contract_hash,
            "refresh anchor/policy changed"
        );
        ensure!(
            policy.evidence_gates.contains(&record.gate)
                && layer.evidence.contains_key(&record.gate),
            "refresh gate not declared in policy/layer"
        );
        for (reference, hash) in [
            (&record.from_reference, &record.from_hash),
            (&record.to_reference, &record.to_hash),
        ] {
            let p = reference
                .strip_prefix("evidence:")
                .context("invalid refresh reference")?;
            ensure!(
                fingerprint(&reads.read(root, p, true)?) == *hash,
                "immutable refresh manifest changed: {p}"
            );
        }
        records.push(record);
    }
    Ok((records, names))
}
fn resolve(
    root: &Path,
    layer_path: &str,
    layer: &WarpSliceLayer,
    required: &WarpRequiredSlice,
    gate: &str,
    all: &[Record],
    reads: &mut Reads,
) -> Result<Vec<Assessment>> {
    let records = all
        .iter()
        .filter(|r| r.layer == layer_path && r.gate == gate)
        .collect::<Vec<_>>();
    let rule = required.gate_rules.get(gate).cloned().unwrap_or_default();
    let mut edges = BTreeMap::new();
    for r in &records {
        ensure!(
            edges.insert(r.from_reference.as_str(), *r).is_none(),
            "conflicting refresh successors"
        );
    }
    let mut used = BTreeSet::new();
    let mut assessments = Vec::new();
    for original in layer.evidence.get(gate).into_iter().flatten() {
        let mut reference = original.as_str();
        let mut visited = BTreeSet::new();
        let mut lineage = Vec::new();
        while let Some(r) = edges.get(reference) {
            ensure!(
                visited.insert(reference) && visited.len() <= 64,
                "refresh cycle/chain limit"
            );
            used.insert(r.refresh_id.as_str());
            let a = manifest(root, &r.from_reference, reads)?;
            let b = manifest(root, &r.to_reference, reads)?;
            relation(&a, &b)?;
            // Current source drift is allowed for history, but read actual bytes.
            let old = assess(root, &r.from_reference, &rule, reads)?;
            ensure!(
                old.status == "checked" || old.status == "stale",
                "historical evidence is failed: {}",
                old.reasons.join("; ")
            );
            reference = &r.to_reference;
            lineage.push(r.refresh_id.clone());
        }
        let mut a = if lineage.is_empty() {
            evidence::assess_with_reader(reference, &rule, |p, j| reads.read(root, p, j))
        } else {
            assess(root, reference, &rule, reads)?
        };
        if !lineage.is_empty() {
            a.method.push_str("; immutable same-contract refresh chain");
            a.reasons.push(format!("original reference: {original}; refreshes: {}; current leaf: {reference}; historical source freshness is not current acceptance",lineage.join(" -> ")));
        }
        assessments.push(a);
    }
    ensure!(
        used.len() == records.len(),
        "orphan/cyclic refresh not rooted in accepted gate"
    );
    Ok(assessments)
}
pub fn gates(
    root: &Path,
    map_path: &str,
    required: &WarpRequiredSlice,
    layer_path: &str,
    layer: &WarpSliceLayer,
) -> Vec<GateAssessment> {
    let fallback = || evidence::gates(root, required, &layer.evidence);
    // Pure snapshot callers already reject external references. No-refresh paths
    // preserve legacy behavior and do not invent acceptance from sidecar metadata.
    if !layer
        .evidence
        .values()
        .flatten()
        .any(|r| r.starts_with("evidence:"))
    {
        return fallback();
    }
    let result = (|| -> Result<Vec<GateAssessment>> {
        let dir = directory(map_path, &layer.warp_id)?;
        if !safe(root, &dir)?.try_exists()? {
            return Ok(fallback());
        }
        let mut reads = Reads::default();
        let map: WarpBubbleMap = serde_json::from_slice(&reads.read(root, map_path, true)?)?;
        let (records, _) = records(root, map_path, &map, &mut reads, None)?;
        required
            .evidence_gates
            .iter()
            .map(|gate| {
                let evidence = resolve(
                    root, layer_path, layer, required, gate, &records, &mut reads,
                )?;
                let status = evidence::combined_status(evidence.iter().map(|a| a.status.as_str()));
                Ok(GateAssessment {
                    slice_id: required.slice_id.clone(),
                    gate: gate.clone(),
                    status: status.into(),
                    satisfied: status == "checked"
                        || (required.evidence_mode == "declared" && status == "declared"),
                    evidence,
                })
            })
            .collect()
    })();
    result.unwrap_or_else(|e| {
        let mut gates = fallback();
        for g in &mut gates {
            g.satisfied = false;
            g.status = "failed".into();
            g.evidence.push(Assessment {
                reference: map_path.into(),
                status: "failed".into(),
                method: "immutable-refresh-validation".into(),
                reasons: vec![format!("{e:#}")],
            });
        }
        gates
    })
}
#[allow(clippy::too_many_arguments)] // Exact explicit wire request; no implicit discovery.
pub fn plan(
    root: &Path,
    map_path: &str,
    layer_path: &str,
    gate: &str,
    from: &str,
    to: &str,
    id: &str,
    reason: &str,
) -> Result<Plan> {
    ensure!(
        crate::recur_lang_evidence::valid_id(id) && !reason.trim().is_empty(),
        "invalid refresh ID/reason"
    );
    let mut reads = Reads::default();
    let map: WarpBubbleMap = serde_json::from_slice(&reads.read(root, map_path, true)?)?;
    crate::warp_bubble::validate_bubble_map(&map, &map.warp_id, &root.join(map_path))?;
    let bytes = reads.read(root, layer_path, true)?;
    let layer: WarpSliceLayer = serde_json::from_slice(&bytes)?;
    let required = anchor(&map, &layer)?;
    ensure!(
        required.evidence_gates.iter().any(|g| g == gate) && layer.evidence.contains_key(gate),
        "gate missing from policy/layer"
    );
    let dir = directory(map_path, &map.warp_id)?;
    let target = format!("{dir}/{id}.json");
    let temp_name = format!("{id}.json.tmp");
    let (all, inventory) = records(root, map_path, &map, &mut reads, Some(&temp_name))?;
    let a = manifest(root, from, &mut reads)?;
    let b = manifest(root, to, &mut reads)?;
    relation(&a, &b)?;
    ensure!(
        b.source.files.keys().all(|p| !p
            .to_ascii_lowercase()
            .starts_with(&format!("{dir}/").to_ascii_lowercase())),
        "refresh transaction files cannot be checked source inputs"
    );
    let from_path = from.strip_prefix("evidence:").unwrap();
    let to_path = to.strip_prefix("evidence:").unwrap();
    ensure!(
        safe(root, from_path)?.canonicalize()? != safe(root, to_path)?.canonicalize()?,
        "replacement must use a distinct immutable manifest"
    );
    let record = Record {
        schema: "warp-evidence-refresh-v1".into(),
        refresh_id: id.into(),
        map: map_path.into(),
        warp_id: map.warp_id.clone(),
        bubble_uuid: map.bubble_uuid.clone(),
        slice_id: required.slice_id.clone(),
        contract_hash: required.contract_hash.clone(),
        policy_hash: policy_hash(&required)?,
        layer: layer_path.into(),
        layer_hash: fingerprint(&bytes),
        gate: gate.into(),
        from_reference: from.into(),
        from_hash: reads.hashes[from_path].clone(),
        to_reference: to.into(),
        to_hash: reads.hashes[to_path].clone(),
        reason: reason.into(),
    };
    let rule = required.gate_rules.get(gate).cloned().unwrap_or_default();
    let new = assess(root, to, &rule, &mut reads)?;
    ensure!(
        new.status == "checked",
        "replacement evidence is {}: {:?}",
        new.status,
        new.reasons
    );
    let existing = all.iter().find(|r| r.refresh_id == id);
    if existing.is_none() {
        let extra = if inventory.contains(&temp_name) { 1 } else { 2 };
        ensure!(
            inventory.len() + extra <= 64,
            "refresh history has no staging/publication capacity"
        );
    }
    let state = if let Some(existing) = existing {
        ensure!(
            *existing == record,
            "refresh ID reused with conflicting request"
        );
        "idempotent"
    } else {
        "planned"
    };
    let current = resolve(root, layer_path, &layer, &required, gate, &all, &mut reads)?;
    if existing.is_none() {
        let leaf = current
            .iter()
            .find(|a| a.reference == from)
            .context("from is not a current leaf of this accepted gate")?;
        ensure!(
            leaf.status == "stale",
            "only source-stale current evidence can be refreshed"
        );
        // Ensure malformed or unavailable inputs cannot hide behind a stale verdict.
        ensure!(
            assess(root, from, &rule, &mut reads)?.status == "stale",
            "predecessor not source-stale"
        );
    }
    let mut candidate = all.clone();
    if existing.is_none() {
        candidate.push(record.clone());
    }
    resolve(
        root, layer_path, &layer, &required, gate, &candidate, &mut reads,
    )?;
    if inventory.contains(&temp_name) {
        ensure!(
            reads.read(root, &format!("{dir}/{temp_name}"), true)?
                == serde_json::to_vec_pretty(&record)?,
            "torn/conflicting staging file: {dir}/{temp_name}"
        );
    }
    Ok(Plan {
        record,
        target,
        state,
        reads,
        inventory,
    })
}
