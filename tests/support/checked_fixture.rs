use recur::warp_evidence::fingerprint;
use serde_json::{json, Value};
use std::{fs, path::Path};

pub const SOURCE: &str = r#"recur 0.1 class Transition
header {
  scope build {
    i(a) := (request: Text)
    o(b) := (artifact: Text)
    f : i(a) -> o(b) ~ "Build with explicit evidence" by forbidden.runner
  }
}
body {
  build sync : i(a) -> f(a) -> o(b)
}
footer {
  event build {
    state build.complete
  }
  warp build : E0(build.current) -> dE(build.f) -> Ef(build.complete)
}
"#;
pub fn put(root: &Path, name: &str, v: &Value) {
    fs::write(root.join(name), serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
pub fn fixture(root: &Path) {
    fs::write(root.join("spec.recur"), SOURCE).unwrap();
    for p in [
        "implementation.rs",
        "test.rs",
        "config.toml",
        "runner.rs",
        "behavior.md",
    ] {
        fs::write(root.join(p), p).unwrap();
    }
    fs::write(root.join("build.current.md"), "Exact E0 bytes\r\nJosé\n").unwrap();
    let policy = json!({"schema":"recur-lang-checked-contract-v1","contract_id":"transition.v1","source":"spec.recur","source_hash":fingerprint(SOURCE.as_bytes()),"scope":"build.f",
        "aliases":{"build.i(a)":"build.i(a)","build.o(b)":"build.o(b)"},"transition":{"current":"build.current","slice":"build.f","desired":"build.complete"},
        "inputs":{"specification":["spec.recur"],"implementation":["implementation.rs"],"tests":["test.rs"],"configuration":["config.toml"],"runner":["runner.rs"],"behavior":["behavior.md"]},"requirements":[{"id":"behavior","cases":["case.1"]}]});
    put(root, "policy.json", &policy);
    put(
        root,
        "result.json",
        &json!({"schema":"warp-external-result-v1","kind":"test","outcome":"passed","exit_code":0,"tests":{"discovered":1,"executed":1,"passed":1,"failed":0,"skipped":0}}),
    );
    let files: serde_json::Map<String, Value> = [
        "spec.recur",
        "implementation.rs",
        "test.rs",
        "config.toml",
        "runner.rs",
        "behavior.md",
        "policy.json",
    ]
    .into_iter()
    .map(|p| {
        (
            p.into(),
            json!(fingerprint(&fs::read(root.join(p)).unwrap())),
        )
    })
    .collect();
    put(
        root,
        "evidence.json",
        &json!({"schema":"warp-external-evidence-v1","kind":"test","producer":"fixture runner","project":"test","configuration":"native","platform":"local","executed_at_unix":1,"result_artifact":"result.json","result_fingerprint":fingerprint(&fs::read(root.join("result.json")).unwrap()),"source":{"dirty":true,"revision":null,"files":files}}),
    );
    put(
        root,
        "attempt.json",
        &json!({"schema":"recur-lang-checked-receipt-v1","attempt_id":"attempt-1","contract_hash":fingerprint(&fs::read(root.join("policy.json")).unwrap()),"source_hash":fingerprint(SOURCE.as_bytes()),"scope":"build.f","phase":"green","producer":"fixture runner","runtime":"native fixture","evidence":"evidence:evidence.json","cases":[{"id":"case.1","outcome":"passed"}]}),
    );
}
