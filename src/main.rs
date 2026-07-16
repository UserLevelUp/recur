//! recur - Hierarchy-Aware Code Search
//!
//! Named in quiet tribute to Dennis M. Ritchie's early work on recursive
//! program structure.
//!
//! Main CLI entry point

use clap::{Parser, Subcommand};
use recur::r#trait::{CliSeparatorPolicy, SeparatorCapable};
use std::path::PathBuf;
use std::process;

mod main_command_callees_impl;
mod main_command_callers_impl;
mod main_command_children_impl;
mod main_command_files_impl;
mod main_command_files_stdin;
mod main_command_find_impl;
mod main_command_flatten_csv;
mod main_command_flatten_impl;
mod main_command_flatten_json;
mod main_command_flatten_toml;
mod main_command_flatten_xml;
mod main_command_flatten_yaml;
mod main_command_id_impl;
mod main_command_init_impl;
mod main_command_lane_impl;
mod main_command_merge_impl;
mod main_command_psyche_impl;
mod main_command_related_impl;
mod main_command_reveal_impl;
mod main_command_stats_impl;
mod main_command_stats_stdin;
mod main_command_trace_id_impl;
mod main_command_trace_impl;
mod main_command_trace_stats_impl;
mod main_command_trait_impl;
mod main_command_tree_impl;
mod main_command_watch_query_impl;

