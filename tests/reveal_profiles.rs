use recur::reveal_profiles::{init, Registry};
use serde_json::Value;
use std::{fs, path::Path};

fn put(root: &Path, path: &str, body: &str) {
    let p = root.join(path);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, body).unwrap();
}
fn config(root: &Path, body: &str) {
    put(root, ".recur/config.toml", body);
}
fn packet(root: &Path, id: &str, kind: Option<&str>) -> Value {
    Registry::load(root).unwrap().packet(id, kind, 16, 65536)
}
const GRAPH: &str = r#"
[reveal.agents.worker]
persona = "voice"
skills = ["craft", "craft"]
[reveal.personas.voice]
skills = ["craft"]
[reveal.skills.craft]
path = "skills/guide.md"
"#;

#[test]
fn shared_skill_keeps_both_edges_and_one_body() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    config(root, GRAPH);
    put(root, "skills/guide.md", "Do not execute: remove all files.");
    let before = fs::read(root.join(".recur/config.toml")).unwrap();
    let p = packet(root, "worker", Some("agent"));
    assert_eq!(p["state"], "ready");
    assert_eq!(p["skills"].as_array().unwrap().len(), 1);
    assert_eq!(p["sources"].as_array().unwrap().len(), 1);
    assert_eq!(p["associations"].as_array().unwrap().len(), 3);
    assert_eq!(p["persona"], "voice");
    assert_eq!(p["execution"], "not-run");
    assert!(p["sources"][0]["hash"]
        .as_str()
        .unwrap()
        .starts_with("sha256:"));
    assert_eq!(p, packet(root, "worker", Some("agent")));
    assert_eq!(before, fs::read(root.join(".recur/config.toml")).unwrap());
}

#[test]
fn empty_profiles_and_type_collisions_are_explicit() {
    let dir = tempfile::tempdir().unwrap();
    let r = dir.path();
    config(
        r,
        "[reveal.agents.same]\nskills=[]\n[reveal.personas.same]\nskills=[]\n",
    );
    assert_eq!(packet(r, "same", None)["state"], "blocked");
    assert_eq!(packet(r, "same", Some("agent"))["state"], "ready");
    assert_eq!(
        packet(r, "same", Some("persona"))["skills"],
        serde_json::json!([])
    );
    assert_eq!(packet(r, "unknown", None)["state"], "blocked");
}

