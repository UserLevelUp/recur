//! recur-git - Git/workflow extension binary for recur.
//!
//! Keeps `recur` focused on hierarchical semantics while this binary composes
//! git + recur-aware workflow operations.

use anyhow::Context;
use clap::{Parser, Subcommand};
use recur::parser::HierarchyPattern;
use recur::search::{FileSearcher, SearchOptions};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(name = "recur-git")]
#[command(
    about = "Git/workflow extension for recur. Keeps recur pure hierarchy semantics.",
    long_about = None
)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture a parallel-lane checkpoint entry (git + active todo leaves)
    ///
    /// Examples:
    ///   recur-git checkpoint --snapshot
    ///   recur-git checkpoint --emit-parallel --checkpoint-id ck-children-01
    ///   recur-git checkpoint --append-parallel --checkpoint-id ck-children-01 -f checkpoints.md
    Checkpoint {
        /// Print checkpoint snapshot (git + lane state + separator)
        #[arg(long)]
        snapshot: bool,

        /// Run `cargo test --quiet` as part of checkpoint
        #[arg(long)]
        run_tests: bool,

        /// Run `julia julia-tests/runtests.jl` as part of checkpoint
        #[arg(long)]
        run_julia_tests: bool,

        /// Emit parallel-lane checkpoint entry to stdout
        #[arg(long)]
        emit_parallel: bool,

        /// Append parallel-lane checkpoint entry to file
        #[arg(long)]
        append_parallel: bool,

        /// Optional checkpoint ID (default: ck-<unix-seconds>)
        #[arg(long, value_name = "ID")]
        checkpoint_id: Option<String>,

        /// File path for checkpoint log (required with --append-parallel)
        #[arg(short = 'f', long = "file", value_name = "PATH")]
        file: Option<PathBuf>,

        /// Source hierarchy separator for src lane queries (default: '_')
        #[arg(long, value_name = "CHAR", default_value = "_")]
        src_sep: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Checkpoint {
            snapshot,
            run_tests,
            run_julia_tests,
            emit_parallel,
            append_parallel,
            checkpoint_id,
            file,
            src_sep,
        } => {
            let src_separator = src_sep.chars().next().unwrap_or('_');
            execute_checkpoint(
                emit_parallel,
                append_parallel,
                checkpoint_id,
                file,
                src_separator,
                snapshot,
                run_tests,
                run_julia_tests,
            )
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(2);
    }
}

fn execute_checkpoint(
    emit_parallel: bool,
    append_parallel: bool,
    checkpoint_id: Option<String>,
    file: Option<PathBuf>,
    src_separator: char,
    snapshot: bool,
    run_tests: bool,
    run_julia_tests: bool,
) -> anyhow::Result<()> {
    let docs_current = find_files_by_pattern(Path::new("docs"), "main.command.**.todo.current", '.')?;
    let src_pattern = format!(
        "main{sep}command{sep}*{sep}todo{sep}current",
        sep = src_separator
    );
    let src_current = find_files_by_pattern(Path::new("src"), &src_pattern, src_separator)?;
    let git_state = collect_git_state();

    if snapshot {
        print_snapshot(&git_state, &docs_current, &src_current, src_separator);
    }

    if run_tests {
        run_cargo_tests_quiet()?;
    }

    if run_julia_tests {
        run_julia_tests_full()?;
    }

    if emit_parallel || append_parallel {
        let checkpoint_id = checkpoint_id.unwrap_or_else(default_checkpoint_id);
        let entry = build_parallel_entry(
            &checkpoint_id,
            &git_state,
            &docs_current,
            &src_current,
            src_separator,
        );

        if emit_parallel {
            println!("{}", entry);
        }

        if append_parallel {
            let log_path = file.ok_or_else(|| {
                anyhow::anyhow!("--file (-f) is required when using --append-parallel")
            })?;
            append_parallel_entry(&log_path, &entry)?;
            println!("Appended parallel checkpoint to {}", log_path.display());
        }
    } else if !snapshot && !run_tests && !run_julia_tests {
        println!(
            "No action requested. Use --emit-parallel, --append-parallel, --snapshot, --run-tests, or --run-julia-tests."
        );
    }

    Ok(())
}

#[derive(Debug)]
struct GitState {
    branch: String,
    head: String,
    worktree: String,
}

