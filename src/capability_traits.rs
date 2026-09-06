//! Capability traits are discoverable metadata, not Rust interfaces or authorization.
use anyhow::{ensure, Result};
use serde::Serialize;
use toml::Value;

#[derive(Serialize)]
pub struct CapabilityTrait {
    pub name: &'static str,
    pub status: &'static str,
    pub description: &'static str,
    pub commands: &'static [&'static str],
    pub configuration: &'static [&'static str],
    pub effect: &'static str,
}

const EFFECT: &str =
    "descriptive-only; preference does not enable, disable, authorize, or execute commands";
pub const CAPABILITIES: &[CapabilityTrait] = &[
    CapabilityTrait {
        name: "warp",
        status: "implemented",
        description: "Slice coordination, evidence and Eventness projections.",
        commands: &["recur warp", "recur-warp"],
        configuration: &["warp.discovery", "warp.suffixes"],
        effect: EFFECT,
    },
    CapabilityTrait {
        name: "watch",
        status: "implemented",
        description: "Inspect watcher state; run an explicitly requested companion watcher.",
        commands: &["recur watch", "recur-watch"],
        configuration: &["recur-watch --help"],
        effect: EFFECT,
    },
    CapabilityTrait {
        name: "merge",
        status: "implemented",
        description: "Merge hierarchy/query results; distinct from Warp slice composition.",
        commands: &["recur merge"],
        configuration: &["recur merge --help"],
        effect: EFFECT,
    },
    CapabilityTrait {
        name: "unmerge",
        status: "proposed",
        description: "Reserved capability concept; no unmerge command is implemented.",
        commands: &[],
        configuration: &[],
        effect: EFFECT,
    },
    CapabilityTrait {
        name: "git",
        status: "implemented",
        description: "Git workflow integration through the companion executable.",
        commands: &["recur-git"],
        configuration: &["recur-git --help"],
        effect: EFFECT,
    },
];

pub fn find(name: &str) -> Option<&'static CapabilityTrait> {
    CAPABILITIES.iter().find(|c| c.name == name)
}

pub fn defaults() -> toml::value::Table {
    let mut table = toml::value::Table::new();
    table.insert("preference".into(), Value::String("unspecified".into()));
    table.insert("notes".into(), Value::String(String::new()));
    table
}

pub fn validate_field(key: &[String], value: &Value) -> Result<()> {
    ensure!(
        key.len() == 3,
        "Capability traits accept only preference and notes (not execution settings)"
    );
    match key[2].as_str() {
        "preference" => ensure!(value.as_str().is_some_and(|s| matches!(s,"unspecified"|"preferred"|"discouraged")),
            "Capability preference must be unspecified, preferred, or discouraged"),
        "notes" => ensure!(value.is_str(), "Capability notes must be a string"),
        _ => anyhow::bail!("Capability trait field '{}' is not supported; use preference or notes. Runtime settings remain at their existing command/config surfaces.", key[2]),
    }
    Ok(())
}

pub fn effective(table: &toml::value::Table) -> Result<toml::value::Table> {
    let mut traits = match table.get("traits") {
        Some(v) => v
            .as_table()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("traits must be a table"))?,
        None => toml::value::Table::new(),
    };
    for capability in CAPABILITIES {
        let mut config = defaults();
        if let Some(existing) = traits.get(capability.name) {
            let existing = existing
                .as_table()
                .ok_or_else(|| anyhow::anyhow!("traits.{} must be a table", capability.name))?;
            for (key, value) in existing {
                validate_field(
                    &["traits".into(), capability.name.into(), key.clone()],
                    value,
                )?;
                config.insert(key.clone(), value.clone());
            }
        }
        // Catalog facts cannot be overridden by project configuration.
        config.insert("status".into(), Value::String(capability.status.into()));
        config.insert("effect".into(), Value::String(EFFECT.into()));
        config.insert(
            "commands".into(),
            Value::Array(
                capability
                    .commands
                    .iter()
                    .map(|s| Value::String((*s).into()))
                    .collect(),
            ),
        );
        traits.insert(capability.name.into(), Value::Table(config));
    }
    Ok(traits)
}

pub fn init_sections() -> String {
    CAPABILITIES
        .iter()
        .map(|c| {
            format!(
                "\n[traits.{}]\npreference = \"unspecified\"\nnotes = \"\"\n",
                c.name
            )
        })
        .collect()
}
