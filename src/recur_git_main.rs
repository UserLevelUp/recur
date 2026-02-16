//! recur-git - Git/workflow extension binary for recur.
//!
//! Keeps `recur` focused on hierarchical semantics while this binary composes
//! git + recur-aware workflow operations.

use anyhow::Context;
use clap::{Parser, Subcommand};
use recur::parser::HierarchyPattern;
use recur::project_config;
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

const DEFAULT_CHECKPOINT_ROOT_PATTERN: &str = "main.command.**.todo";
const DEFAULT_CURRENT_SUFFIX: &str = ".current.md";

#[derive(Debug, Clone)]
struct LaneQuery {
    name: String,
    root: PathBuf,
    display_dir: String,
    separator: char,
}

#[derive(Debug, Clone)]
struct CheckpointSettings {
    lanes: Vec<LaneQuery>,
    root_pattern: String,
    current_suffix: String,
    default_log_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
struct LaneState {
    name: String,
    display_dir: String,
    separator: char,
    tree_scope: String,
    current_files: Vec<PathBuf>,
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

        /// File path for checkpoint log (defaults to [checkpoint].file when configured)
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
    let settings = resolve_checkpoint_settings(src_separator)?;
    let lane_states = collect_lane_states(&settings)?;
    let git_state = collect_git_state();

    if snapshot {
        print_snapshot(&git_state, &lane_states);
    }

    if run_tests {
        run_cargo_tests_quiet()?;
    }

    if run_julia_tests {
        run_julia_tests_full()?;
    }

