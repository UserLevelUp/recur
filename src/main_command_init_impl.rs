//! Implementation of the init command.
//!
//! This module maps to hierarchical name: main.command.init.impl

use recur::project_config::{self, AnalyzeReport, InitResult};
use std::path::PathBuf;

pub fn execute(dir: PathBuf, analyze: bool, force: bool, json: bool) -> anyhow::Result<()> {
    let root = resolve_root(dir)?;

    if analyze {
        let report = project_config::analyze_project(&root)?;
        print_analyze_report(&report, json)?;
    } else {
        let result = project_config::init_project(&root, force)?;
        print_init_result(&result, json)?;
    }

    Ok(())
}

fn resolve_root(dir: PathBuf) -> anyhow::Result<PathBuf> {
    if dir.is_absolute() {
        return Ok(dir);
    }

    Ok(std::env::current_dir()?.join(dir))
}

fn print_init_result(result: &InitResult, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(result)?);
        return Ok(());
    }

    println!("Initialized project config:");
    println!("  root: {}", result.root);
    println!("  config: {}", result.config_path);
    println!("  lanes:");
    for lane in &result.lanes {
        println!("    - {} => dir={}, sep={}", lane.name, lane.dir, lane.sep);
    }
    if result.checkpoints_created {
        println!("  created: .recur/checkpoints.md");
    } else {
        println!("  checkpoints: existing .recur/checkpoints.md preserved");
    }

    Ok(())
}

fn print_analyze_report(report: &AnalyzeReport, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(report)?);
        return Ok(());
    }

    println!("Project analysis:");
    println!("  root: {}", report.root);
    if report.config_found {
        if let Some(path) = &report.config_path {
            println!("  config: {}", path);
        } else {
            println!("  config: found");
        }
    } else {
        println!("  config: not found");
    }

    if report.detected_lanes.is_empty() {
        println!("  detected lanes: none");
    } else {
        println!("  detected lanes:");
        for lane in &report.detected_lanes {
            println!("    - {} => dir={}, sep={}", lane.name, lane.dir, lane.sep);
        }
    }

    if !report.additions.is_empty() {
        println!("  suggested additions:");
        for lane in &report.additions {
            println!(
                "    - add [{}] dir={} sep={}",
                lane.name, lane.dir, lane.sep
            );
        }
    }

    if !report.separator_updates.is_empty() {
        println!("  separator updates:");
        for update in &report.separator_updates {
            println!(
                "    - [{}] dir={} configured={} suggested={}",
                update.name, update.dir, update.configured_sep, update.suggested_sep
            );
        }
    }

    if !report.missing_directories.is_empty() {
        println!("  missing directories in config:");
        for missing in &report.missing_directories {
            println!("    - {}", missing);
        }
    }

    if report.additions.is_empty()
        && report.separator_updates.is_empty()
        && report.missing_directories.is_empty()
    {
        println!("  no updates suggested");
    }

    Ok(())
}
