//! Traversal budget capability trait for depth guardrails.
//!
//! This trait centralizes depth guardrail behavior so commands can share
//! consistent logic for hard-stop vs graceful clamp behavior.

use anyhow::bail;
use std::path::Path;

use crate::project_config;

/// How to handle requests that exceed depth budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthBudgetMode {
    /// Return an error when requested depth exceeds the max.
    HardFail,
    /// Gracefully clamp to max depth and continue.
    Clamp,
}

/// Result of depth budget enforcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DepthBudgetResult {
    pub requested: usize,
    pub effective: usize,
    pub clamped: bool,
}

impl DepthBudgetResult {
    pub fn no_clamp(depth: usize) -> Self {
        Self {
            requested: depth,
            effective: depth,
            clamped: false,
        }
    }
}

/// Resolved policy for depth budgeting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DepthBudgetPolicy {
    pub max_depth: usize,
    pub guard_mode: DepthBudgetMode,
}

/// Parse textual depth-guard mode.
pub fn parse_depth_budget_mode(value: &str) -> anyhow::Result<DepthBudgetMode> {
    match value.to_lowercase().as_str() {
        "hard-fail" | "hard_fail" | "error" | "fail" => Ok(DepthBudgetMode::HardFail),
        "clamp" => Ok(DepthBudgetMode::Clamp),
        _ => anyhow::bail!(
            "Invalid --depth-guard '{}'. Must be 'hard-fail' or 'clamp'",
            value
        ),
    }
}

/// Resolve traversal policy from CLI override + `.recur/config.toml` + code default.
pub fn resolve_depth_budget_policy(
    dir: &Path,
    cli_depth_guard: Option<&str>,
    default_max_depth: usize,
) -> anyhow::Result<DepthBudgetPolicy> {
    let config = project_config::load_nearest(dir)?;
    let trait_traversal_budget = config
        .as_ref()
        .and_then(|cfg| cfg.traits.as_ref())
        .and_then(|traits| traits.traversal_budget.as_ref())
        .filter(|cfg| cfg.enabled.unwrap_or(true));
    let trait_max_depth = trait_traversal_budget.and_then(|cfg| cfg.max_depth);
    let trait_depth_guard = trait_traversal_budget.and_then(|cfg| cfg.depth_guard.as_deref());

    let legacy_max_depth = config
        .as_ref()
        .and_then(|cfg| cfg.traversal.as_ref())
        .and_then(|t| t.max_depth)
        .filter(|depth| *depth > 0);
    let legacy_depth_guard = config
        .as_ref()
        .and_then(|cfg| cfg.traversal.as_ref())
        .and_then(|t| t.depth_guard.as_deref());
    let config_max_depth = trait_max_depth
        .or(legacy_max_depth)
        .filter(|depth| *depth > 0);
    let config_depth_guard = trait_depth_guard.or(legacy_depth_guard);

    let guard_mode = if let Some(cli_mode) = cli_depth_guard {
        parse_depth_budget_mode(cli_mode)?
    } else if let Some(config_mode) = config_depth_guard {
        parse_depth_budget_mode(config_mode)?
    } else {
        DepthBudgetMode::HardFail
    };

    Ok(DepthBudgetPolicy {
        max_depth: config_max_depth.unwrap_or(default_max_depth),
        guard_mode,
    })
}