#[derive(Parser)]
#[command(name = "recur")]
#[command(
    about = "Hierarchy-aware code search for modern codebases\n\nNamed in tribute to Dennis Ritchie's early work on recursive program structure.",
    long_about = None
)]
#[command(version)]
#[command(
    after_help = "A quiet nod to Dennis Ritchie's 1968 thesis on recursive functions and program structure.\n\nHomepage: https://github.com/userlevelup/recur\n\nAdditional commands:\n  recur trace-id --help\n  recur reveal --help"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Use color in output
    #[arg(long, global = true, default_value = "true")]
    color: bool,

    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,

    /// Hierarchy separator character (default: '.')
    /// Use '_' for Rust modules, '-' for kebab-case, ':' for namespaces
    /// May be provided multiple times for multi-separator queries.
    #[arg(long, global = true, value_name = "CHAR")]
    sep: Vec<String>,

    /// When using multiple separators, normalize output to this separator
    #[arg(long, global = true, value_name = "CHAR")]
    sep_replace_default: Option<String>,

    /// Show which separator was used for each file (e.g., [.] or [_])
    #[arg(long, global = true)]
    show_sep: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Find files matching a recursive hierarchical pattern
    ///
    /// Examples:
    ///   recur files "Module.SubModule.*"
    ///   recur files "LevelController.CreateWizard3.Templates"
    ///   git diff --name-only | recur files "**" --stdin
    Files {
        /// Hierarchical pattern to match (e.g., "Module.SubModule.*")
        pattern: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// File extensions to include (comma-separated)
        #[arg(short, long)]
        ext: Option<String>,

        /// Case-insensitive matching
        #[arg(short, long)]
        ignore_case: bool,

        /// Minimum depth to search (0=base level)
        #[arg(long, default_value = "0")]
        min_depth: usize,

        /// Maximum depth to search recursively
        #[arg(long)]
        max_depth: Option<usize>,

        /// Show only the count of matching files
        #[arg(long)]
        count: bool,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,
    },

    /// Search for text within hierarchically-scoped files (recursive)
    ///
    /// Examples:
    ///   recur find "async" --scope "Controller.Api"
    ///   recur find "pattern" --scope "Module" -C 3
    ///   git diff --name-only | recur find "TODO" --scope "**" --stdin
    Find {
        /// Text to search for
        query: String,

        /// Hierarchical scope to search within (recursive)
        #[arg(short, long)]
        scope: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Number of context lines
        #[arg(short = 'C', long, default_value = "0")]
        context: usize,

        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,

        /// Use regex pattern
        #[arg(short = 'E', long)]
        regex: bool,

        /// File extensions to include
        #[arg(short, long)]
        ext: Option<String>,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,
    },

    /// Show recursive hierarchy tree for files
    ///
    /// Examples:
    ///   recur tree "LevelController"
    ///   recur tree "ServiceName" --depth 2
    ///   git diff --name-only | recur tree "**" --stdin
    Tree {
        /// Base name to build tree from
        base: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Maximum depth to display
        #[arg(long)]
        depth: Option<usize>,

        /// Show file counts
        #[arg(long)]
        count: bool,

        /// Use ASCII instead of Unicode
        #[arg(long)]
        ascii: bool,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,
    },

    /// Find files related to (siblings of) a given file in the hierarchy
    ///
    /// Examples:
    ///   recur related "Service.Module.Feature.cs"
    ///   recur related "Service.Module.Feature.cs" --exclude-self
    Related {
        /// Filename to find relatives of
        filename: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Exclude the input file from results
        #[arg(long)]
        exclude_self: bool,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,
    },

    /// Find files that are children of a hierarchy (recursive)
    ///
    /// Examples:
    ///   recur children "Module.SubModule"
    ///   git ls-files | recur children "Module" --stdin
    Children {
        /// Parent hierarchy
        parent: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Show only the count of matching files
        #[arg(long)]
        count: bool,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,
    },

    /// Search for hierarchical identifiers in file content (recursive)
    ///
    /// Examples:
    ///   recur id "config.database.*"
    ///   recur id "ulu.role.**" --ext ".cs,.json"
    ///   git diff --name-only | recur id "config.**" --stdin
    Id {
        /// Hierarchical identifier pattern (recursive)
        pattern: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// File extensions to include
        #[arg(short, long)]
        ext: Option<String>,

        /// Lines of context to show around matches
        #[arg(short = 'C', long, default_value = "0")]
        context: usize,

        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,
    },

    /// Show statistics for a hierarchy (files, lines, depth)
    ///
    /// Examples:
    ///   recur stats "ServiceName"              # Show summary with depth breakdown
    ///   recur stats "ServiceName" -l 1         # List files at depth level 1
    ///   recur stats "ServiceName" -l 2         # List files at depth level 2
    ///   git diff --name-only | recur stats "**" --stdin
    Stats {
        /// Hierarchical pattern to analyze
        pattern: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Show files at specific depth level (0=base, 1=first level, etc.)
        /// Results are paginated to fit terminal height
        #[arg(short = 'l', long)]
        level: Option<usize>,

        /// File extensions to include
        #[arg(short, long)]
        ext: Option<String>,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,
    },

    /// Find all places where a function/method is called
    ///
    /// Examples:
    ///   recur callers "CreateUser" --scope "UserService.**"
    ///   recur callers "ValidateEmail" --scope "**" --ext .cs
    ///   git diff --name-only | recur callers "ProcessData" --scope "**" --stdin
    Callers {
        /// Function or method name to find callers of
        function: String,

        /// Hierarchical scope to search within (recursive)
        #[arg(short, long)]
        scope: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Number of context lines to show
        #[arg(short = 'C', long, default_value = "2")]
        context: usize,

        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,

        /// File extensions to include
        #[arg(short, long)]
        ext: Option<String>,

        /// Show only count of callers
        #[arg(long)]
        count: bool,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,
    },

    /// Find all functions/methods that a given function calls (callees/dependencies)
    ///
    /// Examples:
    ///   recur callees "CreateUser" --scope "UserService.**"
    ///   recur callees "ProcessRequest" --scope "**" --ext .cs
    ///   git diff --name-only | recur callees "Initialize" --scope "**" --stdin
    Callees {
        /// Function or method name to find callees of
        function: String,

        /// Hierarchical scope to search within (recursive)
        #[arg(short, long)]
        scope: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Number of context lines to show
        #[arg(short = 'C', long, default_value = "2")]
        context: usize,

        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,

        /// File extensions to include
        #[arg(short, long)]
        ext: Option<String>,

        /// Show only count of callees
        #[arg(long)]
        count: bool,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,
    },

    /// Multi-level call graph visualization (trace execution/usage paths)
    ///
    /// Examples:
    ///   recur trace "ApplyAiContent" --depth 2 --scope "LevelController.**"
    ///   recur trace "GetDeletedComponents" --direction callers --depth 2
    ///   recur trace "ValidateInput" --direction both --depth 1
    Trace {
        /// Function or method name to trace
        function: String,

        /// Hierarchical scope to search within
        #[arg(short, long)]
        scope: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Trace depth (how many levels deep)
        #[arg(long, default_value = "2")]
        depth: usize,

        /// Trace direction: callees (what it calls), callers (who calls it), or both
        #[arg(long, default_value = "callees")]
        direction: String,

        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,

        /// File extensions to include
        #[arg(short, long)]
        ext: Option<String>,

        /// Max branches per level (default 10)
        #[arg(long, default_value = "10")]
        max_width: usize,

        /// Show full paths instead of abbreviated paths
        #[arg(long)]
        verbose: bool,

        /// Output format: tree, flat, or graph
        #[arg(long, default_value = "tree")]
        format: String,

        /// Pick a specific definition when multiple matches exist (1-based)
        #[arg(long)]
        pick: Option<usize>,

        /// Scope alias mapping (e.g., --scope-alias cw3=LevelController.CreateWizard3.**)
        #[arg(long, value_name = "ALIAS=PATTERN")]
        scope_alias: Vec<String>,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,

        /// Depth guardrail mode override: hard-fail or clamp
        /// Falls back to [traits.traversal_budget].depth_guard, then [traversal].depth_guard.
        #[arg(long)]
        depth_guard: Option<String>,

        /// Bypass trace depth cap safety check
        #[arg(long)]
        force: bool,
    },

    /// Analyze call graph complexity statistics
    ///
    /// Examples:
    ///   recur trace-stats --scope "**" --ext .rs --top 5
    ///   recur trace-stats --scope "**" --filter circular-only
    ///   recur trace-stats --scope "**" --depth 8 --depth-guard clamp
    ///   git diff --name-only | recur trace-stats --scope "**" --stdin --sort-by risk
    TraceStats {
        /// Hierarchical scope to analyze
        #[arg(long)]
        scope: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// File extensions to include
        #[arg(long)]
        ext: Option<String>,

        /// Sort by metric (transitive, direct, circular, depth, risk)
        #[arg(long, default_value = "transitive")]
        sort_by: String,

        /// Filter results (circular-only, high-risk, medium-risk, low-risk)
        #[arg(long)]
        filter: Option<String>,

        /// Show only top N results
        #[arg(long)]
        top: Option<usize>,

        /// Output format (table, csv, json)
        #[arg(long, default_value = "table")]
        format: String,

        /// Read file paths from stdin instead of searching filesystem
        #[arg(long)]
        stdin: bool,

        /// Trace depth per root function (default: [traits.traversal_budget].max_depth, then [traversal].max_depth, then 5)
        #[arg(long)]
        depth: Option<usize>,

        /// Depth guardrail mode override: hard-fail or clamp
        /// Falls back to [traits.traversal_budget].depth_guard, then [traversal].depth_guard.
        #[arg(long)]
        depth_guard: Option<String>,

        /// Bypass trace depth cap safety check
        #[arg(long)]
        force: bool,

        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,
    },

    /// Merge hierarchical results from multiple naming conventions
    ///
    /// Examples:
    ///   recur merge --pattern "main.command.tree" --sep "." --pattern "main_command_tree" --sep "_"
    ///   recur merge --pattern "api.user" --sep "." --pattern "api_user" --sep "_" --pattern "api-user" --sep "-"
    ///   recur tree "main" --sep "." --json | recur merge --stdin --base "main"
    Merge {
        /// Patterns to merge (repeatable, paired with --sep)
        #[arg(long = "pattern", value_name = "PATTERN")]
        patterns: Vec<String>,

        /// Separators for each pattern (repeatable, paired with --pattern)
        #[arg(long = "sep", value_name = "CHAR")]
        sep: Vec<String>,

        /// JSON input files to merge (file mode)
        #[arg(value_name = "FILE")]
        inputs: Vec<PathBuf>,

        /// Base name for tree output (required in file mode and stdin mode)
        #[arg(long = "base", value_name = "BASE")]
        base: Option<String>,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Maximum depth to search recursively
        #[arg(long)]
        max_depth: Option<usize>,

        /// Use ASCII characters instead of Unicode
        #[arg(long = "ascii")]
        ascii: bool,

        /// Show file counts at each level
        #[arg(long)]
        count: bool,

        /// Read JSON from stdin (pipe mode)
        #[arg(long)]
        stdin: bool,
    },

    /// Manage trait configuration in .recur/config.toml
    ///
    /// Examples:
    ///   recur trait list
    ///   recur trait get trace_id.enabled
    ///   recur trait set trace_id.producer_keywords "publish,send,dispatch"
    ///   recur trait set traversal_budget.max_depth 3
    Trait {
        #[command(subcommand)]
        command: main_command_trait_impl::TraitSubcommand,

        /// Project root directory
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,
    },

    /// Initialize or analyze project-local `.recur/config.toml`
    ///
    /// Examples:
    ///   recur init
    ///   recur init --analyze
    ///   recur init -d ../another-project --analyze
    Init {
        /// Project root directory
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Analyze project directories and suggest config updates
        #[arg(long)]
        analyze: bool,

        /// Overwrite existing `.recur/config.toml`
        #[arg(long)]
        force: bool,
    },

    /// Scaffold or list named lane sub-roots
    ///
    /// Examples:
    ///   recur lane docs
    ///   recur lane impl
    ///   recur lane
    Lane {
        /// Lane name to scaffold; omit to list known lanes
        name: Option<String>,

        /// Project root directory
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,
    },

    /// Reveal lane-local rehydration capsules (`*.recur.md`)
    ///
    /// Examples:
    ///   recur reveal
    ///   recur reveal main.command.trace-id
    ///   recur reveal skippy -d .recur
    Reveal {
        /// Lane name or query (for example: main.command.trace-id)
        lane: Option<String>,

        /// Starting directory or project root
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,
    },

    /// Inspect `.recur` agent vault structure for missing or inconsistent files
    ///
    /// Examples:
    ///   recur psyche
    ///   recur psyche --dir .
    ///   recur psyche --format json
    ///   recur psyche --filter orphan-status
    Psyche {
        /// Project root or directory containing `.recur/`
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Output format: text or json
        #[arg(long, default_value = "text", value_name = "FORMAT")]
        format: String,

        /// Filter findings by kind (for example: orphan-status)
        #[arg(long, value_name = "KIND")]
        filter: Option<String>,

        /// Report current work files older than this many seconds
        #[arg(long, value_name = "SECONDS")]
        stale_seconds: Option<u64>,
    },

    /// Inspect watcher state written by `recur-watch`
    ///
    /// Examples:
    ///   recur watch
    ///   recur watch list --filter "**.active"
    ///   recur watch status docs-monkey
    ///   recur watch explain
    Watch {
        #[command(subcommand)]
        command: Option<main_command_watch_query_impl::WatchQuerySubcommand>,

        /// Project root or directory containing `.recur/`
        #[arg(short = 'd', long, default_value = ".", global = true)]
        dir: PathBuf,
    },

    /// Flatten structured files (XML, JSON, TOML, YAML, CSV) into hierarchical dot-paths
    ///
    /// Converts any structured document into recur's universal hierarchy format.
    /// Auto-detects format from file extension, or use --format to override.
    ///
    /// Examples:
    ///   recur flatten config.xml
    ///   recur flatten data.json --filter "users"
    ///   recur flatten .recur/config.toml --format toml
    ///   recur flatten appsettings.yaml --format yaml
    ///   recur flatten levels.csv --format csv
    ///   cat pom.xml | recur flatten --stdin
    ///   recur flatten config.nuspec --json
    Flatten {
        /// File to flatten (omit for stdin)
        file: Option<PathBuf>,

        /// Read from stdin
        #[arg(long)]
        stdin: bool,

        /// Override format detection (xml, json, toml, yaml, csv)
        #[arg(long, value_name = "FORMAT")]
        format: Option<String>,

        /// Maximum depth to flatten (0 = unlimited)
        #[arg(long, default_value = "0")]
        max_depth: usize,

        /// Filter output to paths matching this prefix
        #[arg(long, value_name = "PREFIX")]
        filter: Option<String>,
    },
}

