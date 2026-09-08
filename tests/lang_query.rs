use assert_cmd::Command;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const ALGORITHM: &str = include_str!("../demos/main.lang/main.lang.algorithm-lab.recur");
const SKIPPY: &str = include_str!("../demos/main.lang/main.lang.skippy-watch-coordination.recur");
fn run(root: &Path, args: &[&str], exit: i32) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_recur"))
        .arg("lang")
        .args(args)
        .arg("-d")
        .arg(root)
        .arg("--json")
        .assert()
        .code(exit)
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&output).unwrap()
}
fn inventory(root: &Path) -> BTreeMap<String, Vec<u8>> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .map(Result::unwrap)
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            (
                e.path()
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
                fs::read(e.path()).unwrap(),
            )
        })
        .collect()
}
#[test]
fn pure_queries_preserve_unicode_exact_inventory_and_scope_boundaries() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("λ-project");
    fs::create_dir(&root).unwrap();
    fs::create_dir(root.join(".recur")).unwrap();
    fs::write(
        root.join(".recur/config.toml"),
        "[status]\ncurrent_suffix = 'todo.current'\n",
    )
    .unwrap();
    fs::write(root.join("receipt.json"), "{\"accepted\":false}").unwrap();
    fs::write(parent.path().join("sibling.recur"), ALGORITHM).unwrap();
    fs::write(
        root.join("算法.recur"),
        ALGORITHM.replace("Euclid greatest common divisor", "Ευκλείδης 最大公約数"),
    )
    .unwrap();
    fs::write(root.join("coordination.recur"), SKIPPY).unwrap();
    fs::write(
        root.join("unsupported.recur"),
        ALGORITHM.replace("recur 0.1", "recur 0.3"),
    )
    .unwrap();
    let before = inventory(parent.path());
    let listing = run(&root, &["list"], 0);
    assert_eq!(listing["sources"].as_array().unwrap().len(), 3);
    assert!(listing["sources"]
        .as_array()
        .unwrap()
        .iter()
        .any(|s| s["diagnostics"][0]["code"] == "LANG005"));
    let shown = run(
        &root,
        &["show", "算法.recur", "--scope", "gcd.f", "--expand"],
        0,
    );
    assert_eq!(shown["header"][0]["meaning"], "Ευκλείδης 最大公約数");
    assert_eq!(shown["header"][0]["binding"], "gcd.euclid");
    assert_eq!(shown["footer"]["execution"], "not-run");
    run(
        &root,
        &[
            "report",
            "coordination.recur",
            "--scope",
            "review_bird",
            "--expand",
        ],
        0,
    );
    run(&root, &["check", "coordination.recur"], 0);
    assert_eq!(
        run(&root, &["show", "算法.recur", "--scope", "f"], 2)["diagnostics"][0]["code"],
        "LANG004"
    );
    assert_eq!(
        run(&root, &["show", "../sibling.recur", "--scope", "gcd"], 2)["diagnostics"][0]["code"],
        "LANG002"
    );
    assert_eq!(
        run(&root, &["check", "missing.recur"], 2)["diagnostics"][0]["code"],
        "LANG001"
    );
    assert_eq!(before, inventory(parent.path()));
}
#[test]
fn checks_distinguish_graph_findings_from_input_errors() {
    let root = tempfile::tempdir().unwrap();
    // All syntax remains CIR1; a missing fork receipt is a static finding.
    let broken = SKIPPY.replace(
        "await [csharp_monkey.o(b), web_monkey.o(b), test_bird.o(b)]",
        "await [csharp_monkey.o(b), web_monkey.o(b)]",
    );
    assert_ne!(broken, SKIPPY);
    fs::write(root.path().join("missing-join.recur"), broken).unwrap();
    let result = run(
        root.path(),
        &["check", "missing-join.recur", "--scope", "git_monkey"],
        1,
    );
    assert!(result["footer"]["findings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|f| f["code"] == "SGR004"));
    assert_eq!(result["coverage"]["whole_source_validated"], false);
    fs::write(
        root.path().join("bad.recur"),
        "recur 0.1 class Broken\nscope x {\n",
    )
    .unwrap();
    assert_eq!(
        run(root.path(), &["check", "bad.recur"], 2)["diagnostics"][0]["code"],
        "LANG006"
    );
}
#[test]
fn default_eventness_is_an_observed_file_not_a_requested_transition() {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("algorithm.recur"), ALGORITHM).unwrap();
    let args = ["report", "algorithm.recur", "--eventness", "complete"];
    assert!(run(root.path(), &args, 0)["header"]
        .as_array()
        .unwrap()
        .is_empty());
    fs::write(
        root.path().join("demo.algorithm.gcd.complete.md"),
        "recorded",
    )
    .unwrap();
    assert_eq!(
        run(root.path(), &args, 0)["header"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}
