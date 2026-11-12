//! Implementation of the trace-id command (hierarchical identifier flow lane).
//!
//! This module maps to hierarchical name: main.command.trace-id.impl

use clap::{error::ErrorKind, Parser};
use recur::output::TerminalFormatter;
use recur::parser::HierarchyPattern;
use recur::project_config;
use recur::r#trait::{
    apply_content_search_policy, read_resolved_paths_from_stdin, resolve_depth_budget_policy,
    resolve_trace_id_policy, CliSeparatorPolicy, DepthBudgetMode, SeparatorCapable, TraceIdPolicy,
    TraversalBudgetCapable,
};
use recur::search::{FileSearcher, IdentifierSearcher, SearchOptions, SearchResult};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

const TRACE_ID_MAX_DEPTH: usize = 5;
const TRACE_ID_RUNS_DIR: &str = ".recur/trace-id/runs";
const TRACE_ID_RUN_MANIFEST_VERSION: u32 = 1;

struct TraceIdCommand;

impl TraversalBudgetCapable for TraceIdCommand {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TraceIdFormat {
    Summary,
    Full,
}

impl TraceIdFormat {
    fn parse(value: &str) -> anyhow::Result<Self> {
        match value.to_lowercase().as_str() {
            "summary" => Ok(Self::Summary),
            "full" => Ok(Self::Full),
            _ => anyhow::bail!("Invalid --format '{}'. Must be summary or full", value),
        }
    }
}

#[derive(Debug, Clone)]
struct ScopedLine {
    path: PathBuf,
    line_number: usize,
    line: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TraceIdSite {
    path: String,
    line_number: usize,
    line: String,
    edge_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TraceIdRecord {
    identifier: String,
    define: Vec<TraceIdSite>,
    produce: Vec<TraceIdSite>,
    consume: Vec<TraceIdSite>,
    trigger: Vec<TraceIdSite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TraceIdRequest {
    pattern: String,
    scope: String,
    dir: String,
    ext: Option<String>,
    format: String,
    stdin: bool,
    ignore_case: bool,
    separator: String,
    depth_requested: usize,
    depth_effective: usize,
    depth_cap: usize,
    depth_guard: String,
    force: bool,
    json: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TraceIdPayload {
    request: TraceIdRequest,
    identifier: String,
    define: Vec<TraceIdSite>,
    produce: Vec<TraceIdSite>,
    consume: Vec<TraceIdSite>,
    trigger: Vec<TraceIdSite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum TraceIdSavedJson {
    Single(TraceIdPayload),
    Multi(Vec<TraceIdRecord>),
}

impl TraceIdSavedJson {
    fn into_records(self) -> Vec<TraceIdRecord> {
        match self {
            Self::Single(payload) => vec![TraceIdRecord {
                identifier: payload.identifier,
                define: payload.define,
                produce: payload.produce,
                consume: payload.consume,
                trigger: payload.trigger,
            }],
            Self::Multi(records) => records,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TraceIdRunRequestKey {
    pattern: String,
    scope: String,
    dir: String,
    ext: Option<String>,
    stdin: bool,
    ignore_case: bool,
    separator: String,
    depth_requested: usize,
    depth_effective: usize,
    depth_cap: usize,
    depth_guard: String,
    force: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TraceIdRunManifest {
    version: u32,
    name: String,
    saved_at_unix_s: u64,
    project_root: String,
    latest_json_path: String,
    request: TraceIdRunRequestKey,
    request_fingerprint: String,
    config_fingerprint: Option<String>,
    file_fingerprint: String,
    file_count: usize,
}

#[derive(Debug, Clone)]
struct TraceIdRunPaths {
    name: String,
    project_root: PathBuf,
    run_dir: PathBuf,
    manifest_path: PathBuf,
    latest_json_path: PathBuf,
    config_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
struct TraceIdRunState {
    paths: TraceIdRunPaths,
    request: TraceIdRunRequestKey,
    request_fingerprint: String,
    config_fingerprint: Option<String>,
    file_fingerprint: String,
    file_count: usize,
}

#[derive(Debug, Clone, Serialize)]
struct TraceIdRunCheck {
    name: String,
    status: String,
    reasons: Vec<String>,
    manifest_path: String,
    latest_json_path: String,
    file_count: usize,
    saved_at_unix_s: Option<u64>,
}

#[derive(Parser, Debug)]
#[command(name = "trace-id")]
#[command(about = "Find trace-ID paths, or inspect scoped relationships")]
struct TraceIdCli {
    /// Hierarchical identifier pattern (recursive)
    pattern: String,

    /// Inspect relationships within this file hierarchy; omit for paths only
    #[arg(short, long)]
    scope: Option<String>,

    /// Read explicit trace metadata and revision pins (requires --scope)
    #[arg(long, requires = "scope")]
    declared: bool,

    /// Require at most one explicit defines record per ID in this scope
    #[arg(long, requires = "declared")]
    unique_defines: bool,

    /// List distinct observed IDs and their files (requires --scope)
    #[arg(long, requires = "scope", conflicts_with_all = ["declared", "context"])]
    list_ids: bool,

    /// Show this many neighboring lines around selected IDs, 0..=20 (requires --scope)
    #[arg(long, requires = "scope", conflicts_with = "declared")]
    context: Option<usize>,

    /// Select an exact ID or family within PATTERN; repeat to form a union
    #[arg(long, value_name = "ID_OR_PATTERN")]
    select: Vec<String>,

    /// Maximum ID entries or excerpts in an explicit inspection view (default 200)
    #[arg(long)]
    max_results: Option<usize>,

    /// Maximum serialized result-item bytes in an inspection view (default 32768)
    #[arg(long)]
    max_output_bytes: Option<usize>,

    /// Discovery entry limit for path-only, declared and inspection queries
    #[arg(long)]
    max_entries: Option<usize>,

    /// Total byte limit for path-only, declared and inspection queries
    #[arg(long)]
    max_bytes: Option<usize>,

    /// Root directory to search
    #[arg(short = 'd', long, default_value = ".")]
    dir: PathBuf,

    /// File extensions to include
    #[arg(short, long)]
    ext: Option<String>,

    /// Output format: summary or full
    #[arg(long)]
    format: Option<String>,

    /// Read file paths from stdin instead of searching filesystem
    #[arg(long)]
    stdin: bool,

    /// Traversal depth budget for flow expansion
    #[arg(long)]
    depth: Option<usize>,

    /// Depth guardrail mode override: hard-fail or clamp
    #[arg(long)]
    depth_guard: Option<String>,

    /// Bypass trace-id depth cap safety check
    #[arg(long)]
    force: bool,

    /// Case-insensitive search
    #[arg(short, long)]
    ignore_case: bool,

    /// Hierarchy separator character (default: '.')
    #[arg(long, value_name = "CHAR")]
    sep: Vec<String>,

    /// Output as JSON
    #[arg(long)]
    json: bool,

    /// Save this trace result under `.recur/trace-id/runs/<name>/`
    #[arg(long, requires = "run_name")]
    save_run: bool,

    /// Reuse a saved run when its query/config/input files are still fresh
    #[arg(long, requires = "run_name")]
    reuse_if_fresh: bool,

    /// Check whether a saved run is fresh without executing trace-id
    #[arg(long, requires = "run_name")]
    check_run: bool,

    /// Stable name for saved trace-id run artifacts
    #[arg(long)]
    run_name: Option<String>,

    /// Use color in output
    #[arg(long, default_value = "true")]
    color: bool,
}

pub fn maybe_execute_from_args() -> Option<anyhow::Result<()>> {
    let raw_args: Vec<String> = std::env::args().collect();
    if raw_args.get(1).map(String::as_str) != Some("trace-id") {
        return None;
    }

    let mut trace_args = Vec::with_capacity(raw_args.len().saturating_sub(1));
    trace_args.push("trace-id".to_string());
    trace_args.extend(raw_args.into_iter().skip(2));

    let cli = match TraceIdCli::try_parse_from(trace_args) {
        Ok(cli) => cli,
        Err(err) => {
            let kind = err.kind();
            let _ = err.print();
            let exit_code = match kind {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => 0,
                _ => 2,
            };
            process::exit(exit_code);
        }
    };

    let separator = CliSeparatorPolicy::parse_cli_separators(&cli.sep)
        .last()
        .copied()
        .unwrap_or('.');

    if cli.scope.is_none() || cli.declared || cli.list_ids || cli.context.is_some() {
        return Some(execute_bounded(&cli, separator));
    }
    if cli.max_entries.is_some()
        || cli.max_bytes.is_some()
        || !cli.select.is_empty()
        || cli.max_results.is_some()
        || cli.max_output_bytes.is_some()
    {
        return Some(Err(anyhow::anyhow!(
            "bounded search and selection options require path-only, --declared, --list-ids or --context mode; selection/output limits require an inspection view"
        )));
    }
    Some(execute(
        cli.pattern,
        cli.scope.expect("legacy mode has scope"),
        cli.dir,
        cli.ext,
        cli.format.unwrap_or_else(|| "summary".into()),
        cli.stdin,
        cli.depth.unwrap_or(2),
        cli.depth_guard,
        cli.force,
        cli.ignore_case,
        separator,
        cli.json,
        cli.save_run,
        cli.reuse_if_fresh,
        cli.check_run,
        cli.run_name,
        cli.color,
    ))
}

fn execute_bounded(cli: &TraceIdCli, separator: char) -> anyhow::Result<()> {
    use recur::trace_relationships::{
        declared, hunt, inspect, InspectionOptions, DEFAULT_BYTES, DEFAULT_ENTRIES,
    };
    let inspection = cli.list_ids || cli.context.is_some();
    anyhow::ensure!(inspection || (cli.select.is_empty() && cli.max_results.is_none() && cli.max_output_bytes.is_none()),
        "--select, --max-results and --max-output-bytes require --list-ids or --context with --scope");
    if cli.stdin
        || cli.format.is_some()
        || cli.depth.is_some()
        || cli.depth_guard.is_some()
        || cli.force
        || cli.run_name.is_some()
        || cli.save_run
        || cli.reuse_if_fresh
        || cli.check_run
    {
        if cli.scope.is_none() {
            anyhow::bail!(
                "--stdin, --format, --depth, and saved-run options require --scope <SCOPE> (for example: --scope \"**\")"
            );
        }
        anyhow::bail!(
            "--stdin, --format, --depth, and saved-run options apply to role queries (--scope <SCOPE> without --declared, --list-ids, or --context)"
        );
    }
    anyhow::ensure!(
        !cli.declared || separator == '.',
        "--declared uses dotted trace IDs"
    );
    let parse = |text: &str| -> anyhow::Result<HierarchyPattern> {
        anyhow::ensure!(
            !text.trim().is_empty(),
            "pattern or --scope cannot be empty"
        );
        let value = HierarchyPattern::parse_with_separator(text, separator)?;
        Ok(if cli.ignore_case {
            value.case_insensitive()
        } else {
            value
        })
    };
    let pattern = parse(&cli.pattern)?;
    let mut options = SearchOptions {
        root: cli.dir.clone(),
        ..Default::default()
    };
    if let Some(ext) = &cli.ext {
        options.extensions = ext.split(',').map(|s| s.trim().to_string()).collect();
    }
    apply_content_search_policy(&mut options, &cli.dir)?;
    let max_entries = cli.max_entries.unwrap_or(DEFAULT_ENTRIES);
    let max_bytes = cli.max_bytes.unwrap_or(DEFAULT_BYTES);
    let report = if inspection {
        let scope = parse(cli.scope.as_deref().expect("inspection requires scope"))?;
        let selectors = cli
            .select
            .iter()
            .map(|s| parse(s))
            .collect::<anyhow::Result<Vec<_>>>()?;
        inspect(
            &options,
            &pattern,
            &scope,
            max_entries,
            max_bytes,
            &InspectionOptions {
                context: cli.context,
                selectors,
                max_results: cli.max_results.unwrap_or(200),
                max_output_bytes: cli.max_output_bytes.unwrap_or(32 * 1024),
            },
        )?
    } else if let Some(scope) = cli.scope.as_deref() {
        declared(
            &options,
            &pattern,
            &parse(scope)?,
            max_entries,
            max_bytes,
            cli.unique_defines,
        )?
    } else {
        hunt(&options, &pattern, max_entries, max_bytes)?
    };
    if cli.json || cli.declared {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else if inspection {
        if let Some(ids) = report["ids"].as_array() {
            for id in ids {
                println!(
                    "{} ({} files, {} occurrences)",
                    id["id"].as_str().unwrap_or(""),
                    id["file_count"],
                    id["occurrences"]
                );
                for file in id["files"].as_array().into_iter().flatten() {
                    println!("  {}", file.as_str().unwrap_or(""));
                }
            }
        }
        if let Some(excerpts) = report["excerpts"].as_array() {
            for excerpt in excerpts {
                println!(
                    "{}:{}-{}",
                    excerpt["path"].as_str().unwrap_or(""),
                    excerpt["start_line"],
                    excerpt["end_line"]
                );
                let start = excerpt["start_line"].as_u64().unwrap_or(1);
                for (i, line) in excerpt["lines"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .enumerate()
                {
                    println!("  {} | {}", start + i as u64, line.as_str().unwrap_or(""));
                }
            }
        }
        if report["complete"] == false {
            eprintln!(
                "Trace-ID inspection incomplete: scan={}, output={}",
                report["coverage"]["omissions"], report["output"]["omissions"]
            );
        }
    } else {
        for folder in report["folders"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str())
        {
            println!("{folder}/");
        }
        for file in report["files"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str())
        {
            println!("{file}");
        }
        if report["coverage"]["complete"] == false {
            eprintln!(
                "Trace-ID discovery incomplete: {}",
                report["coverage"]["omissions"]
            );
        }
    }
    if inspection
        && report["complete"] == true
        && report[if cli.list_ids { "ids" } else { "excerpts" }]
            .as_array()
            .is_some_and(Vec::is_empty)
    {
        process::exit(1);
    }
    if !inspection
        && !cli.declared
        && report["coverage"]["complete"] == true
        && report["files"].as_array().is_some_and(Vec::is_empty)
    {
        process::exit(1);
    }
    Ok(())
}

fn depth_budget_mode_as_str(mode: DepthBudgetMode) -> &'static str {
    match mode {
        DepthBudgetMode::HardFail => "hard-fail",
        DepthBudgetMode::Clamp => "clamp",
    }
}

pub fn execute(
    pattern: String,
    scope: String,
    dir: PathBuf,
    ext: Option<String>,
    format: String,
    stdin: bool,
    depth: usize,
    depth_guard: Option<String>,
    force: bool,
    ignore_case: bool,
    separator: char,
    json_output: bool,
    save_run: bool,
    reuse_if_fresh: bool,
    check_run: bool,
    run_name: Option<String>,
    color: bool,
) -> anyhow::Result<()> {
    if scope.trim().is_empty() {
        anyhow::bail!("--scope cannot be empty");
    }
    if check_run && (save_run || reuse_if_fresh) {
        anyhow::bail!("--check-run cannot be combined with --save-run or --reuse-if-fresh");
    }
    if run_name.is_some() && !save_run && !reuse_if_fresh && !check_run {
        anyhow::bail!("--run-name requires one of --save-run, --reuse-if-fresh, or --check-run");
    }

    let output_format = TraceIdFormat::parse(&format)?;
    let depth_policy =
        resolve_depth_budget_policy(&dir, depth_guard.as_deref(), TRACE_ID_MAX_DEPTH)?;
    let depth_budget = TraceIdCommand::enforce_depth_budget(
        depth,
        depth_policy.max_depth,
        force,
        depth_policy.guard_mode,
    )?;
    if depth_budget.clamped {
        eprintln!(
            "Warning: requested depth {} exceeds max depth {}; clamping to {}. Use --force to bypass.",
            depth_budget.requested, depth_policy.max_depth, depth_budget.effective
        );
    }
    let trace_policy = resolve_trace_id_policy(&dir)?;

    let scope_pattern = HierarchyPattern::parse_with_separator(&scope, separator)?;
    let scope_pattern = if ignore_case {
        scope_pattern.case_insensitive()
    } else {
        scope_pattern
    };

    let id_pattern = HierarchyPattern::parse_with_separator(&pattern, separator)?;
    let id_pattern = if ignore_case {
        id_pattern.case_insensitive()
    } else {
        id_pattern
    };

    let mut search_options = SearchOptions {
        root: dir.clone(),
        case_insensitive: ignore_case,
        ..Default::default()
    };

    if let Some(ext_str) = ext.as_deref() {
        search_options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    apply_content_search_policy(&mut search_options, &dir)?;

    if stdin {
        search_options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    // Scope filtering is applied before identifier scanning so --scope, --ext, and --stdin
    // all compose into one deterministic file list.
    let file_searcher = FileSearcher::new(search_options.clone());
    let scoped_files = file_searcher.find(&scope_pattern);
    search_options.input_files = Some(scoped_files.clone());

    let run_request = TraceIdRunRequestKey {
        pattern: pattern.clone(),
        scope: scope.clone(),
        dir: dir.display().to_string(),
        ext: ext.clone(),
        stdin,
        ignore_case,
        separator: separator.to_string(),
        depth_requested: depth_budget.requested,
        depth_effective: depth_budget.effective,
        depth_cap: depth_policy.max_depth,
        depth_guard: depth_budget_mode_as_str(depth_policy.guard_mode).to_string(),
        force,
    };
    let run_state = run_name
        .as_deref()
        .map(|name| build_run_state(name, &dir, run_request, &scoped_files))
        .transpose()?;

    if let Some(run_state) = run_state.as_ref() {
        let status = evaluate_run_status(run_state)?;
        if check_run {
            print_run_status(&status, json_output)?;
            return Ok(());
        }

        if reuse_if_fresh && status.status == "fresh" {
            emit_saved_run(
                &run_state.paths.latest_json_path,
                &pattern,
                output_format,
                json_output,
                color,
            )?;
            return Ok(());
        }

        if reuse_if_fresh && status.status != "fresh" && !status.reasons.is_empty() {
            eprintln!(
                "trace-id run '{}' is {}; recomputing ({})",
                run_state.paths.name,
                status.status,
                status.reasons.join(", ")
            );
        }
    }

    let searcher = IdentifierSearcher::new(search_options);
    let results = searcher.search(&id_pattern);

    if results.is_empty() {
        if json_output {
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({
                    "request": {
                        "pattern": pattern,
                        "scope": scope,
                        "dir": dir.display().to_string(),
                        "ext": ext,
                        "format": format,
                        "stdin": stdin,
                        "ignore_case": ignore_case,
                        "separator": separator.to_string(),
                        "depth_requested": depth_budget.requested,
                        "depth_effective": depth_budget.effective,
                        "depth_cap": depth_policy.max_depth,
                        "depth_guard": depth_budget_mode_as_str(depth_policy.guard_mode),
                        "force": force,
                    },
                    "identifier": pattern,
                    "define": [],
                    "produce": [],
                    "consume": [],
                    "trigger": [],
                }))?
            );
        }
        process::exit(1);
    }

    let mut grouped: BTreeMap<String, Vec<SearchResult>> = BTreeMap::new();
    for result in &results {
        if let Some(identifier) = matched_identifier(result) {
            grouped.entry(identifier).or_default().push(result.clone());
        }
    }

    let scoped_lines = if depth_budget.effective > 0 {
        collect_scoped_lines(scoped_files, &searcher.options)
    } else {
        Vec::new()
    };

    let is_glob = contains_glob(&pattern);
    let mut records = Vec::new();
    if is_glob {
        for (identifier, define_hits) in grouped {
            records.push(build_record(
                &identifier,
                &define_hits,
                &scoped_lines,
                ignore_case,
                depth_budget.effective,
                &trace_policy,
            ));
        }
    } else {
        let exact_key = grouped
            .keys()
            .find(|k| eq_with_case(k, &pattern, ignore_case))
            .cloned()
            .unwrap_or_else(|| pattern.clone());

        let define_hits = grouped.get(&exact_key).cloned().unwrap_or_default();
        records.push(build_record(
            &exact_key,
            &define_hits,
            &scoped_lines,
            ignore_case,
            depth_budget.effective,
            &trace_policy,
        ));
    }

    let request = TraceIdRequest {
        pattern: pattern.clone(),
        scope: scope.clone(),
        dir: dir.display().to_string(),
        ext: ext.clone(),
        format: format.clone(),
        stdin,
        ignore_case,
        separator: separator.to_string(),
        depth_requested: depth_budget.requested,
        depth_effective: depth_budget.effective,
        depth_cap: depth_policy.max_depth,
        depth_guard: depth_budget_mode_as_str(depth_policy.guard_mode).to_string(),
        force,
        json: true,
    };
    let saved_json = render_trace_id_json(&records, &request, is_glob)?;

    if let Some(run_state) = run_state.as_ref() {
        if save_run {
            save_run_artifacts(run_state, &saved_json)?;
            eprintln!(
                "saved trace-id run '{}' -> {}",
                run_state.paths.name,
                run_state.paths.latest_json_path.display()
            );
        }
    }

    if json_output {
        println!("{}", saved_json);
    } else {
        print_terminal(&records, &pattern, output_format, color);
    }

    Ok(())
}

fn render_trace_id_json(
    records: &[TraceIdRecord],
    request: &TraceIdRequest,
    is_glob: bool,
) -> anyhow::Result<String> {
    if is_glob {
        Ok(serde_json::to_string_pretty(records)?)
    } else if let Some(record) = records.first() {
        let payload = TraceIdPayload {
            request: request.clone(),
            identifier: record.identifier.clone(),
            define: record.define.clone(),
            produce: record.produce.clone(),
            consume: record.consume.clone(),
            trigger: record.trigger.clone(),
        };
        Ok(serde_json::to_string_pretty(&payload)?)
    } else {
        Ok("[]".to_string())
    }
}

fn print_terminal(records: &[TraceIdRecord], pattern: &str, format: TraceIdFormat, color: bool) {
    let files: BTreeSet<String> = records
        .iter()
        .flat_map(|r| {
            r.define
                .iter()
                .chain(r.produce.iter())
                .chain(r.consume.iter())
                .chain(r.trigger.iter())
        })
        .map(|r| r.path.clone())
        .collect();

    let total_sites = records
        .iter()
        .map(|r| r.define.len() + r.produce.len() + r.consume.len() + r.trigger.len())
        .sum::<usize>();

    println!(
        "trace-id '{}' -> {} role sites in {} files",
        pattern,
        total_sites,
        files.len()
    );

    if format == TraceIdFormat::Full {
        let grouped = group_sites_for_full_output(records);
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_search_results(&grouped);
    }
}

fn group_sites_for_full_output(records: &[TraceIdRecord]) -> Vec<SearchResult> {
    let mut grouped: BTreeMap<(PathBuf, usize), (Vec<&'static str>, String)> = BTreeMap::new();
    for r in records {
        for (role, sites) in [
            ("define", &r.define),
            ("produce", &r.produce),
            ("consume", &r.consume),
            ("trigger", &r.trigger),
        ] {
            for site in sites {
                let entry = grouped
                    .entry((PathBuf::from(&site.path), site.line_number))
                    .or_insert_with(|| (Vec::new(), site.line.clone()));
                if !entry.0.contains(&role) {
                    entry.0.push(role);
                }
            }
        }
    }

    grouped
        .into_iter()
        .map(|((path, line_number), (roles, line))| SearchResult {
            path,
            line_number,
            line: format!("[{}] {}", roles.join(", "), line),
            match_start: 0,
            match_end: 0,
            context_before: Vec::new(),
            context_after: Vec::new(),
        })
        .collect()
}

fn build_record(
    identifier: &str,
    define_hits: &[SearchResult],
    scoped_lines: &[ScopedLine],
    ignore_case: bool,
    effective_depth: usize,
    trace_policy: &TraceIdPolicy,
) -> TraceIdRecord {
    let raw_define = dedupe_sites(
        define_hits
            .iter()
            .map(|r| result_to_site(r, "define"))
            .collect::<Vec<TraceIdSite>>(),
    );

    if effective_depth == 0 {
        return TraceIdRecord {
            identifier: identifier.to_string(),
            define: raw_define,
            produce: Vec::new(),
            consume: Vec::new(),
            trigger: Vec::new(),
        };
    }

    let mut symbols = BTreeSet::new();
    for d in &raw_define {
        if let Some(name) =
            extract_symbol_name_from_definition(&d.line, identifier, ignore_case, trace_policy)
        {
            symbols.insert(name);
        }
    }

    let mut produce = Vec::new();
    let mut consume = Vec::new();
    let mut trigger = Vec::new();

    for line in scoped_lines {
        if !line_matches_identifier_or_symbol(&line.line, identifier, &symbols, ignore_case) {
            continue;
        }

        if is_produce_line_for(&line.line, identifier, ignore_case, trace_policy) {
            produce.push(TraceIdSite {
                path: line.path.display().to_string(),
                line_number: line.line_number,
                line: line.line.clone(),
                edge_type: "produce".to_string(),
            });
        }
        if is_consume_line_for(&line.line, identifier, ignore_case, trace_policy)
            && line_matches_identifier_or_symbol_token(
                &line.line,
                identifier,
                &symbols,
                ignore_case,
            )
            && !is_structural_consume_noise(&line.line)
        {
            consume.push(TraceIdSite {
                path: line.path.display().to_string(),
                line_number: line.line_number,
                line: line.line.clone(),
                edge_type: "consume".to_string(),
            });
        }
        if is_trigger_line_for(&line.line, identifier, ignore_case, trace_policy) {
            trigger.push(TraceIdSite {
                path: line.path.display().to_string(),
                line_number: line.line_number,
                line: line.line.clone(),
                edge_type: "trigger".to_string(),
            });
        }
    }

    let define = raw_define
        .into_iter()
        .filter(|site| {
            is_define_line(
                &site.line,
                identifier,
                &symbols,
                ignore_case,
                trace_policy,
            )
        })
        .collect();

    TraceIdRecord {
        identifier: identifier.to_string(),
        define,
        produce: dedupe_sites(produce),
        consume: dedupe_sites(consume),
        trigger: dedupe_sites(trigger),
    }
}

fn collect_scoped_lines(paths: Vec<PathBuf>, options: &SearchOptions) -> Vec<ScopedLine> {
    let mut out = Vec::new();
    for path in paths {
        if recur::search::should_skip_content_file(&path, options) {
            continue;
        }
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            for (idx, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    out.push(ScopedLine {
                        path: path.clone(),
                        line_number: idx + 1,
                        line,
                    });
                }
            }
        }
    }
    out
}

fn result_to_site(result: &SearchResult, edge_type: &str) -> TraceIdSite {
    TraceIdSite {
        path: result.path.display().to_string(),
        line_number: result.line_number,
        line: result.line.clone(),
        edge_type: edge_type.to_string(),
    }
}

fn dedupe_sites(sites: Vec<TraceIdSite>) -> Vec<TraceIdSite> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for site in sites {
        let key = (site.path.clone(), site.line_number, site.line.clone());
        if seen.insert(key) {
            out.push(site);
        }
    }
    out
}

fn matched_identifier(result: &SearchResult) -> Option<String> {
    if result.match_start >= result.match_end {
        return None;
    }
    result
        .line
        .get(result.match_start..result.match_end)
        .map(str::to_string)
}

fn contains_glob(pattern: &str) -> bool {
    pattern.contains('*') || pattern.contains('?')
}

fn build_run_state(
    run_name: &str,
    dir: &Path,
    request: TraceIdRunRequestKey,
    scoped_files: &[PathBuf],
) -> anyhow::Result<TraceIdRunState> {
    let paths = resolve_run_paths(run_name, dir)?;
    let request_fingerprint = stable_hash_hex(&toml::to_string(&request)?);
    let config_fingerprint = match paths.config_path.as_ref() {
        Some(config_path) if config_path.exists() => {
            let text = fs::read_to_string(config_path)?;
            Some(stable_hash_hex(&text))
        }
        _ => None,
    };
    let file_fingerprint = fingerprint_scoped_files(scoped_files, &paths.project_root)?;

    Ok(TraceIdRunState {
        paths,
        request,
        request_fingerprint,
        config_fingerprint,
        file_fingerprint,
        file_count: scoped_files.len(),
    })
}

fn resolve_run_paths(run_name: &str, dir: &Path) -> anyhow::Result<TraceIdRunPaths> {
    let safe_name = sanitize_run_name(run_name);
    if safe_name.is_empty() {
        anyhow::bail!("--run-name must contain at least one letter or digit");
    }

    let resolved_dir = resolve_absolute_path(dir)?;
    let config = project_config::load_nearest(&resolved_dir)?;
    let project_root = config
        .as_ref()
        .map(|cfg| cfg.project_root.clone())
        .unwrap_or_else(|| resolved_dir.clone());
    let run_dir = project_root.join(TRACE_ID_RUNS_DIR).join(&safe_name);

    Ok(TraceIdRunPaths {
        name: safe_name,
        project_root,
        run_dir: run_dir.clone(),
        manifest_path: run_dir.join("manifest.toml"),
        latest_json_path: run_dir.join("latest.json"),
        config_path: config.as_ref().map(|cfg| cfg.config_path.clone()),
    })
}

fn resolve_absolute_path(path: &Path) -> anyhow::Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn sanitize_run_name(name: &str) -> String {
    let mut sanitized = String::with_capacity(name.len());
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-') {
            sanitized.push(ch);
        } else if !sanitized.ends_with('_') {
            sanitized.push('_');
        }
    }

    sanitized
        .trim_matches(|ch| ch == '_' || ch == '.')
        .to_string()
}

fn fingerprint_scoped_files(paths: &[PathBuf], project_root: &Path) -> anyhow::Result<String> {
    let mut normalized: Vec<String> = Vec::new();
    for path in paths {
        let metadata = fs::metadata(path)?;
        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        normalized.push(format!(
            "{}|{}|{}",
            normalize_display_path(path, project_root),
            metadata.len(),
            modified
        ));
    }
    normalized.sort();
    Ok(stable_hash_hex(&normalized.join("\n")))
}

fn stable_hash_hex(input: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn normalize_display_path(path: &Path, project_root: &Path) -> String {
    let relative = path.strip_prefix(project_root).unwrap_or(path);
    relative.display().to_string().replace('\\', "/")
}

fn save_run_artifacts(run_state: &TraceIdRunState, saved_json: &str) -> anyhow::Result<()> {
    fs::create_dir_all(&run_state.paths.run_dir)?;
    fs::write(&run_state.paths.latest_json_path, saved_json)?;

    let manifest = TraceIdRunManifest {
        version: TRACE_ID_RUN_MANIFEST_VERSION,
        name: run_state.paths.name.clone(),
        saved_at_unix_s: current_unix_seconds(),
        project_root: run_state.paths.project_root.display().to_string(),
        latest_json_path: normalize_display_path(
            &run_state.paths.latest_json_path,
            &run_state.paths.project_root,
        ),
        request: run_state.request.clone(),
        request_fingerprint: run_state.request_fingerprint.clone(),
        config_fingerprint: run_state.config_fingerprint.clone(),
        file_fingerprint: run_state.file_fingerprint.clone(),
        file_count: run_state.file_count,
    };
    fs::write(
        &run_state.paths.manifest_path,
        toml::to_string_pretty(&manifest)?,
    )?;
    Ok(())
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn evaluate_run_status(run_state: &TraceIdRunState) -> anyhow::Result<TraceIdRunCheck> {
    if !run_state.paths.manifest_path.exists() {
        return Ok(TraceIdRunCheck {
            name: run_state.paths.name.clone(),
            status: "missing".to_string(),
            reasons: vec!["saved run not found".to_string()],
            manifest_path: run_state.paths.manifest_path.display().to_string(),
            latest_json_path: run_state.paths.latest_json_path.display().to_string(),
            file_count: run_state.file_count,
            saved_at_unix_s: None,
        });
    }

    let manifest_text = fs::read_to_string(&run_state.paths.manifest_path)?;
    let manifest: TraceIdRunManifest = toml::from_str(&manifest_text)?;

    let mut reasons = Vec::new();
    if manifest.request_fingerprint != run_state.request_fingerprint {
        reasons.push("request changed".to_string());
    }
    if manifest.config_fingerprint != run_state.config_fingerprint {
        reasons.push("config changed".to_string());
    }
    if manifest.file_fingerprint != run_state.file_fingerprint {
        reasons.push("input files changed".to_string());
    }
    if !run_state.paths.latest_json_path.exists() {
        reasons.push("saved payload missing".to_string());
    }

    Ok(TraceIdRunCheck {
        name: run_state.paths.name.clone(),
        status: if reasons.is_empty() {
            "fresh".to_string()
        } else {
            "stale".to_string()
        },
        reasons,
        manifest_path: run_state.paths.manifest_path.display().to_string(),
        latest_json_path: run_state.paths.latest_json_path.display().to_string(),
        file_count: manifest.file_count,
        saved_at_unix_s: Some(manifest.saved_at_unix_s),
    })
}

fn print_run_status(status: &TraceIdRunCheck, json_output: bool) -> anyhow::Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(status)?);
    } else {
        println!("trace-id run '{}' -> {}", status.name, status.status);
        println!("manifest: {}", status.manifest_path);
        println!("latest: {}", status.latest_json_path);
        if !status.reasons.is_empty() {
            println!("reasons: {}", status.reasons.join(", "));
        }
    }
    Ok(())
}

fn emit_saved_run(
    latest_json_path: &Path,
    pattern: &str,
    output_format: TraceIdFormat,
    json_output: bool,
    color: bool,
) -> anyhow::Result<()> {
    let saved_text = fs::read_to_string(latest_json_path)?;
    if json_output {
        println!("{saved_text}");
        return Ok(());
    }

    let saved: TraceIdSavedJson = serde_json::from_str(&saved_text)?;
    let records = saved.into_records();
    print_terminal(&records, pattern, output_format, color);
    Ok(())
}

fn eq_with_case(left: &str, right: &str, ignore_case: bool) -> bool {
    if ignore_case {
        left.eq_ignore_ascii_case(right)
    } else {
        left == right
    }
}

fn line_matches_identifier_or_symbol(
    line: &str,
    identifier: &str,
    symbols: &BTreeSet<String>,
    ignore_case: bool,
) -> bool {
    if contains_with_case(line, identifier, ignore_case) {
        return true;
    }

    symbols
        .iter()
        .any(|sym| contains_with_case(line, sym, ignore_case))
}

fn line_matches_identifier_or_symbol_token(
    line: &str,
    identifier: &str,
    symbols: &BTreeSet<String>,
    ignore_case: bool,
) -> bool {
    if contains_token_with_case(line, identifier, ignore_case) {
        return true;
    }

    symbols
        .iter()
        .any(|sym| contains_token_with_case(line, sym, ignore_case))
}

fn contains_with_case(haystack: &str, needle: &str, ignore_case: bool) -> bool {
    if ignore_case {
        haystack.to_lowercase().contains(&needle.to_lowercase())
    } else {
        haystack.contains(needle)
    }
}

fn contains_token_with_case(haystack: &str, needle: &str, ignore_case: bool) -> bool {
    if needle.is_empty() {
        return false;
    }

    let hay_cmp = if ignore_case {
        haystack.to_ascii_lowercase()
    } else {
        haystack.to_string()
    };
    let needle_cmp = if ignore_case {
        needle.to_ascii_lowercase()
    } else {
        needle.to_string()
    };

    if needle_cmp.len() > hay_cmp.len() {
        return false;
    }

    let hay_bytes = hay_cmp.as_bytes();
    let needle_len = needle_cmp.len();
    let mut offset = 0usize;

    while let Some(found) = hay_cmp[offset..].find(&needle_cmp) {
        let start = offset + found;
        let end = start + needle_len;

        let before_ok = start == 0 || !is_identifier_char(hay_bytes[start - 1]);
        let after_ok = end == hay_bytes.len() || !is_identifier_char(hay_bytes[end]);

        if before_ok && after_ok {
            return true;
        }

        offset = start + 1;
    }

    false
}

fn is_identifier_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

const RESERVED_SYMBOL_NAMES: &[&str] = &[
    "define",
    "defines",
    "defined",
    "produce",
    "produces",
    "produced",
    "producer",
    "producers",
    "consume",
    "consumes",
    "consumed",
    "consumer",
    "consumers",
    "trigger",
    "triggers",
    "triggered",
    "publish",
    "publishes",
    "subscribe",
    "subscribes",
    "emit",
    "emits",
    "send",
    "sends",
    "bind",
    "binds",
    "register",
    "registers",
    "solve",
    "solves",
    "id",
    "ids",
    "trace_id",
    "trace_ids",
    "warp_id",
    "legacy_trace_id",
    "edge_type",
];

const DECLARATION_MODIFIERS: &[&str] = &[
    "public",
    "private",
    "protected",
    "internal",
    "const",
    "static",
    "readonly",
    "let",
    "var",
    "val",
    "final",
    "def",
    "export",
    "local",
    "global",
    "mut",
    "shared",
    "constexpr",
];

fn find_standalone_assignment_eq(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();
    for (idx, &b) in bytes.iter().enumerate() {
        if b != b'=' {
            continue;
        }
        let prev_is_op = idx > 0 && matches!(bytes[idx - 1], b'=' | b'!' | b'<' | b'>');
        let next_is_op = idx + 1 < bytes.len() && matches!(bytes[idx + 1], b'=' | b'>');
        if !prev_is_op && !next_is_op {
            return Some(idx);
        }
    }
    None
}

fn is_valid_symbol_identifier(token: &str) -> bool {
    let bytes = token.as_bytes();
    if bytes.len() < 2 {
        return false;
    }
    if !(bytes[0].is_ascii_alphabetic() || bytes[0] == b'_') {
        return false;
    }
    bytes[1..]
        .iter()
        .all(|b| b.is_ascii_alphanumeric() || *b == b'_')
}

fn is_reserved_or_role_symbol(token: &str, policy: &TraceIdPolicy) -> bool {
    let lower = token.to_ascii_lowercase();
    if RESERVED_SYMBOL_NAMES.contains(&lower.as_str()) {
        return true;
    }
    policy
        .producer_keywords
        .iter()
        .chain(policy.consumer_keywords.iter())
        .chain(policy.trigger_keywords.iter())
        .any(|kw| kw.eq_ignore_ascii_case(&lower))
}

fn extract_symbol_name_from_definition(
    line: &str,
    identifier: &str,
    ignore_case: bool,
    policy: &TraceIdPolicy,
) -> Option<String> {
    let eq_idx = find_standalone_assignment_eq(line)?;
    let left = &line[..eq_idx];
    let right = &line[eq_idx + 1..];

    if !contains_with_case(right, identifier, ignore_case) {
        return None;
    }

    let left_head = left.split(':').next().unwrap_or(left);
    if left_head.chars().any(|c| {
        matches!(
            c,
            '(' | ')' | '[' | ']' | '{' | '}' | '.' | '"' | '\'' | '`' | '-' | '*' | '#' | '/' | ','
        )
    }) {
        return None;
    }

    let tokens: Vec<&str> = left_head
        .split_whitespace()
        .filter(|t| !t.is_empty())
        .collect();
    if tokens.is_empty() {
        return None;
    }

    let candidate = tokens[tokens.len() - 1];
    if !is_valid_symbol_identifier(candidate) || is_reserved_or_role_symbol(candidate, policy) {
        return None;
    }

    let has_decl_modifier = tokens[..tokens.len() - 1]
        .iter()
        .any(|t| DECLARATION_MODIFIERS.iter().any(|m| t.eq_ignore_ascii_case(m)));

    if !has_decl_modifier {
        let valid_unmodified_lhs =
            tokens.len() == 1 || (tokens.len() == 2 && is_valid_symbol_identifier(tokens[0]));
        if !valid_unmodified_lhs {
            return None;
        }

        let trimmed = line.trim_end();
        if trimmed.ends_with(',') {
            return None;
        }

        let right_opens = right.bytes().filter(|b| *b == b'(').count();
        let right_closes = right.bytes().filter(|b| *b == b')').count();
        if right_closes > right_opens {
            return None;
        }

        let is_indented = line.chars().next().map(char::is_whitespace).unwrap_or(false);
        let tight_eq =
            !left.ends_with(char::is_whitespace) || !right.starts_with(char::is_whitespace);
        if is_indented && tight_eq {
            return None;
        }
    }

    Some(candidate.to_string())
}

fn has_explicit_define_marker(
    line: &str,
    identifier: &str,
    ignore_case: bool,
    policy: &TraceIdPolicy,
) -> bool {
    let masked = mask_identifier(line, identifier, ignore_case);
    if contains_role_keyword(&masked, "define")
        || contains_role_keyword(&masked, "defines")
        || contains_role_keyword(&masked, "trace_id")
        || contains_role_keyword(&masked, "warp_id")
    {
        return true;
    }

    if extract_symbol_name_from_definition(line, identifier, ignore_case, policy).is_some() {
        return true;
    }

    if let Some(eq_idx) = find_standalone_assignment_eq(line) {
        let left = &line[..eq_idx];
        if !left.contains('(') && contains_with_case(left, identifier, ignore_case) {
            return true;
        }
    }

    false
}

fn is_define_line(
    line: &str,
    identifier: &str,
    symbols: &BTreeSet<String>,
    ignore_case: bool,
    policy: &TraceIdPolicy,
) -> bool {
    if has_explicit_define_marker(line, identifier, ignore_case, policy) {
        return true;
    }

    let matched_produce = is_produce_line_for(line, identifier, ignore_case, policy);
    let matched_consume = is_consume_line_for(line, identifier, ignore_case, policy)
        && line_matches_identifier_or_symbol_token(line, identifier, symbols, ignore_case)
        && !is_structural_consume_noise(line);
    let matched_trigger = is_trigger_line_for(line, identifier, ignore_case, policy);

    !(matched_produce || matched_consume || matched_trigger)
}

fn mask_identifier(line: &str, identifier: &str, ignore_case: bool) -> String {
    if identifier.is_empty() {
        return line.to_string();
    }

    let hay_cmp = if ignore_case {
        line.to_ascii_lowercase()
    } else {
        line.to_string()
    };
    let needle_cmp = if ignore_case {
        identifier.to_ascii_lowercase()
    } else {
        identifier.to_string()
    };

    if needle_cmp.is_empty() || needle_cmp.len() > hay_cmp.len() {
        return line.to_string();
    }

    let mut out = String::with_capacity(line.len());
    let mut cursor = 0usize;
    while let Some(rel) = hay_cmp[cursor..].find(&needle_cmp) {
        let start = cursor + rel;
        let end = start + needle_cmp.len();
        if line.is_char_boundary(cursor)
            && line.is_char_boundary(start)
            && line.is_char_boundary(end)
        {
            out.push_str(&line[cursor..start]);
            for _ in 0..(end - start) {
                out.push(' ');
            }
            cursor = end;
        } else {
            cursor = start + 1;
            while cursor < line.len() && !line.is_char_boundary(cursor) {
                cursor += 1;
            }
        }
    }
    if cursor <= line.len() && line.is_char_boundary(cursor) {
        out.push_str(&line[cursor..]);
    }
    out
}

fn is_produce_line_for(
    line: &str,
    identifier: &str,
    ignore_case: bool,
    policy: &TraceIdPolicy,
) -> bool {
    let masked = mask_identifier(line, identifier, ignore_case);
    is_produce_line(&masked, policy)
}

fn is_consume_line_for(
    line: &str,
    identifier: &str,
    ignore_case: bool,
    policy: &TraceIdPolicy,
) -> bool {
    let masked = mask_identifier(line, identifier, ignore_case);
    is_consume_line(&masked, policy)
}

fn is_trigger_line_for(
    line: &str,
    identifier: &str,
    ignore_case: bool,
    policy: &TraceIdPolicy,
) -> bool {
    let masked = mask_identifier(line, identifier, ignore_case);
    is_trigger_line(&masked, policy)
}

fn is_produce_line(line: &str, policy: &TraceIdPolicy) -> bool {
    line_contains_any_keyword(line, &policy.producer_keywords)
}

fn is_consume_line(line: &str, policy: &TraceIdPolicy) -> bool {
    line_contains_any_keyword(line, &policy.consumer_keywords)
}

fn is_trigger_line(line: &str, policy: &TraceIdPolicy) -> bool {
    line_contains_any_keyword(line, &policy.trigger_keywords)
}

fn is_structural_consume_noise(line: &str) -> bool {
    let lower = line.trim_start().to_ascii_lowercase();
    is_type_declaration_line(&lower)
        || is_signature_like_line(&lower)
        || is_logger_reference_line(&lower)
}

fn is_type_declaration_line(lower: &str) -> bool {
    lower.starts_with("class ")
        || lower.starts_with("record ")
        || lower.starts_with("struct ")
        || lower.starts_with("interface ")
        || lower.starts_with("enum ")
        || lower.contains(" class ")
        || lower.contains(" record ")
        || lower.contains(" struct ")
        || lower.contains(" interface ")
        || lower.contains(" enum ")
}

fn is_signature_like_line(lower: &str) -> bool {
    let has_visibility = lower.starts_with("public ")
        || lower.starts_with("private ")
        || lower.starts_with("protected ")
        || lower.starts_with("internal ");

    if !has_visibility {
        return false;
    }

    if !lower.contains('(') || !lower.contains(')') {
        return false;
    }

    if lower.contains("=>") {
        return false;
    }

    !lower.contains('.')
}

fn is_logger_reference_line(lower: &str) -> bool {
    lower.contains("ilogger<") || lower.contains("_logger.") || lower.contains("logger.")
}

fn contains_role_keyword(line: &str, keyword: &str) -> bool {
    let trimmed_kw = keyword.trim();
    if trimmed_kw.is_empty() {
        return false;
    }

    let lower_line = line.to_ascii_lowercase();
    let lower_kw = trimmed_kw.to_ascii_lowercase();
    if lower_kw.len() > lower_line.len() {
        return false;
    }

    let orig_bytes = line.as_bytes();
    let kw_bytes = lower_kw.as_bytes();
    let check_before = is_identifier_char(kw_bytes[0]);
    let check_after = is_identifier_char(kw_bytes[kw_bytes.len() - 1]);

    let mut offset = 0usize;
    while let Some(rel) = lower_line[offset..].find(&lower_kw) {
        let start = offset + rel;
        let end = start + kw_bytes.len();

        let before_ok = !check_before
            || start == 0
            || !is_identifier_char(orig_bytes[start - 1])
            || (orig_bytes[start - 1].is_ascii_lowercase() && orig_bytes[start].is_ascii_uppercase());

        let after_ok = !check_after
            || end == orig_bytes.len()
            || !is_identifier_char(orig_bytes[end])
            || (orig_bytes[end - 1].is_ascii_lowercase() && orig_bytes[end].is_ascii_uppercase());

        if before_ok && after_ok {
            return true;
        }

        offset = start + 1;
    }

    false
}

fn line_contains_any_keyword(line: &str, keywords: &[String]) -> bool {
    keywords
        .iter()
        .any(|keyword| contains_role_keyword(line, keyword))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_define_hit() -> SearchResult {
        let line = "public const string OwnershipCreate = \"ulu.topic.dot.ownership.create\";";
        let match_start = line
            .find("ulu.topic.dot.ownership.create")
            .expect("fixture identifier start");
        let match_end = match_start + "ulu.topic.dot.ownership.create".len();
        SearchResult {
            path: PathBuf::from("DotControlEvents.cs"),
            line_number: 2,
            line: line.to_string(),
            match_start,
            match_end,
            context_before: Vec::new(),
            context_after: Vec::new(),
        }
    }

    fn sample_scoped_lines() -> Vec<ScopedLine> {
        vec![
            ScopedLine {
                path: PathBuf::from("DotControlEvents.cs"),
                line_number: 2,
                line: "public const string OwnershipCreate = \"ulu.topic.dot.ownership.create\";"
                    .to_string(),
            },
            ScopedLine {
                path: PathBuf::from("DotWatcherHostedService.cs"),
                line_number: 4,
                line: "registry.Register(\"*.level.create\", async dot => await _bus.PublishAsync(DotControlTopics.OwnershipCreate));".to_string(),
            },
            ScopedLine {
                path: PathBuf::from("OwnershipCreateSubscriber.cs"),
                line_number: 3,
                line: "channel.QueueBind(\"q.ownership\", \"x.dot\", routingKey: DotControlTopics.OwnershipCreate);".to_string(),
            },
        ]
    }

    #[test]
    fn build_record_classifies_define_produce_consume_trigger() {
        let define_hits = vec![sample_define_hit()];
        let scoped_lines = sample_scoped_lines();
        let policy = TraceIdPolicy::default();

        let record = build_record(
            "ulu.topic.dot.ownership.create",
            &define_hits,
            &scoped_lines,
            false,
            2,
            &policy,
        );

        assert_eq!(record.identifier, "ulu.topic.dot.ownership.create");
        assert!(!record.define.is_empty());
        assert!(!record.produce.is_empty());
        assert!(!record.consume.is_empty());
        assert!(!record.trigger.is_empty());
    }

    #[test]
    fn build_record_depth_zero_keeps_define_only() {
        let define_hits = vec![sample_define_hit()];
        let scoped_lines = sample_scoped_lines();
        let policy = TraceIdPolicy::default();

        let record = build_record(
            "ulu.topic.dot.ownership.create",
            &define_hits,
            &scoped_lines,
            false,
            0,
            &policy,
        );

        assert!(!record.define.is_empty());
        assert!(record.produce.is_empty());
        assert!(record.consume.is_empty());
        assert!(record.trigger.is_empty());
    }

    #[test]
    fn matched_identifier_extracts_slice() {
        let hit = sample_define_hit();
        let id = matched_identifier(&hit).unwrap_or_default();
        assert_eq!(id, "ulu.topic.dot.ownership.create");
    }

    #[test]
    fn symbol_extraction_and_role_detection_work_for_fixture_lines() {
        let define = sample_define_hit();
        let policy = TraceIdPolicy::default();
        let symbol = extract_symbol_name_from_definition(
            &define.line,
            "ulu.topic.dot.ownership.create",
            false,
            &policy,
        )
        .unwrap_or_default();
        assert_eq!(symbol, "OwnershipCreate");

        let produce_line = "registry.Register(\"*.level.create\", async dot => await _bus.PublishAsync(DotControlTopics.OwnershipCreate));";
        let consume_line =
            "channel.QueueBind(\"q.ownership\", \"x.dot\", routingKey: DotControlTopics.OwnershipCreate);";

        assert!(line_matches_identifier_or_symbol(
            produce_line,
            "ulu.topic.dot.ownership.create",
            &BTreeSet::from([symbol.clone()]),
            false
        ));
        assert!(is_produce_line(produce_line, &policy));
        assert!(is_trigger_line(produce_line, &policy));

        assert!(line_matches_identifier_or_symbol(
            consume_line,
            "ulu.topic.dot.ownership.create",
            &BTreeSet::from([symbol]),
            false
        ));
        assert!(is_consume_line(consume_line, &policy));
    }

    #[test]
    fn token_matching_avoids_partial_symbol_matches() {
        assert!(contains_token_with_case(
            "channel.QueueBind(... DotControlTopics.OwnershipCreate);",
            "OwnershipCreate",
            false
        ));
        assert!(!contains_token_with_case(
            "public class OwnershipCreateConsumer {",
            "OwnershipCreate",
            false
        ));
    }

    #[test]
    fn consume_classification_ignores_structural_noise_lines() {
        let define_hits = vec![sample_define_hit()];
        let policy = TraceIdPolicy::default();
        let scoped_lines = vec![
            ScopedLine {
                path: PathBuf::from("OwnershipCreateConsumer.cs"),
                line_number: 1,
                line: "public class OwnershipCreateConsumer {".to_string(),
            },
            ScopedLine {
                path: PathBuf::from("OwnershipCreateConsumer.cs"),
                line_number: 3,
                line: "public OwnershipCreateConsumer(ILogger<OwnershipCreateConsumer> logger) {"
                    .to_string(),
            },
            ScopedLine {
                path: PathBuf::from("OwnershipCreateSubscriber.cs"),
                line_number: 7,
                line: "channel.QueueBind(\"q.ownership\", \"x.dot\", routingKey: DotControlTopics.OwnershipCreate);".to_string(),
            },
        ];

        let record = build_record(
            "ulu.topic.dot.ownership.create",
            &define_hits,
            &scoped_lines,
            false,
            2,
            &policy,
        );

        assert_eq!(record.consume.len(), 1);
        assert!(record.consume[0].line.contains("QueueBind"));
    }

    #[test]
    fn glob_detection_matches_expected_patterns() {
        assert!(contains_glob("ulu.topic.dot.**"));
        assert!(contains_glob("ulu.topic.dot.*"));
        assert!(!contains_glob("ulu.topic.dot.ownership.create"));
    }

    #[test]
    fn sanitize_run_name_replaces_pathy_characters() {
        assert_eq!(
            sanitize_run_name("../trace id::primary"),
            "trace_id_primary"
        );
        assert_eq!(sanitize_run_name("trace.id_ok"), "trace.id_ok");
    }

    #[test]
    fn saved_json_single_turns_back_into_records() {
        let saved = TraceIdSavedJson::Single(TraceIdPayload {
            request: TraceIdRequest {
                pattern: "ulu.topic.dot.ownership.create".to_string(),
                scope: "**".to_string(),
                dir: ".".to_string(),
                ext: Some(".cs".to_string()),
                format: "summary".to_string(),
                stdin: false,
                ignore_case: false,
                separator: ".".to_string(),
                depth_requested: 2,
                depth_effective: 2,
                depth_cap: 5,
                depth_guard: "hard-fail".to_string(),
                force: false,
                json: true,
            },
            identifier: "ulu.topic.dot.ownership.create".to_string(),
            define: vec![result_to_site(&sample_define_hit(), "define")],
            produce: Vec::new(),
            consume: Vec::new(),
            trigger: Vec::new(),
        });

        let records = saved.into_records();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].identifier, "ulu.topic.dot.ownership.create");
    }

    #[test]
    fn symbol_extraction_rejects_kwargs_and_reserved_role_names() {
        let mut policy = TraceIdPolicy::default();
        policy.consumer_keywords.push("consumes".to_string());

        assert_eq!(
            extract_symbol_name_from_definition(
                "              consumes=['robo.electronics.carriers.handoff',",
                "robo.electronics.carriers.handoff",
                false,
                &policy,
            ),
            None
        );
        assert_eq!(
            extract_symbol_name_from_definition(
                "register_step(custom_arg=['robo.electronics.carriers.handoff'])",
                "robo.electronics.carriers.handoff",
                false,
                &policy,
            ),
            None
        );
        assert_eq!(
            extract_symbol_name_from_definition(
                "    custom_arg=['robo.electronics.carriers.handoff']",
                "robo.electronics.carriers.handoff",
                false,
                &policy,
            ),
            None
        );
        assert_eq!(
            extract_symbol_name_from_definition(
                "transition.audit.order.42 = todo.current",
                "transition.audit.order.42",
                false,
                &policy,
            ),
            None
        );
        assert_eq!(
            extract_symbol_name_from_definition(
                "TOPIC_HANDOFF = 'robo.electronics.carriers.handoff'",
                "robo.electronics.carriers.handoff",
                false,
                &policy,
            ),
            Some("TOPIC_HANDOFF".to_string())
        );
    }

    #[test]
    fn role_keyword_boundaries_reject_snake_case_and_self_matching_ids() {
        let mut policy = TraceIdPolicy::default();
        policy.consumer_keywords.push("bind".to_string());

        assert!(!is_trigger_line_for(
            r#""board_register": "docs/robo.electronics.carriers.board_register.md""#,
            "robo.electronics.carriers.board_register",
            false,
            &policy,
        ));
        assert!(!is_trigger_line_for(
            r#"load_board_register("robo.electronics.carriers.board_register")"#,
            "robo.electronics.carriers.board_register",
            false,
            &policy,
        ));
        assert!(!is_trigger_line_for(
            "robo.electronics.carriers.sourcing.r13.register = active",
            "robo.electronics.carriers.sourcing.r13.register",
            false,
            &policy,
        ));
        assert!(is_trigger_line_for(
            r#"registry.Register("*.level.create", dot => dot);"#,
            "ulu.topic.dot.ownership.create",
            false,
            &policy,
        ));
        assert!(is_consume_line_for(
            r#"channel.QueueBind("q", "x", routingKey: DotControlTopics.OwnershipCreate);"#,
            "ulu.topic.dot.ownership.create",
            false,
            &policy,
        ));
    }

    #[test]
    fn full_output_groups_multiple_roles_per_line() {
        let record = TraceIdRecord {
            identifier: "ulu.topic.dot.ownership.create".to_string(),
            define: Vec::new(),
            produce: vec![TraceIdSite {
                path: "DotWatcherHostedService.cs".to_string(),
                line_number: 4,
                line: r#"registry.Register("*.level.create", () => _bus.PublishAsync(Topic));"#.to_string(),
                edge_type: "produce".to_string(),
            }],
            consume: Vec::new(),
            trigger: vec![TraceIdSite {
                path: "DotWatcherHostedService.cs".to_string(),
                line_number: 4,
                line: r#"registry.Register("*.level.create", () => _bus.PublishAsync(Topic));"#.to_string(),
                edge_type: "trigger".to_string(),
            }],
        };

        let grouped = group_sites_for_full_output(&[record]);
        assert_eq!(grouped.len(), 1);
        assert!(grouped[0].line.starts_with("[produce, trigger] "));
    }
}
