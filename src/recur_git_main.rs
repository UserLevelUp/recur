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
    project_root: PathBuf,
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

#[derive(Debug, Clone, Default)]
struct TestReceiptState {
    passed: Vec<PathBuf>,
    failed: Vec<PathBuf>,
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

    /// Run one bounded test target and write an immutable eventness receipt
    ///
    /// Examples:
    ///   recur-git test-receipt main.command.tree.wildcard-current --julia-file julia-tests/runtests.tree.jl
    ///   recur-git test-receipt main.release.cargo --cargo
    ///   recur-git test-receipt main.release.full-suite --julia-full
    TestReceipt {
        /// Dot-separated behavior identifier, such as main.command.tree.wildcard-current
        test_id: String,

        /// Run `cargo test --quiet`
        #[arg(long)]
        cargo: bool,

        /// Run `julia julia-tests/runtests.jl`
        #[arg(long)]
        julia_full: bool,

        /// Run one Julia test file relative to the project root
        #[arg(long, value_name = "PATH")]
        julia_file: Option<PathBuf>,
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
        Commands::TestReceipt {
            test_id,
            cargo,
            julia_full,
            julia_file,
        } => execute_test_receipt(test_id, cargo, julia_full, julia_file),
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
    let test_receipts = collect_test_receipts(&settings.project_root)?;
    let git_state = collect_git_state();

    if snapshot {
        print_snapshot(&git_state, &lane_states, &test_receipts);
    }

    if run_tests {
        run_cargo_tests_quiet()?;
    }

    if run_julia_tests {
        run_julia_tests_full()?;
    }

    if emit_parallel || append_parallel {
        let checkpoint_id = checkpoint_id.unwrap_or_else(default_checkpoint_id);
        let entry = build_parallel_entry(&checkpoint_id, &git_state, &lane_states, &test_receipts);

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
            project_root: cwd.clone(),
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
        project_root: config.project_root.clone(),
        lanes,
        root_pattern,
        current_suffix,
        default_log_path,
    })
}

