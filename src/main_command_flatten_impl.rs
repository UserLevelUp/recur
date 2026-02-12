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

// ── Tests ────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to get path-value pairs for easy assertion
    fn flat(entries: &[FlatEntry]) -> Vec<(&str, Option<&str>)> {
        entries
            .iter()
            .map(|e| (e.path.as_str(), e.value.as_deref()))
            .collect()
    }

    // ── XML: Basic elements ──────────────────────────────────

    #[test]
    fn xml_simple_elements() {
        let xml = r#"<root><name>hello</name><version>1.0</version></root>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert_eq!(pairs, vec![
            ("root.name", Some("hello")),
            ("root.version", Some("1.0")),
        ]);
    }

    #[test]
    fn xml_nested_elements() {
        let xml = r#"<a><b><c>deep</c></b></a>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "a.b.c");
        assert_eq!(entries[0].value.as_deref(), Some("deep"));
    }

    #[test]
    fn xml_empty_element() {
        let xml = r#"<root><empty></empty><has>text</has></root>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        // Empty element produces no entry (no text, no attrs)
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "root.has");
    }

    // ── XML: Attributes ──────────────────────────────────────

    #[test]
    fn xml_attributes() {
        let xml = r#"<file src="tools\**" target="tools" />"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert_eq!(pairs, vec![
            ("file@src", Some("tools\\**")),
            ("file@target", Some("tools")),
        ]);
    }

    #[test]
    fn xml_element_with_attrs_and_text() {
        let xml = r#"<item id="42" type="widget">gadget</item>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert_eq!(pairs, vec![
            ("item@id", Some("42")),
            ("item@type", Some("widget")),
            ("item", Some("gadget")),
        ]);
    }

    // ── XML: Repeated siblings ───────────────────────────────

    #[test]
    fn xml_repeated_siblings_get_indexed() {
        let xml = r#"<list><item>a</item><item>b</item><item>c</item></list>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert_eq!(pairs, vec![
            ("list.item[0]", Some("a")),
            ("list.item[1]", Some("b")),
            ("list.item[2]", Some("c")),
        ]);
    }

    #[test]
    fn xml_single_element_not_indexed() {
        let xml = r#"<list><item>only</item></list>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        assert_eq!(entries[0].path, "list.item"); // No [0]
    }

    #[test]
    fn xml_mixed_siblings_only_duplicates_indexed() {
        let xml = r#"<root><name>foo</name><tag>a</tag><tag>b</tag></root>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert_eq!(pairs, vec![
            ("root.name", Some("foo")),
            ("root.tag[0]", Some("a")),
            ("root.tag[1]", Some("b")),
        ]);
    }

    // ── XML: CDATA ───────────────────────────────────────────

    #[test]
    fn xml_cdata_treated_as_text() {
        let xml = r#"<doc><content><![CDATA[Hello <world>]]></content></doc>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        assert_eq!(entries[0].path, "doc.content");
        assert_eq!(entries[0].value.as_deref(), Some("Hello <world>"));
    }

    // ── XML: Whitespace handling ─────────────────────────────

    #[test]
    fn xml_whitespace_only_text_ignored() {
        let xml = r#"<root>
            <child>value</child>
        </root>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        // Only the actual text content, not the formatting whitespace
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, "root.child");
        assert_eq!(entries[0].value.as_deref(), Some("value"));
    }

    // ── XML: Self-closing tags ───────────────────────────────

    #[test]
    fn xml_self_closing_with_attrs() {
        let xml = r#"<config><db host="localhost" port="5432" /></config>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert_eq!(pairs, vec![
            ("config.db@host", Some("localhost")),
            ("config.db@port", Some("5432")),
        ]);
    }

    // ── XML: Custom separator ────────────────────────────────

    #[test]
    fn xml_custom_separator() {
        let xml = r#"<a><b><c>val</c></b></a>"#;
        let entries = flatten_xml(xml, '/', 0).unwrap();
        assert_eq!(entries[0].path, "a/b/c");
    }

    // ── XML: Max depth ───────────────────────────────────────

    #[test]
    fn xml_max_depth_limits_traversal() {
        let xml = r#"<a><b><c><d>deep</d></c></b></a>"#;
        let entries = flatten_xml(xml, '.', 2).unwrap();
        // Depth 2 means: a (depth 0) -> b (depth 1) -> c (depth 2, but children skipped)
        assert!(entries.is_empty() || !entries.iter().any(|e| e.path.contains("d")));
    }

    // ── XML: Namespaces ──────────────────────────────────────

    #[test]
    fn xml_namespace_prefix_preserved() {
        let xml = r#"<soap:Envelope><soap:Body><ns:Data>val</ns:Data></soap:Body></soap:Envelope>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        assert_eq!(entries[0].path, "soap:Envelope.soap:Body.ns:Data");
        assert_eq!(entries[0].value.as_deref(), Some("val"));
    }

    // ── XML: Real-world nuspec ───────────────────────────────

    #[test]
    fn xml_nuspec_structure() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<package>
  <metadata>
    <id>recur</id>
    <version>0.2.1</version>
    <title>recur</title>
  </metadata>
  <files>
    <file src="tools\**" target="tools" />
  </files>
