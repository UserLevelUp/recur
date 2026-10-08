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

pub(crate) fn intelligence_policy(settings: &toml::Value) -> anyhow::Result<(i8, String)> {
    let policy = settings.get("warp").and_then(|w| w.get("intelligence"));
    ensure!(policy.map_or(true, |p| p.is_table()), "warp.intelligence must be a table");
    let tick = match policy.and_then(|p| p.get("default_tick")) {
        Some(value) => value.as_integer().context("warp.intelligence.default_tick must be an integer")?,
        None => 0,
    };
    ensure!((-1..=1).contains(&tick), "warp.intelligence.default_tick must be -1, 0 or 1");
    let baseline = match policy.and_then(|p| p.get("baseline")) {
        Some(value) => value.as_str().context("warp.intelligence.baseline must be a string")?,
        None => "host-current",
    };
    ensure!(!baseline.trim().is_empty(), "warp.intelligence.baseline must not be blank");
    Ok((tick as i8, baseline.to_owned()))
}

fn apply_intelligence_ticks(map: &mut Value, requests: &[String], default_tick: i8) -> anyhow::Result<()> {
    let slices = map["required_slices"].as_array_mut().context("required_slices must be an array")?;
    for slice in slices.iter_mut() {
        if slice.get("intelligence_tick").is_none() {
            slice["intelligence_tick"] = default_tick.into();
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    for request in requests {
        let (id, value) = request.split_once('=').context("slice intelligence must be SLICE=-1|0|1")?;
        ensure!(seen.insert(id), "duplicate slice intelligence request for '{id}'");
        let tick: i8 = value.parse().context("intelligence tick must be -1, 0 or 1")?;
        ensure!((-1..=1).contains(&tick), "intelligence tick must be -1, 0 or 1");
        let slice = slices.iter_mut().find(|s| s["slice_id"].as_str() == Some(id))
            .with_context(|| format!("unknown slice intelligence target '{id}'"))?;
        slice["intelligence_tick"] = tick.into();
    }
    for slice in slices {
        let tick = slice["intelligence_tick"].as_i64().context("slice intelligence_tick must be an integer")?;
        ensure!((-1..=1).contains(&tick), "slice intelligence_tick must be -1, 0 or 1");
    }
    Ok(())
}

pub fn create(root: &Path, warp: &str, goal: &str, slice_intelligence: &[String], confirm: bool) -> anyhow::Result<Value> {
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
    let (default_tick, baseline) = intelligence_policy(&settings)?;
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
    apply_intelligence_ticks(&mut map, slice_intelligence, default_tick)?;
    map["intelligence_baseline"] = baseline.into();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_policy_template_and_cli_precedence() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir(root.path().join(".recur")).unwrap();
        fs::write(root.path().join(".recur/config.toml"), "[warp.creation]\ntemplate = '.recur/template.json'\n[warp.intelligence]\ndefault_tick = -1\nbaseline = 'medium'\n").unwrap();
        let mut template = starter_template();
        template["required_slices"][1]["intelligence_tick"] = json!(1);
        fs::write(root.path().join(".recur/template.json"), serde_json::to_vec(&template).unwrap()).unwrap();
        let preview = create(root.path(), "demo", "Goal", &[], false).unwrap();
        assert_eq!(preview["map"]["intelligence_baseline"], "medium");
        assert_eq!(preview["map"]["required_slices"][0]["intelligence_tick"], -1);
        assert_eq!(preview["map"]["required_slices"][1]["intelligence_tick"], 1);
        let override_map = create(root.path(), "demo", "Goal", &["slice-final=0".into()], false).unwrap();
        assert_eq!(override_map["map"]["required_slices"][1]["intelligence_tick"], 0);
        for policy in ["default_tick = 2", "default_tick = 'high'", "baseline = ''", "baseline = 1"] {
            let settings: toml::Value = toml::from_str(&format!("[warp.intelligence]\n{policy}")).unwrap();
            assert!(intelligence_policy(&settings).is_err());
        }
    }

    #[test]
    fn intelligence_ticks_preview_publish_and_reject_without_writing() {
        let root = tempfile::tempdir().unwrap();
        let requests = vec!["slice-0=-1".into(), "slice-final=1".into()];
        let preview = create(root.path(), "demo", "Goal", &requests, false).unwrap();
        assert_eq!(preview["map"]["required_slices"][0]["intelligence_tick"], -1);
        assert_eq!(preview["map"]["required_slices"][1]["intelligence_tick"], 1);
        assert!(!root.path().join("warps").exists());
        for request in ["missing=1", "slice-0=2", "slice-0=high", "slice-0"] {
            assert!(create(root.path(), "bad", "Goal", &[request.into()], true).is_err());
            assert!(!root.path().join("warps").exists());
        }
        assert!(create(root.path(), "bad", "Goal", &["slice-0=1".into(), "slice-0=-1".into()], true).is_err());
        let written = create(root.path(), "demo", "Goal", &requests, true).unwrap();
        let stored: Value = serde_json::from_slice(&fs::read(written["path"].as_str().unwrap()).unwrap()).unwrap();
        assert_eq!(stored, written["map"]);
        let parsed: WarpBubbleMap = serde_json::from_value(stored).unwrap();
        assert_eq!(parsed.required_slices[0].intelligence_tick, Some(-1));
        assert_eq!(parsed.required_slices[1].intelligence_tick, Some(1));
        assert!(create(root.path(), "demo", "Goal", &requests, true).is_err());
    }

    #[test]
    fn creation_defaults_to_hold_and_invalid_template_ticks_fail() {
        let root = tempfile::tempdir().unwrap();
        let legacy = create(root.path(), "legacy", "Goal", &[], false).unwrap();
        assert_eq!(legacy["map"]["required_slices"][0]["intelligence_tick"], 0);
        let mut map = starter_template();
        apply_intelligence_ticks(&mut map, &["slice-0=0".into()], 0).unwrap();
        assert_eq!(map["required_slices"][0]["intelligence_tick"], 0);
        render(&mut map, "demo", "Goal");
        map["required_slices"][0]["intelligence_tick"] = json!(2);
        assert!(apply_intelligence_ticks(&mut map, &[], 0).is_err());
    }
}
