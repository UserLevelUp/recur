#![cfg(windows)]
use serde_json::{json, Value};
use std::{
    fs,
    path::Path,
    process::Command,
    thread,
    time::{Duration, Instant},
};
const ACTOR: &str = env!("CARGO_BIN_EXE_recur-warp");
const CORE: &str = env!("CARGO_BIN_EXE_recur");
const WATCH: &str = env!("CARGO_BIN_EXE_recur-watch");

fn fixture() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    for dir in [".recur", "warps", "a", "b"] {
        fs::create_dir(root.path().join(dir)).unwrap();
    }
    for dir in ["a", "b"] {
        fs::write(root.path().join(dir).join("input.txt"), "baseline").unwrap();
    }
    fs::write(root.path().join("warps/demo.warp-map.json"), serde_json::to_vec(&json!({
        "schema":"warp-bubble-map-v1", "warp_id":"demo", "goal":"Mock dispatch",
        "required_slices":[
            {"slice_id":"a", "contract_hash":"a:v1", "evidence_gates":["test"]},
            {"slice_id":"b", "contract_hash":"b:v1", "evidence_gates":["test"]},
            {"slice_id":"final", "contract_hash":"final:v1", "depends_on":["a","b"], "evidence_gates":["integration"]}
        ]})).unwrap()).unwrap();
    fs::write(
        root.path().join(".recur/config.toml"),
        r#"
[warp.dispatch]
enabled = true
default_host = "mock"
max_parallel = 2
timeout_seconds = 20
max_attempts = 3
failure_threshold = 1
easy_success_threshold = 2
easy_seconds = 10
[warp.dispatch.hosts.mock]
enabled = true
program = "powershell.exe"
args = ["-NoProfile", "-Command", "Start-Sleep -Milliseconds 300; Write-Output 'agent done'"]
reasoning_levels = ["low", "medium", "high"]
baseline_index = 1
max_parallel = 2
[warp.dispatch.assignments."demo.a"]
workspace = "a"
context = ["a/input.txt"]
inputs = ["a/input.txt"]
tests = [{program="powershell.exe", args=["-NoProfile", "-Command", "exit 0"]}]
[warp.dispatch.assignments."demo.b"]
workspace = "b"
context = ["b/input.txt"]
inputs = ["b/input.txt"]
tests = [{program="powershell.exe", args=["-NoProfile", "-Command", "exit 0"]}]
"#,
    )
    .unwrap();
    root
}
fn call(exe: &str, root: &Path, args: &[&str]) -> (bool, Value, String) {
    let out = Command::new(exe)
        .args(args)
        .args(["-d", root.to_str().unwrap(), "--json"])
        .output()
        .unwrap();
    (
        out.status.success(),
        serde_json::from_slice(&out.stdout).unwrap_or(Value::Null),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}
fn wait(root: &Path) -> Value {
    let start = Instant::now();
    loop {
        let (ok, value, error) = call(CORE, root, &["warp", "dispatch", "demo"]);
        assert!(ok, "{error}");
        if value["attempts"]
            .as_array()
            .unwrap()
            .iter()
            .all(|a| !matches!(a["state"].as_str(), Some("claimed" | "running")))
        {
            return value;
        }
        if start.elapsed() >= Duration::from_secs(25) {
            let errors = fs::read_dir(recur::warp_dispatch::directory(root, "demo"))
                .unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().to_string_lossy().ends_with("worker.error.txt"))
                .map(|e| fs::read_to_string(e.path()).unwrap_or_default())
                .collect::<Vec<_>>();
            panic!("attempts did not finish: {value}; worker errors: {errors:?}");
        }
        thread::sleep(Duration::from_millis(100));
    }
}
#[test]
fn preview_parallel_claims_deduplication_and_no_implicit_acceptance() {
    let root = fixture();
    let (ok, plan, error) = call(ACTOR, root.path(), &["dispatch", "demo"]);
    assert!(ok, "{error}");
    assert_eq!(plan["assignments"].as_array().unwrap().len(), 2);
    assert!(!root.path().join(".recur/dispatch").exists());
    let (ok, packet, error) = call(ACTOR, root.path(), &["llm", "plan", "demo", "--slice", "a"]);
    assert!(ok, "{error}");
    assert_eq!(packet["reasoning"], "medium");
    assert!(packet["prompt"].as_str().unwrap().contains("acceptance"));
    let (ok, launched, error) = call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]);
    assert!(ok, "{error}");
    assert_eq!(launched["launched"].as_array().unwrap().len(), 2);
    let (ok, again, error) = call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]);
    assert!(ok, "{error}");
    assert!(again["launched"].as_array().unwrap().is_empty());
    let observed = wait(root.path());
    assert_eq!(observed["attempts"].as_array().unwrap().len(), 2);
    assert!(observed["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["state"] == "produced"));
    let (ok, progress, error) = call(CORE, root.path(), &["warp", "show", "demo"]);
    assert!(ok, "{error}");
    assert!(progress["completed_slices"].as_array().unwrap().is_empty());
    assert!(!progress["ready_slices"]
        .as_array()
        .unwrap()
        .contains(&json!("final")));
    // Review is a separate writer action: one accepted prerequisite is
    // insufficient, and both gates must be accepted before final dispatch.
    for id in ["a", "b"] {
        let reference = format!("test={id}/input.txt");
        let (ok, _, error) = call(
            ACTOR,
            root.path(),
            &[
                "complete",
                "demo",
                id,
                "--attempt-id",
                "reviewed-1",
                "--result-hash",
                "mock-reviewed-result",
                "--evidence",
                &reference,
                "--confirm",
            ],
        );
        assert!(ok, "{error}");
        let (ok, progress, error) = call(CORE, root.path(), &["warp", "show", "demo"]);
        assert!(ok, "{error}");
        assert_eq!(
            progress["ready_slices"]
                .as_array()
                .unwrap()
                .contains(&json!("final")),
            id == "b"
        );
    }
    fs::create_dir(root.path().join("final")).unwrap();
    fs::write(root.path().join("final/input.txt"), "integration input").unwrap();
    let config = root.path().join(".recur/config.toml");
    let mut configured = fs::read_to_string(&config).unwrap();
    configured.push_str(
        r#"
[warp.dispatch.assignments."demo.final"]
workspace = "final"
context = ["final/input.txt"]
inputs = ["final/input.txt"]
tests = [{program="powershell.exe", args=["-NoProfile", "-Command", "exit 0"]}]
"#,
    );
    fs::write(config, configured).unwrap();
    let (ok, launched, error) = call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]);
    assert!(ok, "{error}");
    assert_eq!(launched["launched"].as_array().unwrap().len(), 1);
    assert_eq!(launched["launched"][0]["slice_id"], "final");
    let observed = wait(root.path());
    assert_eq!(observed["attempts"].as_array().unwrap().len(), 3);
    assert!(observed["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["state"] == "produced"));
}
#[test]
fn overlapping_workspaces_and_unavailable_hosts_do_not_launch() {
    let root = fixture();
    let path = root.path().join(".recur/config.toml");
    let original = fs::read_to_string(&path).unwrap();
    fs::write(
        &path,
        original.replace("workspace = \"b\"", "workspace = \"a\""),
    )
    .unwrap();
    let (ok, value, error) = call(ACTOR, root.path(), &["dispatch", "demo"]);
    assert!(ok, "{error}");
    assert_eq!(value["assignments"].as_array().unwrap().len(), 1);
    fs::write(
        &path,
        original.replace(
            "program = \"powershell.exe\"",
            "program = \"missing-recur-host-123.exe\"",
        ),
    )
    .unwrap();
    let (ok, value, error) = call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]);
    assert!(ok, "{error}");
    assert!(value["launched"].as_array().unwrap().is_empty());
}

