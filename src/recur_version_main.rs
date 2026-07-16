//! recur-version - artifact versioning companion binary for recur.
//!
//! Keeps write-side snapshot and manifest operations separate from pure
//! `recur version` queries.

use clap::Parser;
use std::path::PathBuf;
use std::process;

#[path = "main_command_version_impl.rs"]
mod main_command_version_impl;

#[derive(Parser)]
#[command(name = "recur-version")]
#[command(
    about = "Save artifact versions and update Recur version manifests",
    long_about = None
)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: main_command_version_impl::VersionWriteSubcommand,

    /// Project root or directory containing `.recur/`
    #[arg(short = 'd', long, default_value = ".", global = true)]
    dir: PathBuf,

    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,
}

fn main() {
    let cli = Cli::parse();
    let result = main_command_version_impl::execute_write(cli.command, cli.dir, cli.json);

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(2);
    }
}
