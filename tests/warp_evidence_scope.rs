use recur::warp_evidence::fingerprint;
use serde_json::{json, Value};
use std::{fs, path::Path, process::{Command, Output}};
use tempfile::tempdir;

fn put(root: &Path, name: &str, value: &Value) {
    fs::write(root.join(name), serde_json::to_vec_pretty(value).unwrap()).unwrap();
}
fn evidence(root: &Path, name: &str) {
    put(root, &format!("evidence/{name}.result.json"), &json!({
        "schema":"warp-external-result-v1", "kind":"test", "outcome":"passed", "exit_code":0,
        "tests":{"discovered":1,"executed":1,"passed":1,"failed":0,"skipped":0}
    }));
    put(root, &format!("evidence/{name}.json"), &json!({
        "schema":"warp-external-evidence-v1", "kind":"test", "producer":"fixture", "project":"demo",
        "configuration":"native", "platform":"local", "executed_at_unix":1,
        "result_artifact":format!("evidence/{name}.result.json"),
        "result_fingerprint":fingerprint(&fs::read(root.join(format!("evidence/{name}.result.json"))).unwrap()),
        "source":{"revision":null,"dirty":true,"files":{"src/source.rs":fingerprint(&fs::read(root.join("src/source.rs")).unwrap())}}
    }));
}
fn fixture(root: &Path, metadata: Option<Value>) {
    for dir in ["warps", "src", "evidence"] { fs::create_dir(root.join(dir)).unwrap(); }
    fs::write(root.join("src/source.rs"), "old").unwrap();
    evidence(root, "old");
    let mut map = json!({"schema":"warp-bubble-map-v1","warp_id":"demo",
        "required_slices":[{"slice_id":"work","contract_hash":"work:v1","evidence_mode":"checked",
        "evidence_gates":["tests"],"gate_rules":{"tests":{"kind":"test","allow_skipped":false}}}]});
    if let Some(value) = metadata { map["evidence_root"] = value; }
    put(root, "warps/demo.warp-map.json", &map);
}
fn invoke(binary: &str, root: &Path, args: &[&str]) -> Output {
    Command::new(binary).args(args).args(["--json", "-d"]).arg(root).output().unwrap()
}
fn core(root: &Path, args: &[&str]) -> Value {
    let output = invoke(env!("CARGO_BIN_EXE_recur"), root, args);
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    serde_json::from_slice(&output.stdout).unwrap()
}
fn complete(root: &Path, confirm: bool) -> Output {
    let mut args = vec!["complete", "demo", "work", "--attempt-id", "first", "--result-hash", "result:v1",
        "--evidence", "tests=evidence:evidence/old.json"];
    if confirm { args.push("--confirm"); }
    invoke(env!("CARGO_BIN_EXE_recur-warp"), root, &args)
}
fn assert_completed(root: &Path) {
    assert_eq!(core(root, &["warp", "show", "demo"])["completed_slices"], json!(["work"]));
    assert_eq!(core(root, &["warp", "merge", "demo"])["covered"], json!(["work"]));
    let list = core(root, &["warp", "list", "--all"]);
    assert_eq!(list["entries"][0]["state"], "complete");
    assert_eq!(list["entries"][0]["evidence_status"], "checked");
}

