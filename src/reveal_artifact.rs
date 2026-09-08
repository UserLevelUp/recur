//! Shared, inert classification of reveal artifacts. No body loading or execution.
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct TypePolicy {
    pub prefix_hints: bool,
    pub prefixes: BTreeMap<String, String>,
}

impl Default for TypePolicy {
    fn default() -> Self {
        Self {
            prefix_hints: true,
            prefixes: BTreeMap::new(),
        }
    }
}

pub fn valid_type(value: &str) -> bool {
    value
        .as_bytes()
        .first()
        .is_some_and(u8::is_ascii_alphabetic)
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
}

impl TypePolicy {
    pub fn parse(value: &toml::Value) -> Result<Self> {
        let policy: Self = value
            .clone()
            .try_into()
            .context("Invalid reveal.types configuration")?;
        for (prefix, kind) in &policy.prefixes {
            if prefix.trim() != prefix
                || prefix.is_empty()
                || prefix
                    .chars()
                    .any(|c| c.is_whitespace() || c == '/' || c == '\\')
            {
                bail!("Invalid reveal.types prefix {prefix:?}");
            }
            if !valid_type(kind) {
                bail!("Invalid reveal.types type {kind:?}");
            }
        }
        Ok(policy)
    }

    pub fn validate_separators(&self, separators: &[char]) -> Result<()> {
        for prefix in self.prefixes.keys() {
            if prefix.split(separators).any(str::is_empty) {
                bail!("Invalid reveal.types prefix {prefix:?}: empty hierarchy segment");
            }
        }
        Ok(())
    }

    pub fn classify<'a>(
        &self,
        lane: &str,
        separators: &[char],
        declarations: impl Iterator<Item = &'a str>,
    ) -> ArtifactType {
        let mut result = ArtifactType {
            r#type: None,
            source: "none",
            status: "untyped",
            diagnostics: Vec::new(),
        };
        let values: Vec<&str> = declarations.collect();
        let unique: BTreeSet<&str> = values.iter().copied().collect();
        let lane_segments: Vec<_> = lane.split(separators).collect();
        let mut hints = BTreeSet::new();
        let mut best = 0;
        if self.prefix_hints {
            for (prefix, kind) in &self.prefixes {
                let parts: Vec<_> = prefix.split(separators).collect();
                if lane_segments.starts_with(&parts) {
                    if parts.len() > best {
                        hints.clear();
                        best = parts.len();
                    }
                    if parts.len() == best {
                        hints.insert(kind.as_str());
                    }
                }
            }
        }
        if !values.is_empty() {
            result.source = "metadata";
            if unique.len() > 1 {
                result.status = "conflict";
                result
                    .diagnostics
                    .push("Conflicting artifact.type declarations".into());
            } else if !valid_type(values[0]) {
                result.status = "invalid";
                result
                    .diagnostics
                    .push("Invalid artifact.type: expected [A-Za-z][A-Za-z0-9_.-]*".into());
            } else {
                result.status = "resolved";
                result.r#type = Some(values[0].into());
            }
            if values.len() > unique.len() {
                result
                    .diagnostics
                    .push("Repeated equal artifact.type declaration".into());
            }
            if hints
                .iter()
                .any(|hint| unique.len() != 1 || !unique.contains(hint))
            {
                result
                    .diagnostics
                    .push("Configured prefix hint disagrees with artifact.type metadata".into());
            }
        } else if !hints.is_empty() {
            result.source = "prefix";
            if hints.len() == 1 {
                result.status = "resolved";
                result.r#type = hints.first().map(|s| (*s).into());
            } else {
                result.status = "conflict";
                result
                    .diagnostics
                    .push("Equally specific configured prefix hints conflict".into());
            }
        }
        result
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ArtifactType {
    pub r#type: Option<String>,
    pub source: &'static str,
    pub status: &'static str,
    pub diagnostics: Vec<String>,
}

impl ArtifactType {
    pub fn matches(&self, filter: Option<&str>) -> bool {
        filter.map_or(true, |kind| {
            self.status == "resolved" && self.r#type.as_deref() == Some(kind)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_errors_cannot_be_rescued_by_prefixes() {
        let policy = TypePolicy {
            prefixes: BTreeMap::from([("skill".into(), "skill".into())]),
            ..TypePolicy::default()
        };
        for declarations in [vec![""], vec!["skill", "agent"], vec!["$(run)"]] {
            let result = policy.classify("skill.team.current", &['.'], declarations.into_iter());
            assert_eq!(result.r#type, None);
            assert_eq!(result.source, "metadata");
            assert!(!result.matches(Some("skill")));
        }
    }

    #[test]
    fn prefix_specificity_uses_segments_and_supports_multiple_separators() {
        let policy = TypePolicy {
            prefixes: BTreeMap::from([
                ("skill".into(), "skill".into()),
                ("skill.team".into(), "custom".into()),
            ]),
            ..TypePolicy::default()
        };
        assert_eq!(
            policy
                .classify("skill_team.expert.current", &['.', '_'], std::iter::empty())
                .r#type
                .as_deref(),
            Some("custom")
        );
        assert_eq!(
            policy
                .classify("skillful.team", &['.'], std::iter::empty())
                .status,
            "untyped"
        );
    }
}
