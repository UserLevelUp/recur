//! Shared, read-only capability prompt catalogs and command adapters.
// defines: recur.prompt.discovery
use clap::Args;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

pub const CONTEXT_KINDS: &[&str] = &["hierarchy", "files", "trace-id", "eventness", "warp"];
const SOURCE_LIMIT: usize = 65536;

#[derive(Debug, Args)]
pub struct PromptArgs {
    /// Prompt ID to inspect, capability to filter, or omit to list all
    pub selector: Option<String>,
    /// Literal user intent; requires an exact prompt ID
    #[arg(long)]
    pub intent: Option<String>,
    /// Hierarchy selection beneath -d
    #[arg(long, default_value = "**")]
    pub scope: String,
    /// Maximum distinct evidence sources
    #[arg(long, default_value = "32")]
    pub max_files: usize,
    /// Maximum serialized UTF-8 bytes in context.items
    #[arg(long, default_value = "65536")]
    pub max_bytes: usize,
}

impl Default for PromptArgs {
    fn default() -> Self {
        Self {
            selector: None,
            intent: None,
            scope: "**".into(),
            max_files: 32,
            max_bytes: 65536,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct PromptError {
    pub code: &'static str,
    pub message: String,
}
pub type Result<T> = std::result::Result<T, PromptError>;
pub(crate) fn error(code: &'static str, message: impl ToString) -> PromptError {
    PromptError {
        code,
        message: message.to_string(),
    }
}
pub fn fingerprint(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Definition {
    capability: String,
    description: String,
    path: String,
    inputs: Vec<String>,
    context: Vec<String>,
}
#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Settings {
    app_defaults: bool,
    registry: BTreeMap<String, Definition>,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            app_defaults: true,
            registry: BTreeMap::new(),
        }
    }
}
pub struct AppPrompt {
    pub id: &'static str,
    pub description: &'static str,
    pub instructions: &'static str,
}
pub struct AppCatalog {
    pub provider: &'static str,
    pub capability: &'static str,
    pub prompts: &'static [AppPrompt],
}
const WARP_PROMPTS: &[AppPrompt] = &[
    AppPrompt {
        id: "warp.naming",
        description: "Find where work belongs in the project hierarchy",
        instructions: include_str!("prompts/warp.naming.md"),
    },
    AppPrompt {
        id: "warp.slicing",
        description: "Propose useful slices and observable acceptance gates",
        instructions: include_str!("prompts/warp.slicing.md"),
    },
    AppPrompt {
        id: "warp.recovery",
        description: "Recover the next action from Warp evidence",
        instructions: include_str!("prompts/warp.recovery.md"),
    },
];
pub const APP_CATALOGS: &[AppCatalog] = &[AppCatalog {
    provider: "recur-warp",
    capability: "warp",
    prompts: WARP_PROMPTS,
}];

struct Entry {
    definition: Definition,
    provider: Option<&'static str>,
    builtin: Option<&'static str>,
    resolved: Option<PathBuf>,
}
pub struct Registry {
    entries: BTreeMap<String, Entry>,
}

fn valid_segment(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}
fn validate(id: &str, d: &Definition) -> Result<()> {
    if !id.contains('.')
        || !id.split('.').all(valid_segment)
        || id.split('.').next() != Some(d.capability.as_str())
        || d.description.trim().is_empty()
        || d.path.trim().is_empty()
        || d.inputs.iter().any(|s| s != "intent")
        || d.context
            .iter()
            .any(|s| !CONTEXT_KINDS.contains(&s.as_str()))
    {
        return Err(error(
            "invalid_registry",
            format!("Invalid prompt definition: {id}"),
        ));
    }
    Ok(())
}

// Validate both lexical spelling and every existing ancestor, including a symlink
// whose final file is missing. A broken project override never falls back.
fn resolve_source(root: &Path, relative: &str) -> Result<Option<PathBuf>> {
    if relative.starts_with(['/', '\\'])
        || relative.contains(':')
        || relative.split(['/', '\\']).any(|s| s == "..")
    {
        return Err(error(
            "unsafe_path",
            format!("Prompt path must stay inside project: {relative}"),
        ));
    }
    let mut path = root.to_path_buf();
    for segment in relative
        .split(['/', '\\'])
        .filter(|s| !s.is_empty() && *s != ".")
    {
        path.push(segment);
        match fs::symlink_metadata(&path) {
            Ok(_) => {
                path = path.canonicalize().map_err(|e| error("unsafe_path", e))?;
                if !path.starts_with(root) {
                    return Err(error("unsafe_path", "Prompt symlink escapes project"));
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(error("unsafe_path", e)),
        }
    }
    if !path.is_file() {
        return Err(error(
            "invalid_registry",
            "Prompt source is not a regular file",
        ));
    }
    Ok(Some(path))
}

impl Registry {
    pub fn load(requested: &Path) -> Result<Self> {
        Self::with_catalogs(requested, APP_CATALOGS)
    }
    pub fn with_catalogs(requested: &Path, catalogs: &[AppCatalog]) -> Result<Self> {
        let requested = requested
            .canonicalize()
            .map_err(|e| error("invalid_input", e))?;
        let config = requested
            .ancestors()
            .map(|p| p.join(".recur/config.toml"))
            .find(|p| p.is_file());
        let settings = if let Some(path) = &config {
            let text = fs::read_to_string(path).map_err(|e| error("invalid_registry", e))?;
            let value: toml::Value =
                toml::from_str(&text).map_err(|e| error("invalid_registry", e))?;
            value
                .get("prompts")
                .map(|v| v.clone().try_into::<Settings>())
                .transpose()
                .map_err(|e| error("invalid_registry", e))?
                .unwrap_or_default()
        } else {
            Settings::default()
        };
        let mut entries = BTreeMap::new();
        if settings.app_defaults {
            for catalog in catalogs {
                for prompt in catalog.prompts {
                    let definition = Definition {
                        capability: catalog.capability.into(),
                        description: prompt.description.into(),
                        path: format!("builtin:{}/{}", catalog.provider, prompt.id),
                        inputs: vec!["intent".into()],
                        context: CONTEXT_KINDS.iter().map(|s| s.to_string()).collect(),
                    };
                    validate(prompt.id, &definition)?;
                    if prompt.instructions.trim().is_empty()
                        || prompt.instructions.len() > SOURCE_LIMIT
                    {
                        return Err(error("invalid_registry", "Invalid bundled prompt body"));
                    }
                    if entries
                        .insert(
                            prompt.id.to_string(),
                            Entry {
                                definition,
                                provider: Some(catalog.provider),
                                builtin: Some(prompt.instructions),
                                resolved: None,
                            },
                        )
                        .is_some()
                    {
                        return Err(error(
                            "invalid_registry",
                            format!("Duplicate app prompt ID: {}", prompt.id),
                        ));
                    }
                }
            }
        }
        let project = config
            .as_ref()
            .map(|p| p.parent().unwrap().parent().unwrap())
            .unwrap_or(&requested);
        for (id, definition) in settings.registry {
            validate(&id, &definition)?;
            let resolved = resolve_source(project, &definition.path)?;
            entries.insert(
                id,
                Entry {
                    definition,
                    resolved,
                    provider: None,
                    builtin: None,
                },
            );
        }
        Ok(Self { entries })
    }
    fn metadata(&self, id: &str, entry: &Entry) -> Value {
        let d = &entry.definition;
        json!({"prompt_id":id, "capability":d.capability, "description":d.description,
            "path":d.path, "status":if entry.builtin.is_some() || entry.resolved.is_some() {"available"} else {"missing"},
            "inputs":d.inputs,"context":d.context,"origin":if entry.provider.is_some(){"app"}else{"project"},"provider":entry.provider})
    }
    pub fn list(&self, capability: Option<&str>) -> Value {
        let entries: Vec<_> = self
            .entries
            .iter()
            .filter(|(_, e)| capability.map_or(true, |c| e.definition.capability == c))
            .map(|(id, e)| self.metadata(id, e))
            .collect();
        json!({"schema":"recur-prompt-list-v1","entries":entries})
    }
    pub fn references(&self, ids: &str) -> Vec<Value> {
        ids.split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|id| {
                if let Some(e) = self.entries.get(id) {
                    let m = self.metadata(id, e);
                    json!({"prompt_id":id,"status":m["status"],"path":m["path"]})
                } else {
                    json!({"prompt_id":id,"status":"unregistered"})
                }
            })
            .collect()
    }
    pub fn show(&self, id: &str) -> Result<Value> {
        let e = self
            .entries
            .get(id)
            .ok_or_else(|| error("unknown_prompt", format!("Unknown prompt: {id}")))?;
        let bytes = if let Some(body) = e.builtin {
            body.as_bytes().to_vec()
        } else {
            let path = e.resolved.as_ref().ok_or_else(|| {
                error(
                    "missing_source",
                    format!("Missing prompt source: {}", e.definition.path),
                )
            })?;
            let file = fs::File::open(path).map_err(|e| error("missing_source", e))?;
            let mut bytes = Vec::new();
            file.take((SOURCE_LIMIT + 1) as u64)
                .read_to_end(&mut bytes)
                .map_err(|e| error("missing_source", e))?;
            bytes
        };
        if bytes.len() > SOURCE_LIMIT {
            return Err(error("invalid_registry", "Prompt source exceeds 64 KiB"));
        }
        let instructions = std::str::from_utf8(&bytes).map_err(|e| error("invalid_registry", e))?;
        let d = &e.definition;
        Ok(
            json!({"schema":"recur-prompt-show-v1","prompt_id":id,"capability":d.capability,"description":d.description,
            "instructions":instructions,"inputs":d.inputs,"context":d.context,
            "source":{"path":d.path,"fingerprint":fingerprint(&bytes),"origin":if e.provider.is_some(){"app"}else{"project"},"provider":e.provider}}),
        )
    }
}

pub fn query(root: &Path, args: &PromptArgs, separators: &[String]) -> Result<Value> {
    if args.max_files == 0 || args.max_bytes < 2 {
        return Err(error(
            "invalid_budget",
            "File budget must be positive; context.items requires at least two bytes",
        ));
    }
    let registry = Registry::load(root)?;
    let exact = args
        .selector
        .as_deref()
        .filter(|s| registry.entries.contains_key(*s));
    if args.intent.is_some() && exact.is_none() {
        return Err(error(
            "invalid_input",
            "Intent requires an exact registered prompt ID",
        ));
    }
    if let Some(id) = exact {
        let prompt = registry.show(id)?;
        if let Some(intent) = &args.intent {
            if args.scope.trim().is_empty() {
                return Err(error("invalid_input", "Scope must not be empty"));
            }
            let kinds = &registry.entries[id].definition.context;
            let context = crate::prompt_context::collect(root, args, separators, kinds)?;
            Ok(
                json!({"schema":"recur-prompt-packet-v1","prompt_id":id,"intent":intent,"scope":args.scope,"prompt":prompt,"context":context}),
            )
        } else {
            Ok(prompt)
        }
    } else if args.selector.as_ref().is_some_and(|s| s.contains('.')) {
        Err(error(
            "unknown_prompt",
            format!("Unknown prompt: {}", args.selector.as_ref().unwrap()),
        ))
    } else {
        Ok(registry.list(args.selector.as_deref()))
    }
}
pub fn execute(
    root: &Path,
    args: &PromptArgs,
    separators: &[String],
    json_output: bool,
) -> anyhow::Result<()> {
    match query(root, args, separators) {
        Ok(value) => {
            if json_output {
                println!("{}", serde_json::to_string_pretty(&value)?);
            } else if let Some(entries) = value["entries"].as_array() {
                if entries.is_empty() {
                    println!(
                        "{}",
                        if args.selector.is_some() {
                            "No prompts available for this capability."
                        } else {
                            "No prompts available."
                        }
                    );
                }
                for entry in entries {
                    println!(
                        "{} — {} ({})",
                        escape(entry["prompt_id"].as_str().unwrap()),
                        escape(entry["description"].as_str().unwrap()),
                        entry["status"].as_str().unwrap()
                    );
                }
            } else if value["schema"] == "recur-prompt-show-v1" {
                println!("{}", escape(value["prompt_id"].as_str().unwrap()));
                for line in value["instructions"].as_str().unwrap().lines() {
                    println!("{}", escape(line));
                }
            } else {
                println!("{}", serde_json::to_string_pretty(&value)?);
            }
            Ok(())
        }
        Err(e) => {
            if json_output {
                println!(
                    "{}",
                    json!({"schema":"recur-prompt-error-v1","state":"blocked","code":e.code,"message":e.message})
                );
            }
            Err(error(e.code, escape(&e.message)).into())
        }
    }
}
pub fn escape(text: &str) -> String {
    text.chars()
        .flat_map(|c| {
            if c.is_control() {
                c.escape_default().collect::<Vec<_>>()
            } else {
                vec![c]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn config(root: &Path, text: &str) {
        fs::create_dir_all(root.join(".recur")).unwrap();
        fs::write(root.join(".recur/config.toml"), text).unwrap();
    }
    fn definition(path: &str) -> String {
        format!("[prompts.registry.\"warp.naming\"]\ncapability='warp'\ndescription='Placement'\npath='{path}'\ninputs=['intent']\ncontext=['files']\n")
    }
    #[test]
    fn catalog_integrity_and_provider_collisions() {
        let dir = tempfile::tempdir().unwrap();
        let registry = Registry::load(dir.path()).unwrap();
        let entries = registry.list(None);
        assert_eq!(entries["entries"].as_array().unwrap().len(), 3);
        let mut bodies = std::collections::BTreeSet::new();
        for entry in entries["entries"].as_array().unwrap() {
            let shown = registry.show(entry["prompt_id"].as_str().unwrap()).unwrap();
            assert!(bodies.insert(shown["instructions"].as_str().unwrap().to_string()));
            assert_eq!(
                shown["source"]["fingerprint"],
                fingerprint(shown["instructions"].as_str().unwrap().as_bytes())
            );
        }
        let duplicate = [
            AppCatalog {
                provider: "one",
                capability: "warp",
                prompts: WARP_PROMPTS,
            },
            AppCatalog {
                provider: "two",
                capability: "warp",
                prompts: WARP_PROMPTS,
            },
        ];
        assert_eq!(
            Registry::with_catalogs(dir.path(), &duplicate)
                .err()
                .unwrap()
                .code,
            "invalid_registry"
        );
        assert_eq!(
            fingerprint(b"abc"),
            "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
    #[test]
    fn invalid_definition_types_and_portable_paths() {
        let dir = tempfile::tempdir().unwrap();
        for text in [
            definition("body.md").replace("warp.naming", "warp..naming"),
            definition("body.md").replace("Placement", ""),
            definition("body.md").replace("['intent']", "[7]"),
            definition("body.md").replace("['files']", "['shell']"),
            definition("body.md").replace("['intent']", "['unknown']"),
            definition("body.md") + "unexpected=true\n",
        ] {
            config(dir.path(), &text);
            assert_eq!(
                Registry::load(dir.path()).err().unwrap().code,
                "invalid_registry"
            );
        }
        for path in [
            "../secret.md",
            "a/../../secret.md",
            "..\\secret.md",
            "C:/secret.md",
            "C:secret.md",
            "\\\\server\\share",
            "/secret.md",
        ] {
            config(dir.path(), &definition(path));
            assert_eq!(
                Registry::load(dir.path()).err().unwrap().code,
                "unsafe_path",
                "{path}"
            );
        }
    }
    #[test]
    fn exact_bytes_size_utf8_and_missing_override() {
        let dir = tempfile::tempdir().unwrap();
        config(dir.path(), &definition("body.md"));
        assert_eq!(
            Registry::load(dir.path())
                .unwrap()
                .show("warp.naming")
                .unwrap_err()
                .code,
            "missing_source"
        );
        for body in [b"\xef\xbb\xbfhello\r\n".to_vec(), vec![b'x'; SOURCE_LIMIT]] {
            fs::write(dir.path().join("body.md"), &body).unwrap();
            let shown = Registry::load(dir.path())
                .unwrap()
                .show("warp.naming")
                .unwrap();
            assert_eq!(shown["instructions"].as_str().unwrap().as_bytes(), body);
            assert_eq!(shown["source"]["fingerprint"], fingerprint(&body));
        }
        for body in [vec![b'x'; SOURCE_LIMIT + 1], vec![0xff]] {
            fs::write(dir.path().join("body.md"), body).unwrap();
            let registry = Registry::load(dir.path()).unwrap();
            assert_eq!(
                registry.show("warp.naming").unwrap_err().code,
                "invalid_registry"
            );
            assert_eq!(
                registry.list(Some("warp"))["entries"][0]["origin"],
                "project"
            );
        }
    }
    #[test]
    fn directory_links_cannot_escape_even_when_source_missing() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        let link = dir.path().join("linked");
        #[cfg(unix)]
        std::os::unix::fs::symlink(outside.path(), &link).unwrap();
        #[cfg(windows)]
        {
            // Junctions do not require the symlink privilege on Windows.
            let output = std::process::Command::new("cmd")
                .args(["/c", "mklink", "/J"])
                .arg(&link)
                .arg(outside.path())
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        config(dir.path(), &definition("linked/missing.md"));
        assert_eq!(
            Registry::load(dir.path()).err().unwrap().code,
            "unsafe_path"
        );
        fs::write(outside.path().join("missing.md"), "outside").unwrap();
        assert_eq!(
            Registry::load(dir.path()).err().unwrap().code,
            "unsafe_path"
        );
    }
    #[test]
    fn human_output_escapes_terminal_controls() {
        assert_eq!(escape("name\x1b[31m\nnext"), "name\\u{1b}[31m\\nnext");
    }
}
