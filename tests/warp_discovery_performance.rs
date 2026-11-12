use serde_json::{json, Value};
use std::{fs, path::Path, process::Command};

fn query(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_recur"))
        .arg("warp")
        .args(args)
        .arg("-d")
        .arg(root)
        .arg("--json")
        .output()
        .unwrap()
}

fn json_output(output: std::process::Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn write_map(dir: &Path) {
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join("fixture.work.warp-map.json"),
        json!({
            "schema": "warp-bubble-map-v1", "warp_id": "fixture.work",
            "required_slices": [{"slice_id": "one", "contract_hash": "one-v1"}]
        })
        .to_string(),
    )
    .unwrap();
}

#[test]
fn targeted_queries_share_inventory_policy_and_keep_scope_diagnostics() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let boards = root.join("boards");
    write_map(&boards);
    write_map(&root.join("target2/deep"));
    write_map(&root.join("__pycache__/deep"));
    let show = json_output(query(root, &["show", "fixture.work"]));
    assert_eq!(show["warp_id"], "fixture.work");
    assert_eq!(show, json_output(query(root, &["slices", "fixture.work"])));
    let all = json_output(query(root, &["list", "--scan-all"]));
    assert_eq!(all["discovered"], 3);
    assert!(query(root, &["map", "fixture.work"]).status.success());
    assert!(query(&root.join("target2"), &["map", "fixture.work"])
        .status
        .success());

    let private = root.join(".recur/work");
    write_map(&private);
    let duplicate = query(root, &["show", "fixture.work"]);
    assert!(!duplicate.status.success());
    assert!(String::from_utf8_lossy(&duplicate.stderr).contains("ambiguous Warp identity"));
    assert_eq!(
        json_output(query(&private, &["show", "fixture.work"]))["warp_id"],
        "fixture.work"
    );

    fs::write(
        root.join(".recur/config.toml"),
        "[warp.discovery]\nroots = [\"target2\"]\nexclude_dirs = []\n",
    )
    .unwrap();
    let configured = json_output(query(root, &["show", "fixture.work"]));
    assert!(configured["root"].as_str().unwrap().contains("target2"));
}

#[test]
fn ring_inventory_uses_its_local_coordinator_layers() {
    let temp = tempfile::tempdir().unwrap();
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("julia-tests/fixtures/warp-ring-v1/complete");
    for relative in [".", "workers/docs-monkey", "workers/test-bird"] {
        let dest = temp.path().join(relative);
        fs::create_dir_all(&dest).unwrap();
        for item in fs::read_dir(fixture.join(relative)).unwrap() {
            let item = item.unwrap();
            if item.file_type().unwrap().is_file() {
                fs::copy(item.path(), dest.join(item.file_name())).unwrap();
            }
        }
    }
    let unrelated = temp.path().join("manufacturing/archive");
    fs::create_dir_all(&unrelated).unwrap();
    fs::write(
        unrelated.join("coordinator.release.accept-tests.unrelated.warp-layer.json"),
        "invalid outside the coordinator",
    )
    .unwrap();
    let inventory = json_output(query(temp.path(), &["list", "--all"]));
    let ring = inventory["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["warp_id"] == "coordinator.release")
        .unwrap();
    assert_eq!(ring["kind"], "ring");
    assert_eq!(ring["state"], "complete");
}
