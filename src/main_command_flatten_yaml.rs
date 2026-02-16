use crate::main_command_flatten_impl::FlatEntry;
use crate::main_command_flatten_json::flatten_json_value;

pub fn flatten_yaml(
    content: &str,
    separator: char,
    max_depth: usize,
) -> anyhow::Result<Vec<FlatEntry>> {
    let value: serde_json::Value = serde_yaml::from_str(content)?;
    let mut entries = Vec::new();
    flatten_json_value(&value, "", separator, max_depth, 0, &mut entries);
    Ok(entries)
}
