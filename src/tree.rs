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
}

impl HierarchyTree {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            root_name: name.into(),
            children: vec![],
            file_path: None,
        }
    }

    pub fn add_child(&mut self, child: HierarchyTree) {
        self.children.push(child);
    }

    pub fn from_paths(base: impl Into<String>, _paths: &[PathBuf]) -> Self {
        // TODO: build tree from file paths
        Self::new(base)
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

    pub fn to_string(&self, _unicode: bool) -> String {
        // TODO: implement tree string formatting with unicode option
        format!("{}", self.root_name)
    }

    pub fn stats(&self) -> TreeStats {
        // TODO: compute real stats
        TreeStats::default()
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