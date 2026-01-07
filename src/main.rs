//! recur - Recursive Hierarchical Search Tool
//!
//! In honor of Dennis M. Ritchie's 1968 PhD thesis on recursive hierarchies.
//! 58 years of recursive hierarchical thinking, now in your terminal.
//!
//! Main CLI entry point

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

use recur::parser::HierarchyPattern;
use recur::search::{FileSearcher, ContentSearcher, IdentifierSearcher, SearchOptions};
use recur::tree::HierarchyTree;
use recur::output::TerminalFormatter;

#[derive(Parser)]
#[command(name = "recur")]
#[command(about = "Recursive hierarchical search tool for modern codebases\n\nHonoring Dennis M. Ritchie's 1968 PhD thesis on recursive hierarchies (58 years)", long_about = None)]
#[command(version)]
#[command(after_help = "Dennis Ritchie (1941-2011) pioneered recursive hierarchical structures in his 1968 thesis.\n58 years later, recur brings hierarchical understanding to code search.\n\nHomepage: https://github.com/userlevelup/recur")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Use color in output
    #[arg(long, global = true, default_value = "true")]
    color: bool,
    
    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Find files matching a recursive hierarchical pattern
    ///
    /// Examples:
    ///   recur files "Module.SubModule.*"
    ///   recur files "LevelController.CreateWizard3.Templates"
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
        
        /// Maximum depth to search recursively
        #[arg(long)]
        max_depth: Option<usize>,

        /// Show only the count of matching files
        #[arg(long)]
        count: bool,
    },
    
    /// Search for text within hierarchically-scoped files (recursive)
    ///
    /// Examples:
    ///   recur find "async" --scope "Controller.Api"
    ///   recur find "pattern" --scope "Module" -C 3
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
    },
    
    /// Show recursive hierarchy tree for files
    ///
    /// Examples:
    ///   recur tree "LevelController"
    ///   recur tree "ServiceName" --depth 2
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
    },
    
    /// Find files related to (siblings of) a given file in the hierarchy
    ///
    /// Examples:
    ///   recur related "Service.Module.Feature.cs"
    Related {
        /// Filename to find relatives of
        filename: String,
        
        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,
    },
    
    /// Find files that are children of a hierarchy (recursive)
    ///
    /// Examples:
    ///   recur children "Module.SubModule"
    Children {
        /// Parent hierarchy
        parent: String,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,

        /// Show only the count of matching files
        #[arg(long)]
        count: bool,
    },
    
    /// Search for hierarchical identifiers in file content (recursive)
    ///
    /// Examples:
    ///   recur id "config.database.*"
    ///   recur id "ulu.role.**" --ext ".cs,.json"
    Id {
        /// Hierarchical identifier pattern (recursive)
        pattern: String,
        
        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,
        
        /// File extensions to include
        #[arg(short, long)]
        ext: Option<String>,
        
        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Commands::Files { pattern, dir, ext, ignore_case, max_depth, count } => {
            cmd_files(pattern, dir, ext, ignore_case, max_depth, count, cli.json, cli.color)
        }
        Commands::Find { query, scope, dir, context, ignore_case, regex, ext } => {
            cmd_find(query, scope, dir, context, ignore_case, regex, ext, cli.json, cli.color)
        }
        Commands::Tree { base, dir, depth, count, ascii } => {
            cmd_tree(base, dir, depth, count, !ascii, cli.json)
        }
        Commands::Related { filename, dir } => {
            cmd_related(filename, dir, cli.json, cli.color)
        }
        Commands::Children { parent, dir, count } => {
            cmd_children(parent, dir, count, cli.json, cli.color)
        }
        Commands::Id { pattern, dir, ext, ignore_case } => {
            cmd_id(pattern, dir, ext, ignore_case, cli.json, cli.color)
        }
    };
    
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(2);
    }
}

fn cmd_files(
    pattern: String,
    dir: PathBuf,
    ext: Option<String>,
    ignore_case: bool,
    max_depth: Option<usize>,
    count_only: bool,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let pattern = HierarchyPattern::parse(&pattern)?;
    let pattern = if ignore_case { pattern.case_insensitive() } else { pattern };
    
    let mut options = SearchOptions {
        root: dir,
        case_insensitive: ignore_case,
        max_depth,
        ..Default::default()
    };
    
    if let Some(ext_str) = ext {
        options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }
    
    let searcher = FileSearcher::new(options);
    let files = searcher.find(&pattern);

    if count_only {
        println!("{} files", files.len());
    } else if json {
        let output = recur::output::JsonFormatter::format_file_list(&files);
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_file_list(&files);
    }

    // Exit code: 0 if found, 1 if not found
    if files.is_empty() {
        process::exit(1);
    }

    Ok(())
}

fn cmd_find(
    query: String,
    scope: String,
    dir: PathBuf,
    context: usize,
    ignore_case: bool,
    use_regex: bool,
    ext: Option<String>,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let scope_pattern = HierarchyPattern::parse(&scope)?;
    let scope_pattern = if ignore_case { scope_pattern.case_insensitive() } else { scope_pattern };
    
    let mut options = SearchOptions {
        root: dir,
        case_insensitive: ignore_case,
        context_lines: context,
        ..Default::default()
    };
    
    if let Some(ext_str) = ext {
        options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
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
        println!("{}", serde_json::to_string_pretty(&output)?);
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
    json: bool,
) -> anyhow::Result<()> {
    // Find all files starting with base (recursive)
    let pattern = HierarchyPattern::parse(&format!("{}.**", base))?;
    
    let options = SearchOptions {
        root: dir,
        max_depth,
        ..Default::default()
    };
    
    let searcher = FileSearcher::new(options);
    let files = searcher.find(&pattern);
    
    if files.is_empty() {
        eprintln!("No files found starting with '{}'", base);
        process::exit(1);
    }
    
    let tree = HierarchyTree::from_paths(base, &files);
    
    if json {
        let output = tree.to_json();
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        print!("{}", tree.to_string(unicode));
        
        if show_count {
            let stats = tree.stats();
            println!("\n{} files, {} directories (recursive)", stats.total_files, stats.total_dirs);
        }
    }
    
    Ok(())
}

fn cmd_related(
    filename: String,
    dir: PathBuf,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let options = SearchOptions {
        root: dir,
        ..Default::default()
    };
    
    let searcher = FileSearcher::new(options);
    let files = searcher.find_related(&filename);
    
    if json {
        let output = recur::output::JsonFormatter::format_file_list(&files);
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_file_list(&files);
    }

    if files.is_empty() {
        process::exit(1);
    }

    Ok(())
}

fn cmd_children(
    parent: String,
    dir: PathBuf,
    count_only: bool,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let options = SearchOptions {
        root: dir,
        ..Default::default()
    };
    
    let searcher = FileSearcher::new(options);
    let files = searcher.find_children(&parent);

    if count_only {
        println!("{} files", files.len());
    } else if json {
        let output = recur::output::JsonFormatter::format_file_list(&files);
        println!("{}", serde_json::to_string_pretty(&output)?);
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
    ignore_case: bool,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let mut options = SearchOptions {
        root: dir,
        case_insensitive: ignore_case,
        ..Default::default()
    };
    
    if let Some(ext_str) = ext {
        options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }
    
    let searcher = IdentifierSearcher::new(options);
    let pattern_parsed = HierarchyPattern::parse(&pattern)?;
    let results = searcher.search(&pattern_parsed);
    
    if json {
        let output = recur::output::JsonFormatter::format_search_results(&results);
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_search_results(&results);
    }
    
    if results.is_empty() {
        process::exit(1);
    }
    
    Ok(())
}