</package>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert!(pairs.contains(&("package.metadata.id", Some("recur"))));
        assert!(pairs.contains(&("package.metadata.version", Some("0.2.1"))));
        assert!(pairs.contains(&("package.metadata.title", Some("recur"))));
        assert!(pairs.contains(&("package.files.file@src", Some("tools\\**"))));
        assert!(pairs.contains(&("package.files.file@target", Some("tools"))));
    }

    // ── XML: Entity references ───────────────────────────────

    #[test]
    fn xml_entities_decoded() {
        let xml = r#"<root><text>a &amp; b &lt; c</text></root>"#;
        let entries = flatten_xml(xml, '.', 0).unwrap();
        assert_eq!(entries[0].value.as_deref(), Some("a & b < c"));
    }

    // ── JSON: Basic objects ──────────────────────────────────

    #[test]
    fn json_simple_object() {
        let json = r#"{"name":"recur","version":"0.2.5"}"#;
        let entries = flatten_json(json, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert!(pairs.contains(&("name", Some("recur"))));
        assert!(pairs.contains(&("version", Some("0.2.5"))));
    }

    #[test]
    fn json_nested_object() {
        let json = r#"{"config":{"database":{"host":"localhost","port":"5432"}}}"#;
        let entries = flatten_json(json, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert!(pairs.contains(&("config.database.host", Some("localhost"))));
        assert!(pairs.contains(&("config.database.port", Some("5432"))));
    }

    // ── JSON: Arrays ─────────────────────────────────────────

    #[test]
    fn json_arrays_indexed() {
        let json = r#"{"tags":["search","grep","hierarchy"]}"#;
        let entries = flatten_json(json, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert!(pairs.contains(&("tags[0]", Some("search"))));
        assert!(pairs.contains(&("tags[1]", Some("grep"))));
        assert!(pairs.contains(&("tags[2]", Some("hierarchy"))));
    }

    #[test]
    fn json_array_of_objects() {
        let json = r#"{"users":[{"name":"Joe"},{"name":"Skippy"}]}"#;
        let entries = flatten_json(json, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert!(pairs.contains(&("users[0].name", Some("Joe"))));
        assert!(pairs.contains(&("users[1].name", Some("Skippy"))));
    }

    // ── JSON: Primitive types ────────────────────────────────

    #[test]
    fn json_booleans_and_null() {
        let json = r#"{"active":true,"deleted":false,"notes":null}"#;
        let entries = flatten_json(json, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert!(pairs.contains(&("active", Some("true"))));
        assert!(pairs.contains(&("deleted", Some("false"))));
        assert!(pairs.contains(&("notes", Some("null"))));
    }

    #[test]
    fn json_numbers() {
        let json = r#"{"count":42,"ratio":3.14}"#;
        let entries = flatten_json(json, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert!(pairs.contains(&("count", Some("42"))));
        assert!(pairs.contains(&("ratio", Some("3.14"))));
    }

    // ── JSON: Max depth ──────────────────────────────────────

    #[test]
    fn json_max_depth_limits_traversal() {
        let json = r#"{"a":{"b":{"c":{"d":"deep"}}}}"#;
        let entries = flatten_json(json, '.', 2).unwrap();
        // At depth 2 we hit "a.b" which contains {"c":{"d":"deep"}}
        // That should be serialized as a raw JSON string
        assert!(!entries.iter().any(|e| e.path == "a.b.c.d"));
        assert!(entries.iter().any(|e| e.path == "a.b"));
    }

    // ── JSON: Custom separator ───────────────────────────────

    #[test]
    fn json_custom_separator() {
        let json = r#"{"a":{"b":"val"}}"#;
        let entries = flatten_json(json, '/', 0).unwrap();
        assert_eq!(entries[0].path, "a/b");
    }

    // ── JSON: Real-world package.json style ──────────────────

    #[test]
    fn json_package_json_style() {
        let json = r#"{
            "name": "my-app",
            "version": "1.0.0",
            "dependencies": {
                "express": "^4.18.0",
                "lodash": "^4.17.21"
            },
            "scripts": {
                "start": "node index.js",
                "test": "jest"
            }
        }"#;
        let entries = flatten_json(json, '.', 0).unwrap();
        let pairs = flat(&entries);
        assert!(pairs.contains(&("name", Some("my-app"))));
        assert!(pairs.contains(&("dependencies.express", Some("^4.18.0"))));
        assert!(pairs.contains(&("scripts.test", Some("jest"))));
    }

    // ── Format detection ─────────────────────────────────────

    #[test]
    fn detect_xml_extensions() {
        assert_eq!(detect_format(Some(&PathBuf::from("file.xml")), None), Format::Xml);
        assert_eq!(detect_format(Some(&PathBuf::from("file.nuspec")), None), Format::Xml);
        assert_eq!(detect_format(Some(&PathBuf::from("file.csproj")), None), Format::Xml);
        assert_eq!(detect_format(Some(&PathBuf::from("file.svg")), None), Format::Xml);
    }

    #[test]
    fn detect_json_extensions() {
        assert_eq!(detect_format(Some(&PathBuf::from("file.json")), None), Format::Json);
        assert_eq!(detect_format(Some(&PathBuf::from("data.jsonl")), None), Format::Json);
    }

    #[test]
    fn detect_format_override() {
        // Override should win over extension
        assert_eq!(detect_format(Some(&PathBuf::from("file.xml")), Some("json")), Format::Json);
        assert_eq!(detect_format(Some(&PathBuf::from("file.json")), Some("xml")), Format::Xml);
    }

    // ── Cross-format: same hierarchy, same output ────────────

    #[test]
    fn xml_and_json_produce_same_paths() {
        let xml = r#"<config><db><host>localhost</host><port>5432</port></db></config>"#;
        let json = r#"{"config":{"db":{"host":"localhost","port":"5432"}}}"#;

        let xml_entries = flatten_xml(xml, '.', 0).unwrap();
        let json_entries = flatten_json(json, '.', 0).unwrap();

        let xml_pairs: Vec<_> = xml_entries.iter().map(|e| (&e.path, e.value.as_deref())).collect();
        let json_pairs: Vec<_> = json_entries.iter().map(|e| (&e.path, e.value.as_deref())).collect();

        // Both should produce config.db.host = localhost and config.db.port = 5432
        assert!(xml_pairs.iter().any(|(p, v)| p.as_str() == "config.db.host" && *v == Some("localhost")));
        assert!(json_pairs.iter().any(|(p, v)| p.as_str() == "config.db.host" && *v == Some("localhost")));
        assert!(xml_pairs.iter().any(|(p, v)| p.as_str() == "config.db.port" && *v == Some("5432")));
        assert!(json_pairs.iter().any(|(p, v)| p.as_str() == "config.db.port" && *v == Some("5432")));
    }
}
