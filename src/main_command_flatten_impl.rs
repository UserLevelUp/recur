//! Implementation of the flatten command.
//!
//! Converts structured documents (XML, JSON) into flat hierarchical dot-paths.
//! This is the universal hierarchy intermediate representation — any structured
//! format becomes searchable/transformable with existing recur commands.
//!
//! This module maps to hierarchical name: main.command.flatten.impl

use std::collections::HashMap;
use std::io::{self, IsTerminal, Read, Write};
use std::path::PathBuf;

use quick_xml::events::Event;
use quick_xml::Reader;
use serde::Serialize;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// ── Output types ─────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct FlatEntry {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub kind: EntryKind,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    Element,
    Attribute,
    Text,
}

// ── Format detection ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Xml,
    Json,
}

fn detect_format(path: Option<&PathBuf>, format_override: Option<&str>) -> Format {
    if let Some(fmt) = format_override {
        return match fmt.to_lowercase().as_str() {
            "json" => Format::Json,
            _ => Format::Xml,
        };
    }

    if let Some(path) = path {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        return match ext.as_str() {
            "json" | "jsonl" => Format::Json,
            // XML family: nuspec, csproj, fsproj, props, targets, config, svg, html, etc.
            _ => Format::Xml,
        };
    }

    // Default for stdin
    Format::Xml
}

// ── XML parsing into lightweight tree ────────────────────────

struct XmlNode {
    name: String,
    attributes: Vec<(String, String)>,
    text: String,
    children: Vec<XmlNode>,
}

fn parse_xml_tree(xml: &str) -> anyhow::Result<Vec<XmlNode>> {
    let mut reader = Reader::from_str(xml);
    let mut root_nodes: Vec<XmlNode> = Vec::new();
    let mut stack: Vec<XmlNode> = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = Vec::new();
                for attr in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let val = String::from_utf8_lossy(&attr.value).to_string();
                    attrs.push((key, val));
                }
                stack.push(XmlNode {
                    name,
                    attributes: attrs,
                    text: String::new(),
                    children: Vec::new(),
                });
            }
            Ok(Event::Empty(ref e)) => {
                // Self-closing tag like <file src="..." />
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut attrs = Vec::new();
                for attr in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let val = String::from_utf8_lossy(&attr.value).to_string();
                    attrs.push((key, val));
                }
                let node = XmlNode {
                    name,
                    attributes: attrs,
                    text: String::new(),
                    children: Vec::new(),
                };
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else {
                    root_nodes.push(node);
                }
            }
            Ok(Event::End(_)) => {
                if let Some(node) = stack.pop() {
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(node);
                    } else {
                        root_nodes.push(node);
                    }
                }
            }
            Ok(Event::Text(ref e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                let trimmed = text.trim().to_string();
                if !trimmed.is_empty() {
                    if let Some(current) = stack.last_mut() {
                        if current.text.is_empty() {
                            current.text = trimmed;
                        } else {
                            current.text.push(' ');
                            current.text.push_str(&trimmed);
                        }
                    }
                }
            }
            Ok(Event::CData(ref e)) => {
                let text = String::from_utf8_lossy(e.as_ref()).to_string();
                let trimmed = text.trim().to_string();
                if !trimmed.is_empty() {
                    if let Some(current) = stack.last_mut() {
                        if current.text.is_empty() {
                            current.text = trimmed;
                        } else {
                            current.text.push(' ');
                            current.text.push_str(&trimmed);
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {} // Skip comments, PI, declarations
            Err(e) => return Err(anyhow::anyhow!("XML parse error: {}", e)),
        }
    }

    Ok(root_nodes)
}

// ── XML tree → flat entries ──────────────────────────────────

fn flatten_xml_tree(
    nodes: &[XmlNode],
    prefix: &str,
    separator: char,
    max_depth: usize,
    current_depth: usize,
    entries: &mut Vec<FlatEntry>,
) {
    // Count sibling names to decide on indexing
    let mut name_counts: HashMap<&str, usize> = HashMap::new();
    for node in nodes {
        *name_counts.entry(&node.name).or_insert(0) += 1;
    }

    let mut name_indices: HashMap<&str, usize> = HashMap::new();

    for node in nodes {
        let count = name_counts[node.name.as_str()];
        let idx = name_indices.entry(&node.name).or_insert(0);

        let segment = if count > 1 {
            format!("{}[{}]", node.name, idx)
        } else {
            node.name.clone()
        };
        *idx += 1;

        let path = if prefix.is_empty() {
            segment
        } else {
            format!("{}{}{}", prefix, separator, segment)
        };

        // Emit attributes
        for (attr_name, attr_value) in &node.attributes {
            entries.push(FlatEntry {
                path: format!("{}@{}", path, attr_name),
                value: Some(attr_value.clone()),
                kind: EntryKind::Attribute,
            });
        }

        // Emit text content
        if !node.text.is_empty() {
            entries.push(FlatEntry {
                path: path.clone(),
                value: Some(node.text.clone()),
                kind: EntryKind::Text,
            });
        }

        // Recurse into children
        if max_depth == 0 || current_depth < max_depth {
            if !node.children.is_empty() {
                flatten_xml_tree(
                    &node.children,
                    &path,
                    separator,
                    max_depth,
                    current_depth + 1,
                    entries,
                );
            }
        }
    }
}

fn flatten_xml(content: &str, separator: char, max_depth: usize) -> anyhow::Result<Vec<FlatEntry>> {
    let nodes = parse_xml_tree(content)?;
    let mut entries = Vec::new();
    flatten_xml_tree(&nodes, "", separator, max_depth, 0, &mut entries);
    Ok(entries)
}

// ── JSON → flat entries ──────────────────────────────────────

fn flatten_json_value(
    value: &serde_json::Value,
    prefix: &str,
    separator: char,
    max_depth: usize,
    current_depth: usize,
    entries: &mut Vec<FlatEntry>,
) {
    match value {
        serde_json::Value::Object(map) => {
            if max_depth > 0 && current_depth >= max_depth {
                entries.push(FlatEntry {
                    path: prefix.to_string(),
                    value: Some(value.to_string()),
                    kind: EntryKind::Text,
                });
                return;
            }
            for (key, val) in map {
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{}{}{}", prefix, separator, key)
                };
                flatten_json_value(val, &path, separator, max_depth, current_depth + 1, entries);
            }
        }
        serde_json::Value::Array(arr) => {
            if max_depth > 0 && current_depth >= max_depth {
                entries.push(FlatEntry {
                    path: prefix.to_string(),
                    value: Some(value.to_string()),
                    kind: EntryKind::Text,
                });
                return;
            }
            for (i, val) in arr.iter().enumerate() {
                let path = format!("{}[{}]", prefix, i);
                flatten_json_value(val, &path, separator, max_depth, current_depth + 1, entries);
            }
        }
        serde_json::Value::String(s) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(s.clone()),
                kind: EntryKind::Text,
            });
        }
        serde_json::Value::Number(n) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(n.to_string()),
                kind: EntryKind::Text,
            });
        }
        serde_json::Value::Bool(b) => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some(b.to_string()),
                kind: EntryKind::Text,
            });
        }
        serde_json::Value::Null => {
            entries.push(FlatEntry {
                path: prefix.to_string(),
                value: Some("null".to_string()),
                kind: EntryKind::Text,
            });
        }
    }
}