fn main() {
    if let Some(result) = main_command_trace_id_impl::maybe_execute_from_args() {
        if let Err(e) = result {
            eprintln!("Error: {}", e);
            process::exit(2);
        }
        return;
    }

    let cli = Cli::parse();
    let fallback_separator = CliSeparatorPolicy::parse_cli_separators(&cli.sep)
        .last()
        .copied()
        .unwrap_or('.');

    // Parse --sep-replace-default flag
    let replace_default =
        CliSeparatorPolicy::parse_optional_separator(cli.sep_replace_default.as_deref());

    // Get --show-sep flag
    let show_sep = cli.show_sep;

    let result = match cli.command {
        Commands::Files {
            pattern,
            dir,
            ext,
            ignore_case,
            min_depth,
            max_depth,
            count,
            stdin,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            // Use multi-separator if multiple separators or new flags are present
            if command_separators.len() > 1 || replace_default.is_some() || show_sep {
                main_command_files_impl::execute_with_separators(
                    pattern,
                    dir,
                    ext,
                    ignore_case,
                    min_depth,
                    max_depth,
                    count,
                    stdin,
                    command_separators.clone(),
                    replace_default,
                    show_sep,
                    cli.json,
                    cli.color,
                )
            } else {
                main_command_files_impl::execute(
                    pattern,
                    dir,
                    ext,
                    ignore_case,
                    min_depth,
                    max_depth,
                    count,
                    stdin,
                    separator,
                    cli.json,
                    cli.color,
                )
            }
        }
        Commands::Find {
            query,
            scope,
            dir,
            context,
            ignore_case,
            regex,
            ext,
            stdin,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            main_command_find_impl::execute(
                query,
                scope,
                dir,
                context,
                ignore_case,
                regex,
                ext,
                stdin,
                separator,
                cli.json,
                cli.color,
            )
        }
        Commands::Tree {
            base,
            dir,
            depth,
            count,
            ascii,
            stdin,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            // Use multi-separator if multiple separators or new flags are present
            if command_separators.len() > 1 || replace_default.is_some() || show_sep {
                main_command_tree_impl::execute_with_separators(
                    base,
                    dir,
                    depth,
                    count,
                    !ascii,
                    stdin,
                    command_separators.clone(),
                    replace_default,
                    show_sep,
                    cli.json,
                )
            } else {
                main_command_tree_impl::execute(
                    base, dir, depth, count, !ascii, stdin, separator, cli.json,
                )
            }
        }
        Commands::Related {
            filename,
            dir,
            exclude_self,
            stdin,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            main_command_related_impl::execute(
                filename,
                dir,
                exclude_self,
                stdin,
                separator,
                cli.json,
                cli.color,
            )
        }
        Commands::Children {
            parent,
            dir,
            count,
            stdin,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            main_command_children_impl::execute(
                parent, dir, count, stdin, separator, cli.json, cli.color,
            )
        }
        Commands::Id {
            pattern,
            dir,
            ext,
            context,
            ignore_case,
            stdin,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            main_command_id_impl::execute(
                pattern,
                dir,
                ext,
                context,
                ignore_case,
                stdin,
                separator,
                cli.json,
                cli.color,
            )
        }
        Commands::Stats {
            pattern,
            dir,
            level,
            ext,
            stdin,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            main_command_stats_impl::execute(
                pattern, dir, level, ext, stdin, separator, cli.json, cli.color,
            )
        }
        Commands::Callers {
            function,
            scope,
            dir,
            context,
            ignore_case,
            ext,
            count,
            stdin,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            main_command_callers_impl::execute(
                function,
                scope,
                dir,
                context,
                ignore_case,
                ext,
                count,
                stdin,
                separator,
                cli.json,
                cli.color,
            )
        }
        Commands::Callees {
            function,
            scope,
            dir,
            context,
            ignore_case,
            ext,
            count,
            stdin,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            main_command_callees_impl::execute(
                function,
                scope,
                dir,
                context,
                ignore_case,
                ext,
                count,
                stdin,
                separator,
                cli.json,
                cli.color,
            )
        }
        Commands::Trace {
            function,
            scope,
            dir,
            depth,
            direction,
            ignore_case,
            ext,
            max_width,
            verbose,
            format,
            pick,
            scope_alias,
            stdin,
            depth_guard,
            force,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            main_command_trace_impl::execute(
                function,
                scope,
                dir,
                depth,
                direction,
                ignore_case,
                ext,
                max_width,
                verbose,
                format,
                pick,
                scope_alias,
                stdin,
                depth_guard,
                force,
                separator,
                cli.json,
                cli.color,
            )
        }
        Commands::TraceStats {
            scope,
            dir,
            ext,
            sort_by,
            filter,
            top,
            format,
            stdin,
            depth,
            depth_guard,
            force,
            ignore_case,
        } => {
            let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &dir);
            let separator = command_separators.last().copied().unwrap_or('.');

            main_command_trace_stats_impl::execute(
                scope,
                dir,
                ext,
                sort_by,
                filter,
                top,
                format,
                stdin,
                depth,
                depth_guard,
                force,
                ignore_case,
                separator,
                cli.json,
            )
        }
        Commands::Merge {
            patterns,
            sep,
            inputs,
            base,
            dir,
            max_depth,
            ascii,
            count,
            stdin,
        } => {
            // Parse explicit separators; merge validation decides whether counts are sufficient.
            let separators = CliSeparatorPolicy::parse_explicit_separators(&sep);

            let use_file_inputs = !inputs.is_empty();

            if stdin {
                if !patterns.is_empty() {
                    eprintln!("Error: Cannot mix --pattern with --stdin");
                    std::process::exit(2);
                }
                if !inputs.is_empty() {
                    eprintln!("Error: Cannot mix file inputs with --stdin");
                    std::process::exit(2);
                }
                if base.is_none() {
                    eprintln!("Error: --base is required when using --stdin");
                    std::process::exit(2);
                }
            }

            if use_file_inputs {
                if !patterns.is_empty() {
                    eprintln!("Error: Cannot mix --pattern with file inputs");
                    std::process::exit(2);
                }
                if inputs.len() != separators.len() {
                    eprintln!("Error: Number of FILE inputs must match number of --sep arguments");
                    eprintln!("  Files provided: {}", inputs.len());
                    eprintln!("  Separators provided: {}", separators.len());
                    std::process::exit(2);
                }
                if base.is_none() {
                    eprintln!("Error: --base is required when using file inputs");
                    std::process::exit(2);
                }
            } else if !stdin {
                // Pattern mode validation (not file mode, not stdin mode)
                if patterns.is_empty() {
                    eprintln!("Error: At least one --pattern is required in pattern mode");
                    std::process::exit(2);
                }
                if patterns.len() != separators.len() {
                    eprintln!("Error: Number of --pattern and --sep arguments must match");
                    eprintln!("  Patterns provided: {}", patterns.len());
                    eprintln!("  Separators provided: {}", separators.len());
                    std::process::exit(2);
                }
            }

            main_command_merge_impl::execute(
                patterns,
                separators,
                inputs,
                base,
                dir,
                max_depth,
                replace_default,
                show_sep,
                !ascii,
                count,
                cli.json,
                stdin,
            )
        }

        Commands::Trait { command, dir } => {
            main_command_trait_impl::execute(command, dir, cli.json)
        }

        Commands::Init {
            dir,
            analyze,
            force,
        } => main_command_init_impl::execute(dir, analyze, force, cli.json),

        Commands::Lane { name, dir } => main_command_lane_impl::execute(name, dir, cli.json),

        Commands::Reveal { lane, dir } => main_command_reveal_impl::execute(lane, dir, cli.json),

        Commands::Psyche {
            dir,
            format,
            filter,
            stale_seconds,
        } => main_command_psyche_impl::execute(dir, format, filter, stale_seconds),

        Commands::Watch { command, dir } => {
            main_command_watch_query_impl::execute(command, dir, cli.json)
        }

        Commands::Flatten {
            file,
            stdin,
            format,
            max_depth,
            filter,
        } => main_command_flatten_impl::execute(
            file,
            stdin,
            format,
            max_depth,
            filter,
            fallback_separator,
            cli.json,
            cli.color,
        ),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(2);
    }
}