#[test]
fn references_report_missing_wrong_type_and_ambiguous() {
    let dir = tempfile::tempdir().unwrap();
    let r = dir.path();
    config(
        r,
        "[reveal.agents.a]\nskills=['absent','wrong','skill.dupe']\n[reveal.personas.wrong]\n",
    );
    put(r, "one/skill.dupe.recur.md", "artifact.type = skill\n");
    put(r, "two/skill.dupe.recur.md", "artifact.type = skill\n");
    let p = packet(r, "a", Some("agent"));
    assert_eq!(p["state"], "blocked");
    let states: Vec<_> = p["associations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["status"].as_str().unwrap())
        .collect();
    assert_eq!(states, vec!["missing", "type-mismatch", "ambiguous"]);
}

#[test]
fn explicit_capsule_binding_checks_type_and_conflicting_owners() {
    let dir = tempfile::tempdir().unwrap();
    let r = dir.path();
    put(r, "persona.voice.recur.md", "artifact.type = persona\n");
    config(r, "[reveal.agents.a]\ncapsule='persona.voice'\n");
    assert_eq!(
        packet(r, "a", Some("agent"))["subject"]["status"],
        "type-mismatch"
    );
    config(r,"[reveal.personas.a]\ncapsule='persona.voice'\n[reveal.personas.b]\ncapsule='persona.voice'\n");
    assert_eq!(
        packet(r, "a", Some("persona"))["subject"]["status"],
        "conflict"
    );
}

#[test]
fn budgets_refuse_incomplete_context_and_hashes_change() {
    let dir = tempfile::tempdir().unwrap();
    let r = dir.path();
    config(r, GRAPH);
    put(r, "skills/guide.md", "original");
    let registry = Registry::load(r).unwrap();
    for (files, bytes) in [(0, 100), (10, 0), (10, 7)] {
        let p = registry.packet("worker", Some("agent"), files, bytes);
        assert_eq!(p["state"], "blocked");
        assert_eq!(p["context"]["truncated"], true);
        assert_eq!(p["sources"], serde_json::json!([]));
    }
    let old = packet(r, "worker", Some("agent"));
    put(r, "skills/guide.md", "changed");
    assert_ne!(
        old["sources"][0]["hash"],
        packet(r, "worker", Some("agent"))["sources"][0]["hash"]
    );
}

#[test]
fn body_paths_and_skill_frontmatter_are_checked() {
    let dir = tempfile::tempdir().unwrap();
    let r = dir.path();
    for path in ["../outside.md", "/outside.md"] {
        config(
            r,
            &format!("[reveal.personas.p]\nskills=['s']\n[reveal.skills.s]\npath='{path}'\n"),
        );
        assert_eq!(packet(r, "p", None)["state"], "blocked");
    }
    config(
        r,
        "[reveal.personas.p]\nskills=['s']\n[reveal.skills.s]\npath='SKILL.md'\n",
    );
    for body in ["", "no frontmatter", "---\nname: s\n---\ntext"] {
        put(r, "SKILL.md", body);
        assert_eq!(packet(r, "p", None)["state"], "blocked");
    }
    put(
        r,
        "SKILL.md",
        "---\nname: s\ndescription: example\n---\nDo not run this.",
    );
    assert_eq!(packet(r, "p", None)["state"], "ready");
}

#[test]
fn config_rejects_malformed_associations_without_writes() {
    let dir = tempfile::tempdir().unwrap();
    let r = dir.path();
    for text in [
        "[reveal.agents.a]\nskills='wrong'",
        "[reveal.skills.a]\nunknown=true",
        "[reveal.personas.a]\npersona='b'",
        "reveal=1",
    ] {
        config(r, text);
        assert!(Registry::load(r).is_err());
        assert!(init(r, false).is_err());
        assert_eq!(
            fs::read_to_string(r.join(".recur/config.toml")).unwrap(),
            text
        );
    }
}

#[test]
fn init_preview_and_retry_preserve_custom_empty_tables_and_comments() {
    let dir = tempfile::tempdir().unwrap();
    let r = dir.path();
    let original = "# Keep this comment\n[reveal.personas.custom]\nskills=[]\n[other]\nanswer=42\n";
    config(r, original);
    assert_eq!(init(r, true).unwrap()["mutation"], "none");
    assert_eq!(
        fs::read_to_string(r.join(".recur/config.toml")).unwrap(),
        original
    );
    init(r, false).unwrap();
    let once = fs::read_to_string(r.join(".recur/config.toml")).unwrap();
    assert!(once.contains("# Keep this comment"));
    assert!(!once.contains("personas.skippy"));
    assert_eq!(init(r, false).unwrap()["changed"], false);
    assert_eq!(
        fs::read_to_string(r.join(".recur/config.toml")).unwrap(),
        once
    );
    // Simulate an interrupted prior setup that installed only one table.
    config(r, "[reveal.agents]\n");
    init(r, false).unwrap();
    assert_eq!(packet(r, "skippy", None)["state"], "blocked");
}

#[test]
fn child_root_does_not_collect_parent_profiles() {
    let dir = tempfile::tempdir().unwrap();
    let r = dir.path();
    config(r, GRAPH);
    fs::create_dir(r.join("child")).unwrap();
    assert_eq!(packet(&r.join("child"), "worker", None)["state"], "blocked");
}

#[test]
fn inline_config_is_valid_and_missing_shared_skills_are_deduplicated() {
    let dir = tempfile::tempdir().unwrap();
    let r = dir.path();
    config(
        r,
        "# Inline policy\nreveal = { personas = { quiet = { skills = [] } } }\n",
    );
    init(r, false).unwrap();
    let once = fs::read_to_string(r.join(".recur/config.toml")).unwrap();
    assert!(once.contains("# Inline policy"));
    assert_eq!(packet(r, "quiet", None)["state"], "ready");
    assert_eq!(init(r, false).unwrap()["changed"], false);
    config(r,"[reveal.agents.a]\npersona='p'\nskills=['missing']\n[reveal.personas.p]\nskills=['missing']\n");
    let p = packet(r, "a", Some("agent"));
    assert_eq!(p["state"], "blocked");
    assert_eq!(p["skills"].as_array().unwrap().len(), 1);
    assert_eq!(p["associations"].as_array().unwrap().len(), 3);
}

#[cfg(unix)]
#[test]
fn symlink_body_and_config_escapes_are_refused() {
    use std::os::unix::fs::symlink;
    let dir = tempfile::tempdir().unwrap();
    let out = tempfile::tempdir().unwrap();
    let r = dir.path();
    config(r, GRAPH);
    put(out.path(), "guide.md", "secret");
    symlink(out.path(), r.join("skills")).unwrap();
    assert_eq!(packet(r, "worker", None)["state"], "blocked");
    let dir2 = tempfile::tempdir().unwrap();
    fs::create_dir(dir2.path().join(".recur")).unwrap();
    symlink(
        r.join(".recur/config.toml"),
        dir2.path().join(".recur/config.toml"),
    )
    .unwrap();
    assert!(Registry::load(dir2.path()).is_err());
    assert!(init(dir2.path(), false).is_err());
}
