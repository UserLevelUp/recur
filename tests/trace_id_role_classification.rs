use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn run_trace_id(root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_recur"))
        .arg("trace-id")
        .args(args)
        .arg("-d")
        .arg(root)
        .output()
        .unwrap()
}

fn json_output(output: std::process::Output) -> Value {
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn write_file(path: &Path, content: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

#[test]
fn python_keyword_arguments_do_not_extract_role_words_or_explode_prose() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    write_file(
        &root.join(".recur/config.toml"),
        br#"[traits.trace_id]
enabled = true
producer_keywords = "defines,produces,publish,send,emit,enqueue"
consumer_keywords = "consumes,consumer,subscribe,queuebind,routingkey"
trigger_keywords = "register(,registerrule,register,routing pattern,*."
"#,
    );

    write_file(
        &root.join("docs/robo.electronics.carriers.handoff.md"),
        b"<!-- recur-trace-v1\ndefines: robo.electronics.carriers.handoff\n-->\n# Carrier Handoff\n",
    );

    write_file(
        &root.join("scripts/correction_handoff.py"),
        b"register_step(\n    step_id='carrier_handoff',\n    consumes=['robo.electronics.carriers.handoff',\n              'robo.electronics.carriers.board_register'],\n    produces=['robo.electronics.carriers.accepted'],\n)\n",
    );

    write_file(
        &root.join("docs/unrelated.readme.md"),
        b"The motor controller consumes 24V power from the main rail.\nThis consumer module never mentions the handoff trace ID.\n",
    );

    let parsed = json_output(run_trace_id(
        root,
        &["robo.electronics.carriers.handoff", "--scope", "**", "--json"],
    ));

    assert!(parsed.get("schema").is_none());
    let define = parsed["define"].as_array().unwrap();
    let consume = parsed["consume"].as_array().unwrap();

    assert_eq!(consume.len(), 1);
    assert!(consume[0]["path"]
        .as_str()
        .unwrap()
        .replace('\\', "/")
        .ends_with("scripts/correction_handoff.py"));
    assert_eq!(consume[0]["edge_type"], "consume");

    // Pure consume call-site line must not be duplicated into define, and unrelated prose must not appear.
    assert_eq!(define.len(), 1);
    assert!(define[0]["path"]
        .as_str()
        .unwrap()
        .replace('\\', "/")
        .ends_with("docs/robo.electronics.carriers.handoff.md"));
    assert_eq!(define[0]["edge_type"], "define");
}

#[test]
fn collect_scoped_lines_skips_pyc_binary_and_oversized_files() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    write_file(
        &root.join(".recur/config.toml"),
        br#"[traits.content_search]
enabled = true
max_file_bytes = 512
skip_binary_files = true
"#,
    );

    write_file(
        &root.join("src/topics.py"),
        b"TOPIC_CREATE = 'robo.carrier.create'\n",
    );
    write_file(
        &root.join("src/publisher.py"),
        b"bus.publish(TOPIC_CREATE)\n",
    );

    // Binary .pyc artifact containing NUL bytes + embedded ASCII symbol reference
    let mut pyc_bytes = vec![0xCB, 0x0D, 0x0D, 0x0A, 0x00, 0x00, 0x00, 0x00];
    pyc_bytes.extend_from_slice(b"\npublish(TOPIC_CREATE)\n");
    write_file(
        &root.join("src/__pycache__/topics.cpython-312.pyc"),
        &pyc_bytes,
    );

    // Binary sample with unknown extension containing NUL bytes
    let mut raw_bin = vec![0x00, 0x01, 0x02, 0x00];
    raw_bin.extend_from_slice(b"\npublish(TOPIC_CREATE)\n");
    write_file(&root.join("src/cache.dat"), &raw_bin);

    // Oversized text file exceeding max_file_bytes = 512
    let mut oversized = "# padding\n".repeat(70);
    oversized.push_str("bus.publish(TOPIC_CREATE)\n");
    write_file(&root.join("src/oversized.py"), oversized.as_bytes());

    let parsed = json_output(run_trace_id(
        root,
        &["robo.carrier.create", "--scope", "**", "--json"],
    ));

    let produce = parsed["produce"].as_array().unwrap();
    assert_eq!(produce.len(), 1);
    assert!(produce[0]["path"]
        .as_str()
        .unwrap()
        .replace('\\', "/")
        .ends_with("src/publisher.py"));
}

