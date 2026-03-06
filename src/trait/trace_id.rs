//! trace-id capability trait policy for role keyword configuration.
//!
//! This trait policy centralizes trace-id role keyword settings so projects can
//! tune classification behavior through `.recur/config.toml`.

use crate::project_config;
use std::path::Path;

/// Policy for trace-id role classification heuristics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceIdPolicy {
    pub producer_keywords: Vec<String>,
    pub consumer_keywords: Vec<String>,
    pub trigger_keywords: Vec<String>,
}

impl Default for TraceIdPolicy {
    fn default() -> Self {
        Self {
            producer_keywords: vec![
                "publish".to_string(),
                "send".to_string(),
                "emit".to_string(),
                "enqueue".to_string(),
            ],
            consumer_keywords: vec![
                "subscribe".to_string(),
                "queuebind".to_string(),
                "routingkey".to_string(),
                "consumer".to_string(),
            ],
            trigger_keywords: vec![
                "register(".to_string(),
                "registerrule".to_string(),
                "register".to_string(),
                "routing pattern".to_string(),
                "*.".to_string(),
            ],
        }
    }
}

/// Resolve trace-id policy from `.recur/config.toml` (trait-aware).
pub fn resolve_trace_id_policy(root: &Path) -> anyhow::Result<TraceIdPolicy> {
    let lookup_root = if root.is_absolute() {
        root.to_path_buf()
    } else if let Ok(cwd) = std::env::current_dir() {
        cwd.join(root)
    } else {
        root.to_path_buf()
    };

    let config = project_config::load_nearest(&lookup_root)?;
    let trait_trace_id = config
        .as_ref()
        .and_then(|cfg| cfg.traits.as_ref())
        .and_then(|traits| traits.trace_id.as_ref())
        .filter(|cfg| cfg.enabled.unwrap_or(true));

    let mut policy = TraceIdPolicy::default();
    if let Some(cfg) = trait_trace_id {
        if let Some(raw) = cfg.producer_keywords.as_deref() {
            let parsed = parse_keywords(raw);
            if !parsed.is_empty() {
                policy.producer_keywords = parsed;
            }
        }
        if let Some(raw) = cfg.consumer_keywords.as_deref() {
            let parsed = parse_keywords(raw);
            if !parsed.is_empty() {
                policy.consumer_keywords = parsed;
            }
        }
        if let Some(raw) = cfg.trigger_keywords.as_deref() {
            let parsed = parse_keywords(raw);
            if !parsed.is_empty() {
                policy.trigger_keywords = parsed;
            }
        }
    }

    Ok(policy)
}

fn parse_keywords(raw: &str) -> Vec<String> {
    let mut out: Vec<String> = raw
        .split(',')
        .map(|part| part.trim().to_ascii_lowercase())
        .filter(|part| !part.is_empty())
        .collect();
    out.sort();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn resolve_trace_id_policy_defaults_without_config() {
        let temp = tempdir().unwrap();
        let policy = resolve_trace_id_policy(temp.path()).unwrap();
        assert!(policy.producer_keywords.contains(&"publish".to_string()));
        assert!(policy.consumer_keywords.contains(&"subscribe".to_string()));
        assert!(policy.trigger_keywords.contains(&"register(".to_string()));
    }

    #[test]
    fn resolve_trace_id_policy_reads_trait_settings() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".recur")).unwrap();
        fs::write(
            temp.path().join(".recur/config.toml"),
            r#"
[traits.trace_id]
enabled = true
producer_keywords = "pub,sendx"
consumer_keywords = "subx,rx"
trigger_keywords = "hook,router"
"#,
        )
        .unwrap();

        let policy = resolve_trace_id_policy(temp.path()).unwrap();
        assert_eq!(
            policy.producer_keywords,
            vec!["pub".to_string(), "sendx".to_string()]
        );
        assert_eq!(
            policy.consumer_keywords,
            vec!["rx".to_string(), "subx".to_string()]
        );
        assert_eq!(
            policy.trigger_keywords,
            vec!["hook".to_string(), "router".to_string()]
        );
    }

    #[test]
    fn resolve_trace_id_policy_uses_defaults_when_trait_disabled() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".recur")).unwrap();
        fs::write(
            temp.path().join(".recur/config.toml"),
            r#"
[traits.trace_id]
enabled = false
producer_keywords = "pubx"
"#,
        )
        .unwrap();

        let policy = resolve_trace_id_policy(temp.path()).unwrap();
        assert!(policy.producer_keywords.contains(&"publish".to_string()));
        assert!(!policy.producer_keywords.contains(&"pubx".to_string()));
    }
}
