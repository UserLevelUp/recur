//! Implementation of the trait command (central trait config management).
//!
//! This module maps to hierarchical name: main.command.trait.impl

use anyhow::{bail, Context};
use clap::Subcommand;
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value as TomlValue;

const RECUR_DIR: &str = ".recur";
const CONFIG_FILE: &str = "config.toml";

#[derive(Subcommand, Debug)]
pub enum TraitSubcommand {
    /// List configured traits and fields
    List,

    /// Get a trait key (e.g., trace_id.enabled or traits.trace_id.enabled)
    Get {
        /// Trait key path
        key: String,
    },

    /// Set a trait key (e.g., trace_id.enabled true)
    Set {
        /// Trait key path
        key: String,
        /// New value (bool/number/quoted string/raw string)
        value: String,
    },
}

pub fn execute(command: TraitSubcommand, dir: PathBuf, json: bool) -> anyhow::Result<()> {
    let root = resolve_root(dir)?;

    match command {
        TraitSubcommand::List => list_traits(&root, json),
        TraitSubcommand::Get { key } => get_trait_value(&root, &key, json),
        TraitSubcommand::Set { key, value } => set_trait_value(&root, &key, &value, json),
    }
}

fn resolve_root(dir: PathBuf) -> anyhow::Result<PathBuf> {
    if dir.is_absolute() {
        return Ok(dir);
    }
    Ok(std::env::current_dir()?.join(dir))
}

fn config_path(root: &Path) -> PathBuf {
    root.join(RECUR_DIR).join(CONFIG_FILE)
}

fn ensure_config_exists(root: &Path) -> anyhow::Result<PathBuf> {
    let path = config_path(root);
    if path.exists() {
        return Ok(path);
    }

    bail!("{} not found. Run `recur init` first.", path.display())
}

fn load_config_table(path: &Path) -> anyhow::Result<toml::value::Table> {
    let text =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let value: TomlValue =
        toml::from_str(&text).with_context(|| format!("Failed to parse {}", path.display()))?;

    match value {
        TomlValue::Table(table) => Ok(table),
        _ => bail!("Top-level TOML must be a table in {}", path.display()),
    }
}

