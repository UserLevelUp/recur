//! Read-only project discovery policy and directory filters.
use anyhow::{Context, Result};
use serde::Serialize;
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

pub const DEFAULT_EXCLUDE_DIRS: &[&str] = &[
    ".git",
    "target",
    "target2",
    "build",
    "dist",
    "node_modules",
    "__pycache__",
    "fixtures",
];

#[derive(Serialize)]
pub struct DiscoveryPolicy {
    pub source: String,
    pub roots: Vec<PathBuf>,
    pub exclude_dirs: Vec<String>,
    pub scan_all: bool,
}

impl DiscoveryPolicy {
    pub fn load(requested: &Path, scan_all: bool) -> Result<Self> {
        let requested = fs::canonicalize(requested)?;
        let mut policy = Self {
            source: "defaults".into(),
            roots: vec![requested.clone()],
            exclude_dirs: DEFAULT_EXCLUDE_DIRS.iter().map(|s| s.to_string()).collect(),
            scan_all,
        };
        if scan_all {
            policy.source = "scan-all".into();
            policy.exclude_dirs.clear();
            return Ok(policy);
        }
        if let Some(config) = requested
            .ancestors()
            .map(|p| p.join(".recur/config.toml"))
            .find(|p| p.is_file())
        {
            let value: toml::Value = toml::from_str(&fs::read_to_string(&config)?)
                .with_context(|| format!("invalid discovery config '{}'", config.display()))?;
            if let Some(section) = value.get("warp").and_then(|v| v.get("discovery")) {
                anyhow::ensure!(section.is_table(), "warp.discovery must be a table");
                policy.source = config.display().to_string();
                if let Some(value) = section.get("exclude_dirs") {
                    policy.exclude_dirs = parse_exclusions(value)?;
                }
                if let Some(value) = section.get("roots") {
                    let roots = strings(value, "roots")?;
                    anyhow::ensure!(!roots.is_empty(), "discovery roots must not be empty");
                    let project = config.parent().unwrap().parent().unwrap();
                    policy.roots.clear();
                    for relative in roots {
                        let path = Path::new(&relative);
                        anyhow::ensure!(
                            !relative.is_empty()
                                && path
                                    .components()
                                    .all(|c| matches!(c, Component::Normal(_) | Component::CurDir)),
                            "discovery roots must be contained relative paths"
                        );
                        let root = fs::canonicalize(project.join(path))
                            .with_context(|| format!("invalid discovery root '{relative}'"))?;
                        anyhow::ensure!(
                            root.is_dir() && root.starts_with(project),
                            "discovery root escapes project or is not a directory"
                        );
                        // -d always narrows configured scope; never silently widen it.
                        if root.starts_with(&requested) {
                            policy.roots.push(root);
                        } else if requested.starts_with(&root) {
                            policy.roots.push(requested.clone());
                        }
                    }
                    policy.roots.sort();
                    policy.roots.dedup();
                }
            }
        }
        Ok(policy)
    }

    pub fn keep(&self, entry: &walkdir::DirEntry) -> bool {
        keep_entry(entry, &self.exclude_dirs)
    }
}

pub fn keep_entry(entry: &walkdir::DirEntry, exclusions: &[String]) -> bool {
    entry.depth() == 0
        || !entry.file_type().is_dir()
        || !exclusions.iter().any(|name| {
            entry
                .file_name()
                .to_string_lossy()
                .eq_ignore_ascii_case(name)
        })
}

pub fn load_recursive_exclusions(root: &Path) -> Result<Vec<String>> {
    let root = root.canonicalize()?;
    if let Some(config) = root
        .ancestors()
        .map(|p| p.join(".recur/config.toml"))
        .find(|p| p.is_file())
    {
        let value: toml::Value = toml::from_str(&fs::read_to_string(&config)?)
            .with_context(|| format!("invalid discovery config '{}'", config.display()))?;
        if let Some(exclusions) = value
            .get("warp")
            .and_then(|v| v.get("discovery"))
            .and_then(|v| v.get("exclude_dirs"))
        {
            return parse_exclusions(exclusions);
        }
    }
    // Explicit queries historically discover fixtures below the requested root.
    Ok(DEFAULT_EXCLUDE_DIRS
        .iter()
        .filter(|&&name| name != "fixtures")
        .map(|name| name.to_string())
        .collect())
}

fn parse_exclusions(value: &toml::Value) -> Result<Vec<String>> {
    let exclusions = strings(value, "exclude_dirs")?;
    for name in &exclusions {
        anyhow::ensure!(
            !name.is_empty()
                && name != "."
                && name != ".."
                && !name.contains(['/', '\\', '*', '?', ':']),
            "exclude_dirs entries must be literal directory names"
        );
    }
    Ok(exclusions)
}

fn strings(value: &toml::Value, field: &str) -> Result<Vec<String>> {
    value
        .as_array()
        .with_context(|| format!("warp.discovery.{field} must be an array"))?
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_string)
                .with_context(|| format!("warp.discovery.{field} entries must be strings"))
        })
        .collect()
}
