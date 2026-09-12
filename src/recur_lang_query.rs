//! Pure source-bound views over WIR1, CIR1 and SGR1. No execution or policy.
use crate::recur_lang_concurrent_ir::{self as cir, ConcurrentIr};
use crate::recur_lang_graph;
use crate::recur_lang_ir::{self as wir, WarpIr};
use clap::Subcommand;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub const QUERY_SCHEMA: &str = "recur-lang-query-v1";

#[derive(Debug, Subcommand)]
pub enum LangCommand {
    /// Assess explicit WIR1 test evidence without executing its producer
    Evidence(crate::recur_lang_evidence::EvidenceArgs),
    /// Discover .recur sources under the read root, including unsupported inputs
    List,
    /// Inspect one scoped function or lane; bindings are never executed
    Show {
        source: PathBuf,
        #[arg(long)]
        scope: String,
        #[arg(long)]
        expand: bool,
    },
    /// Explain header, body and footer; filter recorded Eventness without ranking
    Report {
        source: PathBuf,
        #[arg(long)]
        scope: Option<String>,
        #[arg(long)]
        eventness: Option<String>,
        #[arg(long)]
        expand: bool,
    },
    /// Check the supported fragment: exit 0 sound, 1 findings, 2 invalid input
    Check {
        source: PathBuf,
        #[arg(long)]
        scope: Option<String>,
    },
}

#[derive(Debug)]
struct Error {
    code: String,
    message: String,
    detail: Value,
}
fn error(code: &str, message: impl Into<String>) -> Error {
    Error {
        code: code.into(),
        message: message.into(),
        detail: Value::Null,
    }
}
impl From<wir::IrDiagnostic> for Error {
    fn from(value: wir::IrDiagnostic) -> Self {
        Self {
            code: "LANG006".into(),
            message: value.to_string(),
            detail: json!(value),
        }
    }
}
impl From<cir::ConcurrentDiagnostic> for Error {
    fn from(value: cir::ConcurrentDiagnostic) -> Self {
        Self {
            code: "LANG006".into(),
            message: value.to_string(),
            detail: json!(value),
        }
    }
}

enum Model {
    Warp(Vec<WarpIr>),
    Concurrent(Box<ConcurrentIr>),
}

/// Relative paths resolve at -d. Canonical containment also rejects symlink escapes.
fn source_path(root: &Path, source: &Path) -> Result<PathBuf, Error> {
    let root = root
        .canonicalize()
        .map_err(|e| error("LANG001", e.to_string()))?;
    let path = root.join(source).canonicalize().map_err(|e| {
        error(
            "LANG001",
            format!("Cannot read source {}: {e}", source.display()),
        )
    })?;
    if !path.starts_with(root) {
        return Err(error("LANG002", "Source is outside the explicit read root"));
    }
    if !path.is_file() {
        return Err(error("LANG001", "Source must be a UTF-8 file"));
    }
    Ok(path)
}

fn candidates(root: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut result = Vec::new();
    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            e.depth() == 0
                || !matches!(
                    e.file_name().to_str(),
                    Some(
                        ".git" | "target" | "target2" | "build" | "dist" | "node_modules" | ".venv"
                    )
                )
        })
    {
        let entry = entry.map_err(|e| error("LANG001", e.to_string()))?;
        // Never follow file or directory symlinks during discovery/evidence inventory.
        if entry.file_type().is_file() {
            result.push(entry.into_path());
        }
    }
    result.sort();
    Ok(result)
}

