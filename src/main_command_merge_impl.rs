///! merge command implementation
///!
///! Merges hierarchical results from multiple pattern/separator pairs into unified view.
///! Follows Unix philosophy: explicit composition over automatic conversion.
use anyhow::{bail, Context, Result};
use recur::parser::HierarchyPattern;
use recur::search::{FileSearcher, SearchOptions};
use recur::tree::HierarchyTree;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::io::{stdin, Read};
use std::path::PathBuf;

/// Execute merge command
pub fn execute(
    patterns: Vec<String>,
    separators: Vec<char>,
    inputs: Vec<PathBuf>,
    base: Option<String>,
    dir: PathBuf,
    max_depth: Option<usize>,
    replace_default: Option<char>,
    show_sep: bool,
    unicode: bool,
    show_count: bool,
    json: bool,
    use_stdin: bool,
) -> Result<()> {
    if use_stdin {
        return execute_stdin_mode(
            separators,
            base,
            replace_default,
            show_sep,
            unicode,
            show_count,
            json,
        );
    }

    if !inputs.is_empty() {
        return execute_file_mode(
            inputs,
            separators,
            base,
            replace_default,
            show_sep,
            unicode,
            show_count,
            json,
        );
    }

    // Step 1: Collect files from all pattern/separator pairs
    let mut all_files: Vec<PathBuf> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut file_separators: std::collections::HashMap<PathBuf, char> =
        std::collections::HashMap::new();

    for (pattern, separator) in patterns.iter().zip(separators.iter()) {
        let files = find_files_for_pattern(pattern, *separator, &dir, max_depth)?;
        let count = files.len();
        let _ = count;
        for file in files {
            // Deduplicate: only add if not seen before
            if seen.insert(file.clone()) {
                all_files.push(file.clone());
                file_separators.insert(file, *separator);
            }
        }
    }

    // Step 2: Check if we found anything
    if all_files.is_empty() {
        println!("No files found");
        return Ok(());
    }

    // Step 3: Normalize paths (if requested) and display unified tree
    let tree_separator = replace_default.unwrap_or(separators[0]);
    let base_pattern = normalize_pattern_for_separator(&patterns[0], tree_separator);
    let show_markers = show_sep && separators.len() > 1;

    let tree_files: Vec<PathBuf> = all_files
        .iter()
        .map(|path| {
            let original_sep = file_separators.get(path).copied().unwrap_or(separators[0]);
            let mut display_path = normalize_path_separator(path, original_sep, tree_separator);

            if show_markers {
                if let Some(filename) = display_path.file_name() {
                    let marked_filename =
                        format!("{} [{}]", filename.to_string_lossy(), original_sep);
                    display_path.set_file_name(marked_filename);
                }
            }

            display_path
        })
        .collect();

    display_tree(
        &tree_files,
        &base_pattern,
        tree_separator,
        unicode,
        show_count,
        json,
    )?;

    Ok(())
}

fn execute_file_mode(
    inputs: Vec<PathBuf>,
    separators: Vec<char>,
    base: Option<String>,
    replace_default: Option<char>,
    show_sep: bool,
    unicode: bool,
    show_count: bool,
    json: bool,
) -> Result<()> {
    let base = base.context("--base is required when using file inputs")?;

    let mut all_files: Vec<PathBuf> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut file_separators: std::collections::HashMap<PathBuf, char> =
        std::collections::HashMap::new();

    for (input, separator) in inputs.iter().zip(separators.iter()) {
        let files = load_files_from_json_file(input)
            .with_context(|| format!("Failed to read JSON input: {}", input.display()))?;

        for file in files {
            if seen.insert(file.clone()) {
                all_files.push(file.clone());
                file_separators.insert(file, *separator);
            }
        }
    }

    if all_files.is_empty() {
        println!("No files found");
        return Ok(());
    }

    let tree_separator = replace_default.unwrap_or(separators[0]);
    let base_pattern = normalize_pattern_for_separator(&base, tree_separator);
    let show_markers = show_sep && separators.len() > 1;

    let tree_files: Vec<PathBuf> = all_files
        .iter()
        .map(|path| {
            let original_sep = file_separators.get(path).copied().unwrap_or(separators[0]);
            let mut display_path = normalize_path_separator(path, original_sep, tree_separator);

            if show_markers {
                if let Some(filename) = display_path.file_name() {
                    let marked_filename =
                        format!("{} [{}]", filename.to_string_lossy(), original_sep);
                    display_path.set_file_name(marked_filename);
                }
            }

            display_path
        })
        .collect();

    display_tree(
        &tree_files,
        &base_pattern,
        tree_separator,
        unicode,
        show_count,
        json,
    )?;

    Ok(())
}

