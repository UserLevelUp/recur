//! Project defaults for trace inspection. Core owns the actual queries.
use anyhow::{ensure, Context};
use clap::{Args, Parser, Subcommand};
use recur::companion_config_init::{self, InitSpec};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

const SPEC: InitSpec = InitSpec {
    schema: "recur-trace-id-init-v1",
    error_prefix: "TINIT",
};

const DEFAULTS: &str = r#"
[recur-trace-id]
scope = "**"
extensions = ".md,.recur,.jl,.rs"

[recur-trace-id.inventory]
max_results = 100
max_output_bytes = 16384

[recur-trace-id.context]
lines = 2
max_results = 20
max_output_bytes = 8192
"#;

#[derive(Parser)]
#[command(
    name = "recur-trace-id",
    version,
    about = "Project preferences for trace-ID inspection"
)]
struct Cli {
    /// Search root and starting point for project config discovery.
    #[arg(short = 'd', long, global = true, default_value = ".")]
    dir: PathBuf,
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Add missing inspection preferences to the nearest project config.
    Init {
        #[arg(long)]
        dry_run: bool,
    },
    /// List observed IDs and their files using project preferences.
    Inventory(Query),
    /// Extract bounded source context around selected IDs.
    Context {
        #[command(flatten)]
        query: Query,
        #[arg(long)]
        lines: Option<usize>,
    },
}

#[derive(Args)]
struct Query {
    pattern: String,
    #[arg(long)]
    scope: Option<String>,
    #[arg(short = 'e', long)]
    ext: Option<String>,
    #[arg(long)]
    select: Vec<String>,
    #[arg(long)]
    max_results: Option<usize>,
    #[arg(long)]
    max_output_bytes: Option<usize>,
}

#[derive(Deserialize)]
struct Config {
    #[serde(rename = "recur-trace-id")]
    trace: Preferences,
}

#[derive(Deserialize)]
struct Preferences {
    scope: String,
    extensions: String,
    inventory: OutputLimits,
    context: ContextPreferences,
}

#[derive(Deserialize)]
struct ContextPreferences {
    lines: usize,
    #[serde(flatten)]
    limits: OutputLimits,
}

#[derive(Deserialize)]
struct OutputLimits {
    max_results: usize,
    max_output_bytes: usize,
}

enum View {
    Inventory,
    Context { lines: Option<usize> },
}

fn render(text: &str) -> anyhow::Result<(String, Preferences)> {
    let updated = companion_config_init::with_defaults(text, DEFAULTS)?;
    let config: Config =
        toml::from_str(&updated).context("invalid trace inspection preferences")?;
    let preferences = config.trace;
    ensure!(
        !preferences.scope.trim().is_empty(),
        "recur-trace-id.scope cannot be blank"
    );
    ensure!(
        preferences.context.lines <= 20,
        "recur-trace-id.context.lines must be in 0..=20"
    );
    for (view, limits) in [
        ("inventory", &preferences.inventory),
        ("context", &preferences.context.limits),
    ] {
        ensure!(
            (1..=10_000).contains(&limits.max_results),
            "recur-trace-id.{view}.max_results must be in 1..=10000"
        );
        ensure!(
            (1..=1_048_576).contains(&limits.max_output_bytes),
            "recur-trace-id.{view}.max_output_bytes must be in 1..=1048576"
        );
    }
    Ok((updated, preferences))
}

fn run_query(root: &Path, json: bool, query: Query, view: View) -> anyhow::Result<i32> {
    let config = companion_config_init::read_config(root, SPEC)?.unwrap_or_default();
    let (_, preferences) = render(&config)?;
    let executable = format!("recur{}", std::env::consts::EXE_SUFFIX);
    let sibling = std::env::current_exe()?.with_file_name(&executable);
    let mut command = if sibling.is_file() {
        Command::new(sibling)
    } else {
        Command::new(executable)
    };
    command
        .arg("trace-id")
        .arg(&query.pattern)
        .arg("-d")
        .arg(root)
        .arg("--scope")
        .arg(query.scope.unwrap_or(preferences.scope))
        .arg("--ext")
        .arg(query.ext.unwrap_or(preferences.extensions));
    let limits = match view {
        View::Inventory => {
            command.arg("--list-ids");
            preferences.inventory
        }
        View::Context { lines } => {
            command
                .arg("--context")
                .arg(lines.unwrap_or(preferences.context.lines).to_string());
            preferences.context.limits
        }
    };
    command
        .arg("--max-results")
        .arg(query.max_results.unwrap_or(limits.max_results).to_string())
        .arg("--max-output-bytes")
        .arg(
            query
                .max_output_bytes
                .unwrap_or(limits.max_output_bytes)
                .to_string(),
        );
    for selector in query.select {
        command.arg("--select").arg(selector);
    }
    if json {
        command.arg("--json");
    }
    let status = command
        .status()
        .context("could not start the core recur executable")?;
    Ok(status.code().unwrap_or(2))
}

fn run(cli: Cli) -> anyhow::Result<i32> {
    match cli.command {
        Action::Init { dry_run } => {
            let result = companion_config_init::initialize(
                &cli.dir,
                dry_run,
                SPEC,
                |text| {
                    render(text)
                        .map(|(preview, _)| preview)
                        .map_err(|e| SPEC.error(2, e))
                },
                |_| Ok(()),
            )?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!(
                    "{}: {}",
                    result["state"].as_str().unwrap(),
                    result["config_path"].as_str().unwrap()
                );
            }
            Ok(0)
        }
        Action::Inventory(query) => run_query(&cli.dir, cli.json, query, View::Inventory),
        Action::Context { query, lines } => {
            run_query(&cli.dir, cli.json, query, View::Context { lines })
        }
    }
}

fn main() {
    match run(Cli::parse()) {
        Ok(code) => process::exit(code),
        Err(error) => {
            eprintln!("Error: {error:#}");
            process::exit(2);
        }
    }
}