fn flatten_json(content: &str, separator: char, max_depth: usize) -> anyhow::Result<Vec<FlatEntry>> {
    let value: serde_json::Value = serde_json::from_str(content)?;
    let mut entries = Vec::new();
    flatten_json_value(&value, "", separator, max_depth, 0, &mut entries);
    Ok(entries)
}

// ── Terminal formatter ───────────────────────────────────────

struct FlatFormatter {
    stdout: StandardStream,
    color: bool,
}

impl FlatFormatter {
    fn new(color: bool) -> Self {
        let is_tty = std::io::stdout().is_terminal();
        let should_color = color && is_tty;
        let choice = if should_color {
            ColorChoice::Always
        } else {
            ColorChoice::Never
        };
        Self {
            stdout: StandardStream::stdout(choice),
            color: should_color,
        }
    }

    fn print_entry(&mut self, entry: &FlatEntry) {
        // Split path at @ for attribute coloring
        if let Some(at_pos) = entry.path.find('@') {
            let (base, attr_part) = entry.path.split_at(at_pos);

            if self.color {
                let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
            }
            let _ = write!(self.stdout, "{}", base);

            if self.color {
                let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
            }
            let _ = write!(self.stdout, "{}", attr_part);
        } else {
            if self.color {
                let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
            }
            let _ = write!(self.stdout, "{}", entry.path);
        }

        if let Some(ref value) = entry.value {
            if self.color {
                let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
            }
            let _ = write!(self.stdout, " = ");

            if self.color {
                let _ = self.stdout.reset();
            }
            let _ = writeln!(self.stdout, "{}", value);
        } else {
            let _ = writeln!(self.stdout);
        }

        if self.color {
            let _ = self.stdout.reset();
        }
    }
}

// ── Main entry point ─────────────────────────────────────────

pub fn execute(
    file: Option<PathBuf>,
    stdin: bool,
    format: Option<String>,
    max_depth: usize,
    filter: Option<String>,
    separator: char,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    // Read content
    let content = if stdin || file.is_none() {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        std::fs::read_to_string(file.as_ref().unwrap())?
    };

    // Detect format
    let fmt = detect_format(file.as_ref(), format.as_deref());

    // Flatten
    let entries = match fmt {
        Format::Xml => flatten_xml(&content, separator, max_depth)?,
        Format::Json => flatten_json(&content, separator, max_depth)?,
    };

    // Apply filter
    let entries: Vec<_> = if let Some(ref prefix) = filter {
        entries
            .into_iter()
            .filter(|e| e.path.starts_with(prefix.as_str()))
            .collect()
    } else {
        entries
    };

    // Output
    if json {
        let output = serde_json::to_string_pretty(&entries)?;
        println!("{}", output);
    } else {
        let mut formatter = FlatFormatter::new(color);
        for entry in &entries {
            formatter.print_entry(entry);
        }
    }

    if entries.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}
