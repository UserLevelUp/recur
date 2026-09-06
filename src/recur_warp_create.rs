//! Single-map scaffolding. Templates are data, never executable instructions.
use anyhow::{ensure, Context};
use recur::warp_bubble::{validate_bubble_map, WarpBubbleMap};
use serde_json::{json, Value};
use std::{
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

fn portable_component(value: &str) -> bool {
    let stem = value.split('.').next().unwrap_or("").to_ascii_uppercase();
    !value.is_empty()
        && !value.ends_with(['.', ' '])
        && !value
            .chars()
            .any(|c| c.is_control() || "<>:\"/\\|?*".contains(c))
        && ![
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ]
        .contains(&stem.as_str())
}

pub(crate) fn relative(base: &Path, text: &str) -> anyhow::Result<PathBuf> {
    ensure!(!text.is_empty(), "empty configured path");
    let path = Path::new(text);
    for part in path.components() {
        match part {
            Component::Normal(p) => ensure!(
                portable_component(&p.to_string_lossy()),
                "unsafe configured path"
            ),
            Component::CurDir => (),
            _ => anyhow::bail!("configured paths must be relative without parent traversal"),
        }
    }
    Ok(base.join(path))
}

pub(crate) fn contained(path: &Path, root: &Path) -> anyhow::Result<()> {
    ensure!(path.starts_with(root), "configured output escapes -d scope");
    let mut current = root.to_path_buf();
    for part in path.strip_prefix(root)?.components() {
        current.push(part);
        match fs::symlink_metadata(&current) {
            Ok(meta) => {
                ensure!(
                    !meta.file_type().is_symlink(),
                    "symlink paths are not supported for creation"
                );
                ensure!(
                    fs::canonicalize(&current)?.starts_with(root),
                    "resolved path escapes scope"
                );
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (),
            Err(error) => return Err(error.into()),
        }
    }
    Ok(())
}

fn render(value: &mut Value, warp: &str, goal: &str) {
    match value {
        Value::String(s) => *s = s.replace("{warp}", warp).replace("{goal}", goal),
        Value::Array(items) => items.iter_mut().for_each(|v| render(v, warp, goal)),
        Value::Object(items) => items.values_mut().for_each(|v| render(v, warp, goal)),
        _ => (),
    }
}

pub(crate) fn starter_template() -> Value {
    json!({"schema":"warp-bubble-map-v1", "warp_id":"{warp}", "goal":"{goal}",
        "invariants":[], "current_slice":"slice-0", "required_slices":[
            {"slice_id":"slice-0", "contract_hash":"contract:{warp}.slice-0:v1", "depends_on":[], "evidence_gates":["baseline"], "evidence_mode":"declared"},
            {"slice_id":"slice-final", "contract_hash":"contract:{warp}.slice-final:v1", "depends_on":["slice-0"], "evidence_gates":["acceptance"], "evidence_mode":"declared"}]})
}

// UUIDv7: 48-bit Unix milliseconds, version/variant bits, and 74 random bits.
fn uuid7() -> anyhow::Result<String> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).map_err(|e| anyhow::anyhow!("UUID randomness: {e}"))?;
    let millis = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    ensure!(millis < (1u128 << 48), "UUIDv7 timestamp out of range");
    bytes[..6].copy_from_slice(&(millis as u64).to_be_bytes()[2..]);
    bytes[6] = (bytes[6] & 0x0f) | 0x70;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    Ok(format!(
        "{}-{}-{}-{}-{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..]
    ))
}

pub fn create(root: &Path, warp: &str, goal: &str, confirm: bool) -> anyhow::Result<Value> {
    ensure!(
        portable_component(warp)
            && warp.len() <= 128
            && !warp.starts_with('.')
            && !warp.contains("..")
            && warp
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b".-_".contains(&c)),
        "invalid portable Warp identity"
    );
    ensure!(!goal.trim().is_empty(), "goal must not be blank");
    let root = fs::canonicalize(root)?;
    ensure!(root.is_dir(), "-d must be a directory");
    let config = root
        .ancestors()
        .map(|p| p.join(".recur/config.toml"))
        .find(|p| p.is_file());
    let project = config
        .as_ref()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .unwrap_or(&root);
    let settings: toml::Value = match &config {
        Some(p) => toml::from_str(&fs::read_to_string(p)?)?,
        None => toml::Value::Table(Default::default()),
    };
    recur::warp_policy::WarpRemovalPolicy::from_config(&settings)?;
    let creation = settings.get("warp").and_then(|v| v.get("creation"));
    if let Some(value) = creation {
        ensure!(value.is_table(), "warp.creation must be a table");
    }
    let setting = |key: &str| -> anyhow::Result<Option<&str>> {
        creation
            .and_then(|v| v.get(key))
            .map(|v| {
                v.as_str()
                    .with_context(|| format!("warp.creation.{key} must be a string"))
            })
            .transpose()
    };
    let directory = relative(project, setting("directory")?.unwrap_or("warps"))?;
    contained(&directory, &root)?;
    let target = directory.join(format!("{warp}.warp-map.json"));
    contained(&target, &root)?;
    ensure!(
        !target.exists(),
        "refusing to overwrite {}",
        target.display()
    );
    let mut map = if let Some(template) = setting("template")? {
        let path = relative(project, template)?;
        contained(&path, project)?;
        ensure!(
            fs::metadata(&path)?.len() <= 1_048_576,
            "template exceeds 1 MiB"
        );
        serde_json::from_slice(&fs::read(path)?)?
    } else {
        starter_template()
    };
    render(&mut map, warp, goal);
    map.as_object_mut()
        .context("template must be a JSON object")?
        .insert("goal".into(), goal.into());
    map["bubble_uuid"] = uuid7()?.into();
    for slice in map["required_slices"]
        .as_array_mut()
        .context("required_slices must be an array")?
    {
        slice
            .as_object_mut()
            .context("slice must be an object")?
            .insert("slice_uuid".into(), uuid7()?.into());
    }
    let parsed: WarpBubbleMap = serde_json::from_value(map.clone())?;
    validate_bubble_map(&parsed, warp, &target)?;
    for slice in &parsed.required_slices {
        ensure!(
            !slice.evidence_gates.is_empty(),
            "every generated slice requires acceptance gates"
        );
    }
    if let Some(current) = map.get("current_slice") {
        ensure!(
            current.is_null()
                || parsed
                    .required_slices
                    .iter()
                    .any(|s| Some(s.slice_id.as_str()) == current.as_str()),
            "current_slice must name a declared slice or be null"
        );
    }
    let bytes = serde_json::to_vec_pretty(&map)?;
    if confirm {
        contained(&target, &root)?;
        fs::create_dir_all(&directory)?;
        contained(&directory, &root)?;
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let temp = directory.join(format!(
            ".recur-warp-create-{}-{stamp}.tmp",
            std::process::id()
        ));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        let result = (|| -> anyhow::Result<()> {
            file.write_all(&bytes)?;
            file.sync_all()?;
            contained(&target, &root)?;
            // Atomic no-clobber publication; unsupported filesystems fail closed.
            fs::hard_link(&temp, &target)
                .context("cannot publish map without overwriting (hard-link support required)")?;
            Ok(())
        })();
        drop(file);
        let cleanup = fs::remove_file(&temp);
        result?;
        cleanup.context("map published but temporary link cleanup failed")?;
    }
    Ok(
        json!({"schema":"warp-create-v1", "state":if confirm {"written"} else {"planned"},
        "warp_id":warp, "path":target, "configuration_source":config, "map":map}),
    )
}
