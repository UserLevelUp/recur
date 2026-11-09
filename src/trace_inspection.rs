//! Explicit scoped token inventories and bounded source excerpts.
//! defines: main.improvement.34.implementation.trace.context

use super::{matches, scan, token_ids};
use crate::parser::HierarchyPattern;
use crate::search::SearchOptions;
use anyhow::{ensure, Result};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub struct InspectionOptions {
    pub context: Option<usize>,
    pub selectors: Vec<HierarchyPattern>,
    pub max_results: usize,
    pub max_output_bytes: usize,
}

#[derive(Serialize)]
struct OutputCoverage {
    complete: bool,
    max_results: usize,
    max_output_bytes: usize,
    returned_bytes: usize,
    omissions: BTreeMap<&'static str, usize>,
}

impl OutputCoverage {
    fn omit(&mut self, reason: &'static str) {
        self.complete = false;
        *self.omissions.entry(reason).or_default() += 1;
    }

    fn fits(&mut self, old_bytes: usize, new_bytes: usize) -> bool {
        if self.returned_bytes - old_bytes + new_bytes > self.max_output_bytes {
            self.omit("output_byte_budget");
            return false;
        }
        self.returned_bytes = self.returned_bytes - old_bytes + new_bytes;
        true
    }
}

#[derive(Clone, Serialize)]
struct InventoryEntry {
    id: String,
    namespace: String,
    files: BTreeSet<String>,
    file_count: usize,
    occurrences: usize,
}

#[derive(Serialize)]
struct Excerpt {
    path: String,
    start_line: usize,
    end_line: usize,
    ids: BTreeSet<String>,
    lines: Vec<String>,
}

struct Window<'a> {
    start: usize,
    end: usize,
    ids: BTreeSet<&'a str>,
}

fn serialized_len(value: &impl Serialize) -> usize {
    // Only concrete string/number/list records are serialized here.
    serde_json::to_vec(value)
        .expect("inspection records serialize")
        .len()
}

fn append_excerpt(
    excerpts: &mut Vec<Excerpt>,
    output: &mut OutputCoverage,
    path: &str,
    lines: &[&str],
    window: Window<'_>,
) {
    if excerpts.len() == output.max_results {
        output.omit("result_budget");
        return;
    }
    let excerpt = Excerpt {
        path: path.into(),
        start_line: window.start + 1,
        end_line: window.end + 1,
        ids: window.ids.into_iter().map(str::to_owned).collect(),
        lines: lines[window.start..=window.end]
            .iter()
            .map(|s| (*s).into())
            .collect(),
    };
    if output.fits(0, serialized_len(&excerpt)) {
        excerpts.push(excerpt);
    }
}

pub fn inspect(
    options: &SearchOptions,
    pattern: &HierarchyPattern,
    scope: &HierarchyPattern,
    entries: usize,
    bytes: usize,
    view: &InspectionOptions,
) -> Result<Value> {
    ensure!(
        (1..=10_000).contains(&view.max_results),
        "--max-results must be in 1..=10000"
    );
    ensure!(
        (1..=1024 * 1024).contains(&view.max_output_bytes),
        "--max-output-bytes must be in 1..=1048576"
    );
    ensure!(
        view.context.is_none_or(|n| n <= 20),
        "--context must be in 0..=20"
    );
    ensure!(
        view.selectors.len() <= 64,
        "at most 64 --select patterns are supported"
    );
    let selected = |id: &str| {
        matches(pattern, id)
            && (view.selectors.is_empty() || view.selectors.iter().any(|p| matches(p, id)))
    };
    let mut output = OutputCoverage {
        complete: true,
        max_results: view.max_results,
        max_output_bytes: view.max_output_bytes,
        returned_bytes: 0,
        omissions: BTreeMap::new(),
    };
    let mut inventory = BTreeMap::<String, InventoryEntry>::new();
    let mut excerpts = Vec::new();
    let coverage = scan(options, Some(scope), false, entries, bytes, |path, text| {
        if let Some(radius) = view.context {
            let lines = text.lines().collect::<Vec<_>>();
            let mut current: Option<Window<'_>> = None;
            for (line, body) in lines.iter().enumerate() {
                let ids = token_ids(body, pattern.separator)
                    .filter(|id| selected(id))
                    .collect::<BTreeSet<_>>();
                if ids.is_empty() {
                    continue;
                }
                let start = line.saturating_sub(radius);
                let end = (line + radius).min(lines.len() - 1);
                if let Some(window) = &mut current {
                    if start <= window.end + 1 {
                        window.end = end;
                        window.ids.extend(ids);
                        continue;
                    }
                }
                if let Some(window) = current.take() {
                    append_excerpt(&mut excerpts, &mut output, &path, &lines, window);
                }
                current = Some(Window { start, end, ids });
            }
            if let Some(window) = current {
                append_excerpt(&mut excerpts, &mut output, &path, &lines, window);
            }
        } else {
            // Counts are accumulated once per file, without retaining every site.
            let mut counts = BTreeMap::<&str, usize>::new();
            for id in token_ids(text, pattern.separator).filter(|id| selected(id)) {
                if let Some(count) = counts.get_mut(id) {
                    *count += 1;
                } else if counts.len() < view.max_results {
                    counts.insert(id, 1);
                } else {
                    output.omit("result_budget");
                }
            }
            for (id, count) in counts {
                let old = inventory.get(id);
                if old.is_none() && inventory.len() == view.max_results {
                    output.omit("result_budget");
                    continue;
                }
                let old_bytes = old.map_or(0, serialized_len);
                let mut candidate = old.cloned().unwrap_or_else(|| InventoryEntry {
                    id: id.into(),
                    namespace: id
                        .rsplit_once(pattern.separator)
                        .map_or("", |(parent, _)| parent)
                        .into(),
                    files: BTreeSet::new(),
                    file_count: 0,
                    occurrences: 0,
                });
                candidate.files.insert(path.clone());
                candidate.file_count = candidate.files.len();
                candidate.occurrences += count;
                if output.fits(old_bytes, serialized_len(&candidate)) {
                    inventory.insert(id.into(), candidate);
                }
            }
        }
    })?;
    let complete = coverage.complete && output.complete;
    let mut report = json!({
        "schema": if view.context.is_some() {"recur-trace-id-context-v1"} else {"recur-trace-id-inventory-v1"},
        "provenance":"token-occurrences", "complete":complete,
        "request":{"pattern":pattern.raw,"scope":scope.raw,"root":options.root,
            "selectors":view.selectors.iter().map(|p| &p.raw).collect::<Vec<_>>(),
            "ignore_case":pattern.case_insensitive,"separator":pattern.separator,"context":view.context},
        "coverage":coverage,"output":output,
    });
    if view.context.is_some() {
        report["excerpts"] = serde_json::to_value(excerpts)?;
    } else {
        report["ids"] = serde_json::to_value(inventory.into_values().collect::<Vec<_>>())?;
    }
    Ok(report)
}
