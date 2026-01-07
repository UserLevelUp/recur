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
        let pattern = if self.case_insensitive {
            self.raw.to_lowercase()
        } else {
            self.raw.clone()
        };

        let target = if self.case_insensitive {
            name.full.to_lowercase()
        } else {
            name.full.clone()
        };

        self.matches_pattern(&pattern, &target)
    }

    fn matches_pattern(&self, pattern: &str, target: &str) -> bool {
        // Handle simple cases
        if pattern == "*" || pattern == "**" {
            return true;
        }

        // Split pattern and target by dots
        let pattern_parts: Vec<&str> = pattern.split('.').collect();
        let target_parts: Vec<&str> = target.split('.').collect();

        self.matches_parts(&pattern_parts, &target_parts)
    }

    fn matches_parts(&self, pattern: &[&str], target: &[&str]) -> bool {
        let mut p_idx = 0;
        let mut t_idx = 0;

        while p_idx < pattern.len() && t_idx < target.len() {
            let p_part = pattern[p_idx];

            if p_part == "**" {
                // ** matches zero or more segments
                if p_idx == pattern.len() - 1 {
                    // ** at end matches everything remaining
                    return true;
                }

                // Try matching rest of pattern at each position
                for i in t_idx..=target.len() {
                    if self.matches_parts(&pattern[p_idx + 1..], &target[i..]) {
                        return true;
                    }
                }
                return false;
            } else if p_part == "*" {
                // * matches exactly one segment
                p_idx += 1;
                t_idx += 1;
            } else if p_part.contains('*') {
                // Wildcard within segment (e.g., "Create*" or "*Wizard*")
                if !self.glob_match_segment(p_part, target[t_idx]) {
                    return false;
                }
                p_idx += 1;
                t_idx += 1;
            } else {
                // Exact match required
                if p_part != target[t_idx] {
                    return false;
                }
                p_idx += 1;
                t_idx += 1;
            }
        }

        // Check if we consumed all parts
        // Allow trailing ** in pattern
        while p_idx < pattern.len() && pattern[p_idx] == "**" {
            p_idx += 1;
        }

        p_idx == pattern.len() && t_idx == target.len()
    }

    fn glob_match_segment(&self, pattern: &str, target: &str) -> bool {
        let pattern_parts: Vec<&str> = pattern.split('*').collect();

        if pattern_parts.is_empty() {
            return true;
        }

        let mut target_pos = 0;

        for (i, part) in pattern_parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }

            if i == 0 && !pattern.starts_with('*') {
                // First part must match at start
                if !target.starts_with(part) {
                    return false;
                }
                target_pos = part.len();
            } else if i == pattern_parts.len() - 1 && !pattern.ends_with('*') {
                // Last part must match at end
                if !target.ends_with(part) {
                    return false;
                }
            } else {
                // Middle part must be found
                if let Some(pos) = target[target_pos..].find(part) {
                    target_pos += pos + part.len();
                } else {
                    return false;
                }
            }
        }

        true
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