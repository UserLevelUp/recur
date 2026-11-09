use serde_json::Value;
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

const TRACE: &str = env!("CARGO_BIN_EXE_recur-trace-id");
const CORE: &str = env!("CARGO_BIN_EXE_recur");
const GIT: &str = env!("CARGO_BIN_EXE_recur-git");

fn run(binary: &str, root: &Path, args: &[&str]) -> Output {
    Command::new(binary)
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}

fn json(output: Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn trace_init_preserves_custom_inline_defaults_and_parent_config() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path();
    assert_eq!(
        json(run(TRACE, root, &["init", "--dry-run", "--json"]))["state"],
        "planned"
    );
    assert!(!root.join(".recur").exists());

    let vault = root.join(".recur");
    fs::create_dir(&vault).unwrap();
    let config = vault.join("config.toml");
    let original = "# local preferences\nrecur-trace-id = { extensions = '.md', inventory = { max_results = 1 } }\n\n[traits.trace_id]\nenabled = false\n";
    fs::write(&config, original).unwrap();
    let child = root.join("child");
    fs::create_dir(&child).unwrap();
    let preview = json(run(TRACE, &child, &["init", "--dry-run", "--json"]));
    assert_eq!(fs::read_to_string(&config).unwrap(), original);
    let result = json(run(TRACE, &child, &["init", "--json"]));
    assert_eq!(result["state"], "written");
    assert_eq!(result["preview"], preview["preview"]);
    assert!(!child.join(".recur").exists());
    assert_eq!(fs::read_dir(&vault).unwrap().count(), 1);
    let written = fs::read_to_string(&config).unwrap();
    assert!(written.contains("# local preferences"));
    let config_doc: toml::Value = toml::from_str(&written).unwrap();
    assert_eq!(
        config_doc["traits"]["trace_id"]["enabled"].as_bool(),
        Some(false)
    );
    let policy = &config_doc["recur-trace-id"];
    assert_eq!(policy["extensions"].as_str(), Some(".md"));
    assert_eq!(policy["inventory"]["max_results"].as_integer(), Some(1));
    assert_eq!(policy["context"]["lines"].as_integer(), Some(2));
    assert_eq!(
        json(run(TRACE, &child, &["init", "--json"]))["state"],
        "unchanged"
    );
    assert_eq!(fs::read_to_string(&config).unwrap(), written);
}

#[test]
fn trace_companion_applies_limits_and_keeps_core_queries_independent() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path();
    fs::create_dir(root.join(".recur")).unwrap();
    let config = root.join(".recur/config.toml");
    let original = "[recur-trace-id]\nextensions = '.md'\n[recur-trace-id.inventory]\nmax_results = 1\n[recur-trace-id.context]\nlines = 1\n";
    fs::write(&config, original).unwrap();
    fs::write(
        root.join("area.docs.md"),
        "Before\nexample.area.first\nAfter\n\nexample.area.second\n",
    )
    .unwrap();
    fs::write(root.join("area.noise.txt"), "example.area.third\n").unwrap();

    let inventory = json(run(
        TRACE,
        root,
        &["inventory", "example.area.**", "--json"],
    ));
    assert_eq!(inventory["ids"].as_array().unwrap().len(), 1);
    assert_eq!(inventory["coverage"]["complete"], true);
    assert_eq!(inventory["output"]["complete"], false);
    let inventory = json(run(
        TRACE,
        root,
        &[
            "inventory",
            "example.area.**",
            "--max-results",
            "3",
            "--json",
        ],
    ));
    assert_eq!(inventory["ids"].as_array().unwrap().len(), 2);
    assert_eq!(inventory["complete"], true);
    let context = json(run(
        TRACE,
        root,
        &[
            "context",
            "example.area.**",
            "--select",
            "example.area.first",
            "--json",
        ],
    ));
    assert_eq!(
        context["excerpts"][0]["lines"],
        serde_json::json!(["Before", "example.area.first", "After"])
    );
    let core = json(run(
        CORE,
        root,
        &[
            "trace-id",
            "example.area.**",
            "--scope",
            "**",
            "--list-ids",
            "--json",
        ],
    ));
    assert_eq!(core["ids"].as_array().unwrap().len(), 3);
    assert_eq!(fs::read_to_string(&config).unwrap(), original);
    assert_eq!(
        run(TRACE, root, &["inventory", "example.missing"])
            .status
            .code(),
        Some(1)
    );
}

#[test]
fn git_init_provisions_only_config_and_checkpoint_uses_it_on_append() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path();
    let preview = json(run(GIT, root, &["init", "--dry-run", "--json"]));
    assert!(!root.join(".recur").exists());
    let result = json(run(GIT, root, &["init", "--json"]));
    assert_eq!(result["preview"], preview["preview"]);
    let vault = root.join(".recur");
    assert_eq!(fs::read_dir(&vault).unwrap().count(), 1);
    let log = vault.join("checkpoints.md");
    assert!(!log.exists());
    assert_eq!(
        json(run(GIT, root, &["init", "--json"]))["state"],
        "unchanged"
    );
    let append = run(
        GIT,
        root,
        &[
            "checkpoint",
            "--append-parallel",
            "--checkpoint-id",
            "ck-configured",
        ],
    );
    assert!(
        append.status.success(),
        "{}",
        String::from_utf8_lossy(&append.stderr)
    );
    assert!(fs::read_to_string(log)
        .unwrap()
        .contains("### ck-configured"));
}

#[test]
fn invalid_values_leave_config_bytes_intact() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path();
    fs::create_dir(root.join(".recur")).unwrap();
    let config = root.join(".recur/config.toml");
    for (binary, text) in [
        (TRACE, "[recur-trace-id.context]\nlines = 21\n"),
        (GIT, "[checkpoint]\nfile = false\n"),
    ] {
        fs::write(&config, text).unwrap();
        assert_eq!(run(binary, root, &["init"]).status.code(), Some(2));
        assert_eq!(fs::read_to_string(&config).unwrap(), text);
    }
}