fn parse(source: &str, name: &str, scope: Option<&str>) -> Result<Model, Error> {
    let (version, kind, names) = wir::query_declarations(source)?;
    if !matches!(
        (version.as_str(), kind.as_str()),
        ("0.1", "class") | ("0.2", "coordination")
    ) {
        return Err(error("LANG005", format!("Unsupported Recur {version} {kind}; supported fragments are WIR1 0.1 class and CIR1 0.2 coordination")));
    }
    if names.is_empty() {
        return Err(error("LANG006", "No supported scope or flow declaration"));
    }
    if names.iter().collect::<BTreeSet<_>>().len() != names.len() {
        return Err(error(
            "LANG004",
            "Repeated scope/flow declarations are ambiguous",
        ));
    }
    if kind == "class" {
        let matching: Vec<_> = names
            .iter()
            .filter(|n| {
                scope.map_or(true, |s| {
                    s == n.as_str() || s.starts_with(&format!("{n}.")) || s.len() == 1
                })
            })
            .collect();
        let mut models = Vec::new();
        for name_part in matching {
            let ir = wir::parse_warp_ir(source, name, name_part)?;
            if scope.map_or(true, |s| {
                s == ir.scope.name
                    || s == ir.scope.function.identity
                    || s == ir.scope.function.symbol
            }) {
                models.push(ir);
            }
        }
        if models.is_empty() {
            return Err(error(
                "LANG003",
                format!("Unknown language scope {}", scope.unwrap_or("<all>")),
            ));
        }
        if scope.is_some() && models.len() > 1 {
            return Err(error(
                "LANG004",
                "Local letter matches multiple scopes; use its qualified identity",
            ));
        }
        Ok(Model::Warp(models))
    } else {
        // CIR1 has independent flows, not a unified document AST. Require an
        // explicit flow if several are present; a lane cannot choose one silently.
        let flow = if names.len() == 1 {
            &names[0]
        } else {
            names
                .iter()
                .find(|n| scope == Some(n.as_str()))
                .ok_or_else(|| error("LANG004", "Multiple flows; select an exact flow name"))?
        };
        Ok(Model::Concurrent(Box::new(cir::parse_structure(
            source, name, flow, false,
        )?)))
    }
}

fn state_vocabulary(root: &Path) -> Result<Vec<String>, Error> {
    let config_path = root.join(".recur/config.toml");
    if config_path.exists() {
        source_path(root, &config_path)?;
    }
    let config =
        crate::project_config::load_from_root(root).map_err(|e| error("LANG008", e.to_string()))?;
    let status = config.and_then(|c| c.status).unwrap_or_default();
    Ok(vec![
        status
            .current_suffix
            .unwrap_or_else(|| "todo.current".into()),
        status.todo_suffix.unwrap_or_else(|| "todo".into()),
        status.complete_suffix.unwrap_or_else(|| "complete".into()),
    ]
    .into_iter()
    .map(|s| s.trim_start_matches('.').to_string())
    .collect())
}

fn recorded(
    root: &Path,
    files: &[PathBuf],
    identities: &[String],
    vocabulary: &[String],
) -> Vec<Value> {
    files.iter().filter_map(|path| {
        let stem = path.file_stem()?.to_str()?;
        if !identities.iter().any(|id| id == stem) { return None; }
        let state = vocabulary.iter().find(|s| stem.ends_with(&format!(".{s}"))).cloned();
        Some(json!({"identity":stem,"path":path.strip_prefix(root).ok()?.to_string_lossy().replace('\\',"/"),"state":state,"evidence":"recorded-only"}))
    }).collect()
}

fn compact_contract(mut value: Value, expand: bool) -> Value {
    if !expand {
        value.as_object_mut().unwrap().remove("fields");
    }
    value
}

fn project(
    model: &Model,
    binding: (&str, &str),
    root: &Path,
    files: &[PathBuf],
    scope: Option<&str>,
    eventness: Option<&str>,
    expand: bool,
) -> Result<Value, Error> {
    let vocabulary = state_vocabulary(root)?;
    project_with_vocabulary(model, binding, root, files, scope, eventness, expand, vocabulary)
}

