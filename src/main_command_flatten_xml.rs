use std::collections::HashMap;

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::main_command_flatten_impl::{EntryKind, FlatEntry};

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
            Ok(_) => {}
            Err(e) => return Err(anyhow::anyhow!("XML parse error: {}", e)),
        }
    }

    Ok(root_nodes)
}

fn flatten_xml_tree(
    nodes: &[XmlNode],
    prefix: &str,
    separator: char,
    max_depth: usize,
    current_depth: usize,
    entries: &mut Vec<FlatEntry>,
) {
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

        for (attr_name, attr_value) in &node.attributes {
            entries.push(FlatEntry {
                path: format!("{}@{}", path, attr_name),
                value: Some(attr_value.clone()),
                kind: EntryKind::Attribute,
            });
        }

        if !node.text.is_empty() {
            entries.push(FlatEntry {
                path: path.clone(),
                value: Some(node.text.clone()),
                kind: EntryKind::Text,
            });
        }

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

pub fn flatten_xml(
    content: &str,
    separator: char,
    max_depth: usize,
) -> anyhow::Result<Vec<FlatEntry>> {
    let nodes = parse_xml_tree(content)?;
    let mut entries = Vec::new();
    flatten_xml_tree(&nodes, "", separator, max_depth, 0, &mut entries);
    Ok(entries)
}
