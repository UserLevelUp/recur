//! Implementation of the lane scaffolding command.

use anyhow::Context;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_LANE_ROOT: &str = "lanes";
const DEFAULT_ENTRY_SUFFIX: &str = ".recur.md";

#[derive(Debug, Serialize)]
struct LaneEntry {
    name: String,
    path: String,
}

#[derive(Debug, Serialize)]
struct LaneListOutput {
    root: String,
    lane_root: String,
    lanes: Vec<LaneEntry>,
}

#[derive(Debug, Serialize)]
struct LaneCreateOutput {
    name: String,
    path: String,
    config: String,
    capsule: String,
    created: bool,
}

#[derive(Debug)]
struct LanePolicy {
    root: PathBuf,
    entry_suffix: String,
}

pub fn execute(name: Option<String>, dir: PathBuf, json: bool) -> anyhow::Result<()> {
    let root = absolute_root(dir)?;
    let policy = load_policy(&root)?;

    match name {
        Some(name) => scaffold_lane(&root, &policy, &name, json),
        None => list_lanes(&root, &policy, json),
    }
}

fn absolute_root(dir: PathBuf) -> anyhow::Result<PathBuf> {
    let root = if dir.is_absolute() {
        dir
    } else {
        std::env::current_dir()?.join(dir)
    };
    Ok(root)
}

fn load_policy(root: &Path) -> anyhow::Result<LanePolicy> {
    let config_path = root.join(".recur").join("config.toml");
    if !config_path.is_file() {
        return Ok(LanePolicy {
            root: PathBuf::from(DEFAULT_LANE_ROOT),
            entry_suffix: DEFAULT_ENTRY_SUFFIX.to_string(),
        });
    }

    let text = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read '{}'", config_path.display()))?;
    let value: toml::Value = toml::from_str(&text)
        .with_context(|| format!("failed to parse '{}'", config_path.display()))?;
    let lanes = value.get("lanes").and_then(toml::Value::as_table);

    let configured_root = lanes
        .and_then(|table| table.get("root"))
        .and_then(toml::Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(DEFAULT_LANE_ROOT);
    let entry_suffix = lanes
        .and_then(|table| table.get("entry_suffix"))
        .and_then(toml::Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(DEFAULT_ENTRY_SUFFIX);

    Ok(LanePolicy {
        root: PathBuf::from(configured_root),
        entry_suffix: entry_suffix.to_string(),
    })
}

fn normalized_name(raw: &str) -> anyhow::Result<String> {
    let name = raw.trim();
    if name.is_empty() {
        anyhow::bail!("lane name must not be blank");
    }
    if name == "." || name == ".." || name.contains(['/', '\\']) {
        anyhow::bail!("lane name must be a single directory name");
    }
    Ok(name.to_string())
}

fn lane_root(root: &Path, policy: &LanePolicy) -> PathBuf {
    root.join(&policy.root)
}

fn scaffold_lane(
    root: &Path,
    policy: &LanePolicy,
    raw_name: &str,
    json: bool,
) -> anyhow::Result<()> {
    let name = normalized_name(raw_name)?;
    let lane_path = lane_root(root, policy).join(&name);
    let lane_existed = lane_path.exists();
    fs::create_dir_all(&lane_path)
        .with_context(|| format!("failed to create '{}'", lane_path.display()))?;

    let config_path = lane_path.join(".recur").join("config.toml");
    if !config_path.exists() {
        recur::project_config::init_project(&lane_path, false)?;
    }

    let capsule_path = lane_path
        .join(".recur")
        .join(format!("{}{}", name, policy.entry_suffix));
    if !capsule_path.exists() {
        fs::write(&capsule_path, default_capsule(&name))
            .with_context(|| format!("failed to write '{}'", capsule_path.display()))?;
    }

    let output = LaneCreateOutput {
        name,
        path: lane_path.display().to_string(),
        config: config_path.display().to_string(),
        capsule: capsule_path.display().to_string(),
        created: !lane_existed,
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("Lane {}:", output.name);
        println!("  path: {}", output.path);
        println!("  config: {}", output.config);
        println!("  capsule: {}", output.capsule);
        println!("  created: {}", output.created);
    }
    Ok(())
}

fn list_lanes(root: &Path, policy: &LanePolicy, json: bool) -> anyhow::Result<()> {
    let lane_root = lane_root(root, policy);
    let mut lanes = Vec::new();
    if lane_root.is_dir() {
        for entry in fs::read_dir(&lane_root)
            .with_context(|| format!("failed to read '{}'", lane_root.display()))?
        {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                lanes.push(LaneEntry {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path().display().to_string(),
                });
            }
        }
    }
    lanes.sort_by(|left, right| left.name.cmp(&right.name));
    let output = LaneListOutput {
        root: root.display().to_string(),
        lane_root: lane_root.display().to_string(),
        lanes,
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else if output.lanes.is_empty() {
        println!("No lanes found under {}.", output.lane_root);
    } else {
        println!("Lanes under {}:", output.lane_root);
        for lane in output.lanes {
            println!("  - {} => {}", lane.name, lane.path);
        }
    }
    Ok(())
}

fn default_capsule(name: &str) -> String {
    format!(
        "# {name}.recur\n\nrecur.gift = named lane scaffolded for bounded work\npersona = declared by the lane operator\nagent = read the scope, do the bounded work, and report evidence\nagenda = define the active task before acting\ngoals.now = establish one concrete stop condition\nschedule.next = reveal -> inspect scope -> execute -> verify -> hand off\npull.first = recur reveal {name}\nverify = cargo test\nready.state = lane is scaffolded and awaiting a declared brief\n"
    )
}
