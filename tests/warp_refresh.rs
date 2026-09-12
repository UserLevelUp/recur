use recur::warp_evidence::fingerprint;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    process::{Command, Output},
};
use tempfile::tempdir;
fn put(r: &Path, p: &str, v: &Value) {
    fs::write(r.join(p), serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn get(r: &Path, p: &str) -> Value {
    serde_json::from_slice(&fs::read(r.join(p)).unwrap()).unwrap()
}
fn evidence(r: &Path, name: &str) {
    put(
        r,
        &format!("{name}.result.json"),
        &json!({"schema":"warp-external-result-v1","kind":"test","outcome":"passed","exit_code":0,"tests":{"discovered":1,"executed":1,"passed":1,"failed":0,"skipped":0}}),
    );
    put(
        r,
        &format!("{name}.json"),
        &json!({"schema":"warp-external-evidence-v1","kind":"test","producer":"fixture","project":"demo","configuration":"native","platform":"local","executed_at_unix":1,"result_artifact":format!("{name}.result.json"),"result_fingerprint":fingerprint(&fs::read(r.join(format!("{name}.result.json"))).unwrap()),"source":{"revision":null,"dirty":true,"files":{"source.rs":fingerprint(&fs::read(r.join("source.rs")).unwrap())}}}),
    );
}
fn fixture(r: &Path) {
    fs::write(r.join("source.rs"), "old").unwrap();
    evidence(r, "old");
    put(
        r,
        "demo.warp-map.json",
        &json!({"schema":"warp-bubble-map-v1","warp_id":"demo","required_slices":[{"slice_id":"s0","contract_hash":"contract:v1","evidence_mode":"checked","evidence_gates":["tests"],"gate_rules":{"tests":{"kind":"test","allow_skipped":false}}},{"slice_id":"s1","contract_hash":"contract:next","depends_on":["s0"],"evidence_gates":[]}]}),
    );
    put(
        r,
        "demo.s0.first.warp-layer.json",
        &json!({"schema":"warp-slice-layer-v1","warp_id":"demo","slice_id":"s0","contract_hash":"contract:v1","attempt_id":"first","result_state":"accepted","result_hash":"original-result","evidence":{"tests":["evidence:old.json"]}}),
    );
    fs::write(r.join("source.rs"), "new").unwrap();
    evidence(r, "new");
}
fn call(r: &Path, id: &str, from: &str, to: &str, flags: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_recur-warp"))
        .args([
            "refresh",
            "demo.warp-map.json",
            "demo.s0.first.warp-layer.json",
            "--gate",
            "tests",
            "--from",
            from,
            "--to",
            to,
            "--refresh-id",
            id,
            "--reason",
            "observed rerun after source change",
            "--json",
            "-d",
        ])
        .arg(r)
        .args(flags)
        .output()
        .unwrap()
}
fn show(r: &Path) -> Value {
    let o = Command::new(env!("CARGO_BIN_EXE_recur"))
        .args(["warp", "show", "demo", "--json", "-d"])
        .arg(r)
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    serde_json::from_slice(&o.stdout).unwrap()
}
fn old_bytes(r: &Path) -> BTreeMap<String, Vec<u8>> {
    [
        "demo.warp-map.json",
        "demo.s0.first.warp-layer.json",
        "old.json",
        "old.result.json",
    ]
    .into_iter()
    .map(|p| (p.into(), fs::read(r.join(p)).unwrap()))
    .collect()
}
#[test]
fn torn_staging_and_excess_history_are_bounded_blockers() {
    let d = tempdir().unwrap();
    let r = d.path();
    fixture(r);
    fs::create_dir(r.join("demo.evidence-refresh")).unwrap();
    fs::write(r.join("demo.evidence-refresh/r1.json.tmp"), "torn").unwrap();
    assert_eq!(
        call(
            r,
            "r1",
            "evidence:old.json",
            "evidence:new.json",
            &["--confirm"]
        )
        .status
        .code(),
        Some(2)
    );
    assert_eq!(
        fs::read(r.join("demo.evidence-refresh/r1.json.tmp")).unwrap(),
        b"torn"
    );
    fs::remove_file(r.join("demo.evidence-refresh/r1.json.tmp")).unwrap();
    for i in 0..65 {
        fs::write(r.join(format!("demo.evidence-refresh/{i}.json")), "{}").unwrap();
    }
    assert_eq!(
        call(
            r,
            "r1",
            "evidence:old.json",
            "evidence:new.json",
            &["--confirm"]
        )
        .status
        .code(),
        Some(2)
    );
}
#[test]
fn preview_confirm_restart_and_replay_preserve_history() {
    let d = tempdir().unwrap();
    let r = d.path();
    fixture(r);
    let old = old_bytes(r);
    assert_eq!(show(r)["state"], "blocked");
    assert!(call(r, "r1", "evidence:old.json", "evidence:new.json", &[])
        .status
        .success());
    assert!(!r.join("demo.evidence-refresh").exists());
    let o = call(
        r,
        "r1",
        "evidence:old.json",
        "evidence:new.json",
        &["--confirm"],
    );
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert_eq!(show(r)["covered"], json!(["s0"]));
    assert_eq!(show(r)["ready_slices"], json!(["s1"]));
    assert_eq!(old_bytes(r), old);
    let receipt = fs::read(r.join("demo.evidence-refresh/r1.json")).unwrap();
    assert!(call(
        r,
        "r1",
        "evidence:old.json",
        "evidence:new.json",
        &["--confirm"]
    )
    .status
    .success());
    assert_eq!(
        fs::read(r.join("demo.evidence-refresh/r1.json")).unwrap(),
        receipt
    );
}
#[test]
fn invalid_replacements_and_originals_do_not_write() {
    for case in [
        "failed",
        "zero",
        "skipped",
        "malformed",
        "project",
        "scope",
        "old-result",
        "old-failed",
        "contract",
        "layer",
        "manual",
    ] {
        let d = tempdir().unwrap();
        let r = d.path();
        fixture(r);
        match case {
            "failed" | "zero" | "skipped" => {
                let mut v = get(r, "new.result.json");
                if case == "failed" {
                    v["outcome"] = json!("failed");
                    v["exit_code"] = json!(1);
                } else if case == "zero" {
                    v["tests"] =
                        json!({"discovered":0,"executed":0,"passed":0,"failed":0,"skipped":0});
                } else {
                    v["tests"] =
                        json!({"discovered":2,"executed":1,"passed":1,"failed":0,"skipped":1});
                }
                put(r, "new.result.json", &v);
                let mut e = get(r, "new.json");
                e["result_fingerprint"] =
                    json!(fingerprint(&fs::read(r.join("new.result.json")).unwrap()));
                put(r, "new.json", &e);
            }
            "malformed" => {
                fs::write(r.join("new.json"), "{").unwrap();
            }
            "project" => {
                let mut v = get(r, "new.json");
                v["project"] = json!("other");
                put(r, "new.json", &v);
            }
            "scope" => {
                fs::write(r.join("other.rs"), "other").unwrap();
                let mut v = get(r, "new.json");
                v["source"]["files"] = json!({"other.rs":fingerprint(b"other")});
                put(r, "new.json", &v);
            }
            "old-result" => {
                fs::write(r.join("old.result.json"), "changed").unwrap();
            }
            "old-failed" => {
                let mut v = get(r, "old.result.json");
                v["exit_code"] = json!(1);
                put(r, "old.result.json", &v);
                let mut e = get(r, "old.json");
                e["result_fingerprint"] =
                    json!(fingerprint(&fs::read(r.join("old.result.json")).unwrap()));
                put(r, "old.json", &e);
            }
            "contract" => {
                let mut v = get(r, "demo.warp-map.json");
                v["required_slices"][0]["contract_hash"] = json!("contract:v2");
                put(r, "demo.warp-map.json", &v);
            }
            "layer" => {
                let mut v = get(r, "demo.s0.first.warp-layer.json");
                v["result_state"] = json!("rejected");
                put(r, "demo.s0.first.warp-layer.json", &v);
            }
            _ => {}
        }
        let old = old_bytes(r);
        let to = if case == "manual" {
            "native:ACK"
        } else {
            "evidence:new.json"
        };
        let o = call(r, "r1", "evidence:old.json", to, &["--confirm"]);
        assert_eq!(o.status.code(), Some(2), "{case}");
        assert_eq!(old_bytes(r), old);
        assert!(!r.join("demo.evidence-refresh").exists());
    }
}
#[test]
fn chain_drift_forks_and_historical_tamper_fail_closed() {
    let d = tempdir().unwrap();
    let r = d.path();
    fixture(r);
    assert!(call(
        r,
        "r1",
        "evidence:old.json",
        "evidence:new.json",
        &["--confirm"]
    )
    .status
    .success());
    fs::write(r.join("source.rs"), "third").unwrap();
    evidence(r, "third");
    assert_eq!(show(r)["state"], "blocked");
    assert!(!call(
        r,
        "r1",
        "evidence:old.json",
        "evidence:new.json",
        &["--confirm"]
    )
    .status
    .success());
    let o = call(
        r,
        "r2",
        "evidence:new.json",
        "evidence:third.json",
        &["--confirm"],
    );
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert_eq!(show(r)["covered"], json!(["s0"]));
    assert!(!call(
        r,
        "fork",
        "evidence:old.json",
        "evidence:third.json",
        &["--confirm"]
    )
    .status
    .success());
    let mut v = get(r, "demo.evidence-refresh/r1.json");
    v["reason"] = json!("conflict");
    put(r, "demo.evidence-refresh/copy.json", &v);
    assert_eq!(show(r)["state"], "blocked");
    fs::remove_file(r.join("demo.evidence-refresh/copy.json")).unwrap();
    fs::write(r.join("old.result.json"), "tamper").unwrap();
    assert_eq!(show(r)["state"], "blocked");
}
#[test]
fn other_evidence_and_contract_conflicts_remain_blockers() {
    let d = tempdir().unwrap();
    let r = d.path();
    fixture(r);
    let mut l = get(r, "demo.s0.first.warp-layer.json");
    l["evidence"]["tests"] = json!(["evidence:old.json", "evidence:missing.json"]);
    put(r, "demo.s0.first.warp-layer.json", &l);
    assert!(call(
        r,
        "r1",
        "evidence:old.json",
        "evidence:new.json",
        &["--confirm"]
    )
    .status
    .success());
    assert_eq!(show(r)["state"], "blocked");
}
#[test]
fn traversal_and_transaction_junction_are_rejected() {
    let d = tempdir().unwrap();
    let r = d.path();
    fixture(r);
    assert_eq!(
        call(
            r,
            "../escape",
            "evidence:old.json",
            "evidence:new.json",
            &["--confirm"]
        )
        .status
        .code(),
        Some(2)
    );
    assert_eq!(
        call(
            r,
            "r1",
            "evidence:old.json",
            "evidence:../outside.json",
            &["--confirm"]
        )
        .status
        .code(),
        Some(2)
    );
    let outside = tempdir().unwrap();
    #[cfg(windows)]
    {
        let o = Command::new("cmd")
            .args(["/c", "mklink", "/J"])
            .arg(r.join("demo.evidence-refresh"))
            .arg(outside.path())
            .output()
            .unwrap();
        assert!(o.status.success());
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(outside.path(), r.join("demo.evidence-refresh")).unwrap();
    assert_eq!(
        call(
            r,
            "r1",
            "evidence:old.json",
            "evidence:new.json",
            &["--confirm"]
        )
        .status
        .code(),
        Some(2)
    );
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
}
#[test]
fn orphan_cycle_changed_policy_and_result_conflict_are_not_hidden() {
    for case in ["orphan", "cycle", "policy", "result-conflict"] {
        let d = tempdir().unwrap();
        let r = d.path();
        fixture(r);
        assert!(call(
            r,
            "r1",
            "evidence:old.json",
            "evidence:new.json",
            &["--confirm"]
        )
        .status
        .success());
        match case {
            "orphan" => {
                let mut v = get(r, "demo.evidence-refresh/r1.json");
                v["from_reference"] = json!("evidence:new.json");
                v["from_hash"] = json!(fingerprint(&fs::read(r.join("new.json")).unwrap()));
                put(r, "demo.evidence-refresh/r1.json", &v);
            }
            "cycle" => {
                let mut v = get(r, "demo.evidence-refresh/r1.json");
                v["refresh_id"] = json!("r2");
                v["from_reference"] = json!("evidence:new.json");
                v["from_hash"] = json!(fingerprint(&fs::read(r.join("new.json")).unwrap()));
                v["to_reference"] = json!("evidence:old.json");
                v["to_hash"] = json!(fingerprint(&fs::read(r.join("old.json")).unwrap()));
                put(r, "demo.evidence-refresh/r2.json", &v);
            }
            "policy" => {
                let mut m = get(r, "demo.warp-map.json");
                m["required_slices"][0]["gate_rules"]["tests"]["allow_skipped"] = json!(true);
                put(r, "demo.warp-map.json", &m);
            }
            _ => {
                let mut l = get(r, "demo.s0.first.warp-layer.json");
                l["attempt_id"] = json!("other");
                l["result_hash"] = json!("conflicting-result");
                l["evidence"]["tests"] = json!(["evidence:new.json"]);
                put(r, "demo.s0.other.warp-layer.json", &l);
            }
        }
        let progress = show(r);
        assert!(
            progress["covered"].as_array().unwrap().is_empty(),
            "{case}: {progress}"
        );
        assert_eq!(
            progress["state"],
            if case == "result-conflict" {
                "exploded"
            } else {
                "blocked"
            }
        );
    }
}