fn save_config_table(path: &Path, table: toml::value::Table) -> anyhow::Result<()> {
    let serialized = toml::to_string_pretty(&TomlValue::Table(table))
        .context("Failed to serialize config TOML")?;
    fs::write(path, serialized).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn list_traits(root: &Path, json_output: bool) -> anyhow::Result<()> {
    let path = ensure_config_exists(root)?;
    let table = load_config_table(&path)?;
    let traits = table.get("traits").and_then(|v| v.as_table());

    if json_output {
        let payload = match traits {
            Some(t) => serde_json::to_value(t)?,
            None => json!({}),
        };
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    let Some(traits_table) = traits else {
        println!("No [traits] section found in {}", path.display());
        return Ok(());
    };

    let mut sorted: BTreeMap<&String, &TomlValue> = BTreeMap::new();
    for (name, value) in traits_table {
        sorted.insert(name, value);
    }

    if sorted.is_empty() {
        println!("No trait entries configured in [traits]");
        return Ok(());
    }

    for (name, value) in sorted {
        println!("[traits.{}]", name);
        if let Some(section) = value.as_table() {
            let mut keys: BTreeMap<&String, &TomlValue> = BTreeMap::new();
            for (k, v) in section {
                keys.insert(k, v);
            }
            for (k, v) in keys {
                println!("{} = {}", k, format_value_inline(v));
            }
        } else {
            println!("value = {}", format_value_inline(value));
        }
        println!();
    }

    Ok(())
}

fn get_trait_value(root: &Path, key: &str, json_output: bool) -> anyhow::Result<()> {
    let path = ensure_config_exists(root)?;
    let table = load_config_table(&path)?;
    let key_path = normalize_trait_key_path(key, false)?;
    let value = get_value_at_path(&table, &key_path).with_context(|| {
        format!(
            "Key '{}' not found in {}",
            key_path.join("."),
            path.display()
        )
    })?;

    if json_output {
        let payload = json!({
            "key": key_path.join("."),
            "value": serde_json::to_value(value)?,
            "config_path": path.display().to_string(),
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        println!("{} = {}", key_path.join("."), format_value_inline(value));
    }

    Ok(())
}

fn set_trait_value(
    root: &Path,
    key: &str,
    raw_value: &str,
    json_output: bool,
) -> anyhow::Result<()> {
    let path = ensure_config_exists(root)?;
    let mut table = load_config_table(&path)?;
    let key_path = normalize_trait_key_path(key, true)?;
    let value = parse_cli_value(raw_value);

    set_value_at_path(&mut table, &key_path, value.clone())?;
    save_config_table(&path, table)?;

    if json_output {
        let payload = json!({
            "updated": key_path.join("."),
            "value": serde_json::to_value(value)?,
            "config_path": path.display().to_string(),
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        println!(
            "Updated {} = {} in {}",
            key_path.join("."),
            format_value_inline(&value),
            path.display()
        );
    }

    Ok(())
}

fn normalize_trait_key_path(key: &str, for_set: bool) -> anyhow::Result<Vec<String>> {
    let mut parts: Vec<String> = key
        .split('.')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    if parts.is_empty() {
        bail!("Trait key cannot be empty");
    }

    if parts.first().map(String::as_str) != Some("traits") {
        parts.insert(0, "traits".to_string());
    }

    if parts.len() < 2 {
        bail!("Trait key must include a trait section (e.g., trace_id)");
    }

    if for_set && parts.len() < 3 {
        bail!("Trait set key must include field (e.g., trace_id.enabled)");
    }

    Ok(parts)
}

fn get_value_at_path<'a>(table: &'a toml::value::Table, path: &[String]) -> Option<&'a TomlValue> {
    let mut current = table.get(path.first()?)?;
    for segment in path.iter().skip(1) {
        current = current.as_table()?.get(segment)?;
    }
    Some(current)
}

fn set_value_at_path(
    table: &mut toml::value::Table,
    path: &[String],
    value: TomlValue,
) -> anyhow::Result<()> {
    let mut current = table;

    for segment in &path[..path.len() - 1] {
        let entry = current
            .entry(segment.clone())
            .or_insert_with(|| TomlValue::Table(toml::value::Table::new()));
        if !entry.is_table() {
            bail!(
                "Cannot set {} because '{}' is not a table",
                path.join("."),
                segment
            );
        }
        current = entry
            .as_table_mut()
            .expect("entry should be table after table check");
    }

    current.insert(path.last().expect("path must not be empty").clone(), value);
    Ok(())
}

fn parse_cli_value(raw: &str) -> TomlValue {
    let trimmed = raw.trim();

    if trimmed.eq_ignore_ascii_case("true") {
        return TomlValue::Boolean(true);
    }
    if trimmed.eq_ignore_ascii_case("false") {
        return TomlValue::Boolean(false);
    }
    if let Ok(i) = trimmed.parse::<i64>() {
        return TomlValue::Integer(i);
    }
    if let Ok(f) = trimmed.parse::<f64>() {
        return TomlValue::Float(f);
    }

    // Try parsing TOML literals (quoted strings, arrays, inline tables, etc.).
    let wrapped = format!("value = {}", trimmed);
    if let Ok(parsed) = toml::from_str::<TomlValue>(&wrapped) {
        if let Some(value) = parsed.get("value") {
            return value.clone();
        }
    }

    TomlValue::String(trimmed.to_string())
}

fn format_value_inline(value: &TomlValue) -> String {
    match value {
        TomlValue::String(s) => format!("{:?}", s),
        TomlValue::Integer(i) => i.to_string(),
        TomlValue::Float(f) => f.to_string(),
        TomlValue::Boolean(b) => b.to_string(),
        TomlValue::Datetime(dt) => dt.to_string(),
        _ => serde_json::to_string(value).unwrap_or_else(|_| "\"<complex>\"".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_trait_key_accepts_shorthand_and_full_path() {
        assert_eq!(
            normalize_trait_key_path("trace_id.enabled", true).unwrap(),
            vec!["traits", "trace_id", "enabled"]
        );
        assert_eq!(
            normalize_trait_key_path("traits.stdin.exclude_missing", true).unwrap(),
            vec!["traits", "stdin", "exclude_missing"]
        );
    }

    #[test]
    fn normalize_trait_key_rejects_set_without_field() {
        assert!(normalize_trait_key_path("trace_id", true).is_err());
        assert!(normalize_trait_key_path("traits.trace_id", true).is_err());
        assert!(normalize_trait_key_path("trace_id", false).is_ok());
    }

    #[test]
    fn parse_cli_value_infers_common_scalar_types() {
        assert_eq!(parse_cli_value("true"), TomlValue::Boolean(true));
        assert_eq!(parse_cli_value("42"), TomlValue::Integer(42));
        assert_eq!(
            parse_cli_value("\"hello\""),
            TomlValue::String("hello".to_string())
        );
        assert_eq!(parse_cli_value("raw"), TomlValue::String("raw".to_string()));
    }

    #[test]
    fn set_and_get_value_path_roundtrip() {
        let mut table = toml::value::Table::new();
        let path = vec![
            "traits".to_string(),
            "trace_id".to_string(),
            "enabled".to_string(),
        ];

        set_value_at_path(&mut table, &path, TomlValue::Boolean(true)).unwrap();
        let got = get_value_at_path(&table, &path).cloned();

        assert_eq!(got, Some(TomlValue::Boolean(true)));
    }
}
