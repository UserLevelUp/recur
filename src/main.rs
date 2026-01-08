//! recur - Recursive Hierarchical Search Tool
//!
//! In honor of Dennis M. Ritchie's 1968 PhD thesis on recursive hierarchies.
//! 58 years of recursive hierarchical thinking, now in your terminal.
//!
//! Main CLI entry point

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;
use std::fs;
use std::io::BufRead;

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

        /// Minimum depth to search (0=base level)
        #[arg(long, default_value = "0")]
        min_depth: usize,

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

        /// Lines of context to show around matches
        #[arg(short = 'C', long, default_value = "0")]
        context: usize,

        /// Case-insensitive search
        #[arg(short, long)]
        ignore_case: bool,
    },

    /// Show statistics for a hierarchy (files, lines, depth)
    ///
    /// Examples:
    ///   recur stats "ServiceName"              # Show summary with depth breakdown
    ///   recur stats "ServiceName" -l 1         # List files at depth level 1
    ///   recur stats "ServiceName" -l 2         # List files at depth level 2
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
    },
}

fn main() {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Commands::Files { pattern, dir, ext, ignore_case, min_depth, max_depth, count } => {
            cmd_files(pattern, dir, ext, ignore_case, min_depth, max_depth, count, cli.json, cli.color)
        }
        Commands::Find { query, scope, dir, context, ignore_case, regex, ext } => {
            cmd_find(query, scope, dir, context, ignore_case, regex, ext, cli.json, cli.color)
        }
        Commands::Tree { base, dir, depth, count, ascii } => {
            cmd_tree(base, dir, depth, count, !ascii, cli.json)
        }
        Commands::Related { filename, dir, exclude_self } => {
            cmd_related(filename, dir, exclude_self, cli.json, cli.color)
        }
        Commands::Children { parent, dir, count } => {
            cmd_children(parent, dir, count, cli.json, cli.color)
        }
        Commands::Id { pattern, dir, ext, context, ignore_case } => {
            cmd_id(pattern, dir, ext, context, ignore_case, cli.json, cli.color)
        }
        Commands::Stats { pattern, dir, level, ext } => {
            cmd_stats(pattern, dir, level, ext, cli.json, cli.color)
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
    min_depth: usize,
    max_depth: Option<usize>,
    count_only: bool,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    // Validate depth constraints
    if let Some(max) = max_depth {
        if min_depth > max {
            anyhow::bail!("--min-depth ({}) cannot be greater than --max-depth ({})", min_depth, max);
        }
    }

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
    let all_files = searcher.find(&pattern);

    // Filter by min_depth
    let base_depth = pattern.raw.matches('.').count();
    let files: Vec<_> = all_files.into_iter().filter(|path| {
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            let hier_name = filename.rsplit_once('.')
                .map(|(name, _)| name)
                .unwrap_or(filename);
            let file_depth = hier_name.matches('.').count();
            let relative_depth = file_depth.saturating_sub(base_depth);
            relative_depth >= min_depth
        } else {
            false
        }
    }).collect();

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
    exclude_self: bool,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let options = SearchOptions {
        root: dir.clone(),
        ..Default::default()
    };

    let searcher = FileSearcher::new(options);
    let mut files = searcher.find_related(&filename);

    // Filter out the input file if exclude_self is true
    if exclude_self {
        files.retain(|path| {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                file_name != filename
            } else {
                true
            }
        });
    }

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
    context: usize,
    ignore_case: bool,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let mut options = SearchOptions {
        root: dir,
        case_insensitive: ignore_case,
        context_lines: context,
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

fn cmd_stats(
    pattern: String,
    dir: PathBuf,
    level: Option<usize>,
    ext: Option<String>,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    let mut options = SearchOptions {
        root: dir,
        ..Default::default()
    };

    if let Some(ext_str) = ext {
        options.extensions = ext_str.split(',').map(|s| s.trim().to_string()).collect();
    }

    let pattern_parsed = HierarchyPattern::parse(&pattern)?;
    let searcher = FileSearcher::new(options);
    let files = searcher.find(&pattern_parsed);

    if files.is_empty() {
        if !json {
            eprintln!("No files found matching pattern: {}", pattern);
        }
        process::exit(1);
    }

    // Group files by depth relative to the pattern base
    let base_depth = pattern.matches('.').count();
    let mut depth_map: std::collections::HashMap<usize, Vec<(PathBuf, usize)>> = std::collections::HashMap::new();
    let mut total_lines = 0;

    for file_path in &files {
        if let Some(filename) = file_path.file_name().and_then(|n| n.to_str()) {
            // Remove extension to get hierarchy name
            let hier_name = filename.rsplit_once('.')
                .map(|(name, _)| name)
                .unwrap_or(filename);
            
            let file_depth = hier_name.matches('.').count();
            let relative_depth = file_depth.saturating_sub(base_depth);

            // Count lines
            let line_count = if let Ok(file) = fs::File::open(file_path) {
                let reader = std::io::BufReader::new(file);
                let count = reader.lines().count();
                total_lines += count;
                count
            } else {
                0
            };

            depth_map.entry(relative_depth)
                .or_insert_with(Vec::new)
                .push((file_path.clone(), line_count));
        }
    }

    let max_depth = *depth_map.keys().max().unwrap_or(&0);

    if json {
        let output = serde_json::json!({
            "pattern": pattern,
            "total_files": files.len(),
            "total_lines": total_lines,
            "max_depth": max_depth,
            "depth_breakdown": (0..=max_depth).map(|d| {
                let files_at_depth = depth_map.get(&d).map(|v| v.len()).unwrap_or(0);
                serde_json::json!({
                    "depth": d,
                    "file_count": files_at_depth
                })
            }).collect::<Vec<_>>(),
            "files": if let Some(lvl) = level {
                depth_map.get(&lvl).map(|files_at_level| {
                    files_at_level.iter().map(|(path, lines)| {
                        serde_json::json!({
                            "path": path.display().to_string(),
                            "lines": lines
                        })
                    }).collect::<Vec<_>>()
                })
            } else {
                None
            }
        });
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
        use std::io::Write;

        let mut stdout = StandardStream::stdout(if color { ColorChoice::Auto } else { ColorChoice::Never });

        if color {
            let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true));
        }
        let _ = writeln!(stdout, "\nStatistics for: {}", pattern);
        if color {
            let _ = stdout.reset();
        }

        let _ = writeln!(stdout, "  Total files: {}", files.len());
        let _ = writeln!(stdout, "  Total lines: {}", total_lines);
        let _ = writeln!(stdout, "  Max depth:   {}", max_depth);

        // Show depth breakdown
        if level.is_none() {
            let _ = writeln!(stdout, "\n  Depth breakdown:");
            for d in 0..=max_depth {
                let count = depth_map.get(&d).map(|v| v.len()).unwrap_or(0);
                if count > 0 {
                    if color {
                        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
                    }
                    let _ = write!(stdout, "    Level {}", d);
                    if color {
                        let _ = stdout.reset();
                    }
                    let _ = writeln!(stdout, ": {} files", count);
                }
            }
            let _ = writeln!(stdout, "\n  Use -l <level> to see files at a specific depth");
        } else if let Some(lvl) = level {
            // Get terminal height
            let terminal_height = if let Some((_, terminal_size::Height(h))) = terminal_size::terminal_size() {
                h as usize
            } else {
                24
            };

            let available_lines = terminal_height.saturating_sub(10);

            if let Some(files_at_level) = depth_map.get(&lvl) {
                let _ = writeln!(stdout, "\n  Files at depth level {}:", lvl);
                
                // Sort by line count descending
                let mut sorted_files = files_at_level.clone();
                sorted_files.sort_by_key(|(_, lines)| std::cmp::Reverse(*lines));

                let files_to_show = sorted_files.len().min(available_lines);
                let remaining = sorted_files.len().saturating_sub(available_lines);

                for (path, lines) in sorted_files.iter().take(files_to_show) {
                    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        if color {
                            let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
                        }
                        let _ = write!(stdout, "    {:6}", lines);
                        if color {
                            let _ = stdout.reset();
                        }
                        let _ = write!(stdout, " lines  ");
                        if color {
                            let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                        }
                        let _ = writeln!(stdout, "{}", filename);
                        if color {
                            let _ = stdout.reset();
                        }
                    }
                }

                if remaining > 0 {
                    if color {
                        let _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
                    }
                    let _ = writeln!(stdout, "\n    ... and {} more files (terminal shows {} of {})", 
                        remaining, files_to_show, sorted_files.len());
                    if color {
                        let _ = stdout.reset();
                    }
                }
            } else {
                let _ = writeln!(stdout, "\n  No files at depth level {}", lvl);
            }
        }

        let _ = writeln!(stdout, "");
    }

    Ok(())
}