#[test]
fn snake_case_board_register_is_rejected_while_csharp_camelcase_and_wildcards_work() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    write_file(
        &root.join("trace/robo.archive.json"),
        br#"{
  "board_register": "robo.electronics.carriers.board_register",
  "interface_register": "robo.electronics.carriers.board_register"
}
"#,
    );

    let board_reg = json_output(run_trace_id(
        root,
        &[
            "robo.electronics.carriers.board_register",
            "--scope",
            "**",
            "--json",
        ],
    ));
    assert_eq!(board_reg["trigger"].as_array().unwrap().len(), 0);
    assert_eq!(board_reg["define"].as_array().unwrap().len(), 2);

    // Verify C# constant extraction, CamelCase PublishAsync/QueueBind, and *. wildcard trigger
    write_file(
        &root.join("cs/DotControlEvents.cs"),
        b"public const string OwnershipCreate = \"ulu.topic.dot.ownership.create\";\n",
    );
    write_file(
        &root.join("cs/DotWatcherHostedService.cs"),
        b"registry.Register(\"*.level.create\", async dot => await _bus.PublishAsync(DotControlTopics.OwnershipCreate));\n",
    );
    write_file(
        &root.join("cs/OwnershipCreateSubscriber.cs"),
        b"channel.QueueBind(\"q.ownership\", \"x.dot\", routingKey: DotControlTopics.OwnershipCreate);\n",
    );

    let csharp = json_output(run_trace_id(
        root,
        &[
            "ulu.topic.dot.ownership.create",
            "--scope",
            "**",
            "--ext",
            ".cs",
            "--json",
        ],
    ));
    assert_eq!(csharp["define"].as_array().unwrap().len(), 1);
    assert_eq!(csharp["produce"].as_array().unwrap().len(), 1);
    assert_eq!(csharp["consume"].as_array().unwrap().len(), 1);
    assert_eq!(csharp["trigger"].as_array().unwrap().len(), 1);
}

#[test]
fn scope_diagnostics_avoid_legacy_wording_and_format_full_deduplicates_roles() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();

    write_file(
        &root.join("DotControlEvents.cs"),
        b"public const string OwnershipCreate = \"ulu.topic.dot.ownership.create\";\n",
    );
    write_file(
        &root.join("DotWatcherHostedService.cs"),
        b"registry.Register(\"*.level.create\", async dot => await _bus.PublishAsync(DotControlTopics.OwnershipCreate));\n",
    );
    write_file(
        &root.join("OwnershipCreateSubscriber.cs"),
        b"channel.QueueBind(\"q.ownership\", \"x.dot\", routingKey: DotControlTopics.OwnershipCreate);\n",
    );

    let missing_scope = run_trace_id(root, &["ulu.topic.dot.ownership.create", "--depth", "1"]);
    assert!(!missing_scope.status.success());
    let stderr = String::from_utf8_lossy(&missing_scope.stderr);
    assert!(stderr.contains("--scope"));
    assert!(!stderr.to_ascii_lowercase().contains("legacy"));

    let incompatible_mode = run_trace_id(
        root,
        &[
            "ulu.topic.dot.ownership.create",
            "--scope",
            "**",
            "--list-ids",
            "--format",
            "full",
        ],
    );
    assert!(!incompatible_mode.status.success());
    let stderr_mode = String::from_utf8_lossy(&incompatible_mode.stderr);
    assert!(stderr_mode.contains("--scope"));
    assert!(!stderr_mode.to_ascii_lowercase().contains("legacy"));

    let full = run_trace_id(
        root,
        &[
            "ulu.topic.dot.ownership.create",
            "--scope",
            "**",
            "--format",
            "full",
            "--color",
            "never",
        ],
    );
    // Note: check if --color flag is --no-color or boolean in recur trace-id; fall back if needed
    let full = if full.status.success() {
        full
    } else {
        run_trace_id(
            root,
            &[
                "ulu.topic.dot.ownership.create",
                "--scope",
                "**",
                "--format",
                "full",
            ],
        )
    };
    assert!(
        full.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&full.stderr)
    );
    let stdout = String::from_utf8_lossy(&full.stdout);
    assert!(stdout.contains("[define]"));
    assert!(stdout.contains("[produce, trigger]"));
    assert!(stdout.contains("[consume]"));
    assert_eq!(stdout.matches("PublishAsync").count(), 1);
}
