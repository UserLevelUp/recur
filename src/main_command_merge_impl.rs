///! merge command implementation
///!
///! Merges hierarchical results from multiple pattern/separator pairs into unified view.
///! Follows Unix philosophy: explicit composition over automatic conversion.
use anyhow::{bail, Context, Result};
use recur::parser::HierarchyPattern;
use recur::r#trait::strip_utf8_bom;
use recur::search::{FileSearcher, SearchOptions};
use recur::tree::HierarchyTree;
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::io::{stdin, Read};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
struct MergeInputPath {
    path: PathBuf,
    edge_types: Vec<String>,
}

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

    let edge_types_by_display_path: HashMap<String, Vec<String>> = HashMap::new();
    display_tree(
        &tree_files,
        &edge_types_by_display_path,
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
    let mut file_separators: HashMap<PathBuf, char> = HashMap::new();
    let mut file_edge_types: HashMap<PathBuf, Vec<String>> = HashMap::new();

    for (input, separator) in inputs.iter().zip(separators.iter()) {
        let entries = load_entries_from_json_file(input)
            .with_context(|| format!("Failed to read JSON input: {}", input.display()))?;

        for entry in entries {
            if seen.insert(entry.path.clone()) {
                all_files.push(entry.path.clone());
                file_separators.insert(entry.path.clone(), *separator);
                file_edge_types.insert(entry.path.clone(), entry.edge_types);
            } else {
                merge_edge_types(
                    file_edge_types.entry(entry.path.clone()).or_default(),
                    &entry.edge_types,
                );
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

    let (tree_files, edge_types_by_display_path) = build_display_paths_with_edge_types(
        &all_files,
        &file_separators,
        &file_edge_types,
        tree_separator,
        separators[0],
        show_markers,
    );

    display_tree(
        &tree_files,
        &edge_types_by_display_path,
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
    let mut file_separators: HashMap<PathBuf, char> = HashMap::new();
    let mut file_edge_types: HashMap<PathBuf, Vec<String>> = HashMap::new();

    // Read all stdin into a string
    let mut stdin_content = String::new();
    stdin()
        .read_to_string(&mut stdin_content)
        .context("Failed to read from stdin")?;
    let stdin_content = strip_utf8_bom(&stdin_content);

    // Parse multiple JSON objects from stdin stream
    let stream = serde_json::Deserializer::from_str(stdin_content).into_iter::<Value>();
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
        let entries = extract_entries_from_json(&value).with_context(|| {
            format!(
                "Failed to extract paths from JSON object {}",
                source_idx + 1
            )
        })?;

        // Add files with deduplication and provenance tracking
        for entry in entries {
            if seen.insert(entry.path.clone()) {
                all_files.push(entry.path.clone());
                file_separators.insert(entry.path.clone(), separator);
                file_edge_types.insert(entry.path.clone(), entry.edge_types);
            } else {
                merge_edge_types(
                    file_edge_types.entry(entry.path.clone()).or_default(),
                    &entry.edge_types,
                );
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

    let (tree_files, edge_types_by_display_path) = build_display_paths_with_edge_types(
        &all_files,
        &file_separators,
        &file_edge_types,
        tree_separator,
        separators.first().copied().unwrap_or('.'),
        show_markers,
    );

    display_tree(
        &tree_files,
        &edge_types_by_display_path,
        &base_pattern,
        tree_separator,
        unicode,
        show_count,
        json,
    )?;

    Ok(())
}

fn load_entries_from_json_file(path: &PathBuf) -> Result<Vec<MergeInputPath>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Unable to read file: {}", path.display()))?;
    // Windows tools (notably some PowerShell encodings) can prepend UTF-8 BOM.
    // serde_json rejects BOM-prefixed text, so normalize before parsing.
    let content = strip_utf8_bom(&content);
    let value: Value = serde_json::from_str(content)
        .with_context(|| format!("Invalid JSON in: {}", path.display()))?;
    extract_entries_from_json(&value)
}

fn extract_entries_from_json(value: &Value) -> Result<Vec<MergeInputPath>> {
    let mut out = Vec::new();
    collect_entries_from_json(value, &mut out);

    if out.is_empty() {
        bail!("No file paths found in JSON input");
    }

    Ok(out)
}

fn collect_entries_from_json(value: &Value, out: &mut Vec<MergeInputPath>) {
    match value {
        Value::Array(items) => {
            for item in items {
                match item {
                    Value::String(path) => out.push(MergeInputPath {
                        path: PathBuf::from(path),
                        edge_types: Vec::new(),
                    }),
                    Value::Array(_) | Value::Object(_) => collect_entries_from_json(item, out),
                    _ => {}
                }
            }
        }
        Value::Object(map) => {
            if let Some(Value::String(path)) = map.get("path") {
                out.push(MergeInputPath {
                    path: PathBuf::from(path),
                    edge_types: extract_edge_types(map.get("edge_type")),
                });
            }

            for (key, child) in map {
                if key == "path" || key == "edge_type" {
                    continue;
                }

                if matches!(child, Value::Array(_) | Value::Object(_)) {
                    collect_entries_from_json(child, out);
                }
            }
        }
        _ => {}
    }
}

fn extract_edge_types(value: Option<&Value>) -> Vec<String> {
    let mut edge_types: Vec<String> = match value {
        Some(Value::String(edge_type)) => vec![edge_type.to_string()],
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|item| item.as_str().map(|s| s.to_string()))
            .collect(),
        _ => Vec::new(),
    };

    edge_types.sort();
    edge_types.dedup();
    edge_types
}

fn merge_edge_types(existing: &mut Vec<String>, incoming: &[String]) {
    let mut merged: BTreeSet<String> = existing.iter().cloned().collect();
    merged.extend(incoming.iter().cloned());
    *existing = merged.into_iter().collect();
}

fn build_display_paths_with_edge_types(
    all_files: &[PathBuf],
    file_separators: &HashMap<PathBuf, char>,
    file_edge_types: &HashMap<PathBuf, Vec<String>>,
    tree_separator: char,
    default_separator: char,
    show_markers: bool,
) -> (Vec<PathBuf>, HashMap<String, Vec<String>>) {
    let mut tree_files = Vec::with_capacity(all_files.len());
    let mut edge_types_by_display_path: HashMap<String, Vec<String>> = HashMap::new();

    for path in all_files {
        let original_sep = file_separators
            .get(path)
            .copied()
            .unwrap_or(default_separator);
        let mut display_path = normalize_path_separator(path, original_sep, tree_separator);

        if show_markers {
            if let Some(filename) = display_path.file_name() {
                let marked_filename = format!("{} [{}]", filename.to_string_lossy(), original_sep);
                display_path.set_file_name(marked_filename);
            }
        }

        if let Some(edge_types) = file_edge_types.get(path) {
            if !edge_types.is_empty() {
                merge_edge_types(
                    edge_types_by_display_path
                        .entry(display_path.display().to_string())
                        .or_default(),
                    edge_types,
                );
            }
        }

        tree_files.push(display_path);
    }

    (tree_files, edge_types_by_display_path)
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
    edge_types_by_display_path: &HashMap<String, Vec<String>>,
    base_pattern: &str,
    separator: char,
    unicode: bool,
    show_count: bool,
    json: bool,
) -> Result<()> {
    let (tree_input_files, path_aliases) = prepare_tree_input_files(files, base_pattern, separator);

    // Build hierarchical tree structure
    let tree = HierarchyTree::from_paths_with_separator(base_pattern, &tree_input_files, separator);

    // Display
    if json {
        if edge_types_by_display_path.is_empty() && path_aliases.is_empty() {
            println!("{}", tree.to_json());
        } else {
            println!(
                "{}",
                serde_json::to_string_pretty(&tree_to_json_value(
                    &tree,
                    edge_types_by_display_path,
                    &path_aliases
                ))?
            );
        }
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

fn prepare_tree_input_files(
    files: &[PathBuf],
    base_pattern: &str,
    separator: char,
) -> (Vec<PathBuf>, HashMap<String, String>) {
    if files
        .iter()
        .any(|path| file_matches_base_pattern(path, base_pattern))
    {
        return (files.to_vec(), HashMap::new());
    }

    let mut tree_files = Vec::with_capacity(files.len());
    let mut path_aliases = HashMap::new();

    for path in files {
        let original_display_path = path.display().to_string();
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&original_display_path);

        let synthetic_display_path = if base_pattern.is_empty() {
            filename.to_string()
        } else {
            format!("{base_pattern}{separator}{filename}")
        };

        path_aliases.insert(synthetic_display_path.clone(), original_display_path);
        tree_files.push(PathBuf::from(synthetic_display_path));
    }

    (tree_files, path_aliases)
}

fn file_matches_base_pattern(path: &PathBuf, base_pattern: &str) -> bool {
    let Some(filename) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let filename = strip_display_separator_marker(filename);
    let hier_name = filename
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(filename);

    hier_name.starts_with(base_pattern)
}

fn strip_display_separator_marker(filename: &str) -> &str {
    if filename.len() >= 4 && filename.ends_with(']') {
        let marker_start = filename.len() - 4;
        let marker = &filename[marker_start..];
        if marker.starts_with(" [") {
            return &filename[..marker_start];
        }
    }

    filename
}

fn tree_to_json_value(
    tree: &HierarchyTree,
    edge_types_by_display_path: &HashMap<String, Vec<String>>,
    path_aliases: &HashMap<String, String>,
) -> Value {
    let mut node = json!({
        "name": tree.root_name,
        "children": tree.children.iter().map(|child| tree_to_json_value(child, edge_types_by_display_path, path_aliases)).collect::<Vec<_>>(),
    });

    if let Some(path) = tree.file_path.as_ref() {
        if let Some(map) = node.as_object_mut() {
            let tree_path = path.display().to_string();
            let output_path = path_aliases
                .get(&tree_path)
                .cloned()
                .unwrap_or_else(|| tree_path.clone());

            map.insert("path".to_string(), Value::String(output_path.clone()));
            if let Some(edge_types) = edge_types_by_display_path.get(&output_path) {
                if !edge_types.is_empty() {
                    map.insert("edge_type".to_string(), json!(edge_types));
                }
            }
        }
    }

    node
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

        let entries = load_entries_from_json_file(&json_path)?;
        assert_eq!(
            entries,
            vec![MergeInputPath {
                path: PathBuf::from("UserService.Game.Load.cs"),
                edge_types: Vec::new(),
            }]
        );

        Ok(())
    }

    #[test]
    fn stdin_json_stream_accepts_utf8_bom() -> Result<()> {
        let input = "\u{FEFF}[\"UserService.Game.Load.cs\"]";
        let mut stream =
            serde_json::Deserializer::from_str(strip_utf8_bom(input)).into_iter::<Value>();
        let value = stream.next().expect("expected one JSON object")?;
        let entries = extract_entries_from_json(&value)?;
        assert_eq!(
            entries,
            vec![MergeInputPath {
                path: PathBuf::from("UserService.Game.Load.cs"),
                edge_types: Vec::new(),
            }]
        );
        Ok(())
    }

    #[test]
    fn extract_entries_from_trace_id_json_preserves_edge_type() -> Result<()> {
        let value = json!({
            "request": {
                "pattern": "ulu.topic.dot.ownership.create"
            },
            "define": [
                {
                    "path": "src/Topics.cs",
                    "line_number": 4,
                    "line": "const string OwnershipCreate = \"ulu.topic.dot.ownership.create\";",
                    "edge_type": "define"
                }
            ],
            "produce": [
                {
                    "path": "src/Publisher.cs",
                    "line_number": 12,
                    "line": "await bus.PublishAsync(Topics.OwnershipCreate);",
                    "edge_type": "produce"
                }
            ]
        });

        let entries = extract_entries_from_json(&value)?;

        assert_eq!(
            entries,
            vec![
                MergeInputPath {
                    path: PathBuf::from("src/Topics.cs"),
                    edge_types: vec!["define".to_string()],
                },
                MergeInputPath {
                    path: PathBuf::from("src/Publisher.cs"),
                    edge_types: vec!["produce".to_string()],
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn build_display_paths_with_edge_types_merges_normalized_duplicates() {
        let all_files = vec![
            PathBuf::from("main.command.trace.id.readme.md"),
            PathBuf::from("main_command_trace_id_readme.md"),
        ];
        let file_separators = HashMap::from([
            (PathBuf::from("main.command.trace.id.readme.md"), '.'),
            (PathBuf::from("main_command_trace_id_readme.md"), '_'),
        ]);
        let file_edge_types = HashMap::from([
            (
                PathBuf::from("main.command.trace.id.readme.md"),
                vec!["define".to_string()],
            ),
            (
                PathBuf::from("main_command_trace_id_readme.md"),
                vec!["produce".to_string()],
            ),
        ]);

        let (tree_files, edge_types_by_display_path) = build_display_paths_with_edge_types(
            &all_files,
            &file_separators,
            &file_edge_types,
            '.',
            '.',
            false,
        );

        assert_eq!(tree_files.len(), 2);
        assert_eq!(
            edge_types_by_display_path.get("main.command.trace.id.readme.md"),
            Some(&vec!["define".to_string(), "produce".to_string()])
        );
    }

    #[test]
    fn tree_to_json_value_includes_edge_type_on_leaf_nodes() {
        let tree = HierarchyTree::from_paths_with_separator(
            "pipeline.trace-id",
            &[PathBuf::from("pipeline.trace-id.readme.md")],
            '.',
        );
        let edge_types_by_display_path = HashMap::from([(
            "pipeline.trace-id.readme.md".to_string(),
            vec!["define".to_string(), "produce".to_string()],
        )]);
        let path_aliases = HashMap::new();

        let json = tree_to_json_value(&tree, &edge_types_by_display_path, &path_aliases);
        let readme = &json["children"][0];

        assert_eq!(readme["name"], "readme");
        assert_eq!(readme["path"], "pipeline.trace-id.readme.md");
        assert_eq!(readme["edge_type"], json!(["define", "produce"]));
    }

    #[test]
    fn prepare_tree_input_files_roots_unmatched_files_under_base() {
        let files = vec![
            PathBuf::from("DotControlEvents.cs"),
            PathBuf::from("OwnershipCreateSubscriber.cs"),
        ];

        let (tree_files, path_aliases) = prepare_tree_input_files(&files, "pipeline.trace.id", '.');

        assert_eq!(
            tree_files,
            vec![
                PathBuf::from("pipeline.trace.id.DotControlEvents.cs"),
                PathBuf::from("pipeline.trace.id.OwnershipCreateSubscriber.cs"),
            ]
        );
        assert_eq!(
            path_aliases.get("pipeline.trace.id.DotControlEvents.cs"),
            Some(&"DotControlEvents.cs".to_string())
        );
        assert_eq!(
            path_aliases.get("pipeline.trace.id.OwnershipCreateSubscriber.cs"),
            Some(&"OwnershipCreateSubscriber.cs".to_string())
        );
    }

    #[test]
    fn tree_to_json_value_uses_original_path_alias_for_synthetic_roots() {
        let tree = HierarchyTree::from_paths_with_separator(
            "pipeline.trace.id",
            &[PathBuf::from("pipeline.trace.id.DotControlEvents.cs")],
            '.',
        );
        let edge_types_by_display_path = HashMap::from([(
            "DotControlEvents.cs".to_string(),
            vec!["define".to_string()],
        )]);
        let path_aliases = HashMap::from([(
            "pipeline.trace.id.DotControlEvents.cs".to_string(),
            "DotControlEvents.cs".to_string(),
        )]);

        let json = tree_to_json_value(&tree, &edge_types_by_display_path, &path_aliases);
        let file = &json["children"][0];

        assert_eq!(file["name"], "DotControlEvents");
        assert_eq!(file["path"], "DotControlEvents.cs");
        assert_eq!(file["edge_type"], json!(["define"]));
    }
}
