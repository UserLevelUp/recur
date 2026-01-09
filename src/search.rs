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
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}

/// Result of a caller search (function call detection).
#[derive(Debug, Clone)]
pub struct CallerResult {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
    pub match_start: usize,
    pub match_end: usize,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
    pub is_hierarchical: bool,  // Marks if file is hierarchical
    pub depth: usize,           // Depth in hierarchy (0 = flat)
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

                // Read all lines into a vector for context extraction
                let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

                for (line_num, line) in all_lines.iter().enumerate() {
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
                        let context_lines = self.options.context_lines;

                        // Extract context before
                        let start_before = line_num.saturating_sub(context_lines);
                        let context_before: Vec<String> = all_lines[start_before..line_num]
                            .iter()
                            .cloned()
                            .collect();

                        // Extract context after
                        let end_after = (line_num + 1 + context_lines).min(all_lines.len());
                        let context_after: Vec<String> = all_lines[line_num + 1..end_after]
                            .iter()
                            .cloned()
                            .collect();

                        results.push(SearchResult {
                            path: file_path.clone(),
                            line_number: line_num + 1,
                            line: line.clone(),
                            match_start: pos,
                            match_end: pos + query.len(),
                            context_before,
                            context_after,
                        });
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

                // Read all lines into a vector for context extraction
                let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

                for (line_num, line) in all_lines.iter().enumerate() {
                    if let Some(mat) = regex.find(line) {
                        let context_lines = self.options.context_lines;

                        // Extract context before
                        let start_before = line_num.saturating_sub(context_lines);
                        let context_before: Vec<String> = all_lines[start_before..line_num]
                            .iter()
                            .cloned()
                            .collect();

                        // Extract context after
                        let end_after = (line_num + 1 + context_lines).min(all_lines.len());
                        let context_after: Vec<String> = all_lines[line_num + 1..end_after]
                            .iter()
                            .cloned()
                            .collect();

                        results.push(SearchResult {
                            path: file_path.clone(),
                            line_number: line_num + 1,
                            line: line.clone(),
                            match_start: mat.start(),
                            match_end: mat.end(),
                            context_before,
                            context_after,
                        });
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

                // Read all lines into a vector for context extraction
                let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

                for (line_num, line) in all_lines.iter().enumerate() {
                    // Look for dot-notation identifiers in the line
                    if let Some(matches) = self.find_identifiers_in_line(line, pattern) {
                        for (start, end) in matches {
                            let context_lines = self.options.context_lines;

                            // Extract context before
                            let start_before = line_num.saturating_sub(context_lines);
                            let context_before: Vec<String> = all_lines[start_before..line_num]
                                .iter()
                                .cloned()
                                .collect();

                            // Extract context after
                            let end_after = (line_num + 1 + context_lines).min(all_lines.len());
                            let context_after: Vec<String> = all_lines[line_num + 1..end_after]
                                .iter()
                                .cloned()
                                .collect();

                            results.push(SearchResult {
                                path: file_path.clone(),
                                line_number: line_num + 1,
                                line: line.clone(),
                                match_start: start,
                                match_end: end,
                                context_before,
                                context_after,
                            });
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

/// Searches for function/method callers in hierarchically scoped files.
pub struct CallerSearcher {
    pub options: SearchOptions,
}

impl CallerSearcher {
    pub fn new(options: SearchOptions) -> Self {
        Self { options }
    }

    pub fn find_callers(
        &self,
        function: &str,
        scope: &HierarchyPattern,
    ) -> anyhow::Result<Vec<CallerResult>> {
        let mut results = Vec::new();

        // Step 1: Find all files matching the scope
        let file_searcher = FileSearcher::new(self.options.clone());
        let mut files = file_searcher.find(scope);

        // Step 2: Sort files - hierarchical first, then by depth (deeper first)
        files.sort_by_key(|path| {
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            let hier_name = filename.rsplit_once('.')
                .map(|(name, _)| name)
                .unwrap_or(filename);

            let depth = hier_name.matches('.').count();
            let is_hierarchical = depth > 0;

            // Sort key: (hierarchical last=false, depth reversed)
            // This gives: hierarchical files first, deeper files first
            (!is_hierarchical, std::cmp::Reverse(depth))
        });

        // Step 3: Build regex pattern for function calls
        // Matches: functionName( or functionName ( with optional whitespace
        let pattern_str = if self.options.case_insensitive {
            format!(r"(?i)\b{}\s*\(", regex::escape(function))
        } else {
            format!(r"\b{}\s*\(", regex::escape(function))
        };
        let call_regex = regex::Regex::new(&pattern_str)?;

        // Step 4: Search within each file
        for file_path in files {
            if let Ok(file) = fs::File::open(&file_path) {
                let reader = BufReader::new(file);

                // Read all lines
                let all_lines: Vec<String> = reader.lines()
                    .filter_map(|l| l.ok())
                    .collect();

                // Determine if this file is hierarchical
                let filename = file_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                let hier_name = filename.rsplit_once('.')
                    .map(|(name, _)| name)
                    .unwrap_or(filename);
                let depth = hier_name.matches('.').count();
                let is_hierarchical = depth > 0;

                // Step 5: Search each line for function calls
                for (line_num, line) in all_lines.iter().enumerate() {
                    if let Some(mat) = call_regex.find(line) {
                        // Extract context
                        let context_lines = self.options.context_lines;
                        let start_before = line_num.saturating_sub(context_lines);
                        let context_before: Vec<String> = all_lines[start_before..line_num]
                            .iter()
                            .cloned()
                            .collect();

                        let end_after = (line_num + 1 + context_lines).min(all_lines.len());
                        let context_after: Vec<String> = all_lines[line_num + 1..end_after]
                            .iter()
                            .cloned()
                            .collect();

                        results.push(CallerResult {
                            path: file_path.clone(),
                            line_number: line_num + 1,
                            line: line.clone(),
                            match_start: mat.start(),
                            match_end: mat.end(),
                            context_before,
                            context_after,
                            is_hierarchical,
                            depth,
                        });
                    }
                }
            }
        }

        Ok(results)
    }
}

/// Result of a callee search (functions called by a function).
/// Reuses CallerResult since the structure is the same.
pub type CalleeResult = CallerResult;

/// Searches for function/method callees (functions that a given function calls).
pub struct CalleeSearcher {
    pub options: SearchOptions,
}

impl CalleeSearcher {
    pub fn new(options: SearchOptions) -> Self {
        Self { options }
    }

    pub fn find_callees(
        &self,
        function: &str,
        scope: &HierarchyPattern,
    ) -> anyhow::Result<Vec<CalleeResult>> {
        let mut results = Vec::new();

        // Step 1: Find all files matching the scope
        let file_searcher = FileSearcher::new(self.options.clone());
        let mut files = file_searcher.find(scope);

        // Step 2: Sort files - hierarchical first, then by depth (deeper first)
        files.sort_by_key(|path| {
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            let hier_name = filename.rsplit_once('.')
                .map(|(name, _)| name)
                .unwrap_or(filename);

            let depth = hier_name.matches('.').count();
            let is_hierarchical = depth > 0;

            (!is_hierarchical, std::cmp::Reverse(depth))
        });

        // Step 3: Build regex pattern for function definition
        // Match function declarations (must have access modifier or return type before function name)
        // This matches patterns like: "public void FuncName(" or "async Task FuncName(" or "static int FuncName("
        let func_pattern_str = if self.options.case_insensitive {
            format!(r"(?i)(public|private|protected|internal|static|async|virtual|override|abstract|sealed|\w+)\s+(\w+\s+)*{}\s*\(", regex::escape(function))
        } else {
            format!(r"(public|private|protected|internal|static|async|virtual|override|abstract|sealed|\w+)\s+(\w+\s+)*{}\s*\(", regex::escape(function))
        };
        let func_regex = regex::Regex::new(&func_pattern_str)?;

        // Step 4: Build regex for finding function calls (callees)
        let callee_pattern = regex::Regex::new(r"\b(\w+)\s*\(")?;

        // Step 5: Search within each file
        for file_path in files {
            if let Ok(file) = fs::File::open(&file_path) {
                let reader = BufReader::new(file);
                let all_lines: Vec<String> = reader.lines()
                    .filter_map(|l| l.ok())
                    .collect();

                // Determine if this file is hierarchical
                let filename = file_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                let hier_name = filename.rsplit_once('.')
                    .map(|(name, _)| name)
                    .unwrap_or(filename);
                let depth = hier_name.matches('.').count();
                let is_hierarchical = depth > 0;

                // Step 6: Find the function definition and extract callees
                let mut in_function = false;
                let mut brace_count = 0;

                for (line_num, line) in all_lines.iter().enumerate() {
                    // Check if we're entering the target function
                    let just_entered = !in_function && func_regex.is_match(line);
                    if just_entered {
                        in_function = true;
                        brace_count = line.matches('{').count() as i32 - line.matches('}').count() as i32;
                    }

                    // If we're inside the function, track braces and find callees
                    if in_function {
                        // Update brace count (but not on the line we just entered, we already counted it)
                        if !just_entered {
                            brace_count += line.matches('{').count() as i32 - line.matches('}').count() as i32;
                        }

                        // Find all function calls in this line
                        for cap in callee_pattern.captures_iter(line) {
                            let callee_name = &cap[1];

                            // Skip common keywords and the function itself
                            if callee_name == function ||
                               callee_name == "if" || callee_name == "for" || callee_name == "while" ||
                               callee_name == "switch" || callee_name == "catch" || callee_name == "return" ||
                               callee_name == "async" || callee_name == "await" || callee_name == "new" {
                                continue;
                            }

                            // Skip if this is the function definition itself (on the line we just entered)
                            if just_entered && cap.get(0).map_or(false, |m| m.start() < line.find('{').unwrap_or(line.len())) {
                                // This match is before the opening brace, likely the function definition
                                continue;
                            }

                            let match_pos = line.find(&format!("{}(", callee_name)).unwrap_or(0);

                            // Extract context
                            let context_lines = self.options.context_lines;
                            let start_before = line_num.saturating_sub(context_lines);
                            let context_before: Vec<String> = all_lines[start_before..line_num]
                                .iter()
                                .cloned()
                                .collect();

                            let end_after = (line_num + 1 + context_lines).min(all_lines.len());
                            let context_after: Vec<String> = all_lines[line_num + 1..end_after]
                                .iter()
                                .cloned()
                                .collect();

                            results.push(CalleeResult {
                                path: file_path.clone(),
                                line_number: line_num + 1,
                                line: line.clone(),
                                match_start: match_pos,
                                match_end: match_pos + callee_name.len(),
                                context_before,
                                context_after,
                                is_hierarchical,
                                depth,
                            });
                        }

                        // Check if we've exited the function
                        if brace_count <= 0 {
                            in_function = false;
                            brace_count = 0;
                        }
                    }
                }
            }
        }

        Ok(results)
    }
}