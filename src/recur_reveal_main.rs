use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "recur-reveal",
    version,
    about = "Local Reveal configuration and inert context packets"
)]
struct Cli {
    #[arg(short = 'd', long = "dir", default_value = ".", global = true)]
    dir: PathBuf,
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    /// Add only missing association tables; preserve existing configuration.
    Init {
        #[arg(long)]
        dry_run: bool,
    },
    /// Prepare local guidance without activating or executing it.
    Next {
        id: String,
        #[arg(long="type", value_parser=["agent","persona"])]
        kind: Option<String>,
        #[arg(long, default_value_t = 16)]
        max_files: usize,
        #[arg(long, default_value_t = 65536)]
        max_bytes: usize,
    },
}
fn main() {
    let cli = Cli::parse();
    let result = (|| -> anyhow::Result<serde_json::Value> {
        match cli.command {
            Command::Init { dry_run } => recur::reveal_profiles::init(&cli.dir, dry_run),
            Command::Next {
                id,
                kind,
                max_files,
                max_bytes,
            } => Ok(recur::reveal_profiles::Registry::load(&cli.dir)?.packet(
                &id,
                kind.as_deref(),
                max_files,
                max_bytes,
            )),
        }
    })();
    let (value, code) = match result {
        Ok(value) => {
            let code = if value["state"] == "blocked" { 1 } else { 0 };
            (value, code)
        }
        Err(error) => (
            serde_json::json!({"schema":"recur-reveal-error-v1","diagnostics":[format!("{error:#}")],"mutation":"none"}),
            2,
        ),
    };
    // Human output is the same reviewable structured packet, never a command.
    println!("{}", serde_json::to_string_pretty(&value).unwrap());
    std::process::exit(code);
}
