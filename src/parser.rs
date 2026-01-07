//! Pattern parsing for hierarchical searches.

use std::fmt;

/// A parsed hierarchy pattern (e.g., `Module.SubModule.*`).
#[derive(Debug, Clone)]
pub struct HierarchyPattern {
    pub raw: String,
    pub case_insensitive: bool,
}

impl HierarchyPattern {
    pub fn parse(pattern: &str) -> Result<Self, PatternError> {
        Ok(Self {
            raw: pattern.to_string(),
            case_insensitive: false,
        })
    }

    /// Returns a case-insensitive version of this pattern.
    pub fn case_insensitive(mut self) -> Self {
        self.case_insensitive = true;
        self
    }

    pub fn matches(&self, name: &HierarchicalName) -> bool {
        // TODO: implement real glob/wildcard matching
        if self.case_insensitive {
            name.full.to_lowercase().contains(&self.raw.to_lowercase()) || self.raw == "*"
        } else {
            name.full.contains(&self.raw) || self.raw == "*"
        }
    }
}

/// A hierarchical name (e.g., `std::collections::HashMap`).
#[derive(Debug, Clone)]
pub struct HierarchicalName {
    pub full: String,
}

impl HierarchicalName {
    pub fn new(full: impl Into<String>) -> Self {
        Self { full: full.into() }
    }
}

impl fmt::Display for HierarchicalName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full)
    }
}

/// Error returned when a pattern fails to parse.
#[derive(Debug, Clone)]
pub struct PatternError {
    pub message: String,
}

impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PatternError {}