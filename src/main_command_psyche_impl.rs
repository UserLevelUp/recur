//! Implementation of the psyche command.
//!
//! This module maps to hierarchical name: main.command.psyche.impl

use anyhow::Context;
use serde::Serialize;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

const RECUR_DIR_NAME: &str = ".recur";
const STATUS_SUFFIX: &str = ".status.current.md";
const WORK_SUFFIX: &str = ".work.current.md";
const CAPSULE_SUFFIX: &str = ".recur.md";
const LAST_RUN_SUFFIX: &str = ".last-run.current.md";
const ORPHAN_STATUS_KIND: &str = "orphan-status";
const ORPHAN_WORK_KIND: &str = "orphan-work";
const MISSING_CAPSULE_KIND: &str = "missing-capsule";
const MISSING_LAST_RUN_AFTER_THRUST_KIND: &str = "missing-last-run-after-thrust";
const STALE_CURRENT_KIND: &str = "stale-current";

enum PsycheFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Serialize)]
struct PsycheFinding {
    path: String,
    kind: &'static str,
}

pub fn execute(
    dir: PathBuf,
    format: String,
    filter: Option<String>,
    stale_seconds: Option<u64>,
) -> anyhow::Result<()> {
    validate_dir(&dir)?;

    let format = parse_format(&format)?;
    let filter = filter.map(|value| value.to_ascii_lowercase());
    let findings = collect_findings(&dir, filter.as_deref(), stale_seconds)?;

    emit_findings(&findings, format)?;

    if findings.is_empty() {
        return Ok(());
    }

    process::exit(1);
}

fn validate_dir(dir: &Path) -> anyhow::Result<()> {
    if !dir.exists() {
        anyhow::bail!("invalid --dir '{}': directory not found", dir.display());
    }

    if !dir.is_dir() {
        anyhow::bail!("invalid --dir '{}': path is not a directory", dir.display());
    }

    Ok(())
}

fn parse_format(raw: &str) -> anyhow::Result<PsycheFormat> {
    match raw {
        "text" => Ok(PsycheFormat::Text),
        "json" => Ok(PsycheFormat::Json),
        _ => anyhow::bail!("invalid --format '{}': expected 'text' or 'json'", raw),
    }
}

fn collect_findings(
    dir: &Path,
    filter: Option<&str>,
    stale_seconds: Option<u64>,
) -> anyhow::Result<Vec<PsycheFinding>> {
    let recur_dir = dir.join(RECUR_DIR_NAME);
    if !recur_dir.exists() {
        return Ok(Vec::new());
    }

    let mut findings = Vec::new();

    for entry in fs::read_dir(&recur_dir)
        .with_context(|| format!("failed to read '{}'", recur_dir.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let agent_dir = entry.path();
        let agent_name = entry.file_name().to_string_lossy().to_string();
        let status_path = agent_dir.join(format!("{agent_name}{STATUS_SUFFIX}"));
        let work_path = agent_dir.join(format!("{agent_name}{WORK_SUFFIX}"));
        let capsule_path = agent_dir.join(format!("{agent_name}{CAPSULE_SUFFIX}"));
        let last_run_path = agent_dir.join(format!("{agent_name}{LAST_RUN_SUFFIX}"));

        let has_status = status_path.is_file();
        let has_work = work_path.is_file();
        let has_capsule = capsule_path.is_file();

        if has_status && !has_work {
            push_finding(&mut findings, dir, &status_path, ORPHAN_STATUS_KIND, filter);
        }

        if has_work && !has_status {
            push_finding(&mut findings, dir, &work_path, ORPHAN_WORK_KIND, filter);
        }

        if (has_status || has_work) && !has_capsule {
            push_finding(&mut findings, dir, &agent_dir, MISSING_CAPSULE_KIND, filter);
        }

        if has_status
            && status_marks_stopped_awaiting_merge(&status_path)?
            && !last_run_path.is_file()
        {
            push_finding(
                &mut findings,
                dir,
                &status_path,
                MISSING_LAST_RUN_AFTER_THRUST_KIND,
                filter,
            );
        }

        if let Some(max_age_seconds) = stale_seconds {
            if has_status && has_work && file_is_older_than(&work_path, max_age_seconds)? {
                push_finding(&mut findings, dir, &work_path, STALE_CURRENT_KIND, filter);
            }
        }
    }

    findings.sort_by(|left, right| left.path.cmp(&right.path).then(left.kind.cmp(right.kind)));
    Ok(findings)
}

fn status_marks_stopped_awaiting_merge(status_path: &Path) -> anyhow::Result<bool> {
    let text = fs::read_to_string(status_path)
        .with_context(|| format!("failed to read '{}'", status_path.display()))?;
    Ok(text.lines().any(|line| {
        line.trim()
            .eq_ignore_ascii_case("STATE: stopped-awaiting-merge")
    }))
}

fn file_is_older_than(path: &Path, max_age_seconds: u64) -> anyhow::Result<bool> {
    let modified = fs::metadata(path)
        .with_context(|| format!("failed to read metadata for '{}'", path.display()))?
        .modified()
        .with_context(|| format!("failed to read modified time for '{}'", path.display()))?;
    let age = std::time::SystemTime::now()
        .duration_since(modified)
        .unwrap_or_default();
    Ok(age.as_secs() > max_age_seconds)
}

fn push_finding(
    findings: &mut Vec<PsycheFinding>,
    base_dir: &Path,
    path: &Path,
    kind: &'static str,
    filter: Option<&str>,
) {
    if !kind_matches_filter(kind, filter) {
        return;
    }

    findings.push(PsycheFinding {
        path: relative_display_path(base_dir, path),
        kind,
    });
}

fn kind_matches_filter(kind: &str, filter: Option<&str>) -> bool {
    match filter {
        Some(filter_value) => kind.eq_ignore_ascii_case(filter_value),
        None => true,
    }
}

fn relative_display_path(base_dir: &Path, path: &Path) -> String {
    path.strip_prefix(base_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn emit_findings(findings: &[PsycheFinding], format: PsycheFormat) -> anyhow::Result<()> {
    if findings.is_empty() {
        return Ok(());
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    match format {
        PsycheFormat::Text => {
            for finding in findings {
                writeln!(handle, "{}: {}", finding.kind, finding.path)?;
            }
        }
        PsycheFormat::Json => {
            serde_json::to_writer(&mut handle, findings)?;
            writeln!(handle)?;
        }
    }

    handle.flush()?;
    Ok(())
}