#[allow(clippy::too_many_arguments)] // shared projector; existing query contract stays unchanged
fn project_with_vocabulary(
    model: &Model, binding: (&str, &str), root: &Path, files: &[PathBuf],
    scope: Option<&str>, eventness: Option<&str>, expand: bool, vocabulary: Vec<String>,
) -> Result<Value, Error> {
    let (source, name) = binding;
    if eventness.is_some_and(|s| !vocabulary.iter().any(|v| v == s)) {
        return Err(error(
            "LANG007",
            format!(
                "Unknown recorded state; configured suffixes: {}",
                vocabulary.join(", ")
            ),
        ));
    }
    let hash = wir::stable_source_hash(source.as_bytes());
    let mut header = Vec::new();
    let mut edges = Vec::new();
    let mut boundary = Vec::new();
    let mut events = Vec::new();
    let mut flows = Vec::new();
    let mut contracts = Vec::new();
    let mut graph = Value::Null;
    let mut findings = Vec::new();
    let (ir_schema, excluded) = match model {
        Model::Warp(models) => {
            for ir in models {
                if ir.schema != wir::WARP_IR_SCHEMA
                    || ir.source != name
                    || ir.source_hash != hash
                    || ir.language_version != "0.1"
                {
                    return Err(error("LANG009", "WIR1 source/schema binding mismatch"));
                }
                let s = &ir.scope;
                let ids: Vec<_> = s
                    .events
                    .iter()
                    .map(|e| e.identifier.clone())
                    .chain([s.warp.current.clone(), s.warp.desired.clone()])
                    .collect();
                let records = recorded(root, files, &ids, &vocabulary);
                if eventness.is_some_and(|state| !records.iter().any(|r| r["state"] == state)) {
                    continue;
                }
                let f = &s.function;
                header.push(json!({"identity":f.identity,"symbol":f.symbol,"scope":s.name,"meaning":f.familiar_name,"binding":f.worker,"span":f.span,
                    "input":compact_contract(json!(f.input),expand),"output":compact_contract(json!(f.output),expand)}));
                for c in [&f.input, &f.output] {
                    if !contracts
                        .iter()
                        .any(|v: &Value| v["canonical_identity"] == c.canonical_identity)
                    {
                        contracts.push(
                            json!({"canonical_identity":c.canonical_identity,"fields":c.fields}),
                        );
                    }
                }
                let edge = json!({"identity":format!("{}->{}",f.input.canonical_identity,f.identity),"producer":f.input.canonical_identity,"consumer":f.identity,"input":f.input.local_identity,"span":f.span});
                if !f
                    .input
                    .canonical_identity
                    .starts_with(&format!("{}.", s.name))
                {
                    boundary.push(edge.clone());
                }
                edges.push(edge);
                edges.push(json!({"identity":format!("{}->{}",f.identity,f.output.canonical_identity),"producer":f.identity,"consumer":f.output.canonical_identity,"output":f.output.local_identity,"span":f.span}));
                flows.push(json!({"scope":s.name,"flow":s.flow}));
                events.push(json!({"scope":s.name,"declared_events":s.events,"requested_transition":s.warp,"recorded":records,"checked_receipts":[],"receipt_status":"not-supplied-or-validated"}));
            }
            (
                wir::WARP_IR_SCHEMA,
                vec![
                    "Expansion bodies and worker semantics",
                    "Whole-document composition, unknown statements and nonselected flows",
                    "Runtime checks, receipt contents and execution",
                ],
            )
        }
        Model::Concurrent(ir) => {
            if ir.schema != cir::CONCURRENT_IR_SCHEMA
                || ir.source != name
                || ir.source_hash != hash
                || ir.language_version != "0.2"
            {
                return Err(error("LANG009", "CIR1 source/schema binding mismatch"));
            }
            let selected: Vec<_> = ir
                .lanes
                .iter()
                .filter(|l| {
                    scope.map_or(true, |s| {
                        s == ir.flow.name
                            || s == l.name
                            || s == l.function.identity
                            || s == l.function.identity.rsplit('.').next().unwrap_or("")
                    })
                })
                .collect();
            if selected.is_empty() {
                return Err(error(
                    "LANG003",
                    "Unknown language scope; CIR1 selects flow or lane function identities",
                ));
            }
            if scope.is_some_and(|s| s != ir.flow.name) && selected.len() > 1 {
                return Err(error(
                    "LANG004",
                    "Local letter matches multiple lanes; use its qualified identity",
                ));
            }
            let analysis = recur_lang_graph::analyze(ir);
            let names: BTreeSet<_> = selected.iter().map(|l| l.name.as_str()).collect();
            // No CIR1 lifecycle model exists: a state filter returns no lane
            // details, but retains the full analysis and its coverage limits.
            if eventness.is_none() {
                for lane in &selected {
                    let messages: Vec<_> = lane
                        .input_messages
                        .iter()
                        .map(|m| {
                            let mut value = json!(m);
                            if expand {
                                value["fields"] = json!(ir
                                    .contracts
                                    .iter()
                                    .find(|c| c.name == m.contract)
                                    .map(|c| &c.fields));
                            }
                            value
                        })
                        .collect();
                    let mut input = json!({"identity":format!("{}.i({})",lane.name,lane.input_symbol),"expression":lane.input_expression,"members":lane.input_messages.iter().map(|m| json!({"identity":m.identity,"projection":m.projection,"contract":m.contract})).collect::<Vec<_>>()});
                    if expand {
                        input["messages"] = json!(messages);
                    }
                    header.push(json!({"identity":lane.function.identity,"symbol":lane.function.identity.rsplit('.').next(),"scope":lane.name,"meaning":lane.function.familiar_name,"binding":null,"persona":lane.persona,"span":lane.function.span,"input":input,"output":lane.output_message}));
                    events.push(json!({"scope":lane.name,"requested_receipts":lane.policy.required_receipts,"recorded":[],"checked_receipts":[],"receipt_status":"not-supplied-or-validated"}));
                    for message in lane
                        .input_messages
                        .iter()
                        .chain(std::iter::once(&lane.output_message))
                    {
                        if !contracts
                            .iter()
                            .any(|c: &Value| c["canonical_identity"] == message.contract)
                        {
                            contracts.push(json!({"canonical_identity":message.contract,"fields":ir.contracts.iter().find(|c| c.name == message.contract).map(|c| &c.fields)}));
                        }
                    }
                }
            }
            for edge in &analysis.edges {
                let from = names.contains(edge.producer.as_str());
                let to = names.contains(edge.consumer.as_str());
                if from || to {
                    let mut value = json!(edge);
                    value["identity"] = json!(format!(
                        "{}{}->{}",
                        edge.message.identity,
                        edge.message.projection.as_deref().unwrap_or(""),
                        edge.consumer
                    ));
                    edges.push(value.clone());
                    if from != to {
                        boundary.push(value);
                    }
                }
            }
            flows.push(json!({"scope":ir.flow.name,"flow":ir.flow}));
            findings = analysis.findings.iter().map(|f| json!(f)).collect();
            graph = json!(analysis);
            (cir::CONCURRENT_IR_SCHEMA, vec!["Coordinator input signatures and function bodies", "Whole-document grammar, expansions, watch/runtime/grid/import/feedback sections", "Lane Eventness and receipt validation (CIR1 has no lifecycle model)"])
        }
    };
    Ok(
        json!({"schema":QUERY_SCHEMA,"source":name,"source_hash":hash,"ir_schema":ir_schema,"scope":scope,"expanded":expand,
        "coverage":{"whole_source_validated":false,"analysis":if graph.is_null(){"selected WIR1 scopes"}else{"entire selected CIR1 flow, before filtering"},"excluded":excluded,
            "diagnostics":[{"code":"LANG101","severity":"capability","message":"Only the named IR fragment is validated; excluded syntax and runtime behavior remain unvalidated"}]},
        "header":header,"contracts":contracts,"body":{"flows":flows,"edges":edges,"boundary_edges":boundary},
        "footer":{"execution":"not-run","eventness_vocabulary":vocabulary,"eventness_filter":eventness,"events":events,"findings":findings,"graph":graph,"validation":if findings.is_empty(){"sound-within-coverage"}else{"failed"}}}),
    )
}

