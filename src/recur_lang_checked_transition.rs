//! Explicit checked-mode actor. No producer execution; one bounded durable attempt.
//! defines: main.lang.checked-transition.actor confirmed and recoverable E0 to Ef
use anyhow::{bail, Context, Result};
use recur::recur_lang_evidence::{
    assess, exit_code, relative, valid_id, CheckedStatus, EvidenceArgs, ReadSet,
};
use recur::warp_evidence::fingerprint;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};
pub fn run(
    root: &Path,
    args: &EvidenceArgs,
    eventness: Option<&Path>,
    confirm: bool,
    recover: bool,
) -> Result<(Value, i32)> {
    run_hook(root, args, eventness, confirm, recover, |_| Ok(()))
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Stage {
    BeforeMutation,
    PreparedStaged,
    PreparedPublished,
    Prepared,
    Linked,
    Removed,
    AcceptedStaged,
    AcceptedPublished,
    Accepted,
}
fn exists(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.into()),
    }
}
// Reject all existing symlink/reparse ancestors, even ones pointing within root.
fn safe(root: &Path, name: &str) -> Result<PathBuf> {
    relative(Path::new(name))?;
    let mut path = root.to_path_buf();
    for part in name.split('/') {
        path.push(part);
        match fs::symlink_metadata(&path) {
            Ok(meta) => {
                #[cfg(windows)]
                let reparse = {
                    use std::os::windows::fs::MetadataExt;
                    meta.file_attributes() & 0x400 != 0
                };
                #[cfg(not(windows))]
                let reparse = false;
                if meta.file_type().is_symlink() || reparse {
                    bail!("unsafe symlink/reparse path: {name}");
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(path)
}
fn publish(
    root: &Path,
    name: &str,
    bytes: &[u8],
    recover: bool,
    stage: Stage,
    hook: &mut impl FnMut(Stage) -> Result<()>,
) -> Result<()> {
    let target = safe(root, name)?;
    let tmp_name = format!("{name}.tmp");
    let tmp = safe(root, &tmp_name)?;
    if exists(&target)? {
        bail!("refusing to overwrite {name}");
    }
    if exists(&tmp)? {
        if !recover {
            bail!("staging file requires explicit recovery: {tmp_name}");
        }
        if ReadSet::default().read(root, &tmp_name, true)? != bytes {
            bail!("torn/conflicting staging file {tmp_name}; preserve it and inspect before a new attempt");
        }
        OpenOptions::new().write(true).open(&tmp)?.sync_all()?;
    } else {
        let mut file = OpenOptions::new().write(true).create_new(true).open(&tmp)?;
        file.write_all(bytes)?;
        file.sync_all()?;
    }
    hook(stage)?;
    fs::hard_link(&tmp, &target).with_context(|| {
        format!("no-clobber publication failed: {name}; staged bytes preserved")
    })?;
    hook(if stage == Stage::PreparedStaged {
        Stage::PreparedPublished
    } else {
        Stage::AcceptedPublished
    })?;
    fs::remove_file(tmp)?;
    Ok(())
}
fn run_hook(
    root: &Path,
    args: &EvidenceArgs,
    eventness: Option<&Path>,
    confirm: bool,
    recover: bool,
    mut hook: impl FnMut(Stage) -> Result<()>,
) -> Result<(Value, i32)> {
    let root = fs::canonicalize(root)?;
    let mut report = assess(&root, args);
    report["action"] = json!({"state":"planned","confirmed":false,"ack":false});
    if report["assessment"]["status"] != "checked" {
        let code = if confirm {
            exit_code(&report).max(1)
        } else {
            exit_code(&report)
        };
        report["action"]["state"] = json!("rejected");
        return Ok((report, code));
    }
    let Some(eventness) = eventness else {
        if confirm {
            bail!("checked confirmation requires --eventness");
        }
        return Ok((report, 0));
    };
    let before = relative(eventness)?.to_string();
    let current = report["transition"]["current"]
        .as_str()
        .context("missing E0")?;
    let desired = report["transition"]["desired"]
        .as_str()
        .context("missing Ef")?;
    if eventness.file_stem().and_then(|s| s.to_str()) != Some(current) || !valid_id(desired) {
        bail!("Eventness file does not match exact E0/Ef identity");
    }
    let name = match eventness.extension().and_then(|s| s.to_str()) {
        Some(ext) => format!("{desired}.{ext}"),
        None => desired.to_string(),
    };
    let after = eventness
        .with_file_name(name)
        .to_str()
        .context("non UTF-8 Ef")?
        .replace('\\', "/");
    if before == after {
        bail!("E0 and Ef must differ");
    }
    let e0 = safe(&root, &before)?;
    let ef = safe(&root, &after)?;
    let id = report["observation"]["attempt_id"]
        .as_str()
        .context("missing attempt ID")?
        .to_string();
    if !valid_id(&id) {
        bail!("unsafe attempt ID");
    }
    let lane = format!(".recur/lang/checked/{id}");
    let dir = safe(&root, &lane)?;
    let prepared_name = format!("{lane}/prepared.json");
    let accepted_name = format!("{lane}/accepted.json");
    let prepared_path = safe(&root, &prepared_name)?;
    let accepted_path = safe(&root, &accepted_name)?;
    let inputs: BTreeMap<String, String> =
        serde_json::from_value(report["checked_inputs"].clone())?;
    let mut reads = ReadSet::default();
    let moving: Vec<PathBuf> = [&e0, &ef]
        .into_iter()
        .filter_map(|p| fs::canonicalize(p).ok())
        .collect();
    for (p, h) in &inputs {
        let resolved = fs::canonicalize(root.join(p))?;
        if p == &before
            || p == &after
            || p.to_ascii_lowercase().starts_with(".recur/lang/checked/")
            || moving.contains(&resolved)
        {
            bail!("mutable Eventness/status cannot be an immutable checked input");
        }
        let b = reads.read(
            &root,
            p,
            p.ends_with(".json") || p.ends_with(".recur") || p.ends_with(".toml"),
        )?;
        if fingerprint(&b) != *h {
            bail!("input changed during planning: {p}");
        }
    }
    // Only four fixed transaction files are supported; enumerate at most five entries.
    if exists(&dir)? {
        for entry in fs::read_dir(&dir)?.take(5) {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_str().context("unknown transaction file")?;
            if ![
                "prepared.json",
                "prepared.json.tmp",
                "accepted.json",
                "accepted.json.tmp",
            ]
            .contains(&name)
            {
                bail!("unexpected transaction file: {name}");
            }
            safe(&root, &format!("{lane}/{name}"))?;
        }
    }
    let historical = if exists(&accepted_path)? {
        Some(reads.read(&root, &accepted_name, true)?)
    } else if exists(&prepared_path)? {
        Some(reads.read(&root, &prepared_name, true)?)
    } else {
        None
    };
    let artifact_hash = if exists(&e0)? {
        fingerprint(&reads.read(&root, &before, false)?)
    } else if let Some(b) = &historical {
        let old: CheckedStatus = serde_json::from_slice(b)?;
        old.artifact_hash
    } else {
        bail!("E0 absent without a published intent");
    };
    let source = relative(&args.source)?.to_string();
    let contract = relative(&args.contract)?.to_string();
    let receipt = relative(args.receipt.as_deref().context("receipt required")?)?.to_string();
    let mut expected = CheckedStatus {
        schema: "recur-lang-checked-status-v1".into(),
        state: "prepared".into(),
        source_hash: inputs[&source].clone(),
        source,
        scope: args.scope.clone(),
        contract_hash: inputs[&contract].clone(),
        contract,
        attempt_hash: inputs[&receipt].clone(),
        receipt,
        attempt_id: id,
        before: before.clone(),
        after: after.clone(),
        artifact_hash,
        checked_inputs: inputs.clone(),
    };
    let mut accepted = false;
    if let Some(b) = historical {
        let old: CheckedStatus = serde_json::from_slice(&b)?;
        accepted = exists(&accepted_path)?;
        expected.state = if accepted { "accepted" } else { "prepared" }.into();
        if old != expected {
            bail!("conflicting transaction request or changed recorded inputs");
        }
    }
    if exists(&ef)? {
        if (!exists(&prepared_path)? && !accepted)
            || fingerprint(&reads.read(&root, &after, false)?) != expected.artifact_hash
        {
            bail!("occupied or changed Ef destination");
        }
    } else if accepted || !exists(&e0)? {
        bail!("recorded artifact missing");
    }
    if accepted && exists(&e0)? {
        bail!("accepted transition has unexpected E0");
    }
    let mut residues = Vec::new();
    for (name, state) in [(&prepared_name, "prepared"), (&accepted_name, "accepted")] {
        let tmp_name = format!("{name}.tmp");
        let tmp = safe(&root, &tmp_name)?;
        if exists(&tmp)? {
            let mut staged = expected.clone();
            staged.state = state.into();
            if reads.read(&root, &tmp_name, true)? != serde_json::to_vec_pretty(&staged)? {
                bail!("torn/conflicting staging file {tmp_name}; preserve and inspect before a new attempt");
            }
            if exists(&safe(&root, name)?)? {
                residues.push(tmp);
            }
        }
    }
    report["action"] = json!({"state":"planned","before":before,"after":after,"status":accepted_name,"confirmed":false,"ack":false,"recovery_required":exists(&dir)?&&!accepted});
    if !confirm {
        return Ok((report, 0));
    }
    if exists(&dir)? && !accepted && !recover {
        bail!("incomplete transaction requires --recover --confirm");
    }
    if !residues.is_empty() && !recover {
        bail!("published staging residue requires --recover --confirm");
    }
    hook(Stage::BeforeMutation)?;
    let fresh = assess(&root, args);
    if fresh["assessment"]["status"] != "checked"
        || fresh["checked_inputs"] != report["checked_inputs"]
    {
        bail!("checked evidence changed before mutation");
    }
    // Check all previously read transaction/artifact bytes again, with shared limits.
    let mut last = ReadSet::default();
    for (p, h) in &reads.hashes {
        safe(&root, p)?;
        if fingerprint(&last.read(
            &root,
            p,
            p.ends_with(".json") || p.ends_with(".recur") || p.ends_with(".toml"),
        )?) != *h
        {
            bail!("artifact/input changed before mutation: {p}");
        }
    }
    safe(&root, &lane)?;
    safe(&root, &before)?;
    safe(&root, &after)?;
    for residue in residues {
        fs::remove_file(residue)?;
    }
    if accepted {
        report["action"]["state"] = json!("idempotent");
    } else {
        fs::create_dir_all(&dir)?;
        expected.state = "prepared".into();
        if !exists(&prepared_path)? {
            publish(
                &root,
                &prepared_name,
                &serde_json::to_vec_pretty(&expected)?,
                recover,
                Stage::PreparedStaged,
                &mut hook,
            )?;
        }
        hook(Stage::Prepared)?;
        if !exists(&ef)? {
            fs::hard_link(&e0, &ef).context("no-clobber E0 to Ef link failed; E0 preserved")?;
        }
        hook(Stage::Linked)?;
        if exists(&e0)? {
            fs::remove_file(&e0)?;
        }
        hook(Stage::Removed)?;
        expected.state = "accepted".into();
        publish(
            &root,
            &accepted_name,
            &serde_json::to_vec_pretty(&expected)?,
            recover,
            Stage::AcceptedStaged,
            &mut hook,
        )?;
        hook(Stage::Accepted)?;
        report["action"]["state"] = json!("accepted");
    }
    report["action"]["ack"] = json!(true);
    report["action"]["confirmed"] = json!(true);
    report["action"]["recovery_required"] = json!(false);
    report["transition_status"] =
        json!({"accepted":true,"current_accepted":true,"record":expected});
    Ok((report, 0))
}
#[cfg(test)]
#[path = "../tests/support/checked_fixture.rs"]
mod fixture;
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    fn args() -> EvidenceArgs {
        EvidenceArgs {
            source: "spec.recur".into(),
            scope: "build.f".into(),
            contract: "policy.json".into(),
            receipt: Some("attempt.json".into()),
            status: None,
            expand: false,
        }
    }
    #[test]
    fn every_interruption_requires_explicit_bounded_recovery() {
        for stage in [
            Stage::PreparedStaged,
            Stage::PreparedPublished,
            Stage::Prepared,
            Stage::Linked,
            Stage::Removed,
            Stage::AcceptedStaged,
            Stage::AcceptedPublished,
            Stage::Accepted,
        ] {
            let d = tempfile::tempdir().unwrap();
            let root = d.path();
            fixture::fixture(root);
            let bytes = fs::read(root.join("build.current.md")).unwrap();
            assert!(
                run_hook(
                    root,
                    &args(),
                    Some(Path::new("build.current.md")),
                    true,
                    false,
                    |s| {
                        if s == stage {
                            anyhow::bail!("injected {s:?}")
                        }
                        Ok(())
                    }
                )
                .is_err(),
                "{stage:?} never reached"
            );
            let accepted = root.join(".recur/lang/checked/attempt-1/accepted.json");
            assert_eq!(
                accepted.exists(),
                matches!(stage, Stage::AcceptedPublished | Stage::Accepted)
            );
            let retry = run(
                root,
                &args(),
                Some(Path::new("build.current.md")),
                true,
                false,
            );
            if stage != Stage::Accepted {
                assert!(retry.is_err(), "interruption must need recovery: {stage:?}");
            }
            let recovered = run(
                root,
                &args(),
                Some(Path::new("build.current.md")),
                true,
                true,
            )
            .unwrap();
            assert_eq!(recovered.1, 0);
            assert!(accepted.exists());
            assert!(!root.join("build.current.md").exists());
            assert_eq!(fs::read(root.join("build.complete.md")).unwrap(), bytes);
        }
    }
    #[test]
    fn changed_evidence_at_the_write_boundary_is_rejected() {
        let d = tempfile::tempdir().unwrap();
        let r = d.path();
        fixture::fixture(r);
        assert!(run_hook(
            r,
            &args(),
            Some(Path::new("build.current.md")),
            true,
            false,
            |s| {
                if s == Stage::BeforeMutation {
                    fs::write(r.join("runner.rs"), "drift")?;
                }
                Ok(())
            }
        )
        .is_err());
        assert!(r.join("build.current.md").exists());
        assert!(!r.join(".recur").exists());
    }
    #[test]
    fn recovery_rechecks_artifacts_evidence_and_staged_bytes() {
        for change in ["e0", "ef", "evidence", "stage"] {
            let d = tempfile::tempdir().unwrap();
            let r = d.path();
            fixture::fixture(r);
            let stage = if change == "stage" {
                Stage::AcceptedStaged
            } else {
                Stage::Linked
            };
            assert!(run_hook(
                r,
                &args(),
                Some(Path::new("build.current.md")),
                true,
                false,
                |s| {
                    if s == stage {
                        anyhow::bail!("stop")
                    }
                    Ok(())
                }
            )
            .is_err());
            let path = match change {
                "e0" => "build.current.md",
                "ef" => "build.complete.md",
                "evidence" => "test.rs",
                _ => ".recur/lang/checked/attempt-1/accepted.json.tmp",
            };
            fs::write(r.join(path), "changed").unwrap();
            let result = run(r, &args(), Some(Path::new("build.current.md")), true, true);
            assert!(
                result.is_err() || result.unwrap().1 != 0,
                "{change} must block"
            );
            assert!(!r
                .join(".recur/lang/checked/attempt-1/accepted.json")
                .exists());
            assert_eq!(fs::read(r.join(path)).unwrap(), b"changed");
        }
    }
}
