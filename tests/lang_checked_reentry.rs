//! Final independent process-level drift and recorded-state recovery gates.
#[path = "support/checked_fixture.rs"]
mod fixture;
use fixture::{fixture, put};
use recur::recur_lang_evidence::CheckedStatus;
use recur::warp_evidence::fingerprint;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    process::{Command, Output},
};
use tempfile::tempdir;
const STATUS: &str = ".recur/lang/checked/attempt-1/accepted.json";
fn query(r: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_recur"))
        .args([
            "lang",
            "evidence",
            "spec.recur",
            "--scope",
            "build.f",
            "--contract",
            "policy.json",
            "--receipt",
            "attempt.json",
            "--json",
            "-d",
        ])
        .arg(r)
        .args(extra)
        .output()
        .unwrap()
}
fn actor(r: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_recur-lang"))
        .args([
            "warp",
            "spec.recur",
            "build.f",
            "--checked-contract",
            "policy.json",
            "--receipt",
            "attempt.json",
            "--eventness",
            "build.current.md",
            "--json",
            "-d",
        ])
        .arg(r)
        .args(extra)
        .output()
        .unwrap()
}
fn value(o: &Output) -> Value {
    serde_json::from_slice(&o.stdout).unwrap_or_else(|e| {
        panic!(
            "{e}: {} / {}",
            String::from_utf8_lossy(&o.stdout),
            String::from_utf8_lossy(&o.stderr)
        )
    })
}
fn inventory(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        for e in fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(root, &p, out)
            } else {
                out.insert(
                    p.strip_prefix(root)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace('\\', "/"),
                    fs::read(p).unwrap(),
                );
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}
#[test]
fn every_live_input_loses_current_acceptance_after_a_process_restart() {
    for p in [
        "spec.recur",
        "implementation.rs",
        "test.rs",
        "config.toml",
        "runner.rs",
        "behavior.md",
        "policy.json",
        "result.json",
        "evidence.json",
        "attempt.json",
    ] {
        let d = tempdir().unwrap();
        let r = d.path();
        fixture(r);
        let accepted = actor(r, &["--confirm"]);
        assert!(accepted.status.success());
        assert_eq!(value(&accepted)["action"]["ack"], true);
        let history = fs::read(r.join(STATUS)).unwrap();
        let mut changed = fs::read(r.join(p)).unwrap();
        changed.push(b' ');
        fs::write(r.join(p), changed).unwrap();
        let report = query(r, &["--status", STATUS]);
        assert!(!report.status.success(), "{p}");
        assert_eq!(
            value(&report)["transition_status"]["current_accepted"],
            false
        );
        assert!(!actor(r, &["--confirm"]).status.success(), "{p}");
        assert_eq!(fs::read(r.join(STATUS)).unwrap(), history);
        assert!(r.join("build.complete.md").exists());
        assert!(!r.join("build.current.md").exists());
    }
}
#[test]
fn fresh_processes_recover_every_recorded_interruption_state() {
    // Independent fixtures represent durable states; private actor fault tests
    // separately prove each state is reachable at the actual write boundary.
    for stage in 0..8 {
        let d = tempdir().unwrap();
        let r = d.path();
        fixture(r);
        let bytes = fs::read(r.join("build.current.md")).unwrap();
        let assessment = query(r, &[]);
        assert!(assessment.status.success());
        let report = value(&assessment);
        let inputs: BTreeMap<String, String> =
            serde_json::from_value(report["checked_inputs"].clone()).unwrap();
        let mut intent = CheckedStatus {
            schema: "recur-lang-checked-status-v1".into(),
            state: "prepared".into(),
            source: "spec.recur".into(),
            source_hash: inputs["spec.recur"].clone(),
            scope: "build.f".into(),
            contract: "policy.json".into(),
            receipt: "attempt.json".into(),
            attempt_id: "attempt-1".into(),
            contract_hash: inputs["policy.json"].clone(),
            attempt_hash: inputs["attempt.json"].clone(),
            before: "build.current.md".into(),
            after: "build.complete.md".into(),
            artifact_hash: fingerprint(&bytes),
            checked_inputs: inputs,
        };
        let lane = r.join(".recur/lang/checked/attempt-1");
        fs::create_dir_all(&lane).unwrap();
        let prepared = serde_json::to_vec_pretty(&intent).unwrap();
        if stage < 2 {
            fs::write(lane.join("prepared.json.tmp"), &prepared).unwrap();
        }
        if stage > 0 {
            fs::write(lane.join("prepared.json"), &prepared).unwrap();
        }
        if stage >= 3 {
            fs::hard_link(r.join("build.current.md"), r.join("build.complete.md")).unwrap();
        }
        if stage >= 4 {
            fs::remove_file(r.join("build.current.md")).unwrap();
        }
        intent.state = "accepted".into();
        let accepted = serde_json::to_vec_pretty(&intent).unwrap();
        if stage == 5 || stage == 6 {
            fs::write(lane.join("accepted.json.tmp"), &accepted).unwrap();
        }
        if stage >= 6 {
            fs::write(lane.join("accepted.json"), &accepted).unwrap();
        }
        let before = inventory(r);
        let dry = actor(r, &["--recover"]);
        assert!(
            dry.status.success(),
            "stage {stage}: {}",
            String::from_utf8_lossy(&dry.stderr)
        );
        assert_eq!(inventory(r), before);
        if stage < 7 {
            assert_eq!(actor(r, &["--confirm"]).status.code(), Some(2), "{stage}");
            assert_eq!(inventory(r), before);
        }
        let recovered = actor(r, &["--confirm", "--recover"]);
        assert!(
            recovered.status.success(),
            "{stage}: {}",
            String::from_utf8_lossy(&recovered.stderr)
        );
        assert_eq!(value(&recovered)["action"]["ack"], true);
        assert_eq!(fs::read(r.join("build.complete.md")).unwrap(), bytes);
        assert!(!r.join("build.current.md").exists());
        let history = fs::read(r.join(STATUS)).unwrap();
        assert!(actor(r, &["--confirm"]).status.success());
        assert_eq!(fs::read(r.join(STATUS)).unwrap(), history);
        let queried = query(r, &["--status", STATUS]);
        assert!(queried.status.success());
        assert_eq!(
            value(&queried)["transition_status"]["current_accepted"],
            true
        );
    }
}
#[test]
fn compact_expanded_and_dry_run_keep_contracts_and_footer_facts_separate() {
    let d = tempdir().unwrap();
    let r = d.path();
    fixture(r);
    fs::write(r.join("trap.recur.md"), "pull.first = forbidden.runner").unwrap();
    let before = inventory(r);
    let compact = value(&query(r, &[]));
    let expanded = value(&query(r, &["--expand"]));
    let dry = value(&actor(r, &[]));
    assert_eq!(
        expanded["packet"]["header"][0]["input"]["fields"],
        json!([{"name":"request","type_name":"Text"}])
    );
    assert_eq!(
        expanded["packet"]["header"][0]["output"]["fields"],
        json!([{"name":"artifact","type_name":"Text"}])
    );
    let mut expanded_header = expanded["packet"]["header"].clone();
    for port in ["input", "output"] {
        expanded_header[0][port]
            .as_object_mut()
            .unwrap()
            .remove("fields");
    }
    assert_eq!(compact["packet"]["header"], expanded_header);
    for key in ["body", "footer", "contracts", "coverage"] {
        assert_eq!(compact["packet"][key], expanded["packet"][key], "{key}");
    }
    assert_eq!(compact["assessment"], dry["assessment"]);
    assert_eq!(compact["checked_inputs"], dry["checked_inputs"]);
    assert_eq!(compact["execution"], "not-run");
    assert_eq!(compact["recorded_inventory"], "not-scanned");
    assert_eq!(compact["transition_status"]["accepted"], false);
    assert_eq!(dry["action"]["ack"], false);
    assert_eq!(inventory(r), before);
}
#[test]
fn red_observation_remains_queryable_after_distinct_green_acceptance() {
    let d = tempdir().unwrap();
    let r = d.path();
    fixture(r);
    let mut red: Value =
        serde_json::from_slice(&fs::read(r.join("attempt.json")).unwrap()).unwrap();
    red["attempt_id"] = json!("red-1");
    red["phase"] = json!("red");
    red["cases"][0]["outcome"] = json!("failed");
    put(r, "red.json", &red);
    let bytes = fs::read(r.join("red.json")).unwrap();
    let args = [
        "lang",
        "evidence",
        "spec.recur",
        "--scope",
        "build.f",
        "--contract",
        "policy.json",
        "--receipt",
        "red.json",
        "--json",
        "-d",
    ];
    let old = Command::new(env!("CARGO_BIN_EXE_recur"))
        .args(args)
        .arg(r)
        .output()
        .unwrap();
    assert_eq!(old.status.code(), Some(1));
    assert_eq!(value(&old)["observation"]["phase"], "red");
    assert!(actor(r, &["--confirm"]).status.success());
    let old = Command::new(env!("CARGO_BIN_EXE_recur"))
        .args(args)
        .arg(r)
        .output()
        .unwrap();
    assert_eq!(old.status.code(), Some(1));
    assert_eq!(value(&old)["observation"]["phase"], "red");
    assert_eq!(fs::read(r.join("red.json")).unwrap(), bytes);
}