#[test]
fn explicit_project_root_completion_queries_and_refresh_use_real_source_bytes() {
    let temp = tempdir().unwrap();
    let root = temp.path().join("project"); fs::create_dir(&root).unwrap();
    fixture(&root, Some(json!("..")));
    let result = complete(&root, true);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_completed(&root);
    assert_completed(temp.path());
    let layer = root.join("warps/demo.work.first.warp-layer.json");
    let accepted = fs::read(&layer).unwrap();
    fs::write(root.join("src/source.rs"), "changed actual source").unwrap();
    assert_eq!(core(&root, &["warp", "show", "demo"])["state"], "blocked");
    evidence(&root, "new");
    let result = invoke(env!("CARGO_BIN_EXE_recur-warp"), temp.path(), &[
        "refresh", "project/warps/demo.warp-map.json", "project/warps/demo.work.first.warp-layer.json",
        "--gate", "tests", "--from", "evidence:evidence/old.json", "--to", "evidence:evidence/new.json",
        "--refresh-id", "rerun", "--reason", "observed source rerun", "--confirm"]);
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    assert_completed(&root); assert_completed(temp.path());
    assert_eq!(fs::read(layer).unwrap(), accepted);
    let receipt: Value = serde_json::from_slice(&fs::read(root.join("warps/demo.evidence-refresh/rerun.json")).unwrap()).unwrap();
    assert_eq!(receipt["map"], "warps/demo.warp-map.json");
    assert_eq!(receipt["layer"], "warps/demo.work.first.warp-layer.json");
    fs::write(root.join("src/source.rs"), "changed after refresh").unwrap();
    assert_eq!(core(&root, &["warp", "show", "demo"])["state"], "blocked");
}

#[test]
fn absent_metadata_preserves_manifest_scoped_inventory() {
    let temp = tempdir().unwrap(); let root = temp.path();
    fixture(root, None);
    assert!(complete(root, true).status.success());
    assert_eq!(core(root, &["warp", "show", "demo"])["state"], "blocked");
    assert_eq!(core(root, &["warp", "list", "--all"])["entries"][0]["state"], "blocked");
    assert_eq!(core(root, &["warp", "merge", "demo"])["state"], "complete");
}

#[test]
fn explicit_root_cannot_widen_a_narrow_requested_directory() {
    let temp = tempdir().unwrap(); let root = temp.path();
    fixture(root, Some(json!("..")));
    let narrow = root.join("warps");
    for args in [vec!["warp", "show", "demo"], vec!["warp", "merge", "demo"]] {
        assert!(!invoke(env!("CARGO_BIN_EXE_recur"), &narrow, &args).status.success());
    }
    let list = core(&narrow, &["warp", "list", "--all"]);
    assert_eq!(list["errors"], 1);
    assert!(!complete(&narrow, false).status.success());
    assert!(!root.join("warps/demo.work.first.warp-layer.json").exists());
}

#[test]
fn malformed_absolute_missing_and_outside_roots_are_refused() {
    for metadata in [json!(null), json!(3), json!(""), json!("/"), json!("C:/"), json!("..\\.."),
        json!("../.."), json!("../missing"), json!("../src"), json!("../src/source.rs"), json!("..//")] {
        let temp = tempdir().unwrap(); let root = temp.path();
        fixture(root, Some(metadata.clone()));
        assert!(!complete(root, false).status.success(), "{metadata}");
        assert!(!invoke(env!("CARGO_BIN_EXE_recur"), root, &["warp", "show", "demo"]).status.success(), "{metadata}");
        assert!(!invoke(env!("CARGO_BIN_EXE_recur"), root, &["warp", "merge", "demo"]).status.success(), "{metadata}");
        assert_eq!(core(root, &["warp", "list", "--all"])["errors"], 1, "{metadata}");
        assert!(!root.join("warps/demo.work.first.warp-layer.json").exists());
    }
}

#[test]
fn evidence_root_map_read_obeys_exact_json_byte_boundary() {
    let temp = tempdir().unwrap(); let root = temp.path();
    let map = root.join("demo.warp-map.json");
    let mut bytes = b"{}".to_vec(); bytes.resize(2 * 1024 * 1024, b' ');
    fs::write(&map, &bytes).unwrap();
    assert!(recur::warp_evidence::resolve_root(root, &map, root).is_ok());
    bytes.push(b' '); fs::write(&map, &bytes).unwrap();
    assert!(recur::warp_evidence::resolve_root(root, &map, root).unwrap_err().to_string().contains("2 MiB"));
}
