use assert_cmd::Command;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn tree(root: &Path, args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_recur"))
        .current_dir(root)
        .args(["tree", "main", "--json"])
        .args(args)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    serde_json::from_slice(&output).unwrap()
}

fn paths(node: &Value) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if let Some(path) = node["path"].as_str() {
        result.push(PathBuf::from(path));
    }
    for child in node["children"].as_array().unwrap() {
        result.extend(paths(child));
    }
    result
}

fn inventory(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    walkdir::WalkDir::new(root)
        .into_iter()
        .map(Result::unwrap)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| {
            (
                entry.path().strip_prefix(root).unwrap().to_path_buf(),
                fs::read(entry.path()).unwrap(),
            )
        })
        .collect()
}

#[test]
fn initialized_root_tree_includes_each_lanes_configured_separator() {
    let project = tempfile::tempdir().unwrap();
    let root = project.path();
    fs::create_dir(root.join("src")).unwrap();
    fs::create_dir(root.join("docs")).unwrap();
    fs::write(root.join("src").join("main.rs"), "fn main() {}\n").unwrap();
    fs::write(
        root.join("src").join("main_command_watch_impl.rs"),
        "pub fn execute() {}\n",
    )
    .unwrap();
    fs::write(
        root.join("docs").join("main.command.watch.readme.md"),
        "# Watch\n",
    )
    .unwrap();

    Command::new(env!("CARGO_BIN_EXE_recur"))
        .current_dir(root)
        .arg("init")
        .assert()
        .success();

    let config: toml::Value =
        toml::from_str(&fs::read_to_string(root.join(".recur").join("config.toml")).unwrap())
            .unwrap();
    assert_eq!(config["src"]["sep"].as_str(), Some("_"));
    assert_eq!(config["docs"]["sep"].as_str(), Some("."));

    let before = inventory(root);
    let source_tree = tree(root, &["-d", "src"]);
    assert!(paths(&source_tree)
        .iter()
        .any(|path| path.file_name().unwrap() == "main_command_watch_impl.rs"));

    let root_tree = tree(root, &[]);
    assert_eq!(
        before,
        inventory(root),
        "Tree queries must not modify files"
    );
    let root_paths = paths(&root_tree);
    assert!(root_paths
        .iter()
        .any(|path| path.file_name().unwrap() == "main.command.watch.readme.md"));
    assert!(
        root_paths
            .iter()
            .any(|path| path.file_name().unwrap() == "main_command_watch_impl.rs"),
        "Bare `recur tree main` must include underscore source files using the \
         src lane installed by `recur init`; actual tree: {root_tree:#}"
    );
    let command = root_tree["children"]
        .as_array()
        .unwrap()
        .iter()
        .find(|child| child["name"] == "command")
        .expect("Source and documentation must share the main.command hierarchy");
    let watch = command["children"]
        .as_array()
        .unwrap()
        .iter()
        .find(|child| child["name"] == "watch")
        .unwrap();
    let leaves: Vec<_> = watch["children"]
        .as_array()
        .unwrap()
        .iter()
        .map(|child| child["name"].as_str().unwrap())
        .collect();
    assert!(leaves.contains(&"impl"), "{watch:#}");
    assert!(leaves.contains(&"readme"), "{watch:#}");
}

fn configured_project() -> tempfile::TempDir {
    let project = tempfile::tempdir().unwrap();
    let root = project.path();
    for dir in ["src", "docs", "src/nested", ".recur"] {
        fs::create_dir_all(root.join(dir)).unwrap();
    }
    fs::write(
        root.join(".recur").join("config.toml"),
        "[src]\ndir = 'src'\nsep = '_'\n\
         [docs]\ndir = 'docs'\nsep = '.'\n\
         [nested]\ndir = 'src/nested'\nsep = '-'\n",
    )
    .unwrap();
    for file in [
        "src/main_command_watch_impl.rs",
        "docs/main.command.watch.readme.md",
        "src/nested/main-command-watch-tests.rs",
        "main.command.watch.fallback.md",
        "src/main.command.wrong.md",
        "docs/main_command_wrong.rs",
        "src/nested/main_command_wrong.rs",
    ] {
        fs::write(root.join(file), "fixture\n").unwrap();
    }
    project
}

#[test]
fn most_specific_lane_and_unconfigured_fallback_preserve_real_paths() {
    let project = configured_project();
    let before = inventory(project.path());
    let result = paths(&tree(project.path(), &[]));
    for expected in [
        "main_command_watch_impl.rs",
        "main.command.watch.readme.md",
        "main-command-watch-tests.rs",
        "main.command.watch.fallback.md",
    ] {
        let path = result.iter().find(|p| p.file_name().unwrap() == expected);
        assert!(path.is_some(), "{expected}: {result:?}");
        assert!(project.path().join(path.unwrap()).is_file());
    }
    assert_eq!(
        result.len(),
        4,
        "Wrong lane separators must not be tried: {result:?}"
    );
    assert_eq!(before, inventory(project.path()));
}