#[test]
fn context_only_drift_and_partial_verification_errors_remain_observed() {
    let root = fixture();
    let config = root.path().join(".recur/config.toml");
    let original = fs::read_to_string(&config).unwrap();
    fs::write(root.path().join("a/context.txt"), "bound scope").unwrap();
    fs::write(
        &config,
        original
            .replace(
                "context = [\"a/input.txt\"]",
                "context = [\"a/context.txt\"]",
            )
            .replace("\"exit 0\"", "\"Set-Content context.txt changed; exit 0\""),
    )
    .unwrap();
    assert!(call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]).0);
    let records = wait(root.path());
    let a = records["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["slice_id"] == "a")
        .unwrap();
    assert_eq!(a["state"], "verification_stale");
    assert!(a["test_results"][0]["success"] == true);

    let root = fixture();
    let config = root.path().join(".recur/config.toml");
    let original = fs::read_to_string(&config).unwrap();
    fs::write(config, original.replace(
        "tests = [{program=\"powershell.exe\", args=[\"-NoProfile\", \"-Command\", \"exit 0\"]}]",
        "tests = [{program=\"powershell.exe\", args=[\"-NoProfile\", \"-Command\", \"exit 0\"]}, {program=\"missing-recur-verification-123.exe\", args=[]}]"
    )).unwrap();
    assert!(call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]).0);
    let records = wait(root.path());
    for attempt in records["attempts"].as_array().unwrap() {
        assert_eq!(attempt["state"], "verification_error");
        assert_eq!(attempt["test_results"].as_array().unwrap().len(), 2);
        assert!(attempt["test_results"][0]["success"] == true);
        assert!(attempt["test_results"][0]["stdout_fingerprint"].is_string());
        assert!(attempt["test_results"][1]["execution_error"] == true);
        assert!(attempt["test_results"][1]["error"].is_string());
    }
}