fn execute_stdin_mode(
    separators: Vec<char>,
    base: Option<String>,
    replace_default: Option<char>,
    show_sep: bool,
    unicode: bool,
    show_count: bool,
    json: bool,
) -> Result<()> {
    let base = base.context("--base is required when using --stdin")?;

    let mut all_files: Vec<PathBuf> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();
    let mut file_separators: std::collections::HashMap<PathBuf, char> =
        std::collections::HashMap::new();

    // Read all stdin into a string
    let mut stdin_content = String::new();
    stdin()
        .read_to_string(&mut stdin_content)
        .context("Failed to read from stdin")?;

    // Parse multiple JSON objects from stdin stream
    let stream = serde_json::Deserializer::from_str(&stdin_content).into_iter::<Value>();
    let mut source_idx = 0;

    for result in stream {
        let value = result.with_context(|| {
            format!("Failed to parse JSON object {} from stdin", source_idx + 1)
        })?;

        // Determine separator for this source
        let separator = separators
            .get(source_idx)
            .copied()
            .unwrap_or(separators.first().copied().unwrap_or('.'));

        // Extract file paths from this JSON object
        let files = extract_paths_from_json(&value).with_context(|| {
            format!(
                "Failed to extract paths from JSON object {}",
                source_idx + 1
            )
        })?;

        // Add files with deduplication and provenance tracking
        for file in files {
            if seen.insert(file.clone()) {
                all_files.push(file.clone());
                file_separators.insert(file, separator);
            }
        }

        source_idx += 1;
    }

    if all_files.is_empty() {
        println!("No files found in stdin");
        return Ok(());
    }

    let tree_separator = replace_default.unwrap_or(separators.first().copied().unwrap_or('.'));
    let base_pattern = normalize_pattern_for_separator(&base, tree_separator);
    let show_markers = show_sep && separators.len() > 1;

    let tree_files: Vec<PathBuf> = all_files
        .iter()
        .map(|path| {
            let original_sep = file_separators
                .get(path)
                .copied()
                .unwrap_or(separators.first().copied().unwrap_or('.'));
            let mut display_path = normalize_path_separator(path, original_sep, tree_separator);

            if show_markers {
                if let Some(filename) = display_path.file_name() {
                    let marked_filename =
                        format!("{} [{}]", filename.to_string_lossy(), original_sep);
                    display_path.set_file_name(marked_filename);
                }
            }

            display_path
        })
        .collect();

    display_tree(
        &tree_files,
        &base_pattern,
        tree_separator,
        unicode,
        show_count,
        json,
    )?;

    Ok(())
}

fn load_files_from_json_file(path: &PathBuf) -> Result<Vec<PathBuf>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Unable to read file: {}", path.display()))?;
    // Windows tools (notably some PowerShell encodings) can prepend UTF-8 BOM.
    // serde_json rejects BOM-prefixed text, so normalize before parsing.
    let content = content.strip_prefix('\u{FEFF}').unwrap_or(&content);
    let value: Value = serde_json::from_str(content)
        .with_context(|| format!("Invalid JSON in: {}", path.display()))?;
    extract_paths_from_json(&value)
}

fn extract_paths_from_json(value: &Value) -> Result<Vec<PathBuf>> {
    match value {
        Value::Array(items) => extract_paths_from_array(items),
        Value::Object(map) => {
            if let Some(files) = map.get("files") {
                return extract_paths_from_json(files);
            }

            let mut out: Vec<PathBuf> = Vec::new();
            collect_tree_paths(value, &mut out);
            if out.is_empty() {
                bail!("No file paths found in JSON object");
            }
            Ok(out)
        }
        _ => bail!("Unsupported JSON format for merge inputs"),
    }
}

