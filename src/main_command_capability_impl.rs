//! Capability-card query surface for root `.recur-*` cards.
//!
//! This module maps to hierarchical name: main.command.capability.impl

use anyhow::{bail, Context};
use clap::Subcommand;
use recur::project_config;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

const CARD_PREFIX: &str = ".recur-";
const REQUIRED_CAPABILITIES: &[&str] = &["warp", "watch", "git", "trace-id", "reveal"];

#[derive(Subcommand)]
pub enum CapabilitySubcommand {
    /// List root-level `.recur-*` capability cards
    List,

    /// Print one capability card
    Explain {
        /// Capability name such as warp, watch, git, trace-id, or reveal
        capability: String,
    },

    /// Check whether the standard capability cards are present
    Doctor,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct CapabilityCard {
    name: String,
    path: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct CapabilityListOutput {
    root: String,
    cards: Vec<CapabilityCard>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct CapabilityExplainOutput {
    root: String,
    name: String,
    path: String,
    text: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct CapabilityDoctorOutput {
    root: String,
    status: String,
    required: Vec<String>,
    present: Vec<String>,
    missing: Vec<String>,
}

pub fn execute(
    command: Option<CapabilitySubcommand>,
    dir: PathBuf,
    json: bool,
) -> anyhow::Result<()> {
    match command.unwrap_or(CapabilitySubcommand::List) {
        CapabilitySubcommand::List => emit_list(&resolve_capability_root(&dir)?, json),
        CapabilitySubcommand::Explain { capability } => {
            emit_explain(&resolve_capability_root(&dir)?, &capability, json)
        }
        CapabilitySubcommand::Doctor => emit_doctor(&resolve_capability_root(&dir)?, json),
    }
}

fn resolve_capability_root(dir: &Path) -> anyhow::Result<PathBuf> {
    let requested = resolve_dir(dir)?;

    if let Some(config) = project_config::load_nearest(&requested)? {
        return Ok(config.project_root);
    }

    find_card_root(&requested).unwrap_or(Ok(requested))
}

fn resolve_dir(dir: &Path) -> anyhow::Result<PathBuf> {
    if dir.is_absolute() {
        return Ok(dir.to_path_buf());
    }

    Ok(std::env::current_dir()?.join(dir))
}

fn find_card_root(start: &Path) -> Option<anyhow::Result<PathBuf>> {
    for candidate in start.ancestors() {
        match has_capability_card(candidate) {
            Ok(true) => return Some(Ok(candidate.to_path_buf())),
            Ok(false) => {}
            Err(error) => return Some(Err(error)),
        }
    }

    None
}

fn has_capability_card(dir: &Path) -> anyhow::Result<bool> {
    if !dir.is_dir() {
        return Ok(false);
    }

    for entry in fs::read_dir(dir).with_context(|| format!("failed to read '{}'", dir.display()))? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        if card_name_from_path(&entry.path()).is_some() {
            return Ok(true);
        }
    }

    Ok(false)
}

fn collect_cards(root: &Path) -> anyhow::Result<Vec<CapabilityCard>> {
    let mut cards = Vec::new();

    if !root.exists() {
        return Ok(cards);
    }

    for entry in
        fs::read_dir(root).with_context(|| format!("failed to read '{}'", root.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let path = entry.path();
        let Some(name) = card_name_from_path(&path) else {
            continue;
        };

        cards.push(CapabilityCard {
            name,
            path: relative_display_path(root, &path),
        });
    }

    cards.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(cards)
}

fn card_name_from_path(path: &Path) -> Option<String> {
    let filename = path.file_name()?.to_str()?;
    let name = filename.strip_prefix(CARD_PREFIX)?;
    if name.is_empty() || name.contains('.') {
        return None;
    }
    Some(name.to_string())
}

fn normalize_capability_name(raw: &str) -> String {
    raw.trim()
        .strip_prefix(CARD_PREFIX)
        .unwrap_or(raw.trim())
        .to_string()
}

fn emit_list(root: &Path, json: bool) -> anyhow::Result<()> {
    let output = CapabilityListOutput {
        root: root.display().to_string(),
        cards: collect_cards(root)?,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    if output.cards.is_empty() {
        println!(
            "No capability cards matching '.recur-*' found under {}",
            output.root
        );
        return Ok(());
    }

    println!("Capability cards under {}:", output.root);
    for card in &output.cards {
        println!("  - {} => {}", card.name, card.path);
    }
    println!("Use `recur capability explain <name>` to read one card.");

    Ok(())
}

fn emit_explain(root: &Path, capability: &str, json: bool) -> anyhow::Result<()> {
    let requested = normalize_capability_name(capability);
    let cards = collect_cards(root)?;
    let Some(card) = cards.into_iter().find(|card| card.name == requested) else {
        bail!("capability '{}' not found", requested);
    };

    let absolute_path = root.join(&card.path);
    let text = fs::read_to_string(&absolute_path)
        .with_context(|| format!("failed to read '{}'", absolute_path.display()))?;
    let output = CapabilityExplainOutput {
        root: root.display().to_string(),
        name: card.name,
        path: card.path,
        text,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    println!("Capability: {}", output.name);
    println!("  path: {}", output.path);
    println!();
    print!("{}", output.text);
    if !output.text.ends_with('\n') {
        println!();
    }

    Ok(())
}

fn emit_doctor(root: &Path, json: bool) -> anyhow::Result<()> {
    let cards = collect_cards(root)?;
    let present: Vec<String> = cards.iter().map(|card| card.name.clone()).collect();
    let missing: Vec<String> = REQUIRED_CAPABILITIES
        .iter()
        .filter(|required| !present.iter().any(|card| card == *required))
        .map(|required| required.to_string())
        .collect();
    let status = if missing.is_empty() { "ok" } else { "missing" }.to_string();

    let output = CapabilityDoctorOutput {
        root: root.display().to_string(),
        status,
        required: REQUIRED_CAPABILITIES
            .iter()
            .map(|name| name.to_string())
            .collect(),
        present,
        missing,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&output)?);
        return Ok(());
    }

    println!("Capability cards: {}", output.status);
    println!("  root: {}", output.root);
    println!("  required: {}", output.required.join(", "));
    println!("  present: {}", display_list(&output.present));
    println!("  missing: {}", display_list(&output.missing));

    Ok(())
}

fn display_list(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}

fn relative_display_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_name_accepts_card_filename_or_name() {
        assert_eq!(normalize_capability_name("warp"), "warp");
        assert_eq!(normalize_capability_name(".recur-warp"), "warp");
    }
}
