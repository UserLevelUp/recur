use crate::main_command_flatten_impl::{EntryKind, FlatEntry};

fn normalize_csv_column_name(name: &str, index: usize) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return format!("col{}", index);
    }
    let mut normalized = String::with_capacity(trimmed.len());
    for ch in trimmed.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            normalized.push(ch);
        } else {
            normalized.push('_');
        }
    }
    normalized
}

pub fn flatten_csv(
    content: &str,
    separator: char,
    max_depth: usize,
) -> anyhow::Result<Vec<FlatEntry>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let headers = reader
        .headers()?
        .iter()
        .enumerate()
        .map(|(i, h)| normalize_csv_column_name(h, i))
        .collect::<Vec<_>>();

    let mut entries = Vec::new();
    for (row_idx, row_result) in reader.records().enumerate() {
        let row = row_result?;
        let row_path = format!("rows[{}]", row_idx);

        if max_depth > 0 && max_depth <= 1 {
            entries.push(FlatEntry {
                path: row_path,
                value: Some(row.iter().collect::<Vec<_>>().join(",")),
                kind: EntryKind::Text,
            });
            continue;
        }

        for (col_idx, value) in row.iter().enumerate() {
            let col = headers
                .get(col_idx)
                .cloned()
                .unwrap_or_else(|| format!("col{}", col_idx));
            let path = format!("{}{}{}", row_path, separator, col);
            entries.push(FlatEntry {
                path,
                value: Some(value.to_string()),
                kind: EntryKind::Text,
            });
        }
    }

    Ok(entries)
}