fn query(command: LangCommand, root: &Path) -> Result<Value, Error> {
    let root = root
        .canonicalize()
        .map_err(|e| error("LANG001", format!("Invalid read root: {e}")))?;
    if !root.is_dir() {
        return Err(error("LANG001", "Read root must be a directory"));
    }
    let files = candidates(&root)?;
    let (source, scope, eventness, expand) = match command {
        LangCommand::Evidence(_) => unreachable!("handled before legacy discovery"),
        LangCommand::List => {
            let mut sources = Vec::new();
            for path in files
                .iter()
                .filter(|p| p.extension().is_some_and(|e| e == "recur"))
            {
                let name = path
                    .strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                let result = fs::read_to_string(path)
                    .map_err(|e| error("LANG001", e.to_string()))
                    .and_then(|text| {
                        let model = parse(&text, &name, None)?;
                        project(&model, (&text, &name), &root, &files, None, None, false)
                    });
                sources.push(match result {
                    Ok(value) => json!({"source":name,"source_hash":value["source_hash"],"ir_schema":value["ir_schema"],"status":value["footer"]["validation"],"symbols":value["header"].as_array().unwrap().iter().map(|h| &h["identity"]).collect::<Vec<_>>(),"coverage":value["coverage"],"findings":value["footer"]["findings"]}),
                    Err(e) => json!({"source":name,"status":"input-error","diagnostics":[{"code":e.code,"message":e.message,"detail":e.detail}]}),
                });
            }
            return Ok(
                json!({"schema":"recur-lang-list-v1","sources":sources,"coverage":{"root":".","symlinks":"not-followed","excluded_directories":[".git","target","target2","build","dist","node_modules",".venv"]}}),
            );
        }
        LangCommand::Show {
            source,
            scope,
            expand,
        } => (source, Some(scope), None, expand),
        LangCommand::Report {
            source,
            scope,
            eventness,
            expand,
        } => (source, scope, eventness, expand),
        LangCommand::Check { source, scope } => (source, scope, None, false),
    };
    let path = source_path(&root, &source)?;
    let name = path
        .strip_prefix(&root)
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    let source = fs::read_to_string(&path).map_err(|e| error("LANG001", e.to_string()))?;
    let model = parse(&source, &name, scope.as_deref())?;
    project(
        &model,
        (&source, &name),
        &root,
        &files,
        scope.as_deref(),
        eventness.as_deref(),
        expand,
    )
}