fn extract_paths_from_array(items: &[Value]) -> Result<Vec<PathBuf>> {
    let mut out: Vec<PathBuf> = Vec::new();

    for item in items {
        match item {
            Value::String(path) => out.push(PathBuf::from(path)),
            Value::Object(map) => {
                if let Some(Value::String(path)) = map.get("path") {
                    out.push(PathBuf::from(path));
                }
            }
            _ => {}
        }
    }

    if out.is_empty() {
        bail!("No file paths found in JSON array");
    }

    Ok(out)
}

fn collect_tree_paths(node: &Value, out: &mut Vec<PathBuf>) {
    if let Value::Object(map) = node {
        if let Some(Value::String(path)) = map.get("path") {
            out.push(PathBuf::from(path));
        }

        if let Some(Value::Array(children)) = map.get("children") {
            for child in children {
                collect_tree_paths(child, out);
            }
        }
    }
}

/// Find files matching a specific pattern with specific separator
fn find_files_for_pattern(
    pattern: &str,
    separator: char,
    dir: &PathBuf,
    max_depth: Option<usize>,
) -> Result<Vec<PathBuf>> {
    // Normalize pattern to use the specified separator
    // E.g., "main.command.tree" with sep='_' → "main_command_tree"
    let normalized_pattern = normalize_pattern_for_separator(pattern, separator);

    // Create hierarchical pattern for searching
    // Add ".**" to match all descendants
    let pattern_str = format!("{}{}**", normalized_pattern, separator);
    let hier_pattern = HierarchyPattern::parse_with_separator(&pattern_str, separator)?;

    // Search for files
    let options = SearchOptions {
        root: dir.clone(),
        max_depth,
        ..Default::default()
    };

    let searcher = FileSearcher::new(options);
    let files = searcher.find(&hier_pattern);

    Ok(files)
}

/// Normalize pattern to use specific separator
fn normalize_pattern_for_separator(pattern: &str, target_separator: char) -> String {
    // Replace common separators with target separator
    let mut normalized = pattern.to_string();

    // Replace dots, underscores, dashes, slashes with target
    for source_sep in ['.', '_', '-', '/'] {
        if source_sep != target_separator {
            normalized = normalized.replace(source_sep, &target_separator.to_string());
        }
    }

    normalized
}

/// Normalize a file path's separator to a different character
///
/// Replaces the hierarchy separator in the filename (not directory path)
/// while preserving file extensions.
fn normalize_path_separator(path: &PathBuf, from_sep: char, to_sep: char) -> PathBuf {
    if from_sep == to_sep {
        return path.clone();
    }

    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
        let (base, ext) = filename
            .rsplit_once('.')
            .map(|(b, e)| (b, Some(e)))
            .unwrap_or((filename, None));

        let normalized_base = base.replace(from_sep, &to_sep.to_string());
        let normalized_filename = if let Some(e) = ext {
            format!("{}.{}", normalized_base, e)
        } else {
            normalized_base
        };

        let mut normalized_path = path.clone();
        normalized_path.set_file_name(normalized_filename);
        normalized_path
    } else {
        path.clone()
    }
}

/// Display merged tree
fn display_tree(
    files: &[PathBuf],
    base_pattern: &str,
    separator: char,
    unicode: bool,
    show_count: bool,
    json: bool,
) -> Result<()> {
    // Build hierarchical tree structure
    let tree = HierarchyTree::from_paths_with_separator(base_pattern, files, separator);

    // Display
    if json {
        println!("{}", tree.to_json());
    } else {
        print!("{}", tree.to_string(unicode));

        if show_count {
            let stats = tree.stats();
            println!(
                "\n{} files, {} directories (recursive)",
                stats.total_files, stats.total_dirs
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn load_files_from_json_file_accepts_utf8_bom() -> Result<()> {
        let dir = tempdir()?;
        let json_path = dir.path().join("lane1.json");

        let mut bytes = vec![0xEF, 0xBB, 0xBF];
        bytes.extend_from_slice(br#"["UserService.Game.Load.cs"]"#);
        fs::write(&json_path, bytes)?;

        let files = load_files_from_json_file(&json_path)?;
        assert_eq!(files, vec![PathBuf::from("UserService.Game.Load.cs")]);

        Ok(())
    }
}
