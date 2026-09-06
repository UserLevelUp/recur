//! Shared effective Eventness policy for queries, reveal, and receipt writers.
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct WarpPolicy {
    pub active: Vec<String>,
    pub complete: Vec<String>,
    pub interesting: Vec<String>,
    pub blocked: Vec<String>,
    pub source: String,
    pub field_sources: BTreeMap<String, String>,
}

impl WarpPolicy {
    pub fn load(root: &Path) -> Result<Self> {
        let mut policy = Self {
            active: vec!["current".into()],
            complete: vec!["complete".into()],
            interesting: vec!["strange".into()],
            blocked: vec!["blocked".into()],
            source: "defaults".into(),
            field_sources: BTreeMap::new(),
        };
        // Resolve nearest configuration identically for root and nested lane queries.
        let config = root
            .ancestors()
            .map(|p| p.join(".recur/config.toml"))
            .find(|p| p.is_file());
        let value = if let Some(path) = config {
            policy.source = path.display().to_string();
            toml::from_str::<toml::Value>(&fs::read_to_string(&path)?)
                .with_context(|| format!("invalid Warp policy in '{}'", path.display()))?
        } else {
            toml::Value::Table(Default::default())
        };
        for (key, target) in [
            ("active", &mut policy.active),
            ("complete", &mut policy.complete),
            ("interesting", &mut policy.interesting),
            ("blocked", &mut policy.blocked),
        ] {
            let field = value
                .get("warp")
                .and_then(|v| v.get("suffixes"))
                .and_then(|v| v.get(key));
            let source = if let Some(field) = field {
                let array = field
                    .as_array()
                    .context("Warp suffix policy must contain arrays")?;
                let mut parsed = Vec::new();
                for item in array {
                    let s = item
                        .as_str()
                        .context("Warp suffix must be a string")?
                        .trim()
                        .to_ascii_lowercase();
                    if s.is_empty()
                        || s.split('.').any(|part| {
                            part.is_empty()
                                || !part
                                    .chars()
                                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
                        })
                    {
                        bail!("invalid Warp suffix '{}' for {}", s, key);
                    }
                    if parsed.contains(&s) {
                        bail!("duplicate Warp suffix '{}'", s);
                    }
                    parsed.push(s);
                }
                *target = parsed;
                policy.source.clone()
            } else {
                "defaults".into()
            };
            policy.field_sources.insert(key.into(), source);
        }
        let mut seen = std::collections::BTreeSet::new();
        for suffix in policy
            .active
            .iter()
            .chain(&policy.complete)
            .chain(&policy.interesting)
            .chain(&policy.blocked)
        {
            if !seen.insert(suffix) {
                bail!(
                    "Warp suffix '{}' belongs to conflicting state groups",
                    suffix
                );
            }
        }
        Ok(policy)
    }

    pub fn state(&self, filename: &str) -> Option<String> {
        let stem = filename.strip_suffix(".md")?.to_ascii_lowercase();
        self.active
            .iter()
            .chain(&self.complete)
            .chain(&self.interesting)
            .chain(&self.blocked)
            .filter(|suffix| stem.ends_with(&format!(".{}", suffix)))
            .max_by_key(|suffix| suffix.len())
            .cloned()
    }

    pub fn group(&self, state: &str) -> &'static str {
        if self.active.iter().any(|s| s == state) {
            "active"
        } else if self.complete.iter().any(|s| s == state) {
            "complete"
        } else if self.interesting.iter().any(|s| s == state) {
            "interesting"
        } else if self.blocked.iter().any(|s| s == state) {
            "blocked"
        } else {
            "other"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nearest_policy_and_longest_suffix() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".recur")).unwrap();
        fs::create_dir(dir.path().join("docs")).unwrap();
        fs::write(
            dir.path().join(".recur/config.toml"),
            "[warp.suffixes]\ncomplete=['test.accepted']\nactive=['working']\n",
        )
        .unwrap();
        let p = WarpPolicy::load(&dir.path().join("docs")).unwrap();
        assert_eq!(
            p.state("demo.test.accepted.md"),
            Some("test.accepted".into())
        );
        assert_eq!(p.group("working"), "active");
        assert_eq!(p.field_sources["blocked"], "defaults");
        assert!(p.field_sources["complete"].ends_with("config.toml"));
    }
}