fn text_view(value: &Value) -> String {
    if value["schema"] != QUERY_SCHEMA {
        return serde_json::to_string_pretty(value).unwrap();
    }
    let mut out = format!(
        "{} [{}]\n{} · {}\n",
        value["source"].as_str().unwrap(),
        value["ir_schema"].as_str().unwrap(),
        value["source_hash"].as_str().unwrap(),
        value["footer"]["validation"].as_str().unwrap()
    );
    out.push_str("\nheader\n");
    for h in value["header"].as_array().unwrap() {
        let input = h["input"]["local_identity"]
            .as_str()
            .or(h["input"]["identity"].as_str())
            .unwrap_or("?");
        let output = h["output"]["local_identity"]
            .as_str()
            .or(h["output"]["identity"].as_str())
            .unwrap_or("?");
        out.push_str(&format!(
            "  {} : {} -> {} ~ {} (line {})\n",
            h["identity"].as_str().unwrap(),
            input,
            output,
            h["meaning"].as_str().unwrap(),
            h["span"]["start_line"]
        ));
        if let Some(binding) = h["binding"].as_str() {
            out.push_str(&format!("    binding: {binding} (not run)\n"));
        }
        if let Some(members) = h["input"]["members"].as_array() {
            for member in members {
                out.push_str(&format!(
                    "    {} <- {}{} : {}\n",
                    input,
                    member["identity"].as_str().unwrap(),
                    member["projection"].as_str().unwrap_or(""),
                    member["contract"].as_str().unwrap()
                ));
            }
        }
        for role in ["input", "output"] {
            if let Some(canonical) = h[role]["canonical_identity"].as_str() {
                out.push_str(&format!("    {role} contract: {canonical}\n"));
            }
        }
    }
    if value["expanded"] == true {
        out.push_str("  contracts (exact identities; fields declared once)\n");
        for c in value["contracts"].as_array().unwrap() {
            out.push_str(&format!(
                "    {}: {}\n",
                c["canonical_identity"].as_str().unwrap(),
                c["fields"]
            ));
        }
    }
    out.push_str("\nbody\n");
    for f in value["body"]["flows"].as_array().unwrap() {
        out.push_str(&format!(
            "  {}: {}\n",
            f["scope"].as_str().unwrap(),
            f["flow"]["expression"].as_str().unwrap()
        ));
    }
    for edge in value["body"]["edges"].as_array().unwrap() {
        let boundary = value["body"]["boundary_edges"]
            .as_array()
            .unwrap()
            .contains(edge);
        out.push_str(&format!(
            "  {}{}\n",
            edge["identity"].as_str().unwrap(),
            if boundary { " [boundary]" } else { "" }
        ));
    }
    out.push_str("\nfooter\n  execution: not-run; receipts: not supplied or validated\n");
    out.push_str(&format!(
        "  recorded state filter: {}\n",
        value["footer"]["eventness_filter"]
    ));
    for event in value["footer"]["events"].as_array().unwrap() {
        out.push_str(&format!("  {}\n", event["scope"].as_str().unwrap()));
        if let Some(declared) = event["declared_events"].as_array() {
            for e in declared {
                out.push_str(&format!(
                    "    declared {}: {}\n",
                    e["edge"].as_str().unwrap(),
                    e["identifier"].as_str().unwrap()
                ));
            }
        }
        if let Some(current) = event["requested_transition"]["current"].as_str() {
            out.push_str(&format!(
                "    requested: {} -> {} -> {}\n",
                current,
                event["requested_transition"]["slice"].as_str().unwrap(),
                event["requested_transition"]["desired"].as_str().unwrap()
            ));
        }
        if let Some(required) = event["requested_receipts"].as_array() {
            out.push_str(&format!(
                "    requested receipts: {}\n",
                required
                    .iter()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        let recorded = event["recorded"].as_array().unwrap();
        if recorded.is_empty() {
            out.push_str("    recorded: none\n");
        }
        for record in recorded {
            out.push_str(&format!(
                "    recorded-only: {} [{}]\n",
                record["path"].as_str().unwrap(),
                record["state"]
            ));
        }
    }
    for finding in value["footer"]["findings"].as_array().unwrap() {
        out.push_str(&format!(
            "  [{}] {} · path {} · span {}\n",
            finding["code"].as_str().unwrap(),
            finding["message"].as_str().unwrap(),
            finding["path"],
            finding["span"]
        ));
    }
    out.push_str(&format!(
        "  analysis: {}\n  [LANG101] Whole source is not validated. Excluded:\n",
        value["coverage"]["analysis"].as_str().unwrap()
    ));
    for excluded in value["coverage"]["excluded"].as_array().unwrap() {
        out.push_str(&format!("    {}\n", excluded.as_str().unwrap()));
    }
    if value["footer"]["findings"].as_array().unwrap().is_empty() {
        out.push_str("  no blocking findings within this coverage\n");
    }
    out
}

pub fn execute(command: LangCommand, root: &Path, json_output: bool) -> i32 {
    if let LangCommand::Evidence(args) = command {
        let value = crate::recur_lang_evidence::assess(root, &args);
        println!("{}", if json_output { serde_json::to_string_pretty(&value).unwrap() }
            else { crate::recur_lang_evidence::text(&value) });
        return crate::recur_lang_evidence::exit_code(&value);
    }
    let checking = matches!(command, LangCommand::Check { .. });
    let (value, code) = match query(command, root) {
        Ok(value) => {
            let failed = checking && value["footer"]["validation"] == "failed";
            (value, i32::from(failed))
        }
        Err(e) => (
            json!({"schema":"recur-lang-error-v1","diagnostics":[{"code":e.code,"message":e.message,"detail":e.detail}]}),
            2,
        ),
    };
    println!(
        "{}",
        if json_output {
            serde_json::to_string_pretty(&value).unwrap()
        } else {
            text_view(&value)
        }
    );
    code
}

/// Reuse the exact projector over already bounded input; no inventory or IO.
pub(crate) fn evidence_packet(source: &str, name: &str, scope: &str, expand: bool,
    vocabulary: Vec<String>) -> Result<Value, String> {
    let model = parse(source, name, Some(scope)).map_err(|e| e.message)?;
    if !matches!(model, Model::Warp(_)) { return Err("checked evidence requires WIR1".into()); }
    project_with_vocabulary(&model, (source,name), Path::new("."), &[], Some(scope), None, expand, vocabulary)
        .map_err(|e| e.message)
}

pub(crate) fn evidence_text(packet: &Value) -> String { text_view(packet) }

#[cfg(test)]
mod tests {
    use super::*;
    const ALGORITHM: &str = include_str!("../demos/main.lang/main.lang.algorithm-lab.recur");
    const SKIPPY: &str =
        include_str!("../demos/main.lang/main.lang.skippy-watch-coordination.recur");
    fn view(text: &str, scope: Option<&str>, expand: bool) -> Value {
        let root = tempfile::tempdir().unwrap();
        let model = parse(text, "test.recur", scope).unwrap();
        project(
            &model,
            (text, "test.recur"),
            root.path(),
            &[],
            scope,
            None,
            expand,
        )
        .unwrap()
    }
    #[test]
    fn exact_aliases_and_local_collisions() {
        let value = view(ALGORITHM, Some("merge.f"), true);
        assert_eq!(
            value["header"][0]["input"]["canonical_identity"],
            "bubble.o(b)"
        );
        assert_eq!(value["header"][0]["input"]["local_identity"], "merge.i(b)");
        assert_eq!(value["body"]["boundary_edges"].as_array().unwrap().len(), 1);
        assert!(matches!(parse(ALGORITHM,"test.recur",Some("f")),Err(e) if e.code == "LANG004"));
        assert!(
            matches!(parse(ALGORITHM,"test.recur",Some("missing")),Err(e) if e.code == "LANG003")
        );
    }
    #[test]
    fn fan_in_is_lossless_and_deterministic() {
        let compact = view(SKIPPY, Some("review_bird"), false);
        let expanded = view(SKIPPY, Some("review_bird"), true);
        assert_eq!(
            expanded["header"][0]["input"]["messages"]
                .as_array()
                .unwrap()
                .len(),
            4
        );
        assert_eq!(compact["body"], expanded["body"]);
        assert_eq!(compact["footer"], expanded["footer"]);
        assert_eq!(compact["header"].as_array().unwrap().len(), 1);
        let text = text_view(&expanded);
        for edge in expanded["body"]["edges"].as_array().unwrap() {
            assert!(text.contains(edge["identity"].as_str().unwrap()));
        }
        assert!(text.contains("not-run"));
        assert!(text.contains("[LANG101]"));
        assert!(compact["header"][0]["input"].get("messages").is_none());
        assert_eq!(
            serde_json::to_vec(&expanded).unwrap(),
            serde_json::to_vec(&view(SKIPPY, Some("review_bird"), true)).unwrap()
        );
    }
    #[test]
    fn refuses_foreign_model_and_version() {
        let root = tempfile::tempdir().unwrap();
        let model = parse(ALGORITHM, "test.recur", Some("gcd")).unwrap();
        assert_eq!(
            project(
                &model,
                (&format!("{ALGORITHM}\n"), "test.recur"),
                root.path(),
                &[],
                Some("gcd"),
                None,
                false
            )
            .unwrap_err()
            .code,
            "LANG009"
        );
        assert!(
            matches!(parse(&ALGORITHM.replace("recur 0.1","recur 0.3"),"test.recur",None),Err(e) if e.code == "LANG005")
        );
    }
    #[test]
    fn scoped_cycle_cannot_hide_global_failure() {
        let root = tempfile::tempdir().unwrap();
        let mut model = parse(SKIPPY, "test.recur", None).unwrap();
        if let Model::Concurrent(ir) = &mut model {
            let feedback = ir.lanes[4].output_message.clone();
            ir.lanes[0].input_messages.push(feedback);
            ir.flow.awaits[2].next_consumer = "csharp_monkey".into();
        }
        let value = project(
            &model,
            (SKIPPY, "test.recur"),
            root.path(),
            &[],
            Some("review_bird"),
            None,
            false,
        )
        .unwrap();
        assert_eq!(value["footer"]["validation"], "failed");
        assert!(value["footer"]["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|f| f["code"] == "SGR002" && f["path"].as_array().unwrap().len() > 2));
        assert_eq!(value["header"].as_array().unwrap().len(), 1);
    }
    #[test]
    fn eventness_is_recorded_and_customizable() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir(root.path().join(".recur")).unwrap();
        fs::write(
            root.path().join(".recur/config.toml"),
            "[status]\ncurrent_suffix = 'active'\ncomplete_suffix = 'done'\n",
        )
        .unwrap();
        let text = ALGORITHM
            .replace("todo.current", "active")
            .replace(".complete", ".done");
        let path = root.path().join("demo.algorithm.gcd.active.md");
        fs::write(&path, "recorded only").unwrap();
        let model = parse(&text, "test.recur", None).unwrap();
        let value = project(
            &model,
            (&text, "test.recur"),
            root.path(),
            &[path],
            None,
            Some("active"),
            false,
        )
        .unwrap();
        assert_eq!(value["header"].as_array().unwrap().len(), 1);
        assert_eq!(
            value["footer"]["events"][0]["recorded"][0]["evidence"],
            "recorded-only"
        );
        assert_eq!(value["footer"]["execution"], "not-run");
        assert_eq!(
            project(
                &model,
                (&text, "test.recur"),
                root.path(),
                &[],
                None,
                Some("urgent"),
                false
            )
            .unwrap_err()
            .code,
            "LANG007"
        );
    }
}