#[test]
fn repeated_recovery_preserves_failure_feedback() {
    let root = fixture();
    let config = root.path().join(".recur/config.toml");
    let original = fs::read_to_string(&config).unwrap();
    fs::write(config, original.replace("exit 0", "exit 1")).unwrap();
    assert!(call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]).0);
    let records = wait(root.path());
    assert!(records["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["state"] == "test_failed"));
    for _ in 0..2 {
        assert!(
            call(
                ACTOR,
                root.path(),
                &[
                    "recover",
                    "demo",
                    "--slice",
                    "a",
                    "--attempt",
                    "1",
                    "--reason",
                    "reviewed retry",
                    "--confirm"
                ]
            )
            .0
        );
        let (ok, plan, error) = call(ACTOR, root.path(), &["llm", "plan", "demo", "--slice", "a"]);
        assert!(ok, "{error}");
        assert_eq!(plan["test_failures"], 1);
        assert_eq!(plan["reasoning"], "high");
    }
}

#[test]
fn provider_and_auth_failures_pause_until_explicit_recovery() {
    // trigger: main.command.warp.dispatch.retry.provider_blocked repeated wakeups cannot relaunch
    // trigger: main.command.warp.dispatch.retry.auth_required verified intervention and explicit recovery
    // register: main.command.warp.dispatch.retry.provider_blocked configured trace trigger alias
    // register: main.command.warp.dispatch.retry.auth_required configured trace trigger alias
    for (code, category) in [
        ("UNSUPPORTED_CLIENT", "provider_blocked"),
        ("AUTH_REQUIRED", "auth_required"),
    ] {
        let root = fixture();
        let config = root.path().join(".recur/config.toml");
        let original = fs::read_to_string(&config).unwrap();
        fs::write(
            &config,
            original.replace(
                "Start-Sleep -Milliseconds 300; Write-Output 'agent done'",
                &format!("[Console]::Error.WriteLine('{code}'); exit 3"),
            ),
        )
        .unwrap();
        assert!(call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]).0);
        let records = wait(root.path());
        assert_eq!(records["attempts"].as_array().unwrap().len(), 2);
        for a in records["attempts"].as_array().unwrap() {
            assert_eq!(a["state"], "host_failed");
            assert_eq!(a["failure"]["category"], category, "failure record: {a}");
            assert_eq!(a["failure"]["retryable"], false);
            assert!(a["failure"]["retry_at_unix"].is_null());
        }
        for _ in 0..2 {
            let (ok, dispatch, error) =
                call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]);
            assert!(ok, "{error}");
            assert!(dispatch["launched"].as_array().unwrap().is_empty());
            assert_eq!(
                dispatch["skipped"][0]["reason"],
                "provider intervention required"
            );
        }
        assert_eq!(
            call(ACTOR, root.path(), &["llm", "plan", "demo", "--slice", "a"]).1["test_failures"],
            0
        );
        // Simulate verified human/configuration intervention, then explicitly
        // reopen one slice. The other remains paused and no fallback is guessed.
        fs::write(&config, original).unwrap();
        assert!(
            call(
                ACTOR,
                root.path(),
                &[
                    "recover",
                    "demo",
                    "--slice",
                    "a",
                    "--attempt",
                    "1",
                    "--reason",
                    "access verified",
                    "--confirm"
                ]
            )
            .0
        );
        assert!(call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]).0);
        let records = wait(root.path());
        assert_eq!(records["attempts"].as_array().unwrap().len(), 3);
        assert!(records["attempts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|a| a["slice_id"] == "a" && a["attempt"] == 2 && a["state"] == "produced"));
    }
}

