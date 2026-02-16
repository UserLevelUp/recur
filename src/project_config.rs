//! Project-local `.recur/config.toml` support.
//!
//! Phase 1 goal: make separator policy project-aware without rigid schemas.

use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use walkdir::WalkDir;

const RECUR_DIR: &str = ".recur";
const CONFIG_FILE: &str = "config.toml";
const CHECKPOINT_FILE: &str = "checkpoints.md";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneConfig {
    pub name: String,
    pub dir: PathBuf,
    pub sep: char,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CheckpointConfig {
    pub file: Option<PathBuf>,
    pub root_pattern: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StatusConfig {
    pub current_suffix: Option<String>,
    pub todo_suffix: Option<String>,
    pub complete_suffix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecurConfig {
    pub project_root: PathBuf,
    pub config_path: PathBuf,
    pub lanes: Vec<LaneConfig>,
    pub checkpoint: Option<CheckpointConfig>,
    pub status: Option<StatusConfig>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SerializableLane {
    pub name: String,
    pub dir: String,
    pub sep: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct InitResult {
    pub root: String,
    pub config_path: String,
    pub lanes: Vec<SerializableLane>,
    pub checkpoints_created: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SeparatorUpdate {
    pub name: String,
    pub dir: String,
    pub configured_sep: String,
    pub suggested_sep: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AnalyzeReport {
    pub root: String,
    pub config_found: bool,
    pub config_path: Option<String>,
    pub detected_lanes: Vec<SerializableLane>,
    pub additions: Vec<SerializableLane>,
    pub separator_updates: Vec<SeparatorUpdate>,
    pub missing_directories: Vec<String>,
}

impl LaneConfig {
    pub fn to_serializable(&self) -> SerializableLane {
        SerializableLane {
            name: self.name.clone(),
            dir: normalize_path_for_config(&self.dir),
            sep: self.sep.to_string(),
        }
    }
}

impl RecurConfig {
    fn from_toml(project_root: PathBuf, config_path: PathBuf, toml_text: &str) -> Result<Self> {
        let value: toml::Value = toml::from_str(toml_text)
            .with_context(|| format!("Failed to parse {}", config_path.display()))?;
        let table = value.as_table().with_context(|| {
            format!(
                "Top-level TOML must be a table in {}",
                config_path.display()
            )
        })?;

        let mut lanes = Vec::new();
        let mut checkpoint = None;
        let mut status = None;

        for (section, raw_value) in table {
            let Some(section_table) = raw_value.as_table() else {
                continue;
            };

            match section.as_str() {
                "checkpoint" => {
                    checkpoint = Some(parse_checkpoint_section(section_table));
                }
                "status" => {
                    status = Some(parse_status_section(section_table));
                }
                _ => {
                    if let Some(lane) = parse_lane_section(section, section_table) {
                        lanes.push(lane);
                    }
                }
            }
        }

        sort_lanes(&mut lanes);

        Ok(Self {
            project_root,
            config_path,
            lanes,
            checkpoint,
            status,
        })
    }

    pub fn separator_for_dir(&self, dir: &Path) -> Option<char> {
        let target = self.project_relative_components(dir);
        if target.is_empty() {
            return None;
        }

        let mut best_match: Option<(usize, char)> = None;

        for lane in &self.lanes {
            let lane_components = normalized_components(&lane.dir);
            if lane_components.is_empty() {
                continue;
            }

            if is_prefix(&lane_components, &target) {
                let depth = lane_components.len();
                match best_match {
                    Some((best_depth, _)) if best_depth >= depth => {}
                    _ => best_match = Some((depth, lane.sep)),
                }
            }
        }

        best_match.map(|(_, sep)| sep)
    }

    fn project_relative_components(&self, dir: &Path) -> Vec<String> {
        let relative = if dir.is_absolute() {
            dir.strip_prefix(&self.project_root)
                .unwrap_or(dir)
                .to_path_buf()
        } else {
            dir.to_path_buf()
        };
        normalized_components(&relative)
    }
}

pub fn load_nearest(start: &Path) -> Result<Option<RecurConfig>> {
    let Some(root) = find_config_root(start) else {
        return Ok(None);
    };
    load_from_root(&root)
}

pub fn load_from_root(root: &Path) -> Result<Option<RecurConfig>> {
    let config_path = root.join(RECUR_DIR).join(CONFIG_FILE);
    if !config_path.exists() {
        return Ok(None);
    }

    let text = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read {}", config_path.display()))?;
    let config = RecurConfig::from_toml(root.to_path_buf(), config_path, &text)?;
    Ok(Some(config))
}

pub fn init_project(root: &Path, force: bool) -> Result<InitResult> {
    let root = normalize_root(root);
    let lanes = discover_lanes(&root)?;

    let recur_dir = root.join(RECUR_DIR);
    fs::create_dir_all(&recur_dir)
        .with_context(|| format!("Failed to create {}", recur_dir.display()))?;

    let config_path = recur_dir.join(CONFIG_FILE);
    if config_path.exists() && !force {
        anyhow::bail!(
            "{} already exists. Use --force to overwrite.",
            config_path.display()
        );
    }

    let config_text = render_config_toml(&lanes);
    fs::write(&config_path, config_text)
        .with_context(|| format!("Failed to write {}", config_path.display()))?;

    let checkpoints_path = recur_dir.join(CHECKPOINT_FILE);
    let checkpoints_created = if checkpoints_path.exists() {
        false
    } else {
        fs::write(&checkpoints_path, "# recur checkpoints\n")
            .with_context(|| format!("Failed to write {}", checkpoints_path.display()))?;
        true
    };

    Ok(InitResult {
        root: root.display().to_string(),
        config_path: config_path.display().to_string(),
        lanes: lanes.iter().map(LaneConfig::to_serializable).collect(),
        checkpoints_created,
    })
}

pub fn analyze_project(root: &Path) -> Result<AnalyzeReport> {
    let root = normalize_root(root);
    let detected = discover_lanes(&root)?;
    let detected_serializable: Vec<SerializableLane> =
        detected.iter().map(LaneConfig::to_serializable).collect();

    let loaded = load_from_root(&root)?;
    let Some(config) = loaded else {
        return Ok(AnalyzeReport {
            root: root.display().to_string(),
            config_found: false,
            config_path: None,
            detected_lanes: detected_serializable.clone(),
            additions: detected_serializable,
            separator_updates: Vec::new(),
            missing_directories: Vec::new(),
        });
    };

    let mut configured_by_dir: BTreeMap<String, &LaneConfig> = BTreeMap::new();
    for lane in &config.lanes {
        configured_by_dir.insert(normalize_path_for_compare(&lane.dir), lane);
    }

    let mut additions = Vec::new();
    let mut separator_updates = Vec::new();
    for lane in &detected {
        let lane_key = normalize_path_for_compare(&lane.dir);
        match configured_by_dir.get(&lane_key) {
            Some(existing) if existing.sep != lane.sep => {
                separator_updates.push(SeparatorUpdate {
                    name: existing.name.clone(),
                    dir: normalize_path_for_config(&existing.dir),
                    configured_sep: existing.sep.to_string(),
                    suggested_sep: lane.sep.to_string(),
                });
            }
            None => additions.push(lane.to_serializable()),
            _ => {}
        }
    }

    let detected_dirs: HashSet<String> = detected
        .iter()
        .map(|lane| normalize_path_for_compare(&lane.dir))
        .collect();

    let mut missing_directories = Vec::new();
    for lane in &config.lanes {
        let key = normalize_path_for_compare(&lane.dir);
        if !detected_dirs.contains(&key) {
            let lane_path = root.join(&lane.dir);
            if !lane_path.exists() {
                missing_directories.push(normalize_path_for_config(&lane.dir));
            }
        }
    }

    Ok(AnalyzeReport {
        root: root.display().to_string(),
        config_found: true,
        config_path: Some(config.config_path.display().to_string()),
        detected_lanes: detected_serializable,
        additions,
        separator_updates,
        missing_directories,
    })
}

pub fn discover_lanes(root: &Path) -> Result<Vec<LaneConfig>> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    let preferred = ["src", "docs", "julia-tests", "tests", "test", "scripts"];
    for name in preferred {
        let path = root.join(name);
        if path.is_dir() {
            candidates.push(PathBuf::from(name));
        }
    }

    for entry in fs::read_dir(root).with_context(|| format!("Failed to read {}", root.display()))? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let name = entry.file_name();
        let name = name.to_string_lossy();
        if should_skip_directory_name(&name) {
            continue;
        }

        let relative = PathBuf::from(name.as_ref());
        if candidates.iter().any(|p| p == &relative) {
            continue;
        }

        if contains_hierarchical_files(&entry.path()) {
            candidates.push(relative);
        }
    }

    let mut lanes = Vec::new();
    for dir in candidates {
        let abs = root.join(&dir);
        let sep = detect_separator(&abs);
        let name = normalize_lane_name(dir.file_name().and_then(|n| n.to_str()).unwrap_or("lane"));
        lanes.push(LaneConfig { name, dir, sep });
    }

    sort_lanes(&mut lanes);
    Ok(lanes)
}

fn find_config_root(start: &Path) -> Option<PathBuf> {
    let mut current = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.to_path_buf()
    };

    loop {
        let candidate = current.join(RECUR_DIR).join(CONFIG_FILE);
        if candidate.exists() {
            return Some(current);
        }

        let Some(parent) = current.parent() else {
            break;
        };
        current = parent.to_path_buf();
    }

    None
}

fn parse_lane_section(name: &str, table: &toml::value::Table) -> Option<LaneConfig> {
    let dir = table.get("dir")?.as_str()?;
    let sep = table.get("sep")?.as_str()?;
    let sep_char = sep.chars().next()?;
    if dir.trim().is_empty() {
        return None;
    }

    Some(LaneConfig {
        name: name.to_string(),
        dir: PathBuf::from(dir),
        sep: sep_char,
    })
}

fn parse_checkpoint_section(table: &toml::value::Table) -> CheckpointConfig {
    CheckpointConfig {
        file: table
            .get("file")
            .and_then(|v| v.as_str())
            .map(PathBuf::from),
        root_pattern: table
            .get("root_pattern")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    }
}

fn parse_status_section(table: &toml::value::Table) -> StatusConfig {
    StatusConfig {
        current_suffix: table
            .get("current_suffix")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        todo_suffix: table
            .get("todo_suffix")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        complete_suffix: table
            .get("complete_suffix")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    }
}

fn contains_hierarchical_files(dir: &Path) -> bool {
    for entry in WalkDir::new(dir)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let Some(stem) = entry.path().file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if stem.contains('.') || stem.contains('_') || stem.contains('-') {
            return true;
        }
    }
    false
}

fn detect_separator(dir: &Path) -> char {
    let mut files_with_sep = BTreeMap::from([('.', 0usize), ('_', 0usize), ('-', 0usize)]);
    let mut total_hits = BTreeMap::from([('.', 0usize), ('_', 0usize), ('-', 0usize)]);

    for entry in WalkDir::new(dir)
        .max_depth(6)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let Some(stem) = entry.path().file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        for sep in ['.', '_', '-'] {
            let hits = stem.matches(sep).count();
            if hits > 0 {
                *files_with_sep.get_mut(&sep).unwrap() += 1;
                *total_hits.get_mut(&sep).unwrap() += hits;
            }
        }
    }

    let mut best = '.';
    for candidate in ['.', '_', '-'] {
        let candidate_tuple = (
            files_with_sep.get(&candidate).copied().unwrap_or(0),
            total_hits.get(&candidate).copied().unwrap_or(0),
        );
        let best_tuple = (
            files_with_sep.get(&best).copied().unwrap_or(0),
            total_hits.get(&best).copied().unwrap_or(0),
        );
        if candidate_tuple > best_tuple {
            best = candidate;
        }
    }

    best
}

fn render_config_toml(lanes: &[LaneConfig]) -> String {
    let mut content = String::new();
    content.push_str("# Generated by `recur init`\n\n");

    for lane in lanes {
        content.push_str(&format!("[{}]\n", lane.name));
        content.push_str(&format!(
            "dir = \"{}\"\n",
            normalize_path_for_config(&lane.dir)
        ));
        content.push_str(&format!("sep = \"{}\"\n\n", lane.sep));
    }

    content.push_str("[checkpoint]\n");
    content.push_str("file = \".recur/checkpoints.md\"\n");
    content.push_str("root_pattern = \"**\"\n\n");

    content.push_str("[status]\n");
    content.push_str("current_suffix = \".current.md\"\n");
    content.push_str("todo_suffix = \".todo.md\"\n");
    content.push_str("complete_suffix = \".complete.md\"\n");

    content
}

fn normalize_root(root: &Path) -> PathBuf {
    if root.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        root.to_path_buf()
    }
}

fn normalize_lane_name(raw: &str) -> String {
    let mut name = raw
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();

    while name.contains("--") {
        name = name.replace("--", "-");
    }
    name = name.trim_matches('-').to_string();

    if name.is_empty() {
        "lane".to_string()
    } else {
        name
    }
}

fn normalize_path_for_config(path: &Path) -> String {
    let mut raw = path.to_string_lossy().replace('\\', "/");
    if raw == "." {
        return "./".to_string();
    }
    if !raw.ends_with('/') {
        raw.push('/');
    }
    raw
}

fn normalize_path_for_compare(path: &Path) -> String {
    let mut raw = path.to_string_lossy().replace('\\', "/");
    while raw.ends_with('/') {
        raw.pop();
    }
    while raw.starts_with("./") {
        raw = raw[2..].to_string();
    }
    raw.to_ascii_lowercase()
}

fn normalized_components(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().to_ascii_lowercase()),
            _ => None,
        })
        .collect()
}

