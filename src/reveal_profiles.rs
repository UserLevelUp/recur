//! Explicit local associations and inert, bounded context preparation.
use crate::{
    project_config,
    reveal_query::{RevealEntry, RevealField},
};
use anyhow::{bail, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

pub const DEFAULTS: &str = "\n# Explicit local associations; pointers are not installed or activated.\n[reveal.agents]\n\n[reveal.personas.skippy]\nskills = [\"recur-expert\", \"recur-warp\"]\nguidance_level = \"advanced\"\n\n[reveal.skills.recur-expert]\npath = \"recur-expert/SKILL.md\"\n\n[reveal.skills.recur-warp]\npath = \"recur-warp/SKILL.md\"\n";

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Record {
    pub capsule: Option<String>,
    pub path: Option<String>,
    pub persona: Option<String>,
    pub skills: Vec<String>,
    pub guidance_level: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Profiles(pub BTreeMap<String, BTreeMap<String, Record>>);
impl Profiles {
    pub fn parse(table: &toml::value::Table) -> Result<Self> {
        let mut result = Self::default();
        for (plural, kind) in [
            ("agents", "agent"),
            ("personas", "persona"),
            ("skills", "skill"),
        ] {
            let mut records = BTreeMap::new();
            if let Some(value) = table.get(plural) {
                for (id, value) in value
                    .as_table()
                    .context("reveal association registry must be a table")?
                {
                    ensure!(!id.trim().is_empty(), "Empty reveal record ID");
                    let fields = value.as_table().context("reveal record must be a table")?;
                    ensure!(
                        kind == "agent" || !fields.contains_key("persona"),
                        "Only agents declare persona"
                    );
                    ensure!(
                        kind != "skill" || !fields.contains_key("skills"),
                        "Skills have no outgoing associations"
                    );
                    ensure!(
                        kind == "persona" || !fields.contains_key("guidance_level"),
                        "Only personas declare guidance_level"
                    );
                    let record: Record =
                        value.clone().try_into().context("Invalid reveal record")?;
                    for reference in record
                        .skills
                        .iter()
                        .chain(record.persona.iter())
                        .chain(record.path.iter())
                        .chain(record.capsule.iter())
                    {
                        ensure!(!reference.trim().is_empty(), "Empty reveal reference");
                    }
                    records.insert(id.clone(), record);
                }
            }
            result.0.insert(kind.into(), records);
        }
        Ok(result)
    }
}

fn fingerprint(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
fn local_path(root: &Path, base: &Path, name: &str) -> Result<PathBuf> {
    let p = Path::new(name);
    ensure!(
        !p.is_absolute()
            && !p.components().any(|c| matches!(
                c,
                Component::ParentDir | Component::Prefix(_) | Component::RootDir
            )),
        "Path must be project-relative without parent traversal"
    );
    let candidate = base.join(p);
    // Check the nearest existing ancestor even for missing targets.
    let existing = candidate
        .ancestors()
        .find(|p| p.exists())
        .context("No existing path ancestor")?;
    ensure!(
        existing.canonicalize()?.starts_with(root),
        "Path escapes read root"
    );
    Ok(candidate)
}

#[derive(Clone)]
struct Node {
    configured: bool,
    kind: String,
    id: String,
    record: Record,
    capsule: Option<RevealEntry>,
    status: String,
}
pub struct Registry {
    root: PathBuf,
    base: PathBuf,
    config: Option<PathBuf>,
    config_hash: Option<String>,
    nodes: Vec<Node>,
    pub diagnostics: Vec<String>,
}
impl Registry {
    pub fn load(root: &Path) -> Result<Self> {
        let root = root.canonicalize()?;
        ensure!(root.is_dir(), "Reveal root must be a directory");
        let loaded = project_config::load_nearest(&root)?;
        let reveal = loaded.as_ref().and_then(|c| c.reveal.as_ref());
        let suffix = reveal
            .and_then(|r| r.entry_suffix.as_deref())
            .unwrap_or(project_config::DEFAULT_REVEAL_ENTRY_SUFFIX);
        let types = reveal.map(|r| r.types.clone()).unwrap_or_default();
        let entries = crate::reveal_query::discover_reveal_entries(
            &root,
            suffix,
            &types,
            loaded.as_ref(),
            &[],
        )?;
        Self::from_entries(&root, loaded.as_ref(), &entries)
    }

    pub(crate) fn from_entries(
        root: &Path,
        loaded: Option<&project_config::RecurConfig>,
        entries: &[RevealEntry],
    ) -> Result<Self> {
        let mut registry = Self {
            root: root.to_path_buf(),
            base: root.to_path_buf(),
            config: None,
            config_hash: None,
            nodes: vec![],
            diagnostics: vec![],
        };
        let mut profiles = Profiles::default();
        if let Some(config) = loaded {
            registry.base = config.project_root.canonicalize()?;
            let path = config.config_path.canonicalize()?;
            ensure!(
                path.starts_with(&registry.base),
                "Configuration escapes project root"
            );
            // Parent policy may classify an explicit child, but its profiles are
            // outside that child's inventory and must not leak into results.
            if path.starts_with(root) {
                let bytes = fs::read(&path)?;
                let value: toml::Value = toml::from_str(std::str::from_utf8(&bytes)?)?;
                if let Some(value) = value.get("reveal") {
                    profiles =
                        Profiles::parse(value.as_table().context("reveal must be a table")?)?;
                }
                registry.config_hash = Some(fingerprint(&bytes));
                registry.config = Some(path);
            }
        }
        let mut bound = BTreeMap::<String, usize>::new();
        for (kind, records) in profiles.0 {
            for (id, record) in records {
                let matches: Vec<_> = record
                    .capsule
                    .as_ref()
                    .map(|reference| {
                        entries
                            .iter()
                            .filter(|e| &e.lane == reference || &e.path == reference)
                            .collect()
                    })
                    .unwrap_or_default();
                let mut status = "available";
                let capsule = if record.capsule.is_none() {
                    None
                } else if matches.is_empty() {
                    status = "missing";
                    None
                } else if matches.len() > 1 {
                    status = "ambiguous";
                    None
                } else if !matches[0].artifact.matches(Some(&kind)) {
                    status = "type-mismatch";
                    None
                } else {
                    Some(matches[0].clone())
                };
                if let Some(e) = &capsule {
                    *bound.entry(e.path.clone()).or_default() += 1;
                }
                registry.nodes.push(Node {
                    configured: true,
                    kind: kind.clone(),
                    id,
                    record,
                    capsule,
                    status: status.into(),
                });
            }
        }
        for node in &mut registry.nodes {
            if node
                .capsule
                .as_ref()
                .is_some_and(|e| bound.get(&e.path).is_some_and(|n| *n > 1))
            {
                node.status = "conflict".into();
            }
        }
        for entry in entries {
            if bound.contains_key(&entry.path) {
                continue;
            }
            if let Some(kind) = entry
                .artifact
                .r#type
                .as_deref()
                .filter(|k| ["agent", "persona", "skill"].contains(k))
            {
                registry.nodes.push(Node {
                    configured: false,
                    kind: kind.into(),
                    id: entry.lane.clone(),
                    record: Record::default(),
                    capsule: Some(entry.clone()),
                    status: "available".into(),
                });
            }
        }
        Ok(registry)
    }

    fn resolve(&self, kind: &str, id: &str) -> (String, Option<&Node>) {
        let configured: Vec<_> = self
            .nodes
            .iter()
            .filter(|n| n.configured && n.kind == kind && n.id == id)
            .collect();
        let matches: Vec<_> = if configured.is_empty() {
            self.nodes
                .iter()
                .filter(|n| {
                    n.kind == kind
                        && (n.id == id || n.capsule.as_ref().is_some_and(|c| c.lane == id))
                })
                .collect()
        } else {
            configured
        };
        if matches.len() > 1 {
            return ("ambiguous".into(), None);
        }
        if let Some(node) = matches.first() {
            return (node.status.clone(), Some(node));
        }
        let wrong = self.nodes.iter().any(|n| n.id == id);
        (if wrong { "type-mismatch" } else { "missing" }.into(), None)
    }
    fn edges(&self, node: &Node) -> Vec<Value> {
        let mut refs = Vec::new();
        if let Some(id) = &node.record.persona {
            refs.push(("persona", id));
        }
        refs.extend(node.record.skills.iter().map(|id| ("skill", id)));
        let mut seen = BTreeSet::new();
        let mut edges:Vec<Value>=refs.into_iter().filter_map(|(kind,id)| {
            if !seen.insert((kind,id)) {return None;}
            Some(json!({"owner_type":node.kind,"owner_id":node.id,"relation":if kind=="skill" {"skills"} else {"persona"},
                "target_type":kind,"target_id":id,"status":self.resolve(kind,id).0,
                "source":self.config.as_ref().map(|p| relative(&self.root,p)),"source_hash":self.config_hash}))
        }).collect();
        if let Some(capsule) = &node.record.capsule {
            edges.push(json!({"owner_type":node.kind,"owner_id":node.id,"relation":"capsule","target_type":node.kind,"target_id":capsule,"status":node.status,
                "source":self.config.as_ref().map(|p|relative(&self.root,p)),"source_hash":self.config_hash}));
        }
        edges
    }
    pub(crate) fn associations(&self, entry: &RevealEntry) -> Vec<Value> {
        self.nodes
            .iter()
            .filter(|n| {
                n.capsule.as_ref().is_some_and(|c| c.path == entry.path)
                    || (entry.path.contains("#reveal.")
                        && n.id == entry.lane
                        && entry.artifact.matches(Some(&n.kind)))
            })
            .flat_map(|n| self.edges(n))
            .collect()
    }
    pub(crate) fn configured_entries(&self) -> Vec<RevealEntry> {
        self.nodes
            .iter()
            .filter(|n| n.capsule.is_none() && self.config.is_some())
            .filter_map(|n| {
                // Only registry records, not unbound capsule entries.
                let config = self.config.as_ref()?;
                let mut fields = vec![
                    RevealField {
                        key: "artifact.type".into(),
                        value: n.kind.clone(),
                    },
                    RevealField {
                        key: "association.status".into(),
                        value: n.status.clone(),
                    },
                ];
                if let Some(path)=&n.record.path {
                    fields.push(RevealField {key: if n.kind=="skill" {"skill.path"} else {"body.path"}.into(),value:path.clone()});
                }
                if let Some(level)=&n.record.guidance_level {
                    fields.push(RevealField {key:"guidance_level".into(),value:level.clone()});
                }
                Some(RevealEntry {
                    lane: n.id.clone(),
                    path: format!(
                        "{}#reveal.{}s.{}",
                        relative(&self.root, config),
                        n.kind,
                        n.id
                    ),
                    absolute_path: config.clone(),
                    artifact: crate::reveal_artifact::TypePolicy::default().classify(
                        &n.id,
                        &['.'],
                        std::iter::once(n.kind.as_str()),
                    ),
                    fields,
                    separators: vec!['.'],
                })
            })
            .collect()
    }

    pub fn packet(
        &self,
        id: &str,
        kind: Option<&str>,
        max_files: usize,
        max_bytes: usize,
    ) -> Value {
        let mut candidates = vec![];
        let mut ambiguous = false;
        for k in ["agent", "persona"] {
            if kind.map_or(true, |selected| selected == k) {
                let (status, node) = self.resolve(k, id);
                ambiguous |= status == "ambiguous";
                if let Some(node) = node {
                    candidates.push(node);
                }
            }
        }
        let mut packet = json!({"schema":"recur-reveal-packet-v1","state":"blocked","subject":{"id":id,"type":kind},"persona":null,"skills":[],"associations":[],"sources":[],"context":{"truncated":false,"max_files":max_files,"max_bytes":max_bytes},"diagnostics":[],"execution":"not-run","mutation":"none","config_source":self.config.as_ref().map(|p| relative(&self.root,p)),"config_hash":self.config_hash});
        if candidates.len() != 1 || ambiguous {
            packet["diagnostics"] = json!([if candidates.is_empty() && !ambiguous {
                "missing subject"
            } else {
                "ambiguous subject"
            }]);
            return packet;
        }
        let subject = candidates[0];
        packet["subject"] = json!({"id":subject.id,"type":subject.kind,"status":subject.status});
        let mut queue = vec![subject];
        let mut visited = BTreeSet::new();
        let mut files = BTreeMap::new();
        let mut total = 0usize;
        let mut artifacts = vec![];
        let mut diagnostics = vec![];
        let mut associations = vec![];
        let mut skills = vec![];
        let mut sources = vec![];
        let mut truncated = false;
        while let Some(node) = queue.pop() {
            if !visited.insert((node.kind.clone(), node.id.clone())) {
                continue;
            }
            if let Some(c) = &node.capsule {
                artifacts.push(json!({"type":node.kind,"id":node.id,"path":c.path,"hash":fs::read(&c.absolute_path).ok().map(|b|fingerprint(&b))}));
            }
            if node.kind == "persona" {
                packet["persona"] = json!(node.id);
            }
            if node.status != "available" {
                diagnostics.push(format!("{}:{} {}", node.kind, node.id, node.status));
            }
            let edges = self.edges(node);
            if node.record.skills.iter().collect::<BTreeSet<_>>().len() != node.record.skills.len()
            {
                diagnostics.push(format!(
                    "duplicate skill references in {}:{}",
                    node.kind, node.id
                ));
            }
            let mut children = vec![];
            for edge in &edges {
                if edge["relation"] == "capsule" {
                    continue;
                }
                let (status, child) = self.resolve(
                    edge["target_type"].as_str().unwrap(),
                    edge["target_id"].as_str().unwrap(),
                );
                if status != "available" {
                    diagnostics.push(format!(
                        "{}:{} {status}",
                        edge["target_type"], edge["target_id"]
                    ));
                    if edge["target_type"] == "skill" {
                        skills.push(json!({"id":edge["target_id"],"status":status}));
                    }
                }
                if let Some(child) = child {
                    children.push(child);
                }
            }
            associations.extend(edges);
            queue.extend(children.into_iter().rev());
            let body = node.record.path.clone().or_else(|| {
                node.capsule.as_ref().and_then(|e| {
                    if node.kind == "skill" {
                        e.fields
                            .iter()
                            .find(|f| f.key == "skill.path")
                            .map(|f| f.value.clone())
                    } else {
                        None
                    }
                })
            });
            let path = if let Some(body) = body {
                local_path(&self.root, &self.base, &body)
            } else if let Some(c) = &node.capsule {
                Ok(c.absolute_path.clone())
            } else {
                Ok(PathBuf::new())
            };
            let mut status = node.status.clone();
            if let Err(error) = &path {
                status = "outside-root".into();
                diagnostics.push(error.to_string());
            }
            if let Ok(path) = path {
                if path.as_os_str().is_empty() {
                    if node.kind == "skill" {
                        status = "missing".into();
                        diagnostics.push(format!("No body for skill {}", node.id));
                    }
                } else {
                    let result = (|| -> Result<()> {
                        let path = path.canonicalize()?;
                        ensure!(path.starts_with(&self.root), "Body escapes root");
                        if files.contains_key(&path) {
                            return Ok(());
                        }
                        if files.len() >= max_files {
                            truncated = true;
                            bail!("Body file budget exceeded");
                        }
                        let remaining = max_bytes.saturating_sub(total);
                        let mut bytes = vec![];
                        fs::File::open(&path)?
                            .take(remaining.saturating_add(1) as u64)
                            .read_to_end(&mut bytes)?;
                        if bytes.len() > remaining {
                            truncated = true;
                            bail!("Body byte budget exceeded");
                        }
                        let body = std::str::from_utf8(&bytes)?;
                        ensure!(!body.trim().is_empty(), "Empty body");
                        if path.file_name().is_some_and(|s| s == "SKILL.md") {
                            let normalized = body.replace("\r\n", "\n");
                            let rest = normalized
                                .strip_prefix("---\n")
                                .context("SKILL.md lacks frontmatter")?;
                            let end = rest
                                .find("\n---")
                                .context("Unclosed SKILL.md frontmatter")?;
                            let metadata: serde_yaml::Value = serde_yaml::from_str(&rest[..end])?;
                            for key in ["name", "description"] {
                                ensure!(
                                    metadata[key].as_str().is_some_and(|v| !v.trim().is_empty()),
                                    "Missing skill {key}"
                                );
                            }
                        }
                        total += bytes.len();
                        files.insert(path.clone(), sources.len());
                        sources.push(json!({"path":relative(&self.root,&path),"hash":fingerprint(&bytes),"body":body}));
                        Ok(())
                    })();
                    if let Err(error) = result {
                        status = if error
                            .downcast_ref::<std::io::Error>()
                            .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound)
                        {
                            "missing"
                        } else if truncated {
                            "omitted"
                        } else {
                            "invalid"
                        }
                        .into();
                        diagnostics.push(format!("{}:{}: {error}", node.kind, node.id));
                    }
                }
            }
            if node.kind == "skill" {
                skills.push(json!({"id":node.id,"status":status,"capsule_hash":node.capsule.as_ref().and_then(|c|fs::read(&c.absolute_path).ok()).map(|b|fingerprint(&b))}));
            }
        }
        let mut skill_ids = BTreeSet::new();
        skills.retain(|skill| skill_ids.insert(skill["id"].clone().to_string()));
        // Duplicate-reference diagnostics are informative, not a missing dependency.
        let blocked = diagnostics
            .iter()
            .any(|d| !d.starts_with("duplicate skill references"));
        packet["artifacts"] = json!(artifacts);
        packet["state"] = json!(if blocked { "blocked" } else { "ready" });
        packet["skills"] = json!(skills);
        packet["associations"] = json!(associations);
        packet["sources"] = json!(sources);
        packet["diagnostics"] = json!(diagnostics);
        packet["context"]["truncated"] = json!(truncated);
        packet["context"]["bytes"] = json!(total);
        packet
    }
}

pub fn init(root: &Path, dry_run: bool) -> Result<Value> {
    let root = root.canonicalize()?;
    ensure!(root.is_dir(), "Root must be a directory");
    let path = root
        .ancestors()
        .map(|p| p.join(".recur/config.toml"))
        .find(|p| p.exists())
        .unwrap_or_else(|| root.join(".recur/config.toml"));
    let base = path
        .parent()
        .and_then(Path::parent)
        .context("Invalid config path")?
        .canonicalize()?;
    local_path(&base, &base, ".recur/config.toml")?;
    let original = if path.exists() {
        fs::read_to_string(&path)?
    } else {
        String::new()
    };
    let mut document: toml_edit::DocumentMut = original.parse()?;
    let defaults: toml_edit::DocumentMut = DEFAULTS.parse()?;
    let value: toml::Value = toml::from_str(&original)?;
    if let Some(reveal) = value.get("reveal") {
        Profiles::parse(reveal.as_table().context("reveal must be table")?)?;
    }
    if document.get("reveal").is_none() {
        document["reveal"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    for table in ["agents", "personas", "skills"] {
        if document["reveal"].get(table).is_none() {
            let item = defaults["reveal"][table].clone();
            document["reveal"][table] = if document["reveal"].is_inline_table() {
                toml_edit::Item::Value(
                    item.into_value()
                        .map_err(|_| anyhow::anyhow!("Invalid default table"))?,
                )
            } else {
                item
            };
        }
    }
    let updated = document.to_string();
    let checked: toml::Value = toml::from_str(&updated)?;
    Profiles::parse(
        checked["reveal"]
            .as_table()
            .context("Invalid generated reveal table")?,
    )?;
    let changed = updated != original;
    if changed && !dry_run {
        fs::create_dir_all(path.parent().unwrap())?;
        let mut temp = tempfile::NamedTempFile::new_in(path.parent().unwrap())?;
        temp.write_all(updated.as_bytes())?;
        temp.as_file().sync_all()?;
        ensure!(
            (if path.exists() {
                fs::read_to_string(&path)?
            } else {
                String::new()
            }) == original,
            "Config changed during init"
        );
        temp.persist(&path).map_err(|e| e.error)?;
    }
    Ok(
        json!({"schema":"recur-reveal-init-v1","path":path,"changed":changed,"dry_run":dry_run,"preview":updated,"mutation":if changed&&!dry_run{"config-written"}else{"none"}}),
    )
}
