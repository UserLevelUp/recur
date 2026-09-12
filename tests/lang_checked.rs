use recur::recur_lang_evidence::{assess, EvidenceArgs};
use recur::warp_evidence::fingerprint;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
use tempfile::{tempdir, TempDir};

const SOURCE: &str = r#"recur 0.1 class Checked
header {
  scope seed {
    i(a) := (request: Text)
    o(b) := (value: Text)
    f : i(a) -> o(b) ~ "Produce" by forbidden.runner
  }
  scope build {
    i(b) := seed.o(b)
    o(c) := (value: Text)
    f : i(b) -> o(c) ~ "Consume" by forbidden.runner
  }
}
body {
  seed sync : i(a) -> f(a) -> o(b)
  build sync : i(b) -> f(b) -> o(c)
  share seed.o(b) -> build.i(b)
}
footer {
  event seed {
    state seed.complete
  }
  warp seed : E0(seed.current) -> dE(seed.f) -> Ef(seed.complete)
  event build {
    consume seed.output
    state build.complete
  }
  warp build : E0(build.current) -> dE(build.f) -> Ef(build.complete)
}
"#;
fn write_json(root: &Path, path: &str, v: &Value) {
    fs::write(root.join(path), serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
struct Fixture {
    dir: TempDir,
    args: EvidenceArgs,
    policy: Value,
    attempt: Value,
    evidence: Value,
    result: Value,
}
impl Fixture {
    fn new() -> Self {
        let dir = tempdir().unwrap();
        let r = dir.path();
        fs::write(r.join("spéc.recur"), SOURCE).unwrap();
        for p in [
            "implementation.rs",
            "tests.rs",
            "config.toml",
            "runner.rs",
            "behavior.md",
        ] {
            fs::write(r.join(p), p).unwrap();
        }
        fs::write(r.join("build.current.md"), "preserve E0 bytes\r\nJosé").unwrap();
        fs::write(r.join("trap.recur.md"), "pull.first = forbidden.runner").unwrap();
        let policy = json!({"schema":"recur-lang-checked-contract-v1","contract_id":"test.contract.v1","source":"spéc.recur",
            "source_hash":fingerprint(SOURCE.as_bytes()),"scope":"build.f","aliases":{"build.i(b)":"seed.o(b)","build.o(c)":"build.o(c)"},
            "transition":{"current":"build.current","slice":"build.f","desired":"build.complete"},
            "inputs":{"specification":["spéc.recur"],"implementation":["implementation.rs"],"tests":["tests.rs"],"configuration":["config.toml"],"runner":["runner.rs"],"behavior":["behavior.md"]},
            "requirements":[{"id":"req.one","cases":["one"]},{"id":"req.two","cases":["two"]}]});
        write_json(r, "policy.json", &policy);
        let result = json!({"schema":"warp-external-result-v1","kind":"test","outcome":"passed","exit_code":0,
            "tests":{"discovered":2,"executed":2,"passed":2,"failed":0,"skipped":0}});
        write_json(r, "result.json", &result);
        let files: BTreeMap<_, _> = [
            "spéc.recur",
            "implementation.rs",
            "tests.rs",
            "config.toml",
            "runner.rs",
            "behavior.md",
            "policy.json",
        ]
        .into_iter()
        .map(|p| (p.to_string(), fingerprint(&fs::read(r.join(p)).unwrap())))
        .collect();
        let evidence = json!({"schema":"warp-external-evidence-v1","kind":"test","producer":"honest test fixture","project":"fixture","configuration":"tests","platform":"local","executed_at_unix":1,
            "result_artifact":"result.json","result_fingerprint":fingerprint(&fs::read(r.join("result.json")).unwrap()),"source":{"revision":null,"dirty":true,"files":files}});
        let attempt = json!({"schema":"recur-lang-checked-receipt-v1","attempt_id":"green-1","contract_hash":fingerprint(&fs::read(r.join("policy.json")).unwrap()),
            "source_hash":fingerprint(SOURCE.as_bytes()),"scope":"build.f","phase":"green","producer":"honest test fixture","runtime":"fixture native runtime",
            "evidence":"evidence:evidence.json","cases":[{"id":"one","outcome":"passed"},{"id":"two","outcome":"passed"}]});
        let args = EvidenceArgs {
            source: "spéc.recur".into(),
            scope: "build.f".into(),
            contract: "policy.json".into(),
            receipt: Some("attempt.json".into()),
            status: None,
            expand: false,
        };
        Self {
            dir,
            args,
            policy,
            attempt,
            evidence,
            result,
        }
    }
    fn save(&self) {
        write_json(self.dir.path(), "policy.json", &self.policy);
        write_json(self.dir.path(), "attempt.json", &self.attempt);
        write_json(self.dir.path(), "evidence.json", &self.evidence);
        write_json(self.dir.path(), "result.json", &self.result);
    }
    fn rebind(&mut self) {
        self.save();
        self.attempt["contract_hash"] = json!(fingerprint(
            &fs::read(self.dir.path().join("policy.json")).unwrap()
        ));
        self.evidence["source"]["files"]["policy.json"] = self.attempt["contract_hash"].clone();
        self.evidence["result_fingerprint"] = json!(fingerprint(
            &fs::read(self.dir.path().join("result.json")).unwrap()
        ));
    }
}
type Change = Box<dyn Fn(&mut Fixture)>;
fn bytes(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fs::read_dir(root)
        .unwrap()
        .map(|p| p.unwrap().path())
        .filter(|p| p.is_file() && !p.is_symlink())
        .map(|p| {
            let b = fs::read(&p).unwrap();
            (p, b)
        })
        .collect()
}
fn cases(cases: Vec<(&str, &str, Change)>) {
    let mut failures = Vec::new();
    for (id, expected, change) in cases {
        let mut f = Fixture::new();
        change(&mut f);
        f.save();
        let before = bytes(f.dir.path());
        let report = assess(f.dir.path(), &f.args);
        assert_eq!(before, bytes(f.dir.path()), "purity {id}");
        let actual = report["assessment"]["status"].as_str().unwrap_or("missing");
        println!("CASE {id}: expected={expected} actual={actual}");
        if actual != expected {
            failures.push(format!("{id}: {report}"));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
#[test]
fn exact_identity_and_purity() {
    cases(vec![
        ("identity.valid", "checked", Box::new(|_| {})),
        (
            "identity.source",
            "mismatched",
            Box::new(|f| f.policy["source"] = json!("other.recur")),
        ),
        (
            "identity.scope",
            "mismatched",
            Box::new(|f| f.policy["scope"] = json!("seed.f")),
        ),
        (
            "identity.alias",
            "mismatched",
            Box::new(|f| f.policy["aliases"]["build.i(b)"] = json!("other.o(b)")),
        ),
        (
            "identity.transition",
            "mismatched",
            Box::new(|f| f.policy["transition"]["desired"] = json!("other.complete")),
        ),
        (
            "identity.local",
            "ambiguous",
            Box::new(|f| f.args.scope = "f".into()),
        ),
        (
            "identity.unknown",
            "mismatched",
            Box::new(|f| f.args.scope = "absent.f".into()),
        ),
        (
            "identity.unknown-function",
            "mismatched",
            Box::new(|f| f.args.scope = "build.missing".into()),
        ),
    ]);
}
#[test]
fn schema_and_required_scope() {
    cases(vec![
        (
            "policy.schema",
            "malformed",
            Box::new(|f| f.policy["schema"] = json!("future")),
        ),
        (
            "policy.unknown",
            "malformed",
            Box::new(|f| f.policy["unexpected"] = json!(true)),
        ),
        (
            "policy.missing",
            "malformed",
            Box::new(|f| {
                f.policy.as_object_mut().unwrap().remove("inputs");
            }),
        ),
        (
            "policy.empty-role",
            "malformed",
            Box::new(|f| f.policy["inputs"]["runner"] = json!([])),
        ),
        (
            "policy.duplicate",
            "ambiguous",
            Box::new(|f| f.policy["requirements"][1]["id"] = json!("req.one")),
        ),
        (
            "policy.duplicate-case",
            "ambiguous",
            Box::new(|f| f.policy["requirements"][0]["cases"] = json!(["one", "one"])),
        ),
        (
            "scope.required",
            "mismatched",
            Box::new(|f| {
                f.evidence["source"]["files"]
                    .as_object_mut()
                    .unwrap()
                    .remove("config.toml");
            }),
        ),
        (
            "scope.policy",
            "mismatched",
            Box::new(|f| {
                f.evidence["source"]["files"]
                    .as_object_mut()
                    .unwrap()
                    .remove("policy.json");
            }),
        ),
    ]);
}
#[test]
fn attempt_relevance_and_history() {
    cases(vec![
        (
            "attempt.absent",
            "absent",
            Box::new(|f| f.args.receipt = None),
        ),
        (
            "attempt.manual",
            "declared",
            Box::new(|f| f.attempt["evidence"] = json!("native:accepted-with-missing-test")),
        ),
        (
            "attempt.schema",
            "malformed",
            Box::new(|f| f.attempt["schema"] = json!("recur-lang-warp-receipt-v1")),
        ),
        (
            "attempt.scope",
            "mismatched",
            Box::new(|f| f.attempt["scope"] = json!("seed.f")),
        ),
        (
            "attempt.producer",
            "mismatched",
            Box::new(|f| f.attempt["producer"] = json!("other")),
        ),
        (
            "attempt.hash",
            "stale",
            Box::new(|f| f.attempt["contract_hash"] = json!("old")),
        ),
        (
            "attempt.source-hash",
            "stale",
            Box::new(|f| f.attempt["source_hash"] = json!("old")),
        ),
        (
            "attempt.missing-case",
            "mismatched",
            Box::new(|f| f.attempt["cases"] = json!([{"id":"one","outcome":"passed"}])),
        ),
        (
            "attempt.duplicate",
            "ambiguous",
            Box::new(|f| f.attempt["cases"][1]["id"] = json!("one")),
        ),
        (
            "attempt.red",
            "failed",
            Box::new(|f| {
                f.attempt["phase"] = json!("red");
                f.attempt["cases"][0]["outcome"] = json!("failed");
            }),
        ),
    ]);
}
#[test]
fn each_live_input_drift() {
    let mut rows: Vec<(&str, &str, Change)> = Vec::new();
    for p in [
        "spéc.recur",
        "implementation.rs",
        "tests.rs",
        "config.toml",
        "runner.rs",
        "behavior.md",
    ] {
        rows.push((
            p,
            "stale",
            Box::new(move |f| {
                let path = f.dir.path().join(p);
                let mut s = fs::read(&path).unwrap();
                s.extend_from_slice(b"\n# changed\n");
                fs::write(path, s).unwrap();
            }),
        ));
    }
    rows.push((
        "policy.drift",
        "stale",
        Box::new(|f| f.policy["contract_id"] = json!("revised.contract")),
    ));
    rows.push((
        "result.drift",
        "stale",
        Box::new(|f| f.result["tests"]["discovered"] = json!(3)),
    ));
    // The latter also has invalid counts: failure takes precedence over staleness.
    rows.last_mut().unwrap().1 = "failed";
    cases(rows);
}
#[test]
fn counts_cannot_be_forged() {
    let mut rows: Vec<(&str, &str, Change)> = vec![];
    for (id, counts) in [
        ("zero", [0, 0, 0, 0, 0]),
        ("skipped", [3, 2, 2, 0, 1]),
        ("failed", [2, 2, 1, 1, 0]),
        ("inconsistent", [3, 2, 2, 0, 0]),
        ("undispatched", [2, 1, 1, 0, 0]),
    ] {
        rows.push((id,"failed",Box::new(move|f|{f.result["tests"]=json!({"discovered":counts[0],"executed":counts[1],"passed":counts[2],"failed":counts[3],"skipped":counts[4]});f.rebind();})));
    }
    rows.push((
        "false-labels",
        "failed",
        Box::new(|f| {
            f.result["outcome"] = json!("failed");
            f.result["exit_code"] = json!(1);
            f.rebind();
        }),
    ));
    cases(rows);
}
#[test]
fn rejected_paths_and_transitive_reads() {
    let mut rows: Vec<(&str, &str, Change)> = vec![];
    for p in [
        "../escape",
        "/escape",
        "C:/escape",
        "//server/share",
        "a\\b",
        "a/../b",
        "a//b",
        "./evidence.json",
        "a\0b",
        "missing.json",
    ] {
        rows.push((
            p,
            "malformed",
            Box::new(move |f| f.attempt["evidence"] = json!(format!("evidence:{p}"))),
        ));
    }
    rows.push((
        "directory",
        "malformed",
        Box::new(|f| {
            fs::create_dir(f.dir.path().join("directory")).unwrap();
            f.attempt["evidence"] = json!("evidence:directory");
        }),
    ));
    rows.push((
        "input.escape",
        "malformed",
        Box::new(|f| f.evidence["source"]["files"]["../outside"] = json!("x")),
    ));
    rows.push((
        "result.escape",
        "malformed",
        Box::new(|f| f.evidence["result_artifact"] = json!("../outside")),
    ));
    cases(rows);
}
#[test]
fn source_and_collection_limits() {
    cases(vec![
        (
            "source.limit",
            "malformed",
            Box::new(|f| {
                fs::write(f.dir.path().join("spéc.recur"), vec![b' '; 1048577]).unwrap();
            }),
        ),
        (
            "input.limit",
            "malformed",
            Box::new(|f| {
                fs::write(
                    f.dir.path().join("implementation.rs"),
                    vec![0; 8 * 1024 * 1024 + 1],
                )
                .unwrap();
            }),
        ),
        (
            "input.at-limit",
            "checked",
            Box::new(|f| {
                let p = f.dir.path().join("implementation.rs");
                fs::write(&p, vec![0; 8 * 1024 * 1024]).unwrap();
                f.evidence["source"]["files"]["implementation.rs"] =
                    json!(fingerprint(&fs::read(p).unwrap()));
            }),
        ),
        (
            "total.limit",
            "malformed",
            Box::new(|f| {
                for p in ["implementation.rs", "tests.rs"] {
                    fs::write(f.dir.path().join(p), vec![0; 8 * 1024 * 1024]).unwrap();
                }
            }),
        ),
        (
            "requirements.limit",
            "malformed",
            Box::new(|f| {
                f.policy["requirements"] = json!((0..65)
                    .map(|i| json!({"id":format!("r{i}"),"cases":["one"]}))
                    .collect::<Vec<_>>())
            }),
        ),
        (
            "cases.limit",
            "malformed",
            Box::new(|f| {
                f.attempt["cases"] = json!((0..129)
                    .map(|i| json!({"id":format!("c{i}"),"outcome":"passed"}))
                    .collect::<Vec<_>>())
            }),
        ),
        (
            "files.limit",
            "malformed",
            Box::new(|f| {
                for i in 0..65 {
                    let p = format!("input{i}");
                    fs::write(f.dir.path().join(&p), b"x").unwrap();
                    f.evidence["source"]["files"][p] = json!(fingerprint(b"x"));
                }
            }),
        ),
    ]);
}
#[test]
fn compact_expanded_and_recorded_state_are_distinct() {
    let mut f = Fixture::new();
    f.save();
    let before = bytes(f.dir.path());
    let compact = assess(f.dir.path(), &f.args);
    f.args.expand = true;
    let expanded = assess(f.dir.path(), &f.args);
    assert_eq!(compact["assessment"]["status"], "checked");
    assert_eq!(
        compact["packet"]["header"][0]["input"]["canonical_identity"],
        "seed.o(b)"
    );
    assert_eq!(compact["packet"]["body"], expanded["packet"]["body"]);
    assert_eq!(compact["packet"]["footer"]["execution"], "not-run");
    assert_eq!(compact["recorded_inventory"], "not-scanned");
    assert_eq!(compact["transition_status"]["accepted"], false);
    assert_eq!(before, bytes(f.dir.path()));
}

#[test]
fn real_cli_and_remaining_byte_limits() {
    let mut f = Fixture::new();
    f.save();
    let before = bytes(f.dir.path());
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_recur"))
        .args([
            "lang",
            "evidence",
            "spéc.recur",
            "--scope",
            "build.f",
            "--contract",
            "policy.json",
            "--receipt",
            "attempt.json",
            "-d",
        ])
        .arg(f.dir.path())
        .arg("--json")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let packet: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(packet["assessment"]["status"], "checked");
    assert_eq!(before, bytes(f.dir.path()));
    // JSON and source limits are enforced on raw bytes, including whitespace.
    for path in [
        "policy.json",
        "attempt.json",
        "evidence.json",
        "result.json",
        "spéc.recur",
    ] {
        f.save();
        fs::write(f.dir.path().join("spéc.recur"), SOURCE).unwrap();
        let p = f.dir.path().join(path);
        let original = fs::read(&p).unwrap();
        let mut padded = original.clone();
        padded.resize(1048576, b' ');
        fs::write(&p, &padded).unwrap();
        let at = assess(f.dir.path(), &f.args);
        assert_ne!(
            at["assessment"]["status"], "malformed",
            "at-limit {path}: {at}"
        );
        padded.push(b' ');
        fs::write(&p, &padded).unwrap();
        assert_eq!(
            assess(f.dir.path(), &f.args)["assessment"]["status"],
            "malformed",
            "beyond {path}"
        );
    }
    f.save();
    fs::write(f.dir.path().join("spéc.recur"), SOURCE).unwrap();
    // 64 requirements referencing the same valid case are permitted.
    f.policy["requirements"] = json!((0..64)
        .map(|i| json!({"id":format!("r{i}"),"cases":["one"]}))
        .collect::<Vec<_>>());
    f.rebind();
    f.save();
    assert_eq!(
        assess(f.dir.path(), &f.args)["assessment"]["status"],
        "checked"
    );
    f.attempt["cases"] = json!((0..128)
        .map(|i| json!({"id":format!("c{i}"),"outcome":"passed"}))
        .collect::<Vec<_>>());
    f.policy["requirements"] =
        json!([{"id":"many","cases":(0..128).map(|i|format!("c{i}")).collect::<Vec<_>>()}]);
    f.result["tests"] =
        json!({"discovered":128,"executed":128,"passed":128,"failed":0,"skipped":0});
    f.rebind();
    f.save();
    assert_eq!(
        assess(f.dir.path(), &f.args)["assessment"]["status"],
        "checked"
    );
}

#[test]
fn exact_aggregate_and_file_count_limits() {
    let mut f = Fixture::new();
    f.save();
    // 10 initial unique inputs: source, policy, five roles, attempt, evidence, result.
    for i in 0..54 {
        let p = format!("extra{i}");
        fs::write(f.dir.path().join(&p), b"x").unwrap();
        f.evidence["source"]["files"][p] = json!(fingerprint(b"x"));
    }
    f.save();
    assert_eq!(
        assess(f.dir.path(), &f.args)["assessment"]["status"],
        "checked"
    );
    fs::write(f.dir.path().join("extra54"), b"x").unwrap();
    f.evidence["source"]["files"]["extra54"] = json!(fingerprint(b"x"));
    f.save();
    assert_eq!(
        assess(f.dir.path(), &f.args)["assessment"]["status"],
        "malformed"
    );
    let mut f = Fixture::new();
    f.save();
    let base = assess(f.dir.path(), &f.args);
    let paths = base["checked_inputs"].as_object().unwrap().keys();
    let rest: usize = paths
        .filter(|p| p.as_str() != "implementation.rs" && p.as_str() != "tests.rs")
        .map(|p| fs::metadata(f.dir.path().join(p)).unwrap().len() as usize)
        .sum();
    let first = vec![0; 8 * 1024 * 1024];
    let second = vec![1; 8 * 1024 * 1024 - rest];
    fs::write(f.dir.path().join("implementation.rs"), &first).unwrap();
    fs::write(f.dir.path().join("tests.rs"), &second).unwrap();
    f.evidence["source"]["files"]["implementation.rs"] = json!(fingerprint(&first));
    f.evidence["source"]["files"]["tests.rs"] = json!(fingerprint(&second));
    f.save();
    assert_eq!(
        assess(f.dir.path(), &f.args)["assessment"]["status"],
        "checked"
    );
    let mut bigger = second;
    bigger.push(1);
    fs::write(f.dir.path().join("tests.rs"), &bigger).unwrap();
    assert_eq!(
        assess(f.dir.path(), &f.args)["assessment"]["status"],
        "malformed"
    );
}

#[test]
fn canonical_link_escape() {
    let mut f = Fixture::new();
    let outside = tempdir().unwrap();
    fs::write(outside.path().join("evidence.json"), b"{}").unwrap();
    let link = f.dir.path().join("escape");
    #[cfg(unix)]
    std::os::unix::fs::symlink(outside.path(), &link).unwrap();
    #[cfg(windows)]
    {
        // Directory junctions exercise canonical escapes without administrator privileges.
        let status = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-Command",
                "New-Item -ItemType Junction -Path $env:CE_LINK -Target $env:CE_TARGET | Out-Null",
            ])
            .env("CE_LINK", &link)
            .env("CE_TARGET", outside.path())
            .status()
            .unwrap();
        assert!(status.success());
    }
    f.attempt["evidence"] = json!("evidence:escape/evidence.json");
    f.save();
    assert_eq!(
        assess(f.dir.path(), &f.args)["assessment"]["status"],
        "malformed"
    );
    assert_eq!(
        fs::read(outside.path().join("evidence.json")).unwrap(),
        b"{}"
    );
}

#[test]
fn status_shape_and_current_artifact_are_verified() {
    let mut f = Fixture::new();
    f.save();
    let baseline = assess(f.dir.path(), &f.args);
    let artifact = fs::read(f.dir.path().join("build.current.md")).unwrap();
    fs::rename(
        f.dir.path().join("build.current.md"),
        f.dir.path().join("build.complete.md"),
    )
    .unwrap();
    let status = json!({"schema":"recur-lang-checked-status-v1","state":"accepted","source":"spéc.recur","source_hash":fingerprint(SOURCE.as_bytes()),
        "scope":"build.f","contract":"policy.json","receipt":"attempt.json","attempt_id":"green-1",
        "contract_hash":fingerprint(&fs::read(f.dir.path().join("policy.json")).unwrap()),"attempt_hash":fingerprint(&fs::read(f.dir.path().join("attempt.json")).unwrap()),
        "before":"build.current.md","after":"build.complete.md","artifact_hash":fingerprint(&artifact),"checked_inputs":baseline["checked_inputs"]});
    write_json(f.dir.path(), "status.json", &status);
    f.args.status = Some("status.json".into());
    assert_eq!(
        assess(f.dir.path(), &f.args)["transition_status"]["current_accepted"],
        true
    );
    fs::write(f.dir.path().join("build.complete.md"), b"modified").unwrap();
    let changed = assess(f.dir.path(), &f.args);
    assert_eq!(changed["transition_status"]["accepted"], true);
    assert_eq!(changed["transition_status"]["current_accepted"], false);
    assert_eq!(changed["assessment"]["status"], "stale");
    let mut invalid = status;
    invalid.as_object_mut().unwrap().remove("artifact_hash");
    write_json(f.dir.path(), "status.json", &invalid);
    assert_eq!(
        assess(f.dir.path(), &f.args)["assessment"]["status"],
        "malformed"
    );
}
