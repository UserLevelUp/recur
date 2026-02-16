//! Implementation of the trace-stats command (phase 3 bootstrap).
//!
//! This module maps to hierarchical name: main.command.trace-stats.impl

use recur::parser::HierarchyPattern;
use recur::search::{read_paths_from_stdin, SearchOptions};
use serde_json::json;
use std::path::{Path, PathBuf};

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
            _ => anyhow::bail!(
                "Invalid --format '{}'. Must be table, csv, or json",
                value
            ),
        }
    }
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

    let payload = json!({
        "status": "phase3.bootstrap",
        "message": "trace-stats command surface is wired; metrics pipeline is pending",
        "request": {
            "scope": scope,
            "scope_pattern_debug": format!("{:?}", scope_pattern),
            "dir": dir.display().to_string(),
            "ext": ext,
            "sort_by": sort_mode.as_str(),
            "filter": filter_mode.map(|mode| mode.as_str()),
            "top": top,
            "stdin": stdin,
            "stdin_count": stdin_count,
            "ignore_case": ignore_case,
            "separator": separator.to_string(),
            "format": match output_format {
                OutputFormat::Table => "table",
                OutputFormat::Csv => "csv",
                OutputFormat::Json => "json",
            },
            "json": json_output,
        },
        "functions": [],
        "summary": {
            "total_functions": 0,
            "with_circular": 0,
            "avg_transitive": 0.0,
            "max_depth": 0,
        },
    });

    if json_output || output_format == OutputFormat::Json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    if output_format == OutputFormat::Csv {
        println!("Function,File,Line,Direct,Transitive,Circular,Depth,Risk");
        return Ok(());
    }

    println!("trace-stats (phase3 bootstrap)");
    println!("scope: {}", payload["request"]["scope"].as_str().unwrap_or(""));
    println!(
        "sort-by: {}",
        payload["request"]["sort_by"].as_str().unwrap_or("transitive")
    );
    println!(
        "filter: {}",
        payload["request"]["filter"].as_str().unwrap_or("none")
    );
    println!(
        "stdin files: {}",
        payload["request"]["stdin_count"].as_u64().unwrap_or(0)
    );
    println!("status: metrics pipeline pending");

    Ok(())
}

fn read_resolved_paths_from_stdin(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut resolved = Vec::new();

    for path in read_paths_from_stdin()? {
        if path.is_absolute() || path.exists() {
            resolved.push(path);
            continue;
        }

        if path.is_relative() {
            let candidate = root.join(&path);
            if candidate.exists() {
                resolved.push(candidate);
                continue;
            }
        }

        resolved.push(path);
    }

    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::{Filter, OutputFormat, SortBy};

    #[test]
    fn parse_sort_by_accepts_spec_values() {
        assert!(matches!(SortBy::parse("transitive"), Ok(SortBy::Transitive)));
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
        assert!(matches!(OutputFormat::parse("table"), Ok(OutputFormat::Table)));
        assert!(matches!(OutputFormat::parse("csv"), Ok(OutputFormat::Csv)));
        assert!(matches!(OutputFormat::parse("json"), Ok(OutputFormat::Json)));
    }

    #[test]
    fn parse_output_format_rejects_invalid_value() {
        assert!(OutputFormat::parse("xml").is_err());
    }
}
