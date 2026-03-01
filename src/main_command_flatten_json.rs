use crate::main_command_flatten_impl::{EntryKind, FlatEntry};
use recur::r#trait::strip_utf8_bom;

pub(crate) fn flatten_json_value(
    value: &serde_json::Value,
    prefix: &str,
    separator: char,
    max_depth: usize,
    current_depth: usize,
    entries: &mut Vec<FlatEntry>,
) {
    match value {
        serde_json::Value::Object(map) => {
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
                flatten_json_value(val, &path, separator, max_depth, current_depth + 1, entries);
            }
        }
        serde_json::Value::Array(arr) => {
            if max_depth > 0 && current_depth >= max_depth {
                entries.push(FlatEntry {
                    path: prefix.to_string(),
                    value: Some(value.to_string()),
                    kind: EntryKind::Text,
                });
                return;
            }
            for (i, val) in arr.iter().enumerate() {
                let path = format!("{}[{}]", prefix, i);
                flatten_json_value(val, &path, separator, max_depth, current_depth + 1, entries);
            }
        }
        serde_json::Value::String(s) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(s.clone()),
                kind: EntryKind::Text,
            });
        }
        serde_json::Value::Number(n) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(n.to_string()),
                kind: EntryKind::Text,
            });
        }
        serde_json::Value::Bool(b) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(b.to_string()),
                kind: EntryKind::Text,
            });
        }
        serde_json::Value::Null => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some("null".to_string()),
                kind: EntryKind::Text,
            });
        }
    }
}

pub fn flatten_json(
    content: &str,
    separator: char,
    max_depth: usize,
) -> anyhow::Result<Vec<FlatEntry>> {
    let content = strip_utf8_bom(content);
    let value: serde_json::Value = serde_json::from_str(content)?;
    let mut entries = Vec::new();
    flatten_json_value(&value, "", separator, max_depth, 0, &mut entries);
    Ok(entries)
}
