use crate::main_command_flatten_impl::{EntryKind, FlatEntry};

fn flatten_toml_value(
    value: &toml::Value,
    prefix: &str,
    separator: char,
    max_depth: usize,
    current_depth: usize,
    entries: &mut Vec<FlatEntry>,
) {
    match value {
        toml::Value::Table(map) => {
            if max_depth > 0 && current_depth >= max_depth {
                entries.push(FlatEntry {
                    path: prefix.to_string(),
                    value: Some(value.to_string()),
                    kind: EntryKind::Text,
                });
                return;
            }
            for (key, val) in map {
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}{}{}", prefix, separator, key)
                };
                flatten_toml_value(val, &path, separator, max_depth, current_depth + 1, entries);
            }
        }
        toml::Value::Array(arr) => {
            if max_depth > 0 && current_depth >= max_depth {
                entries.push(FlatEntry {
                    path: prefix.to_string(),
                    value: Some(value.to_string()),
                    kind: EntryKind::Text,
                });
                return;
            }
            for (i, val) in arr.iter().enumerate() {
                let path = if prefix.is_empty() {
                    format!("[{}]", i)
                } else {
                    format!("{}[{}]", prefix, i)
                };
                flatten_toml_value(val, &path, separator, max_depth, current_depth + 1, entries);
            }
        }
        toml::Value::String(s) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(s.clone()),
                kind: EntryKind::Text,
            });
        }
        toml::Value::Integer(i) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(i.to_string()),
                kind: EntryKind::Text,
            });
        }
        toml::Value::Float(f) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(f.to_string()),
                kind: EntryKind::Text,
            });
        }
        toml::Value::Boolean(b) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(b.to_string()),
                kind: EntryKind::Text,
            });
        }
        toml::Value::Datetime(dt) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(dt.to_string()),
                kind: EntryKind::Text,
            });
        }
    }
}

pub fn flatten_toml(
    content: &str,
    separator: char,
    max_depth: usize,
) -> anyhow::Result<Vec<FlatEntry>> {
    let value: toml::Value = toml::from_str(content)?;
    let mut entries = Vec::new();
    flatten_toml_value(&value, "", separator, max_depth, 0, &mut entries);
    Ok(entries)
}
