//! Deterministic evidence collection; no provider calls or source execution.
use crate::parser::{HierarchicalName, HierarchyPattern};
use crate::prompt::{error, fingerprint, PromptArgs, Result};
use crate::r#trait::{CliSeparatorPolicy, SeparatorCapable};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

struct Source {
    path: String,
    absolute: PathBuf,
    bytes: Vec<u8>,
    sep: char,
}

pub(crate) fn collect(
    root: &Path,
    args: &PromptArgs,
    separators: &[String],
    kinds: &[String],
) -> Result<Value> {
    let root = root.canonicalize().map_err(|e| error("invalid_input", e))?;
    let discovery = crate::warp_discovery::DiscoveryPolicy::load(&root, false)
        .map_err(|e| error("invalid_registry", e))?;
    let config =
        crate::project_config::load_nearest(&root).map_err(|e| error("invalid_registry", e))?;
    let policy =
        crate::warp_policy::WarpPolicy::load(&root).map_err(|e| error("invalid_registry", e))?;
    let mut diagnostics = BTreeSet::new();
    let mut truncated = false;
    let mut candidates = std::collections::BTreeMap::new();
    if kinds.is_empty() {
        return Ok(json!({"items":[],"truncated":false,"diagnostics":[]}));
    }
    for scan in &discovery.roots {
        let walker = walkdir::WalkDir::new(scan)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                discovery.keep(e)
                    && (e.depth() == 0 || !e.file_name().to_string_lossy().starts_with('.'))
            });
        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => {
                    truncated = true;
                    diagnostics.insert("Some evidence directories could not be read".to_string());
                    continue;
                }
            };
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let seps = if separators.is_empty() {
                vec![config
                    .as_ref()
                    .and_then(|c| c.separator_for_dir(path.parent().unwrap_or(&root)))
                    .unwrap_or('.')]
            } else {
                CliSeparatorPolicy::parse_cli_separators(separators)
            };
            let name = entry.file_name().to_string_lossy();
            let mut matched = None;
            for sep in seps {
                let subject = HierarchicalName::with_separator(name.as_ref(), sep);
                let scope = if args.scope.contains('*') {
                    args.scope.clone()
                } else {
                    format!("{}{sep}**", args.scope)
                };
                let pattern = HierarchyPattern::parse_with_separator(&scope, sep)
                    .map_err(|e| error("invalid_input", e))?;
                if pattern.matches(&subject) {
                    matched = Some(sep);
                    break;
                }
            }
            let Some(sep) = matched else {
                continue;
            };
            let relative = path
                .strip_prefix(&root)
                .map_err(|e| error("invalid_input", e))?
                .to_string_lossy()
                .replace('\\', "/");
            candidates.insert(relative, (path.to_path_buf(), sep));
        }
    }
    let mut sources = Vec::new();
    if candidates.len() > args.max_files {
        truncated = true;
        diagnostics.insert(format!(
            "File budget {} omitted evidence sources",
            args.max_files
        ));
    }
    for (path, (absolute, sep)) in candidates.into_iter().take(args.max_files) {
        if !absolute.canonicalize().is_ok_and(|p| p.starts_with(&root)) {
            truncated = true;
            diagnostics.insert(format!(
                "Omitted evidence outside the requested directory: {path}"
            ));
            continue;
        }
        // Bound reads as well as output. Oversized files are omitted, never silently sliced.
        let read = (|| -> std::io::Result<Vec<u8>> {
            let mut bytes = Vec::new();
            fs::File::open(&absolute)?
                .take(args.max_bytes.min(1_048_576).saturating_add(1) as u64)
                .read_to_end(&mut bytes)?;
            Ok(bytes)
        })();
        match read {
            Ok(bytes)
                if bytes.len() <= args.max_bytes.min(1_048_576)
                    && std::str::from_utf8(&bytes).is_ok() =>
            {
                sources.push(Source {
                    path,
                    absolute,
                    bytes,
                    sep,
                })
            }
            Ok(_) => {
                truncated = true;
                diagnostics.insert(format!("Omitted oversized or non-UTF-8 evidence: {path}"));
            }
            Err(_) => {
                truncated = true;
                diagnostics.insert(format!("Could not read evidence: {path}"));
            }
        }
    }
    if sources.is_empty() {
        diagnostics.insert("No readable evidence matched the requested scope".into());
    }
    let mut items = Vec::new();
    for source in &sources {
        let text = std::str::from_utf8(&source.bytes).unwrap();
        for kind in kinds {
            let data = match kind.as_str() {
                "files" => json!({"text":text}),
                "hierarchy" => {
                    let tree = crate::tree::HierarchyTree::from_paths_with_separator(
                        "",
                        &[PathBuf::from(&source.path)],
                        source.sep,
                    );
                    json!({"separator":source.sep.to_string(),"tree":serde_json::from_str::<Value>(&tree.to_json()).map_err(|e| error("invalid_input",e))?})
                }
                "trace-id" => json!({"roles":crate::r#trait::trace_id::explicit_roles(text)}),
                "eventness" => {
                    json!({"policy":policy,"separator":source.sep.to_string(),"state":policy.state(source.absolute.file_name().unwrap().to_string_lossy().as_ref())})
                }
                "warp" if source.path.ends_with(".warp-map.json") => {
                    let parent = source.absolute.parent().unwrap();
                    let layers: Vec<_> = sources
                        .iter()
                        .filter(|s| {
                            s.path.ends_with(".warp-layer.json")
                                && s.absolute.parent() == Some(parent)
                        })
                        .map(|s| {
                            (
                                s.absolute
                                    .file_name()
                                    .unwrap()
                                    .to_string_lossy()
                                    .into_owned(),
                                s.bytes.clone(),
                            )
                        })
                        .collect();
                    // Never project completeness from a subset of the selected sources.
                    let raw: Value = match serde_json::from_slice(&source.bytes) {
                        Ok(v) => v,
                        Err(e) => {
                            diagnostics.insert(format!("Invalid Warp map {}: {e}", source.path));
                            continue;
                        }
                    };
                    if truncated {
                        json!({"map":raw,"projection":null,"diagnostic":"Projection omitted because context collection is incomplete"})
                    } else {
                        match crate::warp_query::project_snapshot(
                            parent,
                            source
                                .absolute
                                .file_name()
                                .unwrap()
                                .to_string_lossy()
                                .as_ref(),
                            &source.bytes,
                            &layers,
                        ) {
                            Ok(projection) => json!({"map":raw,"projection":projection}),
                            Err(e) => {
                                diagnostics.insert(format!(
                                    "Warp projection unavailable for {}: {e}",
                                    source.path
                                ));
                                json!({"map":raw,"projection":null})
                            }
                        }
                    }
                }
                "warp" if source.path.ends_with(".warp-layer.json") => {
                    match serde_json::from_slice::<Value>(&source.bytes) {
                        Ok(v) => v,
                        Err(e) => {
                            diagnostics.insert(format!("Invalid Warp layer {}: {e}", source.path));
                            continue;
                        }
                    }
                }
                _ => continue,
            };
            let item = json!({"kind":kind,"path":source.path,"fingerprint":fingerprint(&source.bytes),"data":data});
            items.push(item);
            if serde_json::to_vec(&items)
                .map_err(|e| error("invalid_input", e))?
                .len()
                > args.max_bytes
            {
                items.pop();
                truncated = true;
                diagnostics.insert(format!(
                    "Byte budget {} omitted context items",
                    args.max_bytes
                ));
            }
        }
    }
    Ok(json!({"items":items,"truncated":truncated,"diagnostics":diagnostics}))
}