fn collect_git_state() -> GitState {
    let branch = run_capture("git", &["rev-parse", "--abbrev-ref", "HEAD"])
        .unwrap_or_else(|| "unknown".to_string());
    let head =
        run_capture("git", &["log", "--oneline", "-n", "1"]).unwrap_or_else(|| "unknown".to_string());
    let dirty_count = run_capture("git", &["status", "--short"])
        .map(|s| s.lines().filter(|l| !l.trim().is_empty()).count())
        .unwrap_or(0);
    let worktree = if dirty_count == 0 {
        "clean".to_string()
    } else {
        format!("dirty={}", dirty_count)
    };

    GitState {
        branch,
        head,
        worktree,
    }
}

fn run_capture(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }

    let text = String::from_utf8(output.stdout).ok()?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn find_files_by_pattern(root: &Path, pattern_raw: &str, separator: char) -> anyhow::Result<Vec<PathBuf>> {
    if !root.exists() {
        return Ok(Vec::new());
    }

    let pattern = HierarchyPattern::parse_with_separator(pattern_raw, separator)?;
    let searcher = FileSearcher::new(SearchOptions {
        root: root.to_path_buf(),
        ..Default::default()
    });
    let mut files = searcher.find(&pattern);
    files.sort();
    Ok(files)
}

fn print_snapshot(
    git: &GitState,
    docs_current: &[PathBuf],
    src_current: &[PathBuf],
    src_separator: char,
) {
    println!("\n== Checkpoint Snapshot ==");
    println!("git.branch: {}", git.branch);
    println!("git.head: {}", git.head);
    println!("git.worktree: {}", git.worktree);
    println!("lane.state.docs.current: {}", format_paths(docs_current));
    println!("lane.state.src.current: {}", format_paths(src_current));
    println!("lane.separator.docs_tests: .");
    println!("lane.separator.src: {}", src_separator);
}

fn default_checkpoint_id() -> String {
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("ck-{}", epoch)
}

fn build_parallel_entry(
    checkpoint_id: &str,
    git: &GitState,
    docs_current: &[PathBuf],
    src_current: &[PathBuf],
    src_separator: char,
) -> String {
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format!(
        "### {checkpoint_id}\n\
         - date: unix:{epoch}\n\
         - lane.state.docs.current: {docs}\n\
         - lane.state.src.current: {src}\n\
         - lane.git.branch: {branch}\n\
         - lane.git.head: {head}\n\
         - lane.git.worktree: {worktree}\n\
         - lane.separator.docs_tests: .\n\
         - lane.separator.src: {src_sep}\n\
         - evidence.docs_tree_cmd: recur tree \"main\" -d docs/\n\
         - evidence.src_tree_cmd: recur tree \"main\" -d src/ --sep {src_sep}",
        docs = format_paths(docs_current),
        src = format_paths(src_current),
        branch = git.branch,
        head = git.head,
        worktree = git.worktree,
        src_sep = src_separator
    )
}

fn append_parallel_entry(path: &Path, entry: &str) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }
    }

    let needs_newline = path.exists() && fs::metadata(path)?.len() > 0;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("Failed to open {}", path.display()))?;

    if needs_newline {
        writeln!(file)?;
    }
    writeln!(file, "{}", entry)?;
    Ok(())
}

fn run_cargo_tests_quiet() -> anyhow::Result<()> {
    let status = Command::new("cargo")
        .args(["test", "--quiet"])
        .status()
        .context("Failed to execute `cargo test --quiet`")?;

    if !status.success() {
        anyhow::bail!("`cargo test --quiet` failed");
    }

    Ok(())
}

fn run_julia_tests_full() -> anyhow::Result<()> {
    let status = Command::new("julia")
        .args(["julia-tests/runtests.jl"])
        .status()
        .context("Failed to execute `julia julia-tests/runtests.jl`")?;

    if !status.success() {
        anyhow::bail!("`julia julia-tests/runtests.jl` failed");
    }

    Ok(())
}

fn format_paths(paths: &[PathBuf]) -> String {
    if paths.is_empty() {
        "none".to_string()
    } else {
        paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_checkpoint_id_has_prefix() {
        assert!(default_checkpoint_id().starts_with("ck-"));
    }

    #[test]
    fn build_parallel_entry_contains_checkpoint_id() {
        let git = GitState {
            branch: "dogfooding".to_string(),
            head: "abc123 test".to_string(),
            worktree: "dirty=3".to_string(),
        };
        let entry = build_parallel_entry("ck-test", &git, &[], &[], '_');
        assert!(entry.contains("### ck-test"));
        assert!(entry.contains("lane.separator.src: _"));
    }
}
