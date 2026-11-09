//! Additive project-local Lang preferences; no planner or binding execution.
use recur::companion_config_init::{self, InitError, InitSpec};
use serde_json::Value;
use std::{fmt, path::Path};

const SPEC: InitSpec = InitSpec {
    schema: "recur-lang-init-v1",
    error_prefix: "LINIT",
};

fn invalid_config(message: impl fmt::Display) -> InitError {
    SPEC.error(2, message)
}

type Result<T> = std::result::Result<T, InitError>;

fn render(text: &str) -> Result<String> {
    let settings: toml::Value = toml::from_str(text).map_err(|e| invalid_config(e))?;
    if let Some(lang) = settings.get("recur-lang") {
        if !lang.is_table() {
            return Err(invalid_config("recur-lang must be a table"));
        }
        if lang
            .get("schema_version")
            .is_some_and(|v| v.as_integer() != Some(1))
        {
            return Err(invalid_config(
                "recur-lang.schema_version must be integer 1",
            ));
        }
        if lang
            .get("target")
            .is_some_and(|v| v.as_str().map_or(true, |s| s.trim().is_empty()))
        {
            return Err(invalid_config(
                "recur-lang.target must be a nonblank string",
            ));
        }
        if let Some(planning) = lang.get("planning") {
            if !planning.is_table() {
                return Err(invalid_config("recur-lang.planning must be a table"));
            }
            for key in [
                "specification_first",
                "prioritize_graph_findings",
                "include_test_plan",
            ] {
                if planning.get(key).is_some_and(|v| v.as_bool().is_none()) {
                    return Err(invalid_config(format!(
                        "recur-lang.planning.{key} must be boolean"
                    )));
                }
            }
        }
    }
    let mut doc: toml_edit::DocumentMut = text.parse().map_err(|e| invalid_config(e))?;
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
    let _: toml::Value = toml::from_str(&result).map_err(|e| invalid_config(e))?;
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
    companion_config_init::initialize(root, dry_run, SPEC, render, before_publish)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn overlapping_initializers_use_the_same_publication_lock() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir(root.path().join(".recur")).unwrap();
        let config = root.path().join(".recur/config.toml");
        fs::write(&config, "# keep\n").unwrap();
        init_with_hook(root.path(), false, |_| {
            let error = init(root.path(), false).unwrap_err();
            assert_eq!(error.code, "LINIT004");
            assert!(error.message.contains("config.recur-init.lock"));
            Ok(())
        })
        .unwrap();
        assert!(fs::read_to_string(&config)
            .unwrap()
            .contains("[recur-lang]"));
        assert_eq!(fs::read_dir(config.parent().unwrap()).unwrap().count(), 1);
    }
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