fn is_prefix(prefix: &[String], target: &[String]) -> bool {
    if prefix.len() > target.len() {
        return false;
    }
    prefix
        .iter()
        .zip(target.iter())
        .all(|(left, right)| left == right)
}

fn should_skip_directory_name(name: &str) -> bool {
    name.starts_with('.')
        || matches!(
            name,
            "target" | "node_modules" | "vendor" | "dist" | "build"
        )
}

fn lane_order(dir: &Path) -> usize {
    match normalize_path_for_compare(dir).as_str() {
        "src" => 0,
        "docs" => 1,
        "julia-tests" => 2,
        "tests" => 3,
        "test" => 4,
        "scripts" => 5,
        _ => 100,
    }
}

fn sort_lanes(lanes: &mut [LaneConfig]) {
    lanes.sort_by(|a, b| {
        lane_order(&a.dir)
            .cmp(&lane_order(&b.dir))
            .then_with(|| a.name.cmp(&b.name))
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn parse_flexible_config_keeps_lanes_and_optional_sections() {
        let root = PathBuf::from("C:/tmp/project");
        let config_path = root.join(".recur/config.toml");
        let text = r#"
[src]
dir = "src/"
sep = "_"
extra = "ignored"

[docs]
dir = "docs/"
sep = "."

[checkpoint]
file = ".recur/checkpoints.md"
root_pattern = "**"

[status]
current_suffix = ".current.md"

[misc]
foo = "bar"
"#;

        let config = RecurConfig::from_toml(root, config_path, text).unwrap();
        assert_eq!(config.lanes.len(), 2);
        assert_eq!(config.lanes[0].name, "src");
        assert_eq!(config.lanes[0].sep, '_');
        assert!(config.checkpoint.is_some());
        assert!(config.status.is_some());
    }

    #[test]
    fn separator_for_dir_matches_lane_and_nested_paths() {
        let config = RecurConfig {
            project_root: PathBuf::from("C:/work/recur"),
            config_path: PathBuf::from("C:/work/recur/.recur/config.toml"),
            lanes: vec![
                LaneConfig {
                    name: "src".to_string(),
                    dir: PathBuf::from("src"),
                    sep: '_',
                },
                LaneConfig {
                    name: "docs".to_string(),
                    dir: PathBuf::from("docs"),
                    sep: '.',
                },
            ],
            checkpoint: None,
            status: None,
        };

        assert_eq!(config.separator_for_dir(Path::new("src")), Some('_'));
        assert_eq!(config.separator_for_dir(Path::new("src/nested")), Some('_'));
        assert_eq!(config.separator_for_dir(Path::new("docs")), Some('.'));
        assert_eq!(config.separator_for_dir(Path::new(".")), None);
    }

    #[test]
    fn discover_lanes_detects_common_project_layout() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::create_dir_all(root.join("julia-tests")).unwrap();

        fs::write(root.join("src/main_command_find_impl.rs"), "").unwrap();
        fs::write(root.join("docs/main.command.find.readme.md"), "").unwrap();
        fs::write(root.join("julia-tests/main.command.find.test.jl"), "").unwrap();

        let lanes = discover_lanes(root).unwrap();
        let by_name: BTreeMap<String, char> = lanes
            .iter()
            .map(|lane| (lane.name.clone(), lane.sep))
            .collect();

        assert_eq!(by_name.get("src"), Some(&'_'));
        assert_eq!(by_name.get("docs"), Some(&'.'));
        assert_eq!(by_name.get("julia-tests"), Some(&'.'));
    }

    #[test]
    fn init_project_creates_config_and_checkpoints() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/main_command_tree_impl.rs"), "").unwrap();

        let result = init_project(root, false).unwrap();
        assert!(Path::new(&result.config_path).exists());
        assert!(root.join(".recur/checkpoints.md").exists());

        let config_text = fs::read_to_string(root.join(".recur/config.toml")).unwrap();
        assert!(config_text.contains("[src]"));
        assert!(config_text.contains("sep = \"_\""));
    }

    #[test]
    fn analyze_project_reports_additions_and_separator_changes() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::create_dir_all(root.join(".recur")).unwrap();

        fs::write(root.join("src/main_command_tree_impl.rs"), "").unwrap();
        fs::write(root.join("docs/main.command.tree.readme.md"), "").unwrap();

        let config = r#"
[src]
dir = "src/"
sep = "."
"#;
        fs::write(root.join(".recur/config.toml"), config).unwrap();

        let report = analyze_project(root).unwrap();
        assert!(report.config_found);
        assert_eq!(report.additions.len(), 1);
        assert_eq!(report.additions[0].name, "docs");
        assert_eq!(report.separator_updates.len(), 1);
        assert_eq!(report.separator_updates[0].name, "src");
        assert_eq!(report.separator_updates[0].suggested_sep, "_");
    }
}
