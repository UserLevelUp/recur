#[path = "support/checked_fixture.rs"]
mod fixture;
use fixture::{fixture, put};
use serde_json::{json, Value};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
use tempfile::tempdir;
#[test]
fn immutable_inputs_cannot_alias_the_moving_artifact() {
    use recur::warp_evidence::fingerprint;
    let d = tempdir().unwrap();
    let r = d.path();
    fixture(r);
    let alias = if cfg!(windows) {
        "BUILD.CURRENT.MD"
    } else {
        "build.current.md"
    };
    let mut evidence: Value =
        serde_json::from_slice(&fs::read(r.join("evidence.json")).unwrap()).unwrap();
    evidence["source"]["files"][alias] = json!(fingerprint(&fs::read(r.join(alias)).unwrap()));
    put(r, "evidence.json", &evidence);
    assert_eq!(call(r, &["--confirm"]).status.code(), Some(2));
    assert!(r.join("build.current.md").exists());
    assert!(!r.join(".recur").exists());
}
fn call(root: &Path, extra: &[&str]) -> Output {
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
        .arg(root)
        .args(extra)
        .output()
        .unwrap()
}
#[test]
fn dry_run_confirm_and_exact_replay() {
    let dir = tempdir().unwrap();
    let r = dir.path();
    fixture(r);
    let original = fs::read(r.join("build.current.md")).unwrap();
    let dry = call(r, &[]);
    assert!(dry.status.success());
    assert!(!r.join(".recur").exists());
    assert_eq!(fs::read(r.join("build.current.md")).unwrap(), original);
    let accepted = call(r, &["--confirm"]);
    assert!(
        accepted.status.success(),
        "{}",
        String::from_utf8_lossy(&accepted.stderr)
    );
    assert!(!r.join("build.current.md").exists());
    assert_eq!(fs::read(r.join("build.complete.md")).unwrap(), original);
    let status = r.join(".recur/lang/checked/attempt-1/accepted.json");
    let bytes = fs::read(&status).unwrap();
    let replay = call(r, &["--confirm"]);
    assert!(
        replay.status.success(),
        "{}",
        String::from_utf8_lossy(&replay.stderr)
    );
    assert_eq!(fs::read(status).unwrap(), bytes);
}
#[test]
fn evidence_rejections_preserve_e0() {
    for (id, field, value) in [
        ("wrong-scope", "scope", json!("other.f")),
        ("manual", "evidence", json!("native:accepted")),
        ("red", "phase", json!("red")),
        ("stale", "contract_hash", json!("old")),
    ] {
        let dir = tempdir().unwrap();
        let r = dir.path();
        fixture(r);
        let before = fs::read(r.join("build.current.md")).unwrap();
        let mut attempt: Value =
            serde_json::from_slice(&fs::read(r.join("attempt.json")).unwrap()).unwrap();
        attempt[field] = value;
        put(r, "attempt.json", &attempt);
        let output = call(r, &["--confirm"]);
        assert!(!output.status.success(), "{id} must reject");
        assert_eq!(fs::read(r.join("build.current.md")).unwrap(), before);
        assert!(!r.join("build.complete.md").exists());
        assert!(!r.join(".recur").exists());
    }
}
#[test]
fn existing_destinations_and_attempt_conflicts_do_not_overwrite() {
    let dir = tempdir().unwrap();
    let r = dir.path();
    fixture(r);
    fs::write(r.join("build.complete.md"), "occupied").unwrap();
    assert!(!call(r, &["--confirm"]).status.success());
    assert_eq!(fs::read(r.join("build.complete.md")).unwrap(), b"occupied");
    assert!(r.join("build.current.md").exists());
    let dir = tempdir().unwrap();
    let r = dir.path();
    fixture(r);
    assert!(call(r, &["--confirm"]).status.success());
    let status = r.join(".recur/lang/checked/attempt-1/accepted.json");
    let before = fs::read(&status).unwrap();
    let mut a: Value = serde_json::from_slice(&fs::read(r.join("attempt.json")).unwrap()).unwrap();
    a["runtime"] = json!("different request");
    put(r, "attempt.json", &a);
    assert!(!call(r, &["--confirm"]).status.success());
    assert_eq!(fs::read(status).unwrap(), before);
}
#[test]
fn drift_between_plan_and_confirm_rejects() {
    let dir = tempdir().unwrap();
    let r = dir.path();
    fixture(r);
    assert!(call(r, &[]).status.success());
    fs::write(r.join("implementation.rs"), "changed").unwrap();
    assert!(!call(r, &["--confirm"]).status.success());
    assert!(r.join("build.current.md").exists());
    assert!(!r.join("build.complete.md").exists());
}
#[test]
fn accepted_history_does_not_hide_current_drift() {
    let dir = tempdir().unwrap();
    let r = dir.path();
    fixture(r);
    assert!(call(r, &["--confirm"]).status.success());
    let path = ".recur/lang/checked/attempt-1/accepted.json";
    let before = fs::read(r.join(path)).unwrap();
    fs::write(r.join("test.rs"), "changed tests").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_recur"))
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
            "--status",
            path,
            "-d",
        ])
        .arg(r)
        .arg("--json")
        .output()
        .unwrap();
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["assessment"]["status"], "stale");
    assert_eq!(report["transition_status"]["accepted"], true);
    assert_eq!(report["transition_status"]["current_accepted"], false);
    assert_eq!(fs::read(r.join(path)).unwrap(), before);
    assert!(!call(r, &["--confirm"]).status.success());
}
#[test]
fn exact_exit_codes_and_unrelated_bytes_are_preserved() {
    for (field, value, code) in [
        ("schema", json!("unsupported"), 2),
        ("phase", json!("red"), 1),
        ("evidence", json!("manual:ACK"), 1),
    ] {
        let d = tempdir().unwrap();
        let r = d.path();
        fixture(r);
        fs::write(r.join("unrelated.bin"), [0, 255, 7]).unwrap();
        let mut a: Value =
            serde_json::from_slice(&fs::read(r.join("attempt.json")).unwrap()).unwrap();
        a[field] = value;
        put(r, "attempt.json", &a);
        assert_eq!(call(r, &["--confirm"]).status.code(), Some(code));
        assert_eq!(fs::read(r.join("unrelated.bin")).unwrap(), [0, 255, 7]);
        assert!(r.join("build.current.md").exists());
        assert!(!r.join(".recur").exists());
    }
    let d = tempdir().unwrap();
    fixture(d.path());
    assert_eq!(
        call(d.path(), &["--id", "legacy", "--confirm"])
            .status
            .code(),
        Some(2)
    );
}
#[test]
fn wrong_eventness_and_unsafe_paths_cannot_move_files() {
    for name in [
        "wrong.md",
        "../build.current.md",
        "C:/build.current.md",
        "\\\\server\\share\\build.current.md",
    ] {
        let d = tempdir().unwrap();
        let r = d.path();
        fixture(r);
        fs::write(r.join("wrong.md"), "unrelated").unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_recur-lang"))
            .args([
                "warp",
                "spec.recur",
                "build.f",
                "--checked-contract",
                "policy.json",
                "--receipt",
                "attempt.json",
                "--eventness",
                name,
                "--confirm",
                "--json",
                "-d",
            ])
            .arg(r)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        assert!(r.join("build.current.md").exists());
        assert_eq!(fs::read(r.join("wrong.md")).unwrap(), b"unrelated");
        assert!(!r.join(".recur").exists());
    }
}
#[test]
fn existing_status_and_torn_staging_are_bounded_blockers() {
    for name in ["accepted.json", "prepared.json.tmp", "unexpected"] {
        let d = tempdir().unwrap();
        let r = d.path();
        fixture(r);
        let lane = r.join(".recur/lang/checked/attempt-1");
        fs::create_dir_all(&lane).unwrap();
        fs::write(lane.join(name), "unrelated or torn").unwrap();
        for flags in [&["--confirm"][..], &["--recover", "--confirm"][..]] {
            assert_eq!(call(r, flags).status.code(), Some(2));
        }
        assert_eq!(fs::read(lane.join(name)).unwrap(), b"unrelated or torn");
        assert!(r.join("build.current.md").exists());
        assert!(!r.join("build.complete.md").exists());
    }
}
#[test]
fn transaction_junction_escape_is_rejected() {
    let d = tempdir().unwrap();
    let r = d.path();
    fixture(r);
    let outside = tempdir().unwrap();
    #[cfg(windows)]
    {
        let output = Command::new("cmd")
            .args(["/c", "mklink", "/J"])
            .arg(r.join(".recur"))
            .arg(outside.path())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "junction fixture failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(outside.path(), r.join(".recur")).unwrap();
    assert_eq!(call(r, &["--confirm"]).status.code(), Some(2));
    assert!(r.join("build.current.md").exists());
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 0);
}
