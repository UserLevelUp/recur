//! Search implementations for files, content, and identifiers.

use std::path::PathBuf;
use std::fs;
use std::io::{BufRead, BufReader};
use crate::parser::{HierarchyPattern, HierarchicalName};

/// Options controlling search behavior.
#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    pub root: PathBuf,
    pub case_insensitive: bool,
    pub case_sensitive: bool,
    pub max_depth: Option<usize>,
    pub include_hidden: bool,
    pub extensions: Vec<String>,
    pub context_lines: usize,
}

/// Result of a content or identifier search.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
    pub match_start: usize,
    pub match_end: usize,
}

/// Searches for files matching a hierarchy pattern.
pub struct FileSearcher {
    pub options: SearchOptions,
}

impl FileSearcher {
    pub fn new(options: SearchOptions) -> Self {
        Self { options }
    }

    pub fn find(&self, pattern: &HierarchyPattern) -> Vec<PathBuf> {
        let mut results = Vec::new();
        let mut walker = walkdir::WalkDir::new(&self.options.root);

        if let Some(max_depth) = self.options.max_depth {
            walker = walker.max_depth(max_depth);
        }

        for entry in walker {
            if let Ok(entry) = entry {
                // Skip hidden files/directories if requested
                if !self.options.include_hidden {
                    if let Some(name) = entry.file_name().to_str() {
                        if name.starts_with('.') {
                            continue;
                        }
                    }
                }

                if entry.file_type().is_file() {
                    if let Some(filename) = entry.file_name().to_str() {
                        // Filter by extension if specified
                        if !self.options.extensions.is_empty() {
                            let has_valid_ext = self.options.extensions.iter().any(|ext| {
                                filename.ends_with(ext)
                            });
                            if !has_valid_ext {
                                continue;
                            }
                        }

                        // Extract hierarchical name from filename (remove extension)
                        let name_without_ext = filename.rsplit_once('.')
                            .map(|(name, _)| name)
                            .unwrap_or(filename);

                        let hier_name = HierarchicalName::new(name_without_ext);

                        if pattern.matches(&hier_name) {
                            results.push(entry.path().to_path_buf());
                        }
                    }
                }
            }
        }

        results
    }

    pub fn search(&self, pattern: &HierarchyPattern) -> Vec<PathBuf> {
        self.find(pattern)
    }

    pub fn find_related(&self, filename: &str) -> Vec<PathBuf> {
        // Extract the base hierarchy from the filename
        let base = filename.rsplit_once('.')
            .and_then(|(name, _)| name.rsplit_once('.'))
            .map(|(parent, _)| parent)
            .unwrap_or(filename);

        // Find all files that start with the base
        let pattern_str = format!("{}.*", base);
        if let Ok(pattern) = HierarchyPattern::parse(&pattern_str) {
            self.find(&pattern)
        } else {
            vec![]
        }
    }

    pub fn find_children(&self, parent: &str) -> Vec<PathBuf> {
        // Find all files that are children of parent hierarchy
        let pattern_str = format!("{}.**", parent);
        if let Ok(pattern) = HierarchyPattern::parse(&pattern_str) {
            self.find(&pattern)
        } else {
            vec![]
        }
    }

}

/// Searches file contents for a pattern.
pub struct ContentSearcher {
    pub options: SearchOptions,
}

impl ContentSearcher {
    pub fn new(options: SearchOptions) -> Self {
        Self { options }
    }

    pub fn search(&self, query: &str, scope: &HierarchyPattern) -> Vec<SearchResult> {
        let mut results = Vec::new();

        // First find files matching the scope
        let file_searcher = FileSearcher::new(self.options.clone());
        let files = file_searcher.find(scope);

        // Search within each file
        for file_path in files {
            if let Ok(file) = fs::File::open(&file_path) {
                let reader = BufReader::new(file);

                for (line_num, line_result) in reader.lines().enumerate() {
                    if let Ok(line) = line_result {
                        let search_line = if self.options.case_insensitive {
                            line.to_lowercase()
                        } else {
                            line.clone()
                        };

                        let search_query = if self.options.case_insensitive {
                            query.to_lowercase()
                        } else {
                            query.to_string()
                        };

                        if let Some(pos) = search_line.find(&search_query) {
                            results.push(SearchResult {
                                path: file_path.clone(),
                                line_number: line_num + 1,
                                line: line.clone(),
                                match_start: pos,
                                match_end: pos + query.len(),
                            });
                        }
                    }
                }
            }
        }

        results
    }

    pub fn search_regex(&self, regex: &regex::Regex, scope: &HierarchyPattern) -> Vec<SearchResult> {
        let mut results = Vec::new();

        // First find files matching the scope
        let file_searcher = FileSearcher::new(self.options.clone());
        let files = file_searcher.find(scope);

        // Search within each file
        for file_path in files {
            if let Ok(file) = fs::File::open(&file_path) {
                let reader = BufReader::new(file);

                for (line_num, line_result) in reader.lines().enumerate() {
                    if let Ok(line) = line_result {
                        if let Some(mat) = regex.find(&line) {
                            results.push(SearchResult {
                                path: file_path.clone(),
                                line_number: line_num + 1,
                                line: line.clone(),
                                match_start: mat.start(),
                                match_end: mat.end(),
                            });
                        }
                    }
                }
            }
        }

        results
    }
}

/// Searches for identifiers (functions, types, etc.) matching a pattern.
pub struct IdentifierSearcher {
    pub options: SearchOptions,
}

impl IdentifierSearcher {
    pub fn new(options: SearchOptions) -> Self {
        Self { options }
    }

    pub fn search(&self, pattern: &HierarchyPattern) -> Vec<SearchResult> {
        let mut results = Vec::new();

        // Search all files (or filtered by extension)
        let all_pattern = HierarchyPattern::parse("**").unwrap_or_else(|_| pattern.clone());
        let file_searcher = FileSearcher::new(self.options.clone());
        let files = file_searcher.find(&all_pattern);

        // Search for identifiers matching the pattern in file contents
        for file_path in files {
            if let Ok(file) = fs::File::open(&file_path) {
                let reader = BufReader::new(file);

                for (line_num, line_result) in reader.lines().enumerate() {
                    if let Ok(line) = line_result {
                        // Look for dot-notation identifiers in the line
                        if let Some(matches) = self.find_identifiers_in_line(&line, pattern) {
                            for (start, end) in matches {
                                results.push(SearchResult {
                                    path: file_path.clone(),
                                    line_number: line_num + 1,
                                    line: line.clone(),
                                    match_start: start,
                                    match_end: end,
                                });
                            }
                        }
                    }
                }
            }
        }

        results
    }

    fn find_identifiers_in_line(&self, line: &str, pattern: &HierarchyPattern) -> Option<Vec<(usize, usize)>> {
        let mut matches = Vec::new();

        // Simple identifier extraction: look for sequences of word chars separated by dots
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i].is_alphanumeric() || chars[i] == '_' {
                let start = i;
                let mut identifier = String::new();

                // Extract identifier (alphanumeric, underscore, dots)
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '.') {
                    identifier.push(chars[i]);
                    i += 1;
                }

                // Check if identifier contains dots (hierarchical)
                if identifier.contains('.') {
                    let hier_name = HierarchicalName::new(&identifier);
                    if pattern.matches(&hier_name) {
                        matches.push((start, i));
                    }
                }
            } else {
                i += 1;
            }
        }

        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }
}