#[test]
fn explicit_separator_overrides_all_lane_mappings() {
    let project = configured_project();
    let result = paths(&tree(project.path(), &["--sep", "."]));
    assert_eq!(result.len(), 3, "{result:?}");
    assert!(result
        .iter()
        .any(|p| p.file_name().unwrap() == "main.command.wrong.md"));
    assert!(!result
        .iter()
        .any(|p| p.file_name().unwrap() == "main_command_watch_impl.rs"));
}

#[test]
fn subtree_uses_ancestor_config_and_nested_overrides() {
    let project = configured_project();
    let result = paths(&tree(project.path(), &["-d", "src"]));
    assert_eq!(result.len(), 2, "{result:?}");
    assert!(result
        .iter()
        .any(|p| p.file_name().unwrap() == "main_command_watch_impl.rs"));
    assert!(result
        .iter()
        .any(|p| p.file_name().unwrap() == "main-command-watch-tests.rs"));
}

#[test]
fn no_config_retains_dot_fallback_and_explicit_multi_separator_support() {
    let project = tempfile::tempdir().unwrap();
    fs::write(project.path().join("main.command.docs.md"), "docs").unwrap();
    fs::write(project.path().join("main_command_source.rs"), "source").unwrap();
    let before = inventory(project.path());
    let default = paths(&tree(project.path(), &[]));
    assert_eq!(default.len(), 1);
    assert_eq!(default[0].file_name().unwrap(), "main.command.docs.md");
    let merged = tree(project.path(), &["--sep", ".", "--sep", "_"]);
    assert!(merged.to_string().contains("\"source\""));
    assert!(merged.to_string().contains("\"docs\""));
    assert_eq!(before, inventory(project.path()));
}

#[test]
fn stdin_uses_each_selected_paths_lane_and_preserves_selection() {
    let project = configured_project();
    let output = Command::new(env!("CARGO_BIN_EXE_recur"))
        .current_dir(project.path())
        .args(["tree", "main", "--stdin", "--json"])
        .write_stdin("src/main_command_watch_impl.rs\ndocs/main.command.watch.readme.md\n")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let result: Value = serde_json::from_slice(&output).unwrap();
    let selected = paths(&result);
    assert_eq!(selected.len(), 2, "{result:#}");
    assert!(selected
        .iter()
        .all(|path| project.path().join(path).is_file()));
}

#[test]
fn configured_wildcard_scope_preserves_depth_and_separator_markers() {
    let project = configured_project();
    let output = Command::new(env!("CARGO_BIN_EXE_recur"))
        .current_dir(project.path())
        .args([
            "tree",
            "main.command.**",
            "--depth",
            "2",
            "--show-sep",
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let result: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(result["name"], "main.command");
    let selected = paths(&result);
    assert_eq!(selected.len(), 3, "{result:#}");
    assert!(!selected
        .iter()
        .any(|p| p.file_name().unwrap() == "main-command-watch-tests.rs"));
    assert!(selected
        .iter()
        .all(|path| project.path().join(path).is_file()));
}

#[test]
fn malformed_lane_config_fails_without_mutating_files() {
    let project = configured_project();
    fs::write(project.path().join(".recur").join("config.toml"), "[src").unwrap();
    let before = inventory(project.path());
    Command::new(env!("CARGO_BIN_EXE_recur"))
        .current_dir(project.path())
        .args(["tree", "main"])
        .assert()
        .code(2)
        .stderr(predicates::str::contains("Failed to parse"));
    assert_eq!(before, inventory(project.path()));
}

#[test]
fn punctuation_inside_a_segment_is_not_an_extra_separator() {
    let project = configured_project();
    for file in [
        "docs/main.command.trace-id.readme.md",
        "src/main_command_trace-id_impl.rs",
    ] {
        fs::write(project.path().join(file), "trace-id fixture\n").unwrap();
    }
    let output = Command::new(env!("CARGO_BIN_EXE_recur"))
        .current_dir(project.path())
        .args(["tree", "main.command.trace-id", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let result: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(result["name"], "main.command.trace-id");
    let selected = paths(&result);
    assert_eq!(selected.len(), 2, "{result:#}");
    assert!(selected
        .iter()
        .all(|path| project.path().join(path).is_file()));
}