/// Trait for commands that enforce traversal budgets.
pub trait TraversalBudgetCapable {
    /// Enforce a max depth budget with optional force override.
    ///
    /// If `force` is true, the requested depth is always allowed.
    /// If `force` is false and requested depth is above `max_depth`,
    /// behavior depends on `mode`:
    /// - `HardFail`: returns error
    /// - `Clamp`: returns clamped depth result
    fn enforce_depth_budget(
        requested_depth: usize,
        max_depth: usize,
        force: bool,
        mode: DepthBudgetMode,
    ) -> anyhow::Result<DepthBudgetResult> {
        if force || requested_depth <= max_depth {
            return Ok(DepthBudgetResult::no_clamp(requested_depth));
        }

        match mode {
            DepthBudgetMode::HardFail => {
                bail!(
                    "Maximum depth is {} (to prevent exponential explosion)",
                    max_depth
                )
            }
            DepthBudgetMode::Clamp => Ok(DepthBudgetResult {
                requested: requested_depth,
                effective: max_depth,
                clamped: true,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    struct TestCommand;
    impl TraversalBudgetCapable for TestCommand {}

    #[test]
    fn depth_budget_allows_within_limit() {
        let result =
            TestCommand::enforce_depth_budget(3, 5, false, DepthBudgetMode::HardFail).unwrap();
        assert_eq!(result.effective, 3);
        assert!(!result.clamped);
    }

    #[test]
    fn depth_budget_allows_force_override() {
        let result =
            TestCommand::enforce_depth_budget(12, 5, true, DepthBudgetMode::HardFail).unwrap();
        assert_eq!(result.effective, 12);
        assert!(!result.clamped);
    }

    #[test]
    fn depth_budget_fails_in_hard_fail_mode() {
        let result = TestCommand::enforce_depth_budget(7, 5, false, DepthBudgetMode::HardFail);
        assert!(result.is_err());
    }

    #[test]
    fn depth_budget_clamps_in_clamp_mode() {
        let result =
            TestCommand::enforce_depth_budget(9, 5, false, DepthBudgetMode::Clamp).unwrap();
        assert_eq!(result.requested, 9);
        assert_eq!(result.effective, 5);
        assert!(result.clamped);
    }

    #[test]
    fn parse_depth_budget_mode_accepts_supported_values() {
        assert!(matches!(
            parse_depth_budget_mode("hard-fail"),
            Ok(DepthBudgetMode::HardFail)
        ));
        assert!(matches!(
            parse_depth_budget_mode("hard_fail"),
            Ok(DepthBudgetMode::HardFail)
        ));
        assert!(matches!(
            parse_depth_budget_mode("error"),
            Ok(DepthBudgetMode::HardFail)
        ));
        assert!(matches!(
            parse_depth_budget_mode("fail"),
            Ok(DepthBudgetMode::HardFail)
        ));
        assert!(matches!(
            parse_depth_budget_mode("clamp"),
            Ok(DepthBudgetMode::Clamp)
        ));
    }

    #[test]
    fn parse_depth_budget_mode_rejects_invalid_value() {
        assert!(parse_depth_budget_mode("warn").is_err());
    }

    #[test]
    fn resolve_depth_budget_policy_uses_defaults_without_config() {
        let temp = tempdir().unwrap();
        let policy = resolve_depth_budget_policy(temp.path(), None, 5).unwrap();
        assert_eq!(policy.max_depth, 5);
        assert_eq!(policy.guard_mode, DepthBudgetMode::HardFail);
    }

    #[test]
    fn resolve_depth_budget_policy_prefers_config_values() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".recur")).unwrap();
        fs::write(
            temp.path().join(".recur/config.toml"),
            r#"
[traversal]
max_depth = 9
depth_guard = "clamp"
"#,
        )
        .unwrap();

        let policy = resolve_depth_budget_policy(temp.path(), None, 5).unwrap();
        assert_eq!(policy.max_depth, 9);
        assert_eq!(policy.guard_mode, DepthBudgetMode::Clamp);
    }

    #[test]
    fn resolve_depth_budget_policy_cli_overrides_config_guard() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".recur")).unwrap();
        fs::write(
            temp.path().join(".recur/config.toml"),
            r#"
[traversal]
max_depth = 9
depth_guard = "hard-fail"
"#,
        )
        .unwrap();

        let policy = resolve_depth_budget_policy(temp.path(), Some("clamp"), 5).unwrap();
        assert_eq!(policy.max_depth, 9);
        assert_eq!(policy.guard_mode, DepthBudgetMode::Clamp);
    }

    #[test]
    fn resolve_depth_budget_policy_ignores_zero_config_max_depth() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".recur")).unwrap();
        fs::write(
            temp.path().join(".recur/config.toml"),
            r#"
[traversal]
max_depth = 0
depth_guard = "clamp"
"#,
        )
        .unwrap();

        let policy = resolve_depth_budget_policy(temp.path(), None, 5).unwrap();
        assert_eq!(policy.max_depth, 5);
        assert_eq!(policy.guard_mode, DepthBudgetMode::Clamp);
    }

    #[test]
    fn resolve_depth_budget_policy_reads_trait_traversal_budget_section() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".recur")).unwrap();
        fs::write(
            temp.path().join(".recur/config.toml"),
            r#"
[traits.traversal_budget]
enabled = true
max_depth = 12
depth_guard = "clamp"
"#,
        )
        .unwrap();

        let policy = resolve_depth_budget_policy(temp.path(), None, 5).unwrap();
        assert_eq!(policy.max_depth, 12);
        assert_eq!(policy.guard_mode, DepthBudgetMode::Clamp);
    }

    #[test]
    fn resolve_depth_budget_policy_prefers_trait_settings_over_legacy_traversal() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".recur")).unwrap();
        fs::write(
            temp.path().join(".recur/config.toml"),
            r#"
[traversal]
max_depth = 4
depth_guard = "hard-fail"

[traits.traversal_budget]
enabled = true
max_depth = 10
depth_guard = "clamp"
"#,
        )
        .unwrap();

        let policy = resolve_depth_budget_policy(temp.path(), None, 5).unwrap();
        assert_eq!(policy.max_depth, 10);
        assert_eq!(policy.guard_mode, DepthBudgetMode::Clamp);
    }

    #[test]
    fn resolve_depth_budget_policy_uses_legacy_when_trait_section_disabled() {
        let temp = tempdir().unwrap();
        fs::create_dir_all(temp.path().join(".recur")).unwrap();
        fs::write(
            temp.path().join(".recur/config.toml"),
            r#"
[traversal]
max_depth = 8
depth_guard = "clamp"

[traits.traversal_budget]
enabled = false
max_depth = 20
depth_guard = "hard-fail"
"#,
        )
        .unwrap();

        let policy = resolve_depth_budget_policy(temp.path(), None, 5).unwrap();
        assert_eq!(policy.max_depth, 8);
        assert_eq!(policy.guard_mode, DepthBudgetMode::Clamp);
    }
}
