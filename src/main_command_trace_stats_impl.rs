//! Implementation of the trace-stats command (phase 3 core metrics).
//!
//! This module maps to hierarchical name: main.command.trace-stats.impl

use recur::parser::HierarchyPattern;
use recur::r#trait::{
    read_resolved_paths_from_stdin, resolve_depth_budget_policy, DepthBudgetMode,
    TraversalBudgetCapable,
};
use recur::search::{FileSearcher, SearchOptions, TraceDirection, TraceOptions, TraceSearcher};
use serde_json::json;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const TRACE_STATS_MAX_DEPTH: usize = 5;

struct TraceStatsCommand;

impl TraversalBudgetCapable for TraceStatsCommand {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SortBy {
    Transitive,
    Direct,
    Circular,
    Depth,
    Risk,
}

impl SortBy {
    fn parse(value: &str) -> anyhow::Result<Self> {
        match value.to_lowercase().as_str() {
            "transitive" => Ok(Self::Transitive),
            "direct" => Ok(Self::Direct),
            "circular" => Ok(Self::Circular),
            "depth" => Ok(Self::Depth),
            "risk" => Ok(Self::Risk),
            _ => anyhow::bail!(
                "Invalid --sort-by '{}'. Must be transitive, direct, circular, depth, or risk",
                value
            ),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Transitive => "transitive",
            Self::Direct => "direct",
            Self::Circular => "circular",
            Self::Depth => "depth",
            Self::Risk => "risk",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Filter {
    CircularOnly,
    HighRisk,
    MediumRisk,
    LowRisk,
}

impl Filter {
    fn parse(value: &str) -> anyhow::Result<Self> {
        match value.to_lowercase().as_str() {
            "circular-only" => Ok(Self::CircularOnly),
            "high-risk" => Ok(Self::HighRisk),
            "medium-risk" => Ok(Self::MediumRisk),
            "low-risk" => Ok(Self::LowRisk),
            _ => anyhow::bail!(
                "Invalid --filter '{}'. Must be circular-only, high-risk, medium-risk, or low-risk",
                value
            ),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::CircularOnly => "circular-only",
            Self::HighRisk => "high-risk",
            Self::MediumRisk => "medium-risk",
            Self::LowRisk => "low-risk",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Table,
    Csv,
    Json,
}

impl OutputFormat {
    fn parse(value: &str) -> anyhow::Result<Self> {
        match value.to_lowercase().as_str() {
            "table" => Ok(Self::Table),
            "csv" => Ok(Self::Csv),
            "json" => Ok(Self::Json),
            _ => anyhow::bail!("Invalid --format '{}'. Must be table, csv, or json", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RiskLevel {
    Low,
    Medium,
    High,
}

impl RiskLevel {
    fn from_transitive(transitive: usize) -> Self {
        if transitive < 10 {
            Self::Low
        } else if transitive <= 30 {
            Self::Medium
        } else {
            Self::High
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        }
    }

    fn rank(self) -> usize {
        match self {
            Self::Low => 0,
            Self::Medium => 1,
            Self::High => 2,
        }
    }
}

fn depth_budget_mode_as_str(mode: DepthBudgetMode) -> &'static str {
    match mode {
        DepthBudgetMode::HardFail => "hard-fail",
        DepthBudgetMode::Clamp => "clamp",
    }
}

#[derive(Debug, Clone)]
struct FunctionStat {
    name: String,
    file: PathBuf,
    line: usize,
    direct: usize,
    transitive: usize,
    circular: usize,
    depth: usize,
    risk: RiskLevel,
}

pub fn execute(
    scope: String,
    dir: PathBuf,
    ext: Option<String>,
    sort_by: String,
    filter: Option<String>,
    top: Option<usize>,
    format: String,
    stdin: bool,
    depth: Option<usize>,
    depth_guard: Option<String>,
    force: bool,
    ignore_case: bool,
    separator: char,
    json_output: bool,
) -> anyhow::Result<()> {
    if scope.trim().is_empty() {
        anyhow::bail!("--scope cannot be empty");
    }

    let sort_mode = SortBy::parse(&sort_by)?;
    let filter_mode = filter.as_deref().map(Filter::parse).transpose()?;
    let output_format = OutputFormat::parse(&format)?;

    if matches!(top, Some(0)) {
        anyhow::bail!("--top must be greater than 0");
    }

    let depth_policy =
        resolve_depth_budget_policy(&dir, depth_guard.as_deref(), TRACE_STATS_MAX_DEPTH)?;
    let requested_depth = depth.unwrap_or(depth_policy.max_depth);
    let depth_budget = TraceStatsCommand::enforce_depth_budget(
        requested_depth,
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
    let effective_depth = depth_budget.effective;

    let scope_pattern = HierarchyPattern::parse_with_separator(&scope, separator)?;
    let scope_pattern = if ignore_case {
        scope_pattern.case_insensitive()
    } else {
        scope_pattern
    };

    let mut search_options = SearchOptions {
        root: dir.clone(),
        case_insensitive: ignore_case,
        ..Default::default()
    };

    if let Some(ext_str) = ext.as_deref() {
        search_options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    let stdin_count = if stdin {
        let paths = read_resolved_paths_from_stdin(&dir)?;
        let count = paths.len();
        search_options.input_files = Some(paths);
        count
    } else {
        0
    };

    let mut stats = compute_function_stats(
        &scope_pattern,
        &search_options,
        ignore_case,
        effective_depth,
    )?;
    apply_filter(&mut stats, filter_mode);
    sort_stats(&mut stats, sort_mode);
    if let Some(limit) = top {
        stats.truncate(limit);
    }

    if json_output || output_format == OutputFormat::Json {
        print_json(
            &stats,
            &scope,
            &dir,
            ext,
            sort_mode,
            filter_mode,
            top,
            stdin,
            stdin_count,
            ignore_case,
            separator,
            output_format,
            json_output,
            requested_depth,
            effective_depth,
            depth_policy.max_depth,
            depth_policy.guard_mode,
            force,
        )?;
        return Ok(());
    }

    if output_format == OutputFormat::Csv {
        print_csv(&stats);
        return Ok(());
    }

    print_table(&stats);
    Ok(())
}

fn compute_function_stats(
    scope_pattern: &HierarchyPattern,
    search_options: &SearchOptions,
    ignore_case: bool,
    trace_depth: usize,
) -> anyhow::Result<Vec<FunctionStat>> {
    let file_searcher = FileSearcher::new(search_options.clone());
    let files = file_searcher.find(scope_pattern);
    let definitions = discover_function_definitions(&files, ignore_case)?;
    let mut definitions_by_name: BTreeMap<String, usize> = BTreeMap::new();

    for def in definitions {
        *definitions_by_name.entry(def).or_insert(0) += 1;
    }

    let mut stats = Vec::new();

    for (function_name, count) in definitions_by_name {
        let pick = if count > 1 { Some(1) } else { None };
        let trace_options = TraceOptions {
            max_width: 100,
            verbose: false,
            pick,
        };
        let mut trace_searcher = TraceSearcher::new(search_options.clone(), trace_options);

        let trace_result = match trace_searcher.trace(
            &function_name,
            scope_pattern,
            TraceDirection::Callees,
            trace_depth,
        ) {
            Ok(result) => result,
            Err(_) => continue,
        };

        if trace_result.root.path.as_os_str().is_empty() {
            continue;
        }

        let stat = FunctionStat {
            name: function_name,
            file: trace_result.root.path.clone(),
            line: trace_result.root.line_number,
            direct: trace_result.stats.direct_callees,
            transitive: trace_result.stats.transitive_callees,
            circular: trace_result.stats.cycles_detected,
            depth: trace_result.stats.max_depth_reached,
            risk: RiskLevel::from_transitive(trace_result.stats.transitive_callees),
        };

        stats.push(stat);
    }

    Ok(stats)
}

fn discover_function_definitions(
    files: &[PathBuf],
    ignore_case: bool,
) -> anyhow::Result<Vec<String>> {
    let pattern = if ignore_case {
        r"(?i)\b(public|private|protected|internal|static|async|virtual|override|abstract|sealed)\b[^\n;{]*?\b(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\("
    } else {
        r"\b(public|private|protected|internal|static|async|virtual|override|abstract|sealed)\b[^\n;{]*?\b(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*\("
    };
    let regex = regex::Regex::new(pattern)?;

    let mut names = Vec::new();

    for file in files {
        if let Ok(content) = std::fs::read_to_string(file) {
            for cap in regex.captures_iter(&content) {
                if let Some(name) = cap.name("name") {
                    let name = name.as_str();
                    if is_skipped_name(name) {
                        continue;
                    }
                    names.push(name.to_string());
                }
            }
        }
    }

    Ok(names)
}

fn is_skipped_name(name: &str) -> bool {
    matches!(
        name,
        "if" | "for" | "while" | "switch" | "catch" | "return" | "new" | "await"
    )
}

fn apply_filter(stats: &mut Vec<FunctionStat>, filter_mode: Option<Filter>) {
    let Some(filter_mode) = filter_mode else {
        return;
    };

    stats.retain(|item| match filter_mode {
        Filter::CircularOnly => item.circular > 0,
        Filter::HighRisk => item.risk == RiskLevel::High,
        Filter::MediumRisk => item.risk == RiskLevel::Medium,
        Filter::LowRisk => item.risk == RiskLevel::Low,
    });
}

fn sort_stats(stats: &mut [FunctionStat], sort_mode: SortBy) {
    stats.sort_by(|a, b| {
        let cmp = match sort_mode {
            SortBy::Transitive => b.transitive.cmp(&a.transitive),
            SortBy::Direct => b.direct.cmp(&a.direct),
            SortBy::Circular => b.circular.cmp(&a.circular),
            SortBy::Depth => b.depth.cmp(&a.depth),
            SortBy::Risk => b.risk.rank().cmp(&a.risk.rank()),
        };

        if cmp == Ordering::Equal {
            a.name.cmp(&b.name)
        } else {
            cmp
        }
    });
}

fn print_json(
    stats: &[FunctionStat],
    scope: &str,
    dir: &Path,
    ext: Option<String>,
    sort_by: SortBy,
    filter: Option<Filter>,
    top: Option<usize>,
    stdin: bool,
    stdin_count: usize,
    ignore_case: bool,
    separator: char,
    format: OutputFormat,
    json_output: bool,
    requested_depth: usize,
    effective_depth: usize,
    depth_cap: usize,
    depth_guard: DepthBudgetMode,
    force: bool,
) -> anyhow::Result<()> {
    let total_functions = stats.len();
    let with_circular = stats.iter().filter(|s| s.circular > 0).count();
    let max_depth = stats.iter().map(|s| s.depth).max().unwrap_or(0);
    let avg_transitive = if total_functions == 0 {
        0.0
    } else {
        let sum: usize = stats.iter().map(|s| s.transitive).sum();
        sum as f64 / total_functions as f64
    };
    let low_count = stats.iter().filter(|s| s.risk == RiskLevel::Low).count();
    let medium_count = stats.iter().filter(|s| s.risk == RiskLevel::Medium).count();
    let high_count = stats.iter().filter(|s| s.risk == RiskLevel::High).count();

    let payload = json!({
        "request": {
            "scope": scope,
            "dir": dir.display().to_string(),
            "ext": ext,
            "sort_by": sort_by.as_str(),
            "filter": filter.map(|f| f.as_str()),
            "top": top,
            "stdin": stdin,
            "stdin_count": stdin_count,
            "ignore_case": ignore_case,
            "separator": separator.to_string(),
            "depth_requested": requested_depth,
            "depth_effective": effective_depth,
            "depth_cap": depth_cap,
            "depth_guard": depth_budget_mode_as_str(depth_guard),
            "force": force,
            "format": match format {
                OutputFormat::Table => "table",
                OutputFormat::Csv => "csv",
                OutputFormat::Json => "json",
            },
            "json": json_output
        },
        "functions": stats.iter().map(|item| {
            json!({
                "name": item.name,
                "file": item.file.display().to_string(),
                "line": item.line,
                "direct": item.direct,
                "transitive": item.transitive,
                "circular": item.circular,
                "depth": item.depth,
                "risk": item.risk.as_str(),
            })
        }).collect::<Vec<_>>(),
        "summary": {
            "total_functions": total_functions,
            "with_circular": with_circular,
            "avg_transitive": avg_transitive,
            "max_depth": max_depth,
            "risk_distribution": {
                "low": low_count,
                "medium": medium_count,
                "high": high_count,
            }
        }
    });

    println!("{}", serde_json::to_string_pretty(&payload)?);
    Ok(())
}

fn print_csv(stats: &[FunctionStat]) {
    println!("Function,File,Line,Direct,Transitive,Circular,Depth,Risk");
    for item in stats {
        println!(
            "{},{},{},{},{},{},{},{}",
            item.name,
            item.file.display(),
            item.line,
            item.direct,
            item.transitive,
            item.circular,
            item.depth,
            item.risk.as_str()
        );
    }
}

fn print_table(stats: &[FunctionStat]) {
    println!("Function | Direct | Transitive | Circular | Depth | Risk");
    for item in stats {
        println!(
            "{} | {} | {} | {} | {} | {}",
            item.name,
            item.direct,
            item.transitive,
            item.circular,
            item.depth,
            item.risk.as_str()
        );
    }

    let with_circular = stats.iter().filter(|s| s.circular > 0).count();
    let max_depth = stats.iter().map(|s| s.depth).max().unwrap_or(0);
    let avg_transitive = if stats.is_empty() {
        0.0
    } else {
        let sum: usize = stats.iter().map(|s| s.transitive).sum();
        sum as f64 / stats.len() as f64
    };

    println!();
    println!("Summary: {} functions analyzed", stats.len());
    println!("  - {} with circular references", with_circular);
    println!("  - Average transitive count: {:.1}", avg_transitive);
    println!("  - Deepest call chain: {} levels", max_depth);
}

#[cfg(test)]
mod tests {
    use super::{is_skipped_name, Filter, OutputFormat, RiskLevel, SortBy};

    #[test]
    fn parse_sort_by_accepts_spec_values() {
        assert!(matches!(
            SortBy::parse("transitive"),
            Ok(SortBy::Transitive)
        ));
        assert!(matches!(SortBy::parse("direct"), Ok(SortBy::Direct)));
        assert!(matches!(SortBy::parse("circular"), Ok(SortBy::Circular)));
        assert!(matches!(SortBy::parse("depth"), Ok(SortBy::Depth)));
        assert!(matches!(SortBy::parse("risk"), Ok(SortBy::Risk)));
    }

    #[test]
    fn parse_sort_by_rejects_invalid_value() {
        assert!(SortBy::parse("latency").is_err());
    }

    #[test]
    fn parse_filter_accepts_spec_values() {
        assert!(matches!(
            Filter::parse("circular-only"),
            Ok(Filter::CircularOnly)
        ));
        assert!(matches!(Filter::parse("high-risk"), Ok(Filter::HighRisk)));
        assert!(matches!(
            Filter::parse("medium-risk"),
            Ok(Filter::MediumRisk)
        ));
        assert!(matches!(Filter::parse("low-risk"), Ok(Filter::LowRisk)));
    }

    #[test]
    fn parse_filter_rejects_invalid_value() {
        assert!(Filter::parse("critical-only").is_err());
    }

    #[test]
    fn parse_output_format_accepts_spec_values() {
        assert!(matches!(
            OutputFormat::parse("table"),
            Ok(OutputFormat::Table)
        ));
        assert!(matches!(OutputFormat::parse("csv"), Ok(OutputFormat::Csv)));
        assert!(matches!(
            OutputFormat::parse("json"),
            Ok(OutputFormat::Json)
        ));
    }

    #[test]
    fn parse_output_format_rejects_invalid_value() {
        assert!(OutputFormat::parse("xml").is_err());
    }

    #[test]
    fn risk_level_thresholds_match_spec() {
        assert!(matches!(RiskLevel::from_transitive(0), RiskLevel::Low));
        assert!(matches!(RiskLevel::from_transitive(9), RiskLevel::Low));
        assert!(matches!(RiskLevel::from_transitive(10), RiskLevel::Medium));
        assert!(matches!(RiskLevel::from_transitive(30), RiskLevel::Medium));
        assert!(matches!(RiskLevel::from_transitive(31), RiskLevel::High));
    }

    #[test]
    fn skipped_names_excludes_keywords() {
        assert!(is_skipped_name("if"));
        assert!(is_skipped_name("await"));
        assert!(!is_skipped_name("CreateWizard3"));
    }
}
