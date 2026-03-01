//! Separator policy and resolution helpers.
//!
//! Centralizes separator parsing and config-aware resolution so command wiring
//! stays consistent and future separator token upgrades can be isolated here.

use std::path::Path;

/// Trait for commands/components that need separator parsing and resolution.
///
/// Current behavior is character-based for compatibility. Multi-character
/// separator tokens can be introduced later by extending this trait surface.
pub trait SeparatorCapable {
    /// Parse only explicitly provided separator args.
    ///
    /// Each arg currently resolves to its first character.
    fn parse_explicit_separators(sep_args: &[String]) -> Vec<char> {
        sep_args
            .iter()
            .filter_map(|value| value.chars().next())
            .collect()
    }

    /// Parse CLI separator args with fallback to dot separator.
    fn parse_cli_separators(sep_args: &[String]) -> Vec<char> {
        let parsed = Self::parse_explicit_separators(sep_args);
        if parsed.is_empty() {
            vec!['.']
        } else {
            parsed
        }
    }

    /// Parse a single optional separator value.
    fn parse_optional_separator(value: Option<&str>) -> Option<char> {
        value.and_then(|raw| raw.chars().next())
    }

    /// Resolve command separators from CLI args or `.recur/config.toml` lane policy.
    fn resolve_command_separators(sep_args: &[String], dir: &Path) -> Vec<char> {
        if !sep_args.is_empty() {
            return Self::parse_cli_separators(sep_args);
        }

        let lookup_dir = if dir.is_absolute() {
            dir.to_path_buf()
        } else if let Ok(cwd) = std::env::current_dir() {
            cwd.join(dir)
        } else {
            dir.to_path_buf()
        };

        match crate::project_config::load_nearest(&lookup_dir) {
            Ok(Some(cfg)) => {
                if let Some(config_sep) = cfg.separator_for_dir(&lookup_dir) {
                    return vec![config_sep];
                }
            }
            Ok(None) => {}
            Err(err) => {
                eprintln!("Warning: could not load .recur/config.toml: {}", err);
            }
        }

        vec!['.']
    }
}

/// Default CLI separator policy implementation.
#[derive(Debug, Default, Clone, Copy)]
pub struct CliSeparatorPolicy;

impl SeparatorCapable for CliSeparatorPolicy {}

#[cfg(test)]
mod tests {
    use super::{CliSeparatorPolicy, SeparatorCapable};
    use std::fs;
    use std::path::Path;

    #[test]
    fn parse_explicit_separators_takes_first_char_per_arg() {
        let args = vec!["::".to_string(), "._".to_string(), "_".to_string()];
        assert_eq!(
            CliSeparatorPolicy::parse_explicit_separators(&args),
            vec![':', '.', '_']
        );
    }

    #[test]
    fn parse_cli_separators_defaults_to_dot_when_empty() {
        let args: Vec<String> = Vec::new();
        assert_eq!(CliSeparatorPolicy::parse_cli_separators(&args), vec!['.']);
    }

    #[test]
    fn parse_optional_separator_uses_first_char() {
        assert_eq!(
            CliSeparatorPolicy::parse_optional_separator(Some("::")),
            Some(':')
        );
        assert_eq!(CliSeparatorPolicy::parse_optional_separator(None), None);
    }

    #[test]
    fn resolve_command_separators_prefers_explicit_cli_values() {
        let args = vec!["_".to_string()];
        assert_eq!(
            CliSeparatorPolicy::resolve_command_separators(&args, Path::new(".")),
            vec!['_']
        );
    }

    #[test]
    fn resolve_command_separators_uses_lane_config_when_available() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        let src_dir = root.join("src");
        fs::create_dir_all(&src_dir).expect("create src dir");

        let recur_dir = root.join(".recur");
        fs::create_dir_all(&recur_dir).expect("create .recur dir");

        let config = r#"
[src]
dir = "src/"
sep = "_"
"#;
        fs::write(recur_dir.join("config.toml"), config).expect("write config");

        let args: Vec<String> = Vec::new();
        assert_eq!(
            CliSeparatorPolicy::resolve_command_separators(&args, &src_dir),
            vec!['_']
        );
    }

    #[test]
    fn resolve_command_separators_falls_back_to_dot_without_config() {
        let temp = tempfile::tempdir().expect("tempdir");
        let args: Vec<String> = Vec::new();
        assert_eq!(
            CliSeparatorPolicy::resolve_command_separators(&args, temp.path()),
            vec!['.']
        );
    }
}
