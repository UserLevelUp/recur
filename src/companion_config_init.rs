//! Additive configuration publication shared by companion initializers.
use anyhow::{ensure, Context};
use serde_json::{json, Value};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fmt, fs};
use toml_edit::{DocumentMut, TableLike};

#[derive(Clone, Copy)]
pub struct InitSpec {
    pub schema: &'static str,
    pub error_prefix: &'static str,
}

#[derive(Debug)]
pub struct InitError {
    pub code: String,
    pub message: String,
}

impl fmt::Display for InitError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for InitError {}

impl InitSpec {
    pub fn error(self, number: u8, message: impl fmt::Display) -> InitError {
        InitError {
            code: format!("{}{:03}", self.error_prefix, number),
            message: message.to_string(),
        }
    }
}

type Result<T> = std::result::Result<T, InitError>;

fn contained(path: &Path, project: &Path, spec: InitSpec) -> Result<()> {
    for ancestor in path.ancestors().take_while(|p| p.starts_with(project)) {
        match fs::symlink_metadata(ancestor) {
            Ok(_) => {
                let resolved = ancestor.canonicalize().map_err(|e| spec.error(3, e))?;
                if !resolved.starts_with(project) {
                    return Err(spec.error(3, "configuration escapes the selected project"));
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(spec.error(2, e)),
        }
    }
    Ok(())
}

fn read_original(path: &Path, spec: InitSpec) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(spec.error(2, e)),
    }
}

struct ConfigLock(PathBuf);

impl ConfigLock {
    fn acquire(destination: &Path, spec: InitSpec) -> Result<Self> {
        let path = destination.with_extension("recur-init.lock");
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| spec.error(4, format!("cannot acquire {}: {e}", path.display())))?;
        Ok(Self(path))
    }
}

impl Drop for ConfigLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn config_path(root: &Path, spec: InitSpec) -> Result<PathBuf> {
    let root = root.canonicalize().map_err(|e| spec.error(1, e))?;
    if !root.is_dir() {
        return Err(spec.error(1, "root must be a directory"));
    }
    for ancestor in root.ancestors() {
        let vault = ancestor.join(".recur");
        let candidate = vault.join("config.toml");
        match fs::symlink_metadata(&candidate) {
            Ok(_) => {
                contained(&candidate, ancestor, spec)?;
                return Ok(candidate);
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if fs::symlink_metadata(&vault).is_ok() {
                    contained(&vault, ancestor, spec)?;
                    if !vault.is_dir() {
                        return Err(spec.error(2, ".recur must be a directory"));
                    }
                }
            }
            Err(e) => return Err(spec.error(2, e)),
        }
    }
    Ok(root.join(".recur/config.toml"))
}

pub fn read_config(root: &Path, spec: InitSpec) -> Result<Option<String>> {
    read_original(&config_path(root, spec)?, spec)
}

pub fn initialize(
    root: &Path,
    dry_run: bool,
    spec: InitSpec,
    render: impl FnOnce(&str) -> Result<String>,
    before_publish: impl FnOnce(&Path) -> anyhow::Result<()>,
) -> Result<Value> {
    let config = config_path(root, spec)?;
    let vault = config.parent().expect("config is inside a vault");
    let project = vault.parent().expect("vault is inside a project");
    contained(&config, project, spec)?;
    // Preserve an in-project config symlink by publishing to its resolved file.
    let destination = if config.exists() {
        config.canonicalize().map_err(|e| spec.error(2, e))?
    } else {
        config.clone()
    };
    let old = read_original(&destination, spec)?;
    let preview = render(old.as_deref().unwrap_or(""))?;
    let changed = old.as_deref() != Some(preview.as_str());
    if changed && !dry_run {
        let mut created_vault = false;
        let publish = (|| -> Result<()> {
            if !vault.exists() {
                fs::create_dir(vault).map_err(|e| spec.error(4, e))?;
                created_vault = true;
            }
            contained(&config, project, spec)?;
            let _lock = ConfigLock::acquire(&destination, spec)?;
            let mut staged = tempfile::NamedTempFile::new_in(destination.parent().unwrap())
                .map_err(|e| spec.error(4, e))?;
            staged
                .write_all(preview.as_bytes())
                .and_then(|_| staged.as_file().sync_all())
                .map_err(|e| spec.error(4, e))?;
            before_publish(&config).map_err(|e| spec.error(4, e))?;
            contained(&config, project, spec)?;
            if read_original(&destination, spec).map_err(|e| spec.error(4, e))? != old
                || (old.is_some()
                    && config.canonicalize().map_err(|e| spec.error(4, e))? != destination)
            {
                return Err(spec.error(4, "configuration changed during init"));
            }
            if old.is_some() {
                staged.persist(&destination).map_err(|e| spec.error(4, e))?;
            } else {
                staged
                    .persist_noclobber(&destination)
                    .map_err(|e| spec.error(4, e))?;
            }
            Ok(())
        })();
        if publish.is_err() && created_vault {
            let _ = fs::remove_dir(vault);
        }
        publish?;
    }
    Ok(json!({
        "schema": spec.schema, "project_root": project, "config_path": config,
        "dry_run": dry_run, "changed": changed,
        "state": if dry_run { "planned" } else if changed { "written" } else { "unchanged" },
        "preview": preview, "writes": if changed { vec![config.clone()] } else { vec![] },
    }))
}

/// Fill missing leaves, preserving explicit values and comment formatting.
pub fn with_defaults(text: &str, defaults: &str) -> anyhow::Result<String> {
    let mut document: DocumentMut = text.parse().context("invalid project configuration")?;
    let defaults: DocumentMut = defaults.parse().context("invalid companion defaults")?;
    let changed = add_defaults(document.as_table_mut(), defaults.as_table(), "")?;
    if changed {
        Ok(document.to_string())
    } else {
        Ok(text.to_owned())
    }
}

fn add_defaults(
    target: &mut dyn TableLike,
    defaults: &dyn TableLike,
    prefix: &str,
) -> anyhow::Result<bool> {
    let mut changed = false;
    for (key, default) in defaults.iter() {
        let Some(current) = target.get_mut(key) else {
            target.insert(key, default.clone());
            changed = true;
            continue;
        };
        if let Some(children) = default.as_table_like() {
            let path = format!("{prefix}{key}");
            ensure!(current.is_table_like(), "{path} must be a table");
            changed |= add_defaults(
                current.as_table_like_mut().unwrap(),
                children,
                &format!("{path}."),
            )?;
        }
    }
    Ok(changed)
}