    if emit_parallel || append_parallel {
        let checkpoint_id = checkpoint_id.unwrap_or_else(default_checkpoint_id);
        let entry = build_parallel_entry(&checkpoint_id, &git_state, &lane_states);

        if emit_parallel {
            println!("{}", entry);
        }

        if append_parallel {
            let log_path = resolve_checkpoint_log_path(file, &settings)?;
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
    let head = run_capture("git", &["log", "--oneline", "-n", "1"])
        .unwrap_or_else(|| "unknown".to_string());
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

fn resolve_checkpoint_settings(src_separator: char) -> anyhow::Result<CheckpointSettings> {
    let cwd = std::env::current_dir().context("Failed to resolve current working directory")?;
    let config = project_config::load_nearest(&cwd).context("Failed to load .recur/config.toml")?;

    let Some(config) = config else {
        return Ok(CheckpointSettings {
            lanes: default_lane_queries(&cwd, src_separator),
            root_pattern: DEFAULT_CHECKPOINT_ROOT_PATTERN.to_string(),
            current_suffix: DEFAULT_CURRENT_SUFFIX.to_string(),
            default_log_path: None,
        });
    };

    let mut lanes: Vec<LaneQuery> = config
        .lanes
        .iter()
        .map(|lane| LaneQuery {
            name: lane.name.clone(),
            root: config.project_root.join(&lane.dir),
            display_dir: lane.dir.display().to_string(),
            separator: lane.sep,
        })
        .collect();
    if lanes.is_empty() {
        lanes = default_lane_queries(&config.project_root, src_separator);
    }

    let root_pattern = config
        .checkpoint
        .as_ref()
        .and_then(|section| section.root_pattern.as_ref())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "**".to_string());

    let current_suffix = config
        .status
        .as_ref()
        .and_then(|section| section.current_suffix.as_ref())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_CURRENT_SUFFIX.to_string());

    let default_log_path = config
        .checkpoint
        .as_ref()
        .and_then(|section| section.file.as_ref())
        .map(|path| {
            if path.is_absolute() {
                path.clone()
            } else {
                config.project_root.join(path)
            }
        });

    Ok(CheckpointSettings {
        lanes,
        root_pattern,
        current_suffix,
        default_log_path,
    })
}

fn default_lane_queries(base: &Path, src_separator: char) -> Vec<LaneQuery> {
    vec![
        LaneQuery {
            name: "docs".to_string(),
            root: base.join("docs"),
            display_dir: "docs/".to_string(),
            separator: '.',
        },
        LaneQuery {
            name: "src".to_string(),
            root: base.join("src"),
            display_dir: "src/".to_string(),
            separator: src_separator,
        },
    ]
}

fn collect_lane_states(settings: &CheckpointSettings) -> anyhow::Result<Vec<LaneState>> {
    let current_leaf = extract_current_leaf(&settings.current_suffix);
    let mut lane_states = Vec::new();

    for lane in &settings.lanes {
        let tree_scope =
            normalize_root_pattern_for_separator(&settings.root_pattern, lane.separator);
        let current_pattern = build_current_pattern(&tree_scope, &current_leaf, lane.separator);
        let current_files = find_files_by_pattern(&lane.root, &current_pattern, lane.separator)?;

        lane_states.push(LaneState {
            name: lane.name.clone(),
            display_dir: lane.display_dir.clone(),
            separator: lane.separator,
            tree_scope,
            current_files,
        });
    }

    Ok(lane_states)
}

fn resolve_checkpoint_log_path(
    file: Option<PathBuf>,
    settings: &CheckpointSettings,
) -> anyhow::Result<PathBuf> {
    if let Some(path) = file {
        return Ok(path);
    }

    if let Some(path) = settings.default_log_path.clone() {
        return Ok(path);
    }

    anyhow::bail!(
        "--file (-f) is required when using --append-parallel unless [checkpoint].file is set in .recur/config.toml"
    );
}

fn extract_current_leaf(current_suffix: &str) -> String {
    let stem = Path::new(current_suffix)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or(current_suffix);

    stem.split(|c| c == '.' || c == '_' || c == '-')
        .filter(|segment| !segment.is_empty())
        .last()
        .unwrap_or("current")
        .to_string()
}

fn normalize_root_pattern_for_separator(root_pattern: &str, separator: char) -> String {
    let trimmed = root_pattern.trim();
    if trimmed.is_empty() {
        return "**".to_string();
    }

    if separator != '.' && !trimmed.contains(separator) && trimmed.contains('.') {
        return trimmed.replace('.', &separator.to_string());
    }

    trimmed.to_string()
}

fn build_current_pattern(root_pattern: &str, current_leaf: &str, separator: char) -> String {
    if root_pattern.is_empty() {
        return format!("**{separator}{current_leaf}");
    }

    if root_pattern.ends_with(separator) {
        format!("{root_pattern}{current_leaf}")
    } else {
        format!("{root_pattern}{separator}{current_leaf}")
    }
}

fn find_files_by_pattern(
    root: &Path,
    pattern_raw: &str,
    separator: char,
) -> anyhow::Result<Vec<PathBuf>> {
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

fn print_snapshot(git: &GitState, lane_states: &[LaneState]) {
    println!("\n== Checkpoint Snapshot ==");
    println!("git.branch: {}", git.branch);
    println!("git.head: {}", git.head);
    println!("git.worktree: {}", git.worktree);
    for lane in lane_states {
        println!(
            "lane.state.{}.current: {}",
            lane.name,
            format_paths(&lane.current_files)
        );
    }
    for lane in lane_states {
        println!("lane.separator.{}: {}", lane.name, lane.separator);
    }
}

fn default_checkpoint_id() -> String {
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("ck-{}", epoch)
}

fn build_parallel_entry(checkpoint_id: &str, git: &GitState, lane_states: &[LaneState]) -> String {
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut lines = vec![
        format!("### {checkpoint_id}"),
        format!("- date: unix:{epoch}"),
    ];

    for lane in lane_states {
        lines.push(format!(
            "- lane.state.{}.current: {}",
            lane.name,
            format_paths(&lane.current_files)
        ));
    }

    lines.push(format!("- lane.git.branch: {}", git.branch));
    lines.push(format!("- lane.git.head: {}", git.head));
    lines.push(format!("- lane.git.worktree: {}", git.worktree));

    for lane in lane_states {
        lines.push(format!(
            "- lane.separator.{}: {}",
            lane.name, lane.separator
        ));
    }

    for lane in lane_states {
        lines.push(format!(
            "- evidence.{}.tree_cmd: recur tree \"{}\" -d {} --sep {}",
            lane.name, lane.tree_scope, lane.display_dir, lane.separator
        ));
    }

    lines.join("\n")
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
        let cwd = std::env::current_dir().ok();
        paths
            .iter()
            .map(|p| {
                if let Some(cwd) = cwd.as_ref() {
                    if let Ok(relative) = p.strip_prefix(cwd) {
                        return relative.display().to_string();
                    }
                }
                p.display().to_string()
            })
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
        let lanes = vec![LaneState {
            name: "src".to_string(),
            display_dir: "src/".to_string(),
            separator: '_',
            tree_scope: "main_command_**".to_string(),
            current_files: Vec::new(),
        }];
        let entry = build_parallel_entry("ck-test", &git, &lanes);
        assert!(entry.contains("### ck-test"));
        assert!(entry.contains("lane.separator.src: _"));
    }

    #[test]
    fn extract_current_leaf_supports_status_suffix() {
        assert_eq!(extract_current_leaf(".current.md"), "current");
        assert_eq!(extract_current_leaf("_todo_current.md"), "current");
    }

    #[test]
    fn normalize_root_pattern_for_separator_translates_dot_pattern() {
        let pattern = normalize_root_pattern_for_separator("main.command.**.todo", '_');
        assert_eq!(pattern, "main_command_**_todo");
    }

    #[test]
    fn build_current_pattern_appends_current_leaf() {
        let pattern = build_current_pattern("**", "current", '.');
        assert_eq!(pattern, "**.current");
    }
}
