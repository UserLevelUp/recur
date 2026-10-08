//! recur-watch - dedicated watch/subscription binary for recur.
//!
//! Keeps long-running event streaming separate from synchronous recur queries.

use clap::{Parser, Subcommand};
use recur::r#trait::{CliSeparatorPolicy, SeparatorCapable};
use std::path::PathBuf;
use std::process;

#[path = "main_command_watch_impl.rs"]
mod main_command_watch_impl;
mod recur_watch_dispatch;

#[derive(Subcommand)]
enum Commands {
    /// Register or reconcile durable Eventness subscriptions (never launches workers)
    Topic {
        #[command(subcommand)]
        command: TopicCommand,
    },
    /// Coordinate configured asynchronous Warp assignments until quiescent
    Dispatch {
        warp: String,
        #[arg(long)]
        confirm: bool,
        /// Limit scheduler passes; zero runs until no active work remains
        #[arg(long, default_value_t = 0)]
        cycles: u64,
    },
}

#[derive(Subcommand)]
enum TopicCommand {
    /// Bind a trace pattern to an explicit Eventness directory
    Create {
        topic: String,
        #[arg(long)]
        warp: Option<String>,
        #[arg(long)]
        filter: String,
        #[arg(long)]
        eventness_dir: PathBuf,
        #[arg(long)]
        confirm: bool,
    },
    /// Persist interest; existing publications remain discoverable
    Subscribe {
        topic: String,
        #[arg(long)]
        id: String,
        #[arg(long)]
        confirm: bool,
    },
    /// Reconcile artifacts and commit a bounded batch before emitting trace IDs
    Drain {
        topic: String,
        #[arg(long)]
        id: String,
        #[arg(long, default_value_t = 100)]
        max_events: usize,
        #[arg(long)]
        confirm: bool,
    },
    /// Read a previously committed batch after a lost response; never advances the cursor
    Replay {
        topic: String,
        #[arg(long)]
        id: String,
        #[arg(long)]
        sequence: u64,
    },
}

#[derive(Parser)]
#[command(name = "recur-watch", subcommand_negates_reqs = true)]
#[command(
    about = "Watch vault or project files for matching hierarchical events",
    long_about = None
)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Watch runtime id used for .recur/watch status records
    #[arg(long, value_name = "ID")]
    id: Option<String>,

    /// Hierarchical pattern to subscribe to
    #[arg(long, value_name = "PATTERN", required = true)]
    filter: Option<String>,

    /// Root directory to watch
    #[arg(short = 'd', long, default_value = ".", global = true)]
    dir: PathBuf,

    /// Output format: oneline or json
    #[arg(long, default_value = "oneline", value_name = "FORMAT")]
    format: String,

    /// Poll interval in integer seconds; omit for filesystem-event streaming mode
    #[arg(long, value_name = "SECONDS", allow_hyphen_values = true)]
    poll_framing: Option<String>,

    /// Use color in output
    #[arg(long, default_value = "true")]
    color: bool,

    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,

    /// Hierarchy separator character (default: '.')
    /// Use '_' for Rust modules, '-' for kebab-case, ':' for namespaces
    /// May be provided multiple times for multi-separator queries.
    #[arg(long, value_name = "CHAR")]
    sep: Vec<String>,

    /// When using multiple separators, normalize output to this separator
    #[arg(long, value_name = "CHAR")]
    sep_replace_default: Option<String>,

    /// Show which separator was used for each file (e.g., [.] or [_])
    #[arg(long)]
    show_sep: bool,
}

fn main() {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        let result = match command {
            Commands::Dispatch {
                warp,
                confirm,
                cycles,
            } => recur_watch_dispatch::coordinate(&cli.dir, &warp, confirm, cycles),
            Commands::Topic { command } => {
                use recur::watch_eventness as topics;
                let value = match command {
                    TopicCommand::Create {
                        topic,
                        warp,
                        filter,
                        eventness_dir,
                        confirm,
                    } => topics::create(
                        &cli.dir,
                        &topic,
                        warp.as_deref(),
                        &filter,
                        &eventness_dir,
                        confirm,
                    ),
                    TopicCommand::Subscribe { topic, id, confirm } => {
                        topics::subscribe(&cli.dir, &topic, &id, confirm)
                    }
                    TopicCommand::Drain {
                        topic,
                        id,
                        max_events,
                        confirm,
                    } => topics::drain(&cli.dir, &topic, &id, max_events, confirm),
                    TopicCommand::Replay {
                        topic,
                        id,
                        sequence,
                    } => topics::replay(&cli.dir, &topic, &id, sequence),
                };
                value.and_then(|value| {
                    use std::io::Write;
                    let mut out = std::io::stdout().lock();
                    serde_json::to_writer(&mut out, &value)?;
                    writeln!(out)?;
                    out.flush()?;
                    Ok(())
                })
            }
        };
        if let Err(error) = result {
            eprintln!("Error: {error:#}");
            process::exit(2);
        }
        return;
    }
    let command_separators = CliSeparatorPolicy::resolve_command_separators(&cli.sep, &cli.dir);
    let separator = command_separators.last().copied().unwrap_or('.');

    let result = main_command_watch_impl::execute(
        cli.id,
        cli.filter.unwrap(),
        cli.dir,
        cli.format,
        cli.poll_framing,
        separator,
    );

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(2);
    }
}