#[test]
fn coordinator_waits_for_retry_backoff_and_stops_at_budget() {
    // trigger: main.command.warp.dispatch.retry.transient recorded delay and finite retry budget
    // register: main.command.warp.dispatch.retry.transient configured trace trigger alias
    let root = fixture();
    let config = root.path().join(".recur/config.toml");
    let original = fs::read_to_string(&config).unwrap();
    let host = "if (!(Test-Path first-attempt.marker)) { Set-Content first-attempt.marker seen; [Console]::Error.WriteLine('SERVICE_UNAVAILABLE'); exit 3 }; exit 0";
    let policy = "\n[warp.dispatch.retry]\ndelay_seconds=2\nmax_delay_seconds=2\n[watch.dispatch]\npoll_seconds=1\n";
    fs::write(
        &config,
        original.replace(
            "Start-Sleep -Milliseconds 300; Write-Output 'agent done'",
            host,
        ) + policy,
    )
    .unwrap();
    let output = Command::new(WATCH)
        .args([
            "dispatch",
            "demo",
            "--confirm",
            "--cycles",
            "20",
            "-d",
            root.path().to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let passes: Vec<Value> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert!(passes.iter().any(|p| p["active"] == 0
        && p["state"] == "coordinating"
        && p["dispatch"]["skipped"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s["retry_after_seconds"].as_u64().unwrap_or(0) > 0)));
    let records = wait(root.path());
    assert_eq!(records["attempts"].as_array().unwrap().len(), 4);
    for slice in ["a", "b"] {
        let attempts: Vec<_> = records["attempts"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| a["slice_id"] == slice)
            .collect();
        assert_eq!(attempts[0]["failure"]["category"], "transient");
        assert_eq!(attempts[1]["state"], "produced");
        assert!(
            attempts[1]["claimed_at_unix"].as_u64().unwrap()
                >= attempts[0]["finished_at_unix"].as_u64().unwrap() + 2
        );
    }
    let exhausted = fixture();
    fs::write(
        exhausted.path().join(".recur/config.toml"),
        original
            .replace("max_attempts = 3", "max_attempts = 1")
            .replace(
                "Start-Sleep -Milliseconds 300; Write-Output 'agent done'",
                "[Console]::Error.WriteLine('SERVICE_UNAVAILABLE'); exit 3",
            )
            + policy,
    )
    .unwrap();
    assert!(call(ACTOR, exhausted.path(), &["dispatch", "demo", "--confirm"]).0);
    wait(exhausted.path());
    let (ok, dispatch, error) = call(ACTOR, exhausted.path(), &["dispatch", "demo", "--confirm"]);
    assert!(ok, "{error}");
    assert!(dispatch["launched"].as_array().unwrap().is_empty());
    assert!(dispatch["skipped"]
        .as_array()
        .unwrap()
        .iter()
        .all(|s| s["retry_after_seconds"].is_null()));
    assert_eq!(
        wait(exhausted.path())["attempts"].as_array().unwrap().len(),
        2
    );
}

#[test]
fn failures_ramp_slowly_timeouts_do_not_and_coordinator_is_finite() {
    let root = fixture();
    let path = root.path().join(".recur/config.toml");
    let original = fs::read_to_string(&path).unwrap();
    fs::write(&path, original.replace("exit 0", "exit 1")).unwrap();
    let (ok, _, error) = call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]);
    assert!(ok, "{error}");
    let observed = wait(root.path());
    assert!(observed["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["state"] == "test_failed"));
    let (ok, packet, error) = call(ACTOR, root.path(), &["llm", "plan", "demo", "--slice", "a"]);
    assert!(ok, "{error}");
    assert_eq!(packet["reasoning"], "high");
    let out = Command::new(WATCH)
        .args([
            "dispatch",
            "demo",
            "--confirm",
            "--cycles",
            "1",
            "-d",
            root.path().to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&out.stdout).unwrap()["pass"],
        1
    );
    wait(root.path());
    let slow = fixture();
    let config = slow.path().join(".recur/config.toml");
    let text = fs::read_to_string(&config)
        .unwrap()
        .replace("timeout_seconds = 20", "timeout_seconds = 1")
        .replace("-Milliseconds 300", "-Seconds 3");
    fs::write(config, text).unwrap();
    assert!(call(ACTOR, slow.path(), &["dispatch", "demo", "--confirm"]).0);
    let results = wait(slow.path());
    assert!(results["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["state"] == "host_failed"));
    for attempt in results["attempts"].as_array().unwrap() {
        let observation = &attempt["agent_result"];
        assert_eq!(observation["timed_out"], true);
        assert!(observation.get("exit_code").is_some());
        assert!(observation["stdout_fingerprint"].is_string());
        assert!(observation["stderr_fingerprint"].is_string());
        assert!(observation.get("wait_error").is_some());
    }
    assert_eq!(
        call(ACTOR, slow.path(), &["llm", "plan", "demo", "--slice", "a"]).1["reasoning"],
        "medium"
    );
}

#[test]
fn config_drift_and_verification_input_drift_cannot_produce_success() {
    let root = fixture();
    let path = root.path().join(".recur/config.toml");
    let text = fs::read_to_string(&path)
        .unwrap()
        .replace("exit 0", "Set-Content input.txt changed; exit 0");
    fs::write(&path, text).unwrap();
    assert!(call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]).0);
    assert!(wait(root.path())["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["state"] == "verification_stale"));
    let root = fixture();
    let drift_config = root.path().join(".recur/config.toml");
    let slow_text = fs::read_to_string(&drift_config)
        .unwrap()
        .replace("-Milliseconds 300", "-Seconds 3");
    fs::write(&drift_config, slow_text).unwrap();
    assert!(call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]).0);
    let config = root.path().join(".recur/config.toml");
    let text = fs::read_to_string(&config).unwrap();
    fs::write(config, format!("{text}\n# drift\n")).unwrap();
    assert!(wait(root.path())["attempts"]
        .as_array()
        .unwrap()
        .iter()
        .all(|a| a["state"] != "produced"));
}

#[test]
fn simultaneous_schedulers_cannot_duplicate_claims_and_successes_can_lower_request() {
    let root = fixture();
    let mut first = Command::new(ACTOR)
        .args([
            "dispatch",
            "demo",
            "--confirm",
            "-d",
            root.path().to_str().unwrap(),
            "--json",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let _ = call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]);
    first.wait().unwrap();
    let attempts = wait(root.path());
    assert_eq!(attempts["attempts"].as_array().unwrap().len(), 2);
    let (ok, packet, error) = call(ACTOR, root.path(), &["llm", "plan", "demo", "--slice", "a"]);
    assert!(ok, "{error}");
    assert_eq!(packet["reasoning"], "low");
}

#[test]
fn recovery_is_explicit_and_preserves_attempt_history() {
    let root = fixture();
    assert!(call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]).0);
    wait(root.path());
    let (ok, preview, error) = call(
        ACTOR,
        root.path(),
        &[
            "recover",
            "demo",
            "--slice",
            "a",
            "--attempt",
            "1",
            "--reason",
            "Review requires another attempt",
        ],
    );
    assert!(ok, "{error}");
    assert_eq!(preview["state"], "planned");
    assert_eq!(
        call(CORE, root.path(), &["warp", "dispatch", "demo"]).1["attempts"][0]["state"],
        "produced"
    );
    assert!(
        call(
            ACTOR,
            root.path(),
            &[
                "recover",
                "demo",
                "--slice",
                "a",
                "--attempt",
                "1",
                "--reason",
                "Review requires another attempt",
                "--confirm"
            ]
        )
        .0
    );
    assert!(call(ACTOR, root.path(), &["dispatch", "demo", "--confirm"]).0);
    let records = wait(root.path());
    assert_eq!(records["attempts"].as_array().unwrap().len(), 3);
    assert_eq!(records["attempts"][0]["state"], "interrupted");
    assert!(records["attempts"][0]["test_results"].is_array());
    assert!(
        !call(
            ACTOR,
            root.path(),
            &[
                "recover",
                "demo",
                "--slice",
                "a",
                "--attempt",
                "1",
                "--reason",
                "Old attempt",
                "--confirm"
            ]
        )
        .0
    );
}

