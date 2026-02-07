//! recur - Recursive Hierarchical Search Tool
//!
//! In honor of Dennis M. Ritchie's 1968 PhD thesis on recursive hierarchies.
//! 58 years of recursive hierarchical thinking, now in your terminal.
//!
//! Main CLI entry point

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process;

mod main_command_stats_impl;
mod main_command_stats_stdin;
mod main_command_files_impl;
mod main_command_files_stdin;
mod main_command_children_impl;
mod main_command_checkpoint_impl;

use recur::output::TerminalFormatter;
use recur::parser::HierarchyPattern;
use recur::search::{
    read_paths_from_stdin, ContentSearcher, FileSearcher, IdentifierSearcher, SearchOptions,
};
use recur::tree::HierarchyTree;

#[derive(Parser)]
#[command(name = "recur")]
#[command(about = "Recursive hierarchical search tool for modern codebases\n\nHonoring Dennis M. Ritchie's 1968 PhD thesis on recursive hierarchies (58 years)", long_about = None)]
#[command(version)]
#[command(
    after_help = "Dennis Ritchie (1941-2011) pioneered recursive hierarchical structures in his 1968 thesis.\n58 years later, recur brings hierarchical understanding to code search.\n\nHomepage: https://github.com/userlevelup/recur"
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
    /// May be provided multiple times; the last value wins.
    #[arg(long, global = true, value_name = "CHAR")]
    sep: Vec<String>,
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

    /// Optional checkpoint/log workflow for dogfooding state.
    ///
    /// Examples:
    ///   recur checkpoint --snapshot
    ///   recur checkpoint --emit-parallel
    ///   recur checkpoint --append-parallel --checkpoint-id ck-children-01
    Checkpoint {
        /// Print checkpoint snapshot (git + lane state + separator)
        #[arg(long)]
        snapshot: bool,

        /// Run `cargo test --quiet` as part of checkpoint
        #[arg(long)]
        run_tests: bool,

        /// Emit parallel-lane checkpoint entry to stdout
        #[arg(long)]
        emit_parallel: bool,

        /// Append parallel-lane checkpoint entry to file
        #[arg(long)]
        append_parallel: bool,

        /// Optional checkpoint ID (default: ck-<unix-seconds>)
        #[arg(long, value_name = "ID")]
        checkpoint_id: Option<String>,

        /// File path for appended parallel-lane entries
        #[arg(long, default_value = "docs/main.dogfooding.parallel.history.md")]
        parallel_log: PathBuf,

        /// Source hierarchy separator for src lane queries (default: '_')
        #[arg(long, value_name = "CHAR", default_value = "_")]
        src_sep: String,
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
    },
}

fn main() {
    let cli = Cli::parse();

    // Parse separator (take first character, default to '.').
    // If repeated, the last --sep value wins.
    let separator = cli
        .sep
        .last()
        .and_then(|s| s.chars().next())
        .unwrap_or('.');

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
        } => main_command_files_impl::execute(
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
        ),
        Commands::Find {
            query,
            scope,
            dir,
            context,
            ignore_case,
            regex,
            ext,
            stdin,
        } => cmd_find(
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
        ),
        Commands::Tree {
            base,
            dir,
            depth,
            count,
            ascii,
            stdin,
        } => cmd_tree(base, dir, depth, count, !ascii, stdin, separator, cli.json),
        Commands::Related {
            filename,
            dir,
            exclude_self,
            stdin,
        } => cmd_related(
            filename,
            dir,
            exclude_self,
            stdin,
            separator,
            cli.json,
            cli.color,
        ),
        Commands::Children {
            parent,
            dir,
            count,
            stdin,
        } => main_command_children_impl::execute(
            parent, dir, count, stdin, separator, cli.json, cli.color,
        ),
        Commands::Id {
            pattern,
            dir,
            ext,
            context,
            ignore_case,
            stdin,
        } => cmd_id(
            pattern,
            dir,
            ext,
            context,
            ignore_case,
            stdin,
            separator,
            cli.json,
            cli.color,
        ),
        Commands::Stats {
            pattern,
            dir,
            level,
            ext,
            stdin,
        } => main_command_stats_impl::execute(
            pattern, dir, level, ext, stdin, separator, cli.json, cli.color,
        ),
        Commands::Checkpoint {
            snapshot,
            run_tests,
            emit_parallel,
            append_parallel,
            checkpoint_id,
            parallel_log,
            src_sep,
        } => {
            let src_separator = src_sep.chars().next().unwrap_or('_');
            main_command_checkpoint_impl::execute(
                emit_parallel,
                append_parallel,
                checkpoint_id,
                parallel_log,
                src_separator,
                snapshot,
                run_tests,
            )
        },
        Commands::Callers {
            function,
            scope,
            dir,
            context,
            ignore_case,
            ext,
            count,
            stdin,
        } => cmd_callers(
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
        ),
        Commands::Callees {
            function,
            scope,
            dir,
            context,
            ignore_case,
            ext,
            count,
            stdin,
        } => cmd_callees(
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
        ),
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
        } => cmd_trace(
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
            separator,
            cli.json,
            cli.color,
        ),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(2);
    }
}

