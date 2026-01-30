//! Hierarchy tree representation and display.

use std::fmt;
use std::path::PathBuf;

/// Statistics about a hierarchy tree.
#[derive(Debug, Clone, Default)]
pub struct TreeStats {
    pub total_files: usize,
    pub total_dirs: usize,
    pub max_depth: usize,
}

impl fmt::Display for TreeStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} files, {} directories, max depth {}",
            self.total_files, self.total_dirs, self.max_depth
        )
    }
}

/// A tree structure representing hierarchical search results.
#[derive(Debug, Clone, Default)]
pub struct HierarchyTree {
    pub root_name: String,
    pub children: Vec<HierarchyTree>,
    pub file_path: Option<PathBuf>,
    pub extension: Option<String>,
}

impl HierarchyTree {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            root_name: name.into(),
            children: vec![],
            file_path: None,
            extension: None,
        }
    }

    pub fn add_child(&mut self, child: HierarchyTree) {
        self.children.push(child);
    }

    pub fn from_paths(base: impl Into<String>, paths: &[PathBuf]) -> Self {
        let base_str = base.into();
        let mut root = Self::new(&base_str);

        for path in paths {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                // Remove file extension to get hierarchical name
                let (hier_name, ext) = filename.rsplit_once('.')
                    .map(|(name, ext)| (name, Some(ext.to_string())))
                    .unwrap_or((filename, None));

                // Skip if it doesn't start with base
                if !hier_name.starts_with(&base_str) {
                    continue;
                }

                // Get the part after the base
                let suffix = &hier_name[base_str.len()..];

                // Split into parts and add to tree
                if suffix.is_empty() {
                    // This is the base file itself
                    root.file_path = Some(path.clone());
                    root.extension = ext;
                } else if suffix.starts_with('.') {
                    let parts: Vec<&str> = suffix[1..].split('.').collect();
                    root.add_path_parts(&parts, path.clone(), ext);
                }
            }
        }

        root
    }

    fn add_path_parts(&mut self, parts: &[&str], full_path: PathBuf, extension: Option<String>) {
        if parts.is_empty() {
            return;
        }

        let first = parts[0];

        // Find or create child node
        let child = self.children.iter_mut()
            .find(|c| c.root_name == first);

        if let Some(child_node) = child {
            if parts.len() == 1 {
                child_node.file_path = Some(full_path);
                child_node.extension = extension;
            } else {
                child_node.add_path_parts(&parts[1..], full_path, extension);
            }
        } else {
            let mut new_child = HierarchyTree::new(first);
            if parts.len() == 1 {
                new_child.file_path = Some(full_path);
                new_child.extension = extension;
            } else {
                new_child.add_path_parts(&parts[1..], full_path, extension);
            }
            self.children.push(new_child);
        }
    }

    pub fn print(&self) {
        self.print_recursive(0);
    }

    fn print_recursive(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}{}", indent, self.root_name);
        for child in &self.children {
            child.print_recursive(depth + 1);
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn to_string(&self, unicode: bool) -> String {
        let mut output = String::new();
        output.push_str(&self.root_name);
        if self.file_path.is_some() {
            output.push_str(" (base)\n");
        } else {
            output.push('\n');
        }

        let child_count = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let is_last = i == child_count - 1;
            self.format_child(&mut output, child, "", is_last, unicode);
        }

        output
    }

    fn format_child(&self, output: &mut String, node: &HierarchyTree, prefix: &str, is_last: bool, unicode: bool) {
        let (branch, cont) = if unicode {
            if is_last {
                ("└── ", "    ")
            } else {
                ("├── ", "│   ")
            }
        } else {
            if is_last {
                ("+-- ", "    ")
            } else {
                ("|-- ", "|   ")
            }
        };

        output.push_str(prefix);
        output.push_str(branch);
        output.push_str(&node.root_name);
        if node.file_path.is_some() {
            if let Some(ref ext) = node.extension {
                output.push('.');
                output.push_str(ext);
            }
        }
        output.push('\n');

        let child_count = node.children.len();
        let new_prefix = format!("{}{}", prefix, cont);
        for (i, child) in node.children.iter().enumerate() {
            let is_last_child = i == child_count - 1;
            self.format_child(output, child, &new_prefix, is_last_child, unicode);
        }
    }

    pub fn stats(&self) -> TreeStats {
        let mut stats = TreeStats {
            total_files: if self.file_path.is_some() { 1 } else { 0 },
            total_dirs: 0,
            max_depth: 0,
        };

        self.compute_stats(&mut stats, 1);
        stats
    }

    fn compute_stats(&self, stats: &mut TreeStats, depth: usize) {
        if depth > stats.max_depth {
            stats.max_depth = depth;
        }

        for child in &self.children {
            if child.file_path.is_some() {
                stats.total_files += 1;
            }
            if !child.children.is_empty() {
                stats.total_dirs += 1;
            }
            child.compute_stats(stats, depth + 1);
        }
    }
}

impl fmt::Display for HierarchyTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root_name)
    }
}

// Allow JSON serialization
impl serde::Serialize for HierarchyTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("HierarchyTree", 3)?;
        state.serialize_field("name", &self.root_name)?;
        state.serialize_field("children", &self.children)?;
        state.serialize_field("path", &self.file_path)?;
        state.end()
    }
}