//! Search implementations for files, content, and identifiers.

use std::path::PathBuf;
use std::fs;
use std::io::{BufRead, BufReader};
use anyhow::Context;
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
                let mut body_start_pos: Option<usize> = None;

                for (line_num, line) in all_lines.iter().enumerate() {
                    // Check if we're entering the target function
                    let mut just_entered = false;
                    if !in_function {
                        if let Some(mat) = func_regex.find(line) {
                            just_entered = true;
                            in_function = true;

                            let match_end = mat.end();
                            let tail = &line[match_end..];
                            body_start_pos = tail.find('{').map(|offset| match_end + offset);

                            let brace_slice = body_start_pos
                                .and_then(|start| line.get(start..))
                                .unwrap_or("");
                            brace_count = brace_slice.matches('{').count() as i32
                                - brace_slice.matches('}').count() as i32;
                        }
                    }

                    // If we're inside the function, track braces and find callees
                    if in_function {
                        // Update brace count (but not on the line we just entered, we already counted it)
                        if !just_entered {
                            brace_count += line.matches('{').count() as i32 - line.matches('}').count() as i32;
                        }

                        let skip_before = if just_entered {
                            body_start_pos.unwrap_or(line.len())
                        } else {
                            0
                        };

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

                            // Skip call sites before the function body starts on the definition line
                            if just_entered && cap.get(0).map_or(false, |m| m.start() < skip_before) {
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

/// Direction for call tracing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceDirection {
    Callees,  // What this function calls (dependencies)
    Callers,  // Who calls this function (reverse dependencies)
    Both,     // Both directions
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceCallKind {
    Function,
    Method,
    Ctor,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct TraceCallSite {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
    pub call_kind: TraceCallKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceStopReason {
    DepthLimit,
    Unresolved,
    Ambiguous,
    WidthLimit,
}

/// Options for trace searching
#[derive(Debug, Clone)]
pub struct TraceOptions {
    pub max_width: usize,  // Max branches per level
    pub verbose: bool,     // Show full paths vs abbreviated
    pub pick: Option<usize>,
}

/// A node in the call trace tree
#[derive(Debug, Clone)]
pub struct TraceNode {
    pub function: String,
    pub path: PathBuf,
    pub line_number: usize,
    pub is_hierarchical: bool,
    pub depth: usize,
    pub children: Vec<TraceNode>,  // callees or callers depending on direction
    pub is_cycle: bool,
    pub parent_path: Option<PathBuf>,  // For path abbreviation
    pub call_site: Option<TraceCallSite>,
    pub stop_reason: Option<TraceStopReason>,
    pub ambiguous_matches: usize,
    pub truncated_children: usize,
}

impl TraceNode {
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

/// Result of a trace operation
#[derive(Debug, Clone)]
pub struct TraceResult {
    pub root: TraceNode,
    pub direction: TraceDirection,
    pub stats: TraceStats,
}

impl TraceResult {
    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }
}

/// Statistics for a trace result
#[derive(Debug, Clone)]
pub struct TraceStats {
    pub total_nodes: usize,
    pub direct_callees: usize,
    pub transitive_callees: usize,
    pub max_depth_reached: usize,
    pub cycles_detected: usize,
    pub depth_limited: usize,
    pub unresolved_symbols: usize,
    pub ambiguous_symbols: usize,
    pub width_limited: usize,
}

#[derive(Debug, Clone)]
struct FunctionLocation {
    path: PathBuf,
    line_number: usize,
    is_hierarchical: bool,
    depth: usize,
}

/// Trace searcher - builds call graphs using CallerSearcher and CalleeSearcher
pub struct TraceSearcher {
    caller_searcher: CallerSearcher,
    callee_searcher: CalleeSearcher,
    trace_options: TraceOptions,
    visited: std::collections::HashSet<String>,
}

impl TraceSearcher {
    pub fn new(search_options: SearchOptions, trace_options: TraceOptions) -> Self {
        Self {
            caller_searcher: CallerSearcher::new(search_options.clone()),
            callee_searcher: CalleeSearcher::new(search_options),
            trace_options,
            visited: std::collections::HashSet::new(),
        }
    }

    pub fn trace(
        &mut self,
        function: &str,
        scope: &crate::parser::HierarchyPattern,
        direction: TraceDirection,
        max_depth: usize,
    ) -> anyhow::Result<TraceResult> {
        self.visited.clear();

        let root = self.trace_recursive(function, scope, direction, 0, max_depth, None, None, None)?;

        let stats = self.calculate_stats(&root, max_depth);

        Ok(TraceResult {
            root,
            direction,
            stats,
        })
    }

    fn trace_recursive(
        &mut self,
        function: &str,
        scope: &crate::parser::HierarchyPattern,
        direction: TraceDirection,
        current_depth: usize,
        max_depth: usize,
        parent_path: Option<PathBuf>,
        fallback_location: Option<FunctionLocation>,
        call_site: Option<TraceCallSite>,
    ) -> anyhow::Result<TraceNode> {
        let definitions = self.find_function_definitions(function, scope)?;
        let definition_count = definitions.len();
        let definition_missing = definition_count == 0;
        let mut selected_definition = None;

        if definition_count > 1 && current_depth > 0 {
            let location = fallback_location.clone().or_else(|| definitions.first().cloned());
            let (node_path, node_line, node_is_hierarchical, node_depth) = match location {
                Some(loc) => (loc.path, loc.line_number, loc.is_hierarchical, loc.depth),
                None => (PathBuf::new(), 0, false, 0),
            };

            return Ok(TraceNode {
                function: function.to_string(),
                path: node_path,
                line_number: node_line,
                is_hierarchical: node_is_hierarchical,
                depth: node_depth,
                children: Vec::new(),
                is_cycle: false,
                parent_path,
                call_site,
                stop_reason: Some(TraceStopReason::Ambiguous),
                ambiguous_matches: definition_count,
                truncated_children: 0,
            });
        }

        if definition_count > 0 {
            if current_depth == 0 && definition_count > 1 {
                if let Some(pick) = self.trace_options.pick {
                    if pick == 0 || pick > definition_count {
                        anyhow::bail!("Invalid --pick {}. Choose a number between 1 and {}", pick, definition_count);
                    }
                    selected_definition = Some(definitions[pick - 1].clone());
                } else {
                    let mut message = format!(
                        "Multiple matches found for '{}':\n",
                        function
                    );
                    for (idx, item) in definitions.iter().enumerate() {
                        message.push_str(&format!(
                            "  {}) {} in {}:{}\n",
                            idx + 1,
                            function,
                            item.path.display(),
                            item.line_number
                        ));
                    }
                    message.push_str("Use --pick <N> to select a match.");
                    anyhow::bail!(message);
                }
            } else {
                selected_definition = Some(definitions[0].clone());
            }
        }

        let definition_path = selected_definition.as_ref().map(|loc| loc.path.clone());
        let location = selected_definition.clone().or(fallback_location);

        let (node_path, node_line, node_is_hierarchical, node_depth) = match location {
            Some(loc) => (loc.path, loc.line_number, loc.is_hierarchical, loc.depth),
            None => (PathBuf::new(), 0, false, 0),
        };

        if definition_missing {
            return Ok(TraceNode {
                function: function.to_string(),
                path: node_path,
                line_number: node_line,
                is_hierarchical: node_is_hierarchical,
                depth: node_depth,
                children: Vec::new(),
                is_cycle: false,
                parent_path,
                call_site,
                stop_reason: Some(TraceStopReason::Unresolved),
                ambiguous_matches: 0,
                truncated_children: 0,
            });
        }

        // Check depth limit
        if current_depth >= max_depth {
            // Return leaf node (no children)
            return Ok(TraceNode {
                function: function.to_string(),
                path: node_path,
                line_number: node_line,
                is_hierarchical: node_is_hierarchical,
                depth: node_depth,
                children: Vec::new(),
                is_cycle: false,
                parent_path,
                call_site,
                stop_reason: Some(TraceStopReason::DepthLimit),
                ambiguous_matches: 0,
                truncated_children: 0,
            });
        }

        let node_key = visit_key(function, &node_path, node_line);

        // Check for cycles
        let is_cycle = self.visited.contains(&node_key);
        if is_cycle {
            return Ok(TraceNode {
                function: function.to_string(),
                path: node_path,
                line_number: node_line,
                is_hierarchical: node_is_hierarchical,
                depth: node_depth,
                children: Vec::new(),
                is_cycle: true,
                parent_path,
                call_site,
                stop_reason: None,
                ambiguous_matches: 0,
                truncated_children: 0,
            });
        }

        self.visited.insert(node_key.clone());

        // Find children based on direction
        let mut children_results = match direction {
            TraceDirection::Callees => {
                self.callee_searcher.find_callees(function, scope)?
            }
            TraceDirection::Callers => {
                self.caller_searcher.find_callers(function, scope)?
            }
            TraceDirection::Both => {
                // For "both", we'll handle this specially in output formatting
                // For now, just do callees
                self.callee_searcher.find_callees(function, scope)?
            }
        };

        if matches!(direction, TraceDirection::Callees | TraceDirection::Both) {
            if let Some(ref path) = definition_path {
                children_results.retain(|result| result.path == *path);
            }
        }

        // Limit width
        let total_children = children_results.len();
        let limited_results: Vec<_> = children_results
            .into_iter()
            .take(self.trace_options.max_width)
            .collect();
        let truncated_children = total_children.saturating_sub(limited_results.len());
        let node_stop_reason = if truncated_children > 0 {
            Some(TraceStopReason::WidthLimit)
        } else {
            None
        };

        // Recursively trace children
        let mut child_nodes = Vec::new();
        let mut seen_child_names = std::collections::HashSet::new();

        for child_result in &limited_results {
            let (child_function, child_fallback, child_call_site) = match direction {
                TraceDirection::Callees | TraceDirection::Both => {
                    let name = self.extract_match_name(child_result);
                    let fallback = Some(FunctionLocation {
                        path: child_result.path.clone(),
                        line_number: child_result.line_number,
                        is_hierarchical: child_result.is_hierarchical,
                        depth: child_result.depth,
                    });
                    let call_site = name.as_ref().map(|callee| TraceCallSite {
                        path: child_result.path.clone(),
                        line_number: child_result.line_number,
                        line: child_result.line.clone(),
                        call_kind: detect_call_kind(&child_result.line, callee),
                    });
                    (name, fallback, call_site)
                }
                TraceDirection::Callers => {
                    let caller = self.find_enclosing_caller(&child_result.path, child_result.line_number);
                    let call_site = Some(TraceCallSite {
                        path: child_result.path.clone(),
                        line_number: child_result.line_number,
                        line: child_result.line.clone(),
                        call_kind: detect_call_kind(&child_result.line, function),
                    });
                    match caller {
                        Some((name, location)) => (Some(name), Some(location), call_site),
                        None => (None, None, call_site),
                    }
                }
            };

            let Some(child_name) = child_function else {
                continue;
            };

            if !seen_child_names.insert(child_name.clone()) {
                continue;
            }

            let child_node = self.trace_recursive(
                &child_name,
                scope,
                direction,
                current_depth + 1,
                max_depth,
                Some(node_path.clone()),
                child_fallback,
                child_call_site,
            )?;

            child_nodes.push(child_node);
        }

        self.visited.remove(&node_key);

        Ok(TraceNode {
            function: function.to_string(),
            path: node_path,
            line_number: node_line,
            is_hierarchical: node_is_hierarchical,
            depth: node_depth,
            children: child_nodes,
            is_cycle: false,
            parent_path,
            call_site,
            stop_reason: node_stop_reason,
            ambiguous_matches: 0,
            truncated_children,
        })
    }

    fn calculate_stats(&self, root: &TraceNode, _max_depth: usize) -> TraceStats {
        fn count_nodes(
            node: &TraceNode,
            current_depth: usize,
            stats: &mut (usize, usize, usize, usize, usize, usize, usize),
        ) {
            stats.0 += 1; // total_nodes
            if node.is_cycle {
                stats.1 += 1; // cycles_detected
            }
            stats.2 = stats.2.max(current_depth); // max_depth_reached
            if let Some(reason) = node.stop_reason {
                match reason {
                    TraceStopReason::DepthLimit => stats.3 += 1,
                    TraceStopReason::Unresolved => stats.4 += 1,
                    TraceStopReason::Ambiguous => stats.5 += 1,
                    TraceStopReason::WidthLimit => stats.6 += 1,
                }
            }

            for child in &node.children {
                count_nodes(child, current_depth + 1, stats);
            }
        }

        let mut stats_tuple = (0, 0, 0, 0, 0, 0, 0);
        count_nodes(root, 0, &mut stats_tuple);
        let (
            total_nodes,
            cycles_detected,
            max_depth_reached,
            depth_limited,
            unresolved_symbols,
            ambiguous_symbols,
            width_limited,
        ) = stats_tuple;

        let direct_callees = root.children.len();
        let transitive_callees = total_nodes.saturating_sub(1); // Exclude root

        TraceStats {
            total_nodes,
            direct_callees,
            transitive_callees,
            max_depth_reached,
            cycles_detected,
            depth_limited,
            unresolved_symbols,
            ambiguous_symbols,
            width_limited,
        }
    }
}

impl TraceSearcher {
    fn find_function_definitions(
        &self,
        function: &str,
        scope: &crate::parser::HierarchyPattern,
    ) -> anyhow::Result<Vec<FunctionLocation>> {
        let file_searcher = FileSearcher::new(self.callee_searcher.options.clone());
        let mut files = file_searcher.find(scope);

        // Sort files - hierarchical first, then by depth (deeper first)
        files.sort_by_key(|path| {
            let (is_hierarchical, depth) = file_hierarchy_info(path);
            (!is_hierarchical, std::cmp::Reverse(depth))
        });

        let def_regex = build_definition_regex(function, self.callee_searcher.options.case_insensitive)?;
        let mut matches = Vec::new();

        for file_path in &files {
            if let Ok(file) = fs::File::open(file_path) {
                let reader = BufReader::new(file);
                for (line_num, line) in reader.lines().enumerate() {
                    if let Ok(line) = line {
                        if def_regex.is_match(&line) {
                            let (is_hierarchical, depth) = file_hierarchy_info(&file_path);
                            matches.push(FunctionLocation {
                                path: file_path.clone(),
                                line_number: line_num + 1,
                                is_hierarchical,
                                depth,
                            });
                        }
                    }
                }
            }
        }

        Ok(matches)
    }

    fn extract_match_name(&self, result: &CallerResult) -> Option<String> {
        result
            .line
            .get(result.match_start..result.match_end)
            .map(|s| s.to_string())
            .filter(|s| !s.is_empty())
    }

    fn find_enclosing_caller(
        &self,
        path: &PathBuf,
        line_number: usize,
    ) -> Option<(String, FunctionLocation)> {
        let file = fs::File::open(path).ok()?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
        let def_regex = build_any_definition_regex(self.callee_searcher.options.case_insensitive).ok()?;

        let mut last_match: Option<(String, usize)> = None;
        let max_line = line_number.saturating_sub(1);
        for (idx, line) in lines.iter().enumerate().take(max_line) {
            if let Some(caps) = def_regex.captures(line) {
                if let Some(name) = caps.name("name") {
                    last_match = Some((name.as_str().to_string(), idx + 1));
                }
            }
        }

        if let Some((name, def_line)) = last_match {
            let (is_hierarchical, depth) = file_hierarchy_info(path);
            return Some((
                name,
                FunctionLocation {
                    path: path.clone(),
                    line_number: def_line,
                    is_hierarchical,
                    depth,
                },
            ));
        }

        None
    }
}

fn file_hierarchy_info(path: &PathBuf) -> (bool, usize) {
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let hier_name = filename
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(filename);
    let depth = hier_name.matches('.').count();
    let is_hierarchical = depth > 0;
    (is_hierarchical, depth)
}

fn build_definition_regex(function: &str, case_insensitive: bool) -> anyhow::Result<regex::Regex> {
    let pattern = if case_insensitive {
        format!(
            r"(?i)(public|private|protected|internal|static|async|virtual|override|abstract|sealed|\w+)\s+(\w+\s+)*{}\s*\(",
            regex::escape(function)
        )
    } else {
        format!(
            r"(public|private|protected|internal|static|async|virtual|override|abstract|sealed|\w+)\s+(\w+\s+)*{}\s*\(",
            regex::escape(function)
        )
    };
    Ok(regex::Regex::new(&pattern)?)
}

fn build_any_definition_regex(case_insensitive: bool) -> anyhow::Result<regex::Regex> {
    let pattern = if case_insensitive {
        r"(?i)(public|private|protected|internal|static|async|virtual|override|abstract|sealed|\w+)\s+(\w+\s+)*(?P<name>\w+)\s*\("
    } else {
        r"(public|private|protected|internal|static|async|virtual|override|abstract|sealed|\w+)\s+(\w+\s+)*(?P<name>\w+)\s*\("
    };
    Ok(regex::Regex::new(pattern)?)
}

fn detect_call_kind(line: &str, function: &str) -> TraceCallKind {
    let call_token = format!("{}(", function);
    if let Some(pos) = line.find(&call_token) {
        let prefix = &line[..pos];
        if prefix.contains("new ") {
            return TraceCallKind::Ctor;
        }
        if prefix.contains('.') {
            return TraceCallKind::Method;
        }
        return TraceCallKind::Function;
    }
    TraceCallKind::Unknown
}

fn visit_key(function: &str, path: &PathBuf, line_number: usize) -> String {
    if path.as_os_str().is_empty() {
        function.to_string()
    } else {
        format!("{}:{}:{}", function, path.display(), line_number)
    }
}

// ===========================
// STDIN Helper Utility
// ===========================

use std::io::stdin;

/// Read file paths from stdin (one per line)
///
/// Used for Git integration and Unix pipelines:
/// ```bash
/// git diff --name-only | recur files "**" --stdin
/// ```
pub fn read_paths_from_stdin() -> anyhow::Result<Vec<PathBuf>> {
    let stdin = stdin();
    let mut paths = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.context("Failed to read line from stdin")?;
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            paths.push(PathBuf::from(trimmed));
        }
    }

    Ok(paths)
}