fn cmd_find(
    query: String,
    scope: String,
    dir: PathBuf,
    context: usize,
    ignore_case: bool,
    use_regex: bool,
    ext: Option<String>,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let scope_pattern = HierarchyPattern::parse_with_separator(&scope, separator)?;
    let scope_pattern = if ignore_case {
        scope_pattern.case_insensitive()
    } else {
        scope_pattern
    };

    let mut options = SearchOptions {
        root: dir.clone(),
        case_insensitive: ignore_case,
        context_lines: context,
        ..Default::default()
    };

    if let Some(ext_str) = ext {
        options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    if stdin {
        options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    let searcher = ContentSearcher::new(options);

    let results = if use_regex {
        let regex = regex::Regex::new(&query)?;
        searcher.search_regex(&regex, &scope_pattern)
    } else {
        searcher.search(&query, &scope_pattern)
    };

    if json {
        let output = recur::output::JsonFormatter::format_search_results(&results);
        println!("{}", output);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_search_results(&results);
    }

    if results.is_empty() {
        process::exit(1);
    }

    Ok(())
}

fn cmd_tree(
    base: String,
    dir: PathBuf,
    max_depth: Option<usize>,
    show_count: bool,
    unicode: bool,
    stdin: bool,
    separator: char,
    json: bool,
) -> anyhow::Result<()> {
    // Find all files starting with base (recursive)
    // Use the separator in the pattern itself
    let pattern =
        HierarchyPattern::parse_with_separator(&format!("{}{}**", base, separator), separator)?;

    let files = if stdin {
        // Read paths from stdin and filter by pattern
        read_resolved_paths_from_stdin(&dir)?
            .into_iter()
            .filter(|p| {
                // Extract hierarchical name from filename
                if let Some(filename) = p.file_name().and_then(|n| n.to_str()) {
                    let name_without_ext = filename
                        .rsplit_once('.')
                        .map(|(name, _)| name)
                        .unwrap_or(filename);
                    let hier_name = recur::parser::HierarchicalName::with_separator(
                        name_without_ext,
                        separator,
                    );
                    pattern.matches(&hier_name)
                } else {
                    false
                }
            })
            .collect()
    } else {
        // Use filesystem search
        let options = SearchOptions {
            root: dir,
            max_depth,
            ..Default::default()
        };

        let searcher = FileSearcher::new(options);
        searcher.find(&pattern)
    };

    if files.is_empty() {
        eprintln!("No files found starting with '{}'", base);
        process::exit(1);
    }

    let tree = HierarchyTree::from_paths_with_separator(base, &files, separator);

    if json {
        println!("{}", tree.to_json());
    } else {
        print!("{}", tree.to_string(unicode));

        if show_count {
            let stats = tree.stats();
            println!(
                "\n{} files, {} directories (recursive)",
                stats.total_files, stats.total_dirs
            );
        }
    }

    Ok(())
}

fn cmd_related(
    filename: String,
    dir: PathBuf,
    exclude_self: bool,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let stem = filename
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(&filename);
    let base = stem
        .rsplit_once(separator)
        .map(|(parent, _)| parent)
        .unwrap_or(stem);
    let pattern =
        HierarchyPattern::parse_with_separator(&format!("{}{}*", base, separator), separator)?;

    let mut options = SearchOptions {
        root: dir.clone(),
        ..Default::default()
    };
    if stdin {
        options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    let searcher = FileSearcher::new(options);
    let mut files = searcher.find(&pattern);

    // Filter out the input file if exclude_self is true
    if exclude_self {
        let input_name = Path::new(&filename)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&filename);
        files.retain(|path| {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                file_name != input_name
            } else {
                true
            }
        });
    }

    if json {
        let output = recur::output::JsonFormatter::format_file_list(&files);
        println!("{}", output);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_file_list(&files);
    }

    if files.is_empty() {
        process::exit(1);
    }

    Ok(())
}

fn cmd_id(
    pattern: String,
    dir: PathBuf,
    ext: Option<String>,
    context: usize,
    ignore_case: bool,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let mut options = SearchOptions {
        root: dir.clone(),
        case_insensitive: ignore_case,
        context_lines: context,
        ..Default::default()
    };

    if let Some(ext_str) = ext {
        options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    if stdin {
        options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    let searcher = IdentifierSearcher::new(options);
    let pattern_parsed = HierarchyPattern::parse_with_separator(&pattern, separator)?;
    let results = searcher.search(&pattern_parsed);

    if json {
        let output = recur::output::JsonFormatter::format_search_results(&results);
        println!("{}", output);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_search_results(&results);
    }

    if results.is_empty() {
        process::exit(1);
    }

    Ok(())
}

fn cmd_callers(
    function: String,
    scope: String,
    dir: PathBuf,
    context: usize,
    ignore_case: bool,
    ext: Option<String>,
    count_only: bool,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    use recur::output::{JsonFormatter, TerminalFormatter};
    use recur::search::CallerSearcher;

    // Parse scope pattern
    let scope_pattern = HierarchyPattern::parse_with_separator(&scope, separator)?;
    let scope_pattern = if ignore_case {
        scope_pattern.case_insensitive()
    } else {
        scope_pattern
    };

    // Set up search options
    let mut options = SearchOptions {
        root: dir.clone(),
        case_insensitive: ignore_case,
        context_lines: context,
        ..Default::default()
    };

    // Parse extension filter
    if let Some(ext_str) = ext {
        options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    if stdin {
        options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    // Create caller searcher
    let searcher = CallerSearcher::new(options);

    // Perform search
    let results = searcher.find_callers(&function, &scope_pattern)?;

    // Handle count-only mode
    if count_only {
        println!("{}", results.len());
        if results.is_empty() {
            process::exit(1);
        }
        return Ok(());
    }

    // Format and output results
    if json {
        let output = JsonFormatter::format_caller_results(&results);
        println!("{}", output);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_caller_results(&results);
    }

    // Exit with appropriate code
    if results.is_empty() {
        process::exit(1);
    }

    Ok(())
}

fn cmd_callees(
    function: String,
    scope: String,
    dir: PathBuf,
    context: usize,
    ignore_case: bool,
    ext: Option<String>,
    count_only: bool,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    use recur::output::{JsonFormatter, TerminalFormatter};
    use recur::search::CalleeSearcher;

    // Parse scope pattern
    let scope_pattern = HierarchyPattern::parse_with_separator(&scope, separator)?;
    let scope_pattern = if ignore_case {
        scope_pattern.case_insensitive()
    } else {
        scope_pattern
    };

    // Set up search options
    let mut options = SearchOptions {
        root: dir.clone(),
        case_insensitive: ignore_case,
        context_lines: context,
        ..Default::default()
    };

    // Parse extension filter
    if let Some(ext_str) = ext {
        options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    if stdin {
        options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    // Create callee searcher
    let searcher = CalleeSearcher::new(options);

    // Perform search
    let results = searcher.find_callees(&function, &scope_pattern)?;

    // Handle count-only mode
    if count_only {
        println!("{}", results.len());
        if results.is_empty() {
            process::exit(1);
        }
        return Ok(());
    }

    // Format and output results
    if json {
        let output = JsonFormatter::format_callee_results(&results);
        println!("{}", output);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_callee_results(&results);
    }

    // Exit with appropriate code
    if results.is_empty() {
        process::exit(1);
    }

    Ok(())
}

fn cmd_trace(
    function: String,
    scope: String,
    dir: PathBuf,
    depth: usize,
    direction_str: String,
    ignore_case: bool,
    ext: Option<String>,
    max_width: usize,
    verbose: bool,
    format_str: String,
    pick: Option<usize>,
    scope_alias: Vec<String>,
    stdin: bool,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    use recur::output::{JsonFormatter, TerminalFormatter};
    use recur::search::{TraceDirection, TraceOptions, TraceSearcher};

    // Validate depth
    if depth > 5 {
        anyhow::bail!("Maximum depth is 5 (to prevent exponential explosion)");
    }

    // Parse direction
    let direction = match direction_str.to_lowercase().as_str() {
        "callees" => TraceDirection::Callees,
        "callers" => TraceDirection::Callers,
        "both" => TraceDirection::Both,
        _ => anyhow::bail!(
            "Invalid direction '{}'. Must be 'callees', 'callers', or 'both'",
            direction_str
        ),
    };

    // Parse format
    let output_format = match format_str.to_lowercase().as_str() {
        "tree" => recur::output::TraceFormat::Tree,
        "flat" => recur::output::TraceFormat::Flat,
        "graph" => recur::output::TraceFormat::Graph,
        _ => anyhow::bail!(
            "Invalid format '{}'. Must be 'tree', 'flat', or 'graph'",
            format_str
        ),
    };

    let resolved_scope = apply_scope_alias(&scope, &scope_alias)?;

    // Parse scope pattern
    let scope_pattern = HierarchyPattern::parse_with_separator(&resolved_scope, separator)?;
    let scope_pattern = if ignore_case {
        scope_pattern.case_insensitive()
    } else {
        scope_pattern
    };

    // Set up search options
    let mut search_options = SearchOptions {
        root: dir.clone(),
        case_insensitive: ignore_case,
        ..Default::default()
    };

    // Parse extension filter
    if let Some(ext_str) = ext.as_deref() {
        search_options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    if stdin {
        search_options.input_files = Some(read_resolved_paths_from_stdin(&dir)?);
    }

    // Create trace options
    let trace_options = TraceOptions {
        max_width,
        verbose,
        pick,
    };

    // Handle both directions by running callers + callees separately
    if direction == TraceDirection::Both {
        let mut caller_searcher = TraceSearcher::new(search_options.clone(), trace_options.clone());
        let callers_result =
            caller_searcher.trace(&function, &scope_pattern, TraceDirection::Callers, depth)?;

        let mut callee_searcher = TraceSearcher::new(search_options, trace_options);
        let callees_result =
            callee_searcher.trace(&function, &scope_pattern, TraceDirection::Callees, depth)?;

        if json {
            let output = JsonFormatter::format_trace_result_both(&callers_result, &callees_result);
            println!("{}", output);
            if callees_result.root.path.as_os_str().is_empty() {
                process::exit(1);
            }
            return Ok(());
        } else {
            if callees_result.root.path.as_os_str().is_empty() {
                print_trace_not_found(&function, &resolved_scope, ext.as_deref());
                process::exit(1);
            }

            let mut formatter = TerminalFormatter::new(color);
            formatter.print_trace_both(&callers_result, &callees_result, verbose)?;
        }

        return Ok(());
    }

    // Create trace searcher
    let mut searcher = TraceSearcher::new(search_options, trace_options);

    // Perform trace
    let trace_result = searcher.trace(&function, &scope_pattern, direction, depth)?;

    // Output results
    if json {
        let output = JsonFormatter::format_trace_result(&trace_result);
        println!("{}", output);
        if trace_result.root.path.as_os_str().is_empty() {
            process::exit(1);
        }
        return Ok(());
    }

    if trace_result.root.path.as_os_str().is_empty() {
        print_trace_not_found(&function, &resolved_scope, ext.as_deref());
        process::exit(1);
    }

    let mut formatter = TerminalFormatter::new(color);
    formatter.print_trace_result(&trace_result, output_format, verbose)?;

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

fn apply_scope_alias(scope: &str, aliases: &[String]) -> anyhow::Result<String> {
    if aliases.is_empty() {
        return Ok(scope.to_string());
    }

    let mut map = std::collections::HashMap::new();
    for alias in aliases {
        let Some((key, value)) = alias.split_once('=') else {
            anyhow::bail!(
                "Invalid --scope-alias '{}'. Expected format name=pattern",
                alias
            );
        };
        map.insert(key.trim(), value.trim());
    }

    if let Some(replacement) = map.get(scope) {
        Ok((*replacement).to_string())
    } else {
        Ok(scope.to_string())
    }
}

fn print_trace_not_found(function: &str, scope: &str, ext: Option<&str>) {
    println!("No symbols found for '{}'.", function);
    if let Some(ext) = ext {
        println!(
            "Hint: if this is a string reference, try: recur find \"{}\" --scope \"{}\" --ext {}",
            function, scope, ext
        );
    } else {
        println!(
            "Hint: if this is a string reference, try: recur find \"{}\" --scope \"{}\"",
            function, scope
        );
    }
}
