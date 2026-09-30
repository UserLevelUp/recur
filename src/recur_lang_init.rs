//! Additive project-local Lang preferences; no planner or binding execution.
use serde_json::{json, Value};
use std::{fmt, fs, io::Write, path::Path};

#[derive(Debug)]
pub struct InitError {
    pub code: &'static str,
    pub message: String,
}
impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}
fn error(code: &'static str, message: impl fmt::Display) -> InitError {
    InitError {
        code,
        message: message.to_string(),
    }
}
type Result<T> = std::result::Result<T, InitError>;

// Check each existing ancestor, including dangling links, before writing.
fn contained(path: &Path, project: &Path) -> Result<()> {
    for p in path.ancestors().take_while(|p| p.starts_with(project)) {
        match fs::symlink_metadata(p) {
            Ok(_) => {
                let resolved = fs::canonicalize(p).map_err(|e| error("LINIT003", e))?;
                if !resolved.starts_with(project) {
                    return Err(error(
                        "LINIT003",
                        "configuration escapes the selected project",
                    ));
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(error("LINIT002", e)),
        }
    }
    Ok(())
}
fn original(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(s) => Ok(Some(s)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(error("LINIT002", e)),
    }
}
fn render(text: &str) -> Result<String> {
    let settings: toml::Value = toml::from_str(text).map_err(|e| error("LINIT002", e))?;
    if let Some(lang) = settings.get("recur-lang") {
        if !lang.is_table() {
            return Err(error("LINIT002", "recur-lang must be a table"));
        }
        if lang
            .get("schema_version")
            .is_some_and(|v| v.as_integer() != Some(1))
        {
            return Err(error(
                "LINIT002",
                "recur-lang.schema_version must be integer 1",
            ));
        }
        if lang
            .get("target")
            .is_some_and(|v| v.as_str().map_or(true, |s| s.trim().is_empty()))
        {
            return Err(error(
                "LINIT002",
                "recur-lang.target must be a nonblank string",
            ));
        }
        if let Some(planning) = lang.get("planning") {
            if !planning.is_table() {
                return Err(error("LINIT002", "recur-lang.planning must be a table"));
            }
            for key in [
                "specification_first",
                "prioritize_graph_findings",
                "include_test_plan",
            ] {
                if planning.get(key).is_some_and(|v| v.as_bool().is_none()) {
                    return Err(error(
                        "LINIT002",
                        format!("recur-lang.planning.{key} must be boolean"),
                    ));
                }
            }
        }
    }
    let mut doc: toml_edit::DocumentMut = text.parse().map_err(|e| error("LINIT002", e))?;
    let mut changed = false;
    if doc.get("recur-lang").is_none() {
        doc["recur-lang"] = toml_edit::Item::Table(toml_edit::Table::new());
        changed = true;
    }
    if doc["recur-lang"].get("planning").is_none() {
        doc["recur-lang"]["planning"] = if doc["recur-lang"].is_inline_table() {
            toml_edit::value(toml_edit::InlineTable::new())
        } else {
            toml_edit::Item::Table(toml_edit::Table::new())
        };
        changed = true;
    }
    for (key, value) in [
        ("schema_version", toml_edit::value(1)),
        ("target", toml_edit::value("unspecified")),
    ] {
        if doc["recur-lang"].get(key).is_none() {
            doc["recur-lang"][key] = value;
            changed = true;
        }
    }
    for key in [
        "specification_first",
        "prioritize_graph_findings",
        "include_test_plan",
    ] {
        if doc["recur-lang"]["planning"].get(key).is_none() {
            doc["recur-lang"]["planning"][key] = toml_edit::value(true);
            changed = true;
        }
    }
    let result = if changed {
        doc.to_string()
    } else {
        text.to_owned()
    };
    let _: toml::Value = toml::from_str(&result).map_err(|e| error("LINIT002", e))?;
    Ok(result)
}
pub fn init(root: &Path, dry_run: bool) -> Result<Value> {
    init_with_hook(root, dry_run, |_| Ok(()))
}
fn init_with_hook(
    root: &Path,
    dry_run: bool,
    before_publish: impl FnOnce(&Path) -> anyhow::Result<()>,
) -> Result<Value> {
    let root = fs::canonicalize(root).map_err(|e| error("LINIT001", e))?;
    if !root.is_dir() {
        return Err(error("LINIT001", "root must be a directory"));
    }
    let mut config = root.join(".recur/config.toml");
    for ancestor in root.ancestors() {
        let candidate = ancestor.join(".recur/config.toml");
        // An inaccessible or dangling nearest entry must not silently fall back.
        match fs::symlink_metadata(&candidate) {
            Ok(_) => {
                config = candidate;
                break;
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                let vault = ancestor.join(".recur");
                if fs::symlink_metadata(&vault).is_ok() {
                    contained(&vault, ancestor)?;
                    if !vault.is_dir() {
                        return Err(error("LINIT002", ".recur must be a directory"));
                    }
                }
            }
            Err(e) => return Err(error("LINIT002", e)),
        }
    }
    let vault = config.parent().unwrap();
    let project = vault.parent().unwrap();
    contained(&config, project)?;
    // Keep an in-project config symlink intact by publishing to its resolved file.
    let destination = if config.exists() {
        fs::canonicalize(&config).map_err(|e| error("LINIT002", e))?
    } else {
        config.clone()
    };
    let old = original(&destination)?;
    let preview = render(old.as_deref().unwrap_or(""))?;
    let changed = old.as_deref() != Some(preview.as_str());
    if changed && !dry_run {
        let mut created_vault = false;
        let result = (|| -> Result<()> {
            if !vault.exists() {
                fs::create_dir(vault).map_err(|e| error("LINIT004", e))?;
                created_vault = true;
            }
            contained(&config, project)?;
            let mut staged = tempfile::NamedTempFile::new_in(destination.parent().unwrap())
                .map_err(|e| error("LINIT004", e))?;
            staged
                .write_all(preview.as_bytes())
                .and_then(|_| staged.as_file().sync_all())
                .map_err(|e| error("LINIT004", e))?;
            before_publish(&config).map_err(|e| error("LINIT004", e))?;
            contained(&config, project)?;
            if original(&destination).map_err(|e| error("LINIT004", e))? != old
                || (config.exists()
                    && fs::canonicalize(&config).map_err(|e| error("LINIT004", e))? != destination)
            {
                return Err(error("LINIT004", "configuration changed during init"));
            }
            if old.is_some() {
                staged
                    .persist(&destination)
                    .map_err(|e| error("LINIT004", e))?;
            } else {
                staged
                    .persist_noclobber(&destination)
                    .map_err(|e| error("LINIT004", e))?;
            }
            Ok(())
        })();
        if result.is_err() && created_vault {
            let _ = fs::remove_dir(vault);
        }
        result?;
    }
    Ok(
        json!({"schema":"recur-lang-init-v1","project_root":project,"config_path":config,
        "dry_run":dry_run,"changed":changed,"state":if dry_run {"planned"} else if changed {"written"} else {"unchanged"},
        "preview":preview,"writes":if changed {vec![config.clone()]} else {vec![]}}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn concurrent_edit_is_preserved_and_staging_removed() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir(root.path().join(".recur")).unwrap();
        let path = root.path().join(".recur/config.toml");
        fs::write(&path, "# original\n").unwrap();
        let e = init_with_hook(root.path(), false, |p| {
            fs::write(p, "# external edit\n")?;
            Ok(())
        })
        .unwrap_err();
        assert_eq!(e.code, "LINIT004");
        assert_eq!(fs::read_to_string(path).unwrap(), "# external edit\n");
        assert_eq!(fs::read_dir(root.path().join(".recur")).unwrap().count(), 1);
    }
    #[test]
    fn failed_fresh_publication_removes_only_own_staging() {
        let root = tempfile::tempdir().unwrap();
        let e = init_with_hook(root.path(), false, |_| {
            anyhow::bail!("injected publication failure")
        })
        .unwrap_err();
        assert_eq!(e.code, "LINIT004");
        assert!(!root.path().join(".recur").exists());
    }
    #[test]
    fn concurrent_empty_config_is_not_overwritten() {
        let root = tempfile::tempdir().unwrap();
        let e = init_with_hook(root.path(), false, |p| {
            fs::write(p, "")?;
            Ok(())
        })
        .unwrap_err();
        assert_eq!(e.code, "LINIT004");
        assert_eq!(
            fs::read(root.path().join(".recur/config.toml")).unwrap(),
            b""
        );
    }
    #[cfg(windows)]
    #[test]
    fn windows_sharing_lock_preserves_config_and_cleans_staging() {
        use std::os::windows::fs::OpenOptionsExt;
        let root = tempfile::tempdir().unwrap();
        fs::create_dir(root.path().join(".recur")).unwrap();
        let path = root.path().join(".recur/config.toml");
        fs::write(&path, "# keep\n").unwrap();
        let held = fs::OpenOptions::new()
            .read(true)
            .share_mode(1)
            .open(&path)
            .unwrap();
        let e = init(root.path(), false).unwrap_err();
        assert_eq!(e.code, "LINIT004");
        assert_eq!(fs::read_to_string(&path).unwrap(), "# keep\n");
        assert_eq!(fs::read_dir(path.parent().unwrap()).unwrap().count(), 1);
        drop(held);
        assert!(init(root.path(), false).is_ok());
    }
}