fn collect_test_receipts(project_root: &Path) -> anyhow::Result<TestReceiptState> {
    let root = project_root.join(".recur").join("tests");
    Ok(TestReceiptState {
        passed: find_files_by_pattern(&root, "**.passed.complete", '.')?,
        failed: find_files_by_pattern(&root, "**.failed.strange", '.')?,
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

fn print_snapshot(git: &GitState, lane_states: &[LaneState], test_receipts: &TestReceiptState) {
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
    println!(
        "lane.state.tests.passed: {}",
        format_paths(&test_receipts.passed)
    );
    println!(
        "lane.state.tests.failed: {}",
        format_paths(&test_receipts.failed)
    );
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

fn build_parallel_entry(
    checkpoint_id: &str,
    git: &GitState,
    lane_states: &[LaneState],
    test_receipts: &TestReceiptState,
) -> String {
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

    lines.push(format!(
        "- lane.state.tests.passed: {}",
        format_paths(&test_receipts.passed)
    ));
    lines.push(format!(
        "- lane.state.tests.failed: {}",
        format_paths(&test_receipts.failed)
    ));

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

fn execute_test_receipt(
    raw_test_id: String,
    cargo: bool,
    julia_full: bool,
    julia_file: Option<PathBuf>,
) -> anyhow::Result<()> {
    let test_id = validate_test_id(&raw_test_id)?;
    let project_root = resolve_project_root()?;
    require_clean_worktree(&project_root)?;
    let tested_head = git_head(&project_root)?;
    let (program, args, command_text) =
        resolve_test_command(&project_root, cargo, julia_full, julia_file)?;

    println!("Running test receipt {} at {}", test_id, tested_head);
    println!("  command: {}", command_text);
    let output = Command::new(&program)
        .args(&args)
        .current_dir(&project_root)
        .output()
        .with_context(|| format!("failed to execute test command '{}'", command_text))?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));

    let state = if output.status.success() {
        "passed.complete"
    } else {
        "failed.strange"
    };
    let receipt_path = write_test_receipt(
        &project_root,
        &test_id,
        &tested_head,
        state,
        &command_text,
        output.status.code(),
    )?;
    println!("Wrote test receipt {}", receipt_path.display());

    if !output.status.success() {
        anyhow::bail!("test command failed; recorded {}", receipt_path.display());
    }
    Ok(())
}

fn resolve_project_root() -> anyhow::Result<PathBuf> {
    let cwd = std::env::current_dir().context("Failed to resolve current working directory")?;
    Ok(project_config::load_nearest(&cwd)
        .context("Failed to load .recur/config.toml")?
        .map(|config| config.project_root)
        .unwrap_or(cwd))
}

fn validate_test_id(raw: &str) -> anyhow::Result<String> {
    let value = raw.trim();
    if value.is_empty()
        || !value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
        })
    {
        anyhow::bail!("test id must contain only letters, digits, '.', '_', or '-'");
    }
    Ok(value.to_string())
}

fn require_clean_worktree(project_root: &Path) -> anyhow::Result<()> {
    let output = Command::new("git")
        .args(["status", "--short"])
        .current_dir(project_root)
        .output()
        .context("failed to inspect git worktree")?;
    if !output.status.success() {
        anyhow::bail!("could not inspect git worktree");
    }
    if !String::from_utf8_lossy(&output.stdout).trim().is_empty() {
        anyhow::bail!("test receipts require a clean worktree so tested_head is exact");
    }
    Ok(())
}

fn git_head(project_root: &Path) -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .current_dir(project_root)
        .output()
        .context("failed to resolve git HEAD")?;
    if !output.status.success() {
        anyhow::bail!("test receipts require a Git commit at HEAD");
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn resolve_test_command(
    project_root: &Path,
    cargo: bool,
    julia_full: bool,
    julia_file: Option<PathBuf>,
) -> anyhow::Result<(String, Vec<String>, String)> {
    let selected = usize::from(cargo) + usize::from(julia_full) + usize::from(julia_file.is_some());
    if selected != 1 {
        anyhow::bail!("select exactly one of --cargo, --julia-full, or --julia-file");
    }
    if cargo {
        return Ok((
            "cargo".to_string(),
            vec!["test".to_string(), "--quiet".to_string()],
            "cargo test --quiet".to_string(),
        ));
    }
    if julia_full {
        return Ok((
            "julia".to_string(),
            vec!["julia-tests/runtests.jl".to_string()],
            "julia julia-tests/runtests.jl".to_string(),
        ));
    }

    let file = julia_file.expect("selected above");
    let resolved = if file.is_absolute() {
        file
    } else {
        project_root.join(file)
    };
    if !resolved.is_file() || resolved.extension().and_then(|value| value.to_str()) != Some("jl") {
        anyhow::bail!("--julia-file must name an existing .jl file under the project root");
    }
    let display = resolved
        .strip_prefix(project_root)
        .unwrap_or(&resolved)
        .to_string_lossy()
        .replace('\\', "/");
    Ok((
        "julia".to_string(),
        vec![display.clone()],
        format!("julia {display}"),
    ))
}

fn write_test_receipt(
    project_root: &Path,
    test_id: &str,
    tested_head: &str,
    state: &str,
    command: &str,
    exit_code: Option<i32>,
) -> anyhow::Result<PathBuf> {
    let directory = project_root.join(".recur").join("tests");
    fs::create_dir_all(&directory)
        .with_context(|| format!("failed to create {}", directory.display()))?;
    let path = directory.join(format!("{test_id}.test.{tested_head}.{state}.md"));
    if path.exists() {
        anyhow::bail!("test receipt already exists: {}", path.display());
    }
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let body = format!(
        "# Test receipt\n\n\
test.id: {test_id}\n\
test.state: {state}\n\
test.tested-head: {tested_head}\n\
test.command: {command}\n\
test.exit-code: {}\n\
test.recorded-at: unix:{epoch}\n\n\
defines: recur.git.test.receipt immutable eventness result for one bounded test target at one Git head\n\
produces: {test_id}.test.{tested_head}.{state} queryable test eventness evidence\n",
        exit_code.map(|code| code.to_string()).unwrap_or_else(|| "signal".to_string())
    );
    fs::write(&path, body).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(path)
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
        let entry = build_parallel_entry("ck-test", &git, &lanes, &TestReceiptState::default());
        assert!(entry.contains("### ck-test"));
        assert!(entry.contains("lane.separator.src: _"));
        assert!(entry.contains("lane.state.tests.passed: none"));
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