#[test]
fn lang_design_packets_and_implementation_readiness_are_separate() {
    let root = fixture();
    let cfg = root.path().join(".recur/config.toml");
    let text = fs::read_to_string(&cfg).unwrap();
    fs::write(
        root.path().join("a/design.recur"),
        include_str!("../demos/main.lang/main.lang.algorithm-lab.recur"),
    )
    .unwrap();
    let text = text.replace("workspace = \"a\"","workspace = \"a\"\nlang_source = \"a/design.recur\"\nlang_scope = \"gcd.f\"\nphase = \"design\"");
    fs::write(&cfg, &text).unwrap();
    let (ok, packet, error) = call(ACTOR, root.path(), &["llm", "plan", "demo", "--slice", "a"]);
    assert!(ok, "{error}");
    assert_eq!(
        packet["lang"]["footer"]["validation"],
        "sound-within-coverage"
    );
    assert!(packet["prompt"]
        .as_str()
        .unwrap()
        .contains("defer implementation"));
    let broken = include_str!("../demos/main.lang/main.lang.skippy-watch-coordination.recur")
        .replace(
            "await [csharp_monkey.o(b), web_monkey.o(b), test_bird.o(b)]",
            "await [csharp_monkey.o(b), web_monkey.o(b)]",
        );
    fs::write(root.path().join("a/design.recur"), broken).unwrap();
    let text = text.replace("lang_scope = \"gcd.f\"\n", "");
    fs::write(&cfg, &text).unwrap();
    assert!(call(ACTOR, root.path(), &["llm", "plan", "demo", "--slice", "a"]).0);
    fs::write(
        &cfg,
        text.replace("phase = \"design\"", "phase = \"implementation\""),
    )
    .unwrap();
    assert!(!call(ACTOR, root.path(), &["llm", "plan", "demo", "--slice", "a"]).0);
}
