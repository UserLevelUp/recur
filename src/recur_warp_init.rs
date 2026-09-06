//! Add missing editable defaults, preserving user configuration and templates.
use crate::recur_warp_create::{contained, relative, starter_template};
use anyhow::{ensure, Context};
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};

pub fn init(root: &Path, dry_run: bool) -> anyhow::Result<Value> {
    let root = fs::canonicalize(root)?;
    ensure!(root.is_dir(), "-d must be a directory");
    let config = root
        .ancestors()
        .map(|p| p.join(".recur/config.toml"))
        .find(|p| p.exists())
        .unwrap_or_else(|| root.join(".recur/config.toml"));
    let project = config
        .parent()
        .and_then(Path::parent)
        .context("configuration has no project root")?;
    contained(&config, project)?;
    let original = if config.exists() {
        fs::read_to_string(&config)?
    } else {
        String::new()
    };
    let mut document: toml_edit::DocumentMut =
        original.parse().context("invalid project configuration")?;
    let settings: toml::Value = toml::from_str(&original)?;
    recur::warp_policy::WarpRemovalPolicy::from_config(&settings)?;
    let creation = settings.get("warp").and_then(|w| w.get("creation"));
    let mut defaults_added = false;
    ensure!(
        creation.map_or(true, |v| v.is_table()),
        "warp.creation must be a table"
    );
    if document.get("warp").is_none() {
        document["warp"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    for key in ["creation", "removal"] {
        if document["warp"].get(key).is_none() {
            document["warp"][key] = if document["warp"].is_inline_table() {
                toml_edit::value(toml_edit::InlineTable::new())
            } else {
                toml_edit::Item::Table(toml_edit::Table::new())
            };
        }
    }
    for (key, default) in [
        ("directory", "warps"),
        ("template", ".recur/warp-template.json"),
    ] {
        if let Some(value) = creation.and_then(|v| v.get(key)) {
            relative(
                project,
                value
                    .as_str()
                    .with_context(|| format!("warp.creation.{key} must be a string"))?,
            )?;
        } else {
            document["warp"]["creation"][key] = toml_edit::value(default);
            defaults_added = true;
        }
    }
    for (key, default) in [
        ("require_confirmation", true),
        ("require_committed_snapshot", true),
        ("require_preservation_ref", true),
        ("require_pushed_ref", false),
    ] {
        if settings
            .get("warp")
            .and_then(|w| w.get("removal"))
            .and_then(|r| r.get(key))
            .is_none()
        {
            document["warp"]["removal"][key] = toml_edit::value(default);
            defaults_added = true;
        }
    }
    let template = relative(
        project,
        document["warp"]["creation"]["template"]
            .as_str()
            .context("template must be a string")?,
    )?;
    let directory = relative(
        project,
        document["warp"]["creation"]["directory"].as_str().unwrap(),
    )?;
    contained(&template, project)?;
    contained(&directory, project)?;
    ensure!(
        template != config,
        "template must differ from configuration"
    );
    ensure!(
        !template.exists() || template.is_file(),
        "template must be a file"
    );
    ensure!(
        !directory.exists() || directory.is_dir(),
        "creation directory must be a directory"
    );
    let mut rendered = if defaults_added {
        document.to_string()
    } else {
        original.clone()
    };
    if creation.and_then(|v| v.get("directory")).is_none() {
        rendered = format!("# Warp output alternatives: docs/warps or .recur/warps\n{rendered}");
    }
    let _: toml::Value = toml::from_str(&rendered).context("generated configuration is invalid")?;
    let config_changed = rendered != original;
    let template_missing = !template.exists();
    let mut writes = Vec::new();
    if template_missing {
        writes.push(template.clone());
    }
    if config_changed {
        writes.push(config.clone());
    }
    if !dry_run && !writes.is_empty() {
        // Stage all bytes first. Publish the template before the config that points
        // to it; roll back our template if config publication fails.
        let mut created_dirs = Vec::new();
        let mut published_template = false;
        let result = (|| -> anyhow::Result<()> {
            for path in &writes {
                let mut missing = Vec::new();
                let mut parent = path.parent().context("output has no parent")?;
                while !parent.exists() {
                    missing.push(parent.to_path_buf());
                    parent = parent.parent().context("parent has no ancestor")?;
                }
                for dir in missing.into_iter().rev() {
                    fs::create_dir(&dir)?;
                    created_dirs.push(dir);
                }
            }
            let mut staged_template = if template_missing {
                let mut file = tempfile::NamedTempFile::new_in(template.parent().unwrap())?;
                file.write_all(&serde_json::to_vec_pretty(&starter_template())?)?;
                file.as_file().sync_all()?;
                Some(file)
            } else {
                None
            };
            let staged_config = if config_changed {
                let mut file = tempfile::NamedTempFile::new_in(config.parent().unwrap())?;
                file.write_all(rendered.as_bytes())?;
                file.as_file().sync_all()?;
                Some(file)
            } else {
                None
            };
            contained(&template, project)?;
            contained(&config, project)?;
            ensure!(
                if config.exists() {
                    fs::read_to_string(&config)? == original
                } else {
                    original.is_empty()
                },
                "configuration changed during initialization; retry"
            );
            if let Some(file) = staged_template.take() {
                file.persist_noclobber(&template)
                    .map_err(|e| anyhow::anyhow!("template publication: {e}"))?;
                published_template = true;
            }
            if let Some(file) = staged_config {
                if config.exists() {
                    file.persist(&config)
                        .map_err(|e| anyhow::anyhow!("configuration publication: {e}"))?;
                } else {
                    file.persist_noclobber(&config)
                        .map_err(|e| anyhow::anyhow!("configuration publication: {e}"))?;
                }
            }
            Ok(())
        })();
        if result.is_err() {
            if published_template {
                fs::remove_file(&template).context("init rollback of starter template failed")?;
            }
            for dir in created_dirs.iter().rev() {
                let _ = fs::remove_dir(dir);
            }
        }
        result?;
    }
    Ok(
        json!({"schema":"warp-init-v1", "state":if dry_run {"planned"} else if writes.is_empty() {"unchanged"} else {"written"},
        "configuration_source":config, "template":template, "writes":writes,
        "removal_guards_enforced":false}),
    )
}

#[cfg(all(test, windows))]
mod tests {
    use super::*;
    use std::os::windows::fs::OpenOptionsExt;

    #[test]
    fn failed_config_publication_rolls_back_template_and_staging_files() {
        let root = tempfile::tempdir().unwrap();
        let vault = root.path().join(".recur");
        fs::create_dir(&vault).unwrap();
        let config = vault.join("config.toml");
        let original = "# user comment\n[warp.removal]\nrequire_confirmation = false\n";
        fs::write(&config, original).unwrap();
        // Permit readers but deny replacement, forcing failure after the starter
        // template was published. This models an editor holding the config open.
        let held = fs::OpenOptions::new()
            .read(true)
            .share_mode(1)
            .open(&config)
            .unwrap();
        let error = init(root.path(), false).unwrap_err();
        assert!(error.to_string().contains("configuration publication"));
        assert_eq!(fs::read_to_string(&config).unwrap(), original);
        assert_eq!(fs::read_dir(&vault).unwrap().count(), 1);
        drop(held);
        init(root.path(), false).unwrap();
        assert!(vault.join("warp-template.json").is_file());
    }
}
