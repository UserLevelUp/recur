//! Content search capability trait for searching within specific files.
//!
//! This trait provides the core functionality for commands that search file contents
//! (as opposed to just listing files). It allows searching specific file lists,
//! which is useful when combined with stdin input.

use crate::search::{SearchOptions, SearchResult};
use anyhow::Result;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Trait for commands that search file contents.
///
/// Commands implementing this trait can search within specific files for patterns,
/// either as plain text or regex. This is particularly useful when combined with
/// stdin, allowing you to search only in files that match certain criteria.
///
/// # Example
/// ```rust,ignore
/// struct FindCommand;
///
/// impl ContentSearchCapable for FindCommand {}
///
/// // Search specific files from stdin
/// let stdin_paths = read_paths_from_stdin()?;
/// let results = FindCommand::search_files(
///     stdin_paths,
///     "TODO",
///     &options,
/// )?;
/// ```
pub trait ContentSearchCapable {
    /// Search specific files for a text pattern.
    ///
    /// Unlike the ContentSearcher which scans the filesystem, this method searches
    /// a pre-determined list of files. This is useful when you want to search files
    /// that come from stdin or have been pre-filtered.
    ///
    /// # Arguments
    /// * `files` - List of file paths to search within
    /// * `query` - Text pattern to search for
    /// * `options` - Search options (case sensitivity, context lines, etc.)
    ///
    /// # Returns
    /// List of search results with file path, line number, and context
    ///
    /// # Example
    /// ```rust,ignore
    /// let files = vec![PathBuf::from("Module.Feature.cs")];
    /// let options = SearchOptions {
    ///     case_insensitive: true,
    ///     context_lines: 2,
    ///     ..Default::default()
    /// };
    /// let results = MyCommand::search_files(files, "async", &options)?;
    /// ```
    fn search_files(
        files: Vec<PathBuf>,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        for file_path in files {
            if let Ok(file) = fs::File::open(&file_path) {
                let reader = BufReader::new(file);

                // Read all lines into a vector for context extraction
                let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

                for (line_num, line) in all_lines.iter().enumerate() {
                    let search_line = if options.case_insensitive {
                        line.to_lowercase()
                    } else {
                        line.clone()
                    };

                    let search_query = if options.case_insensitive {
                        query.to_lowercase()
                    } else {
                        query.to_string()
                    };

                    if let Some(pos) = search_line.find(&search_query) {
                        let context_lines = options.context_lines;

                        // Extract context before
                        let start_before = line_num.saturating_sub(context_lines);
                        let context_before: Vec<String> =
                            all_lines[start_before..line_num].iter().cloned().collect();

                        // Extract context after
                        let end_after = (line_num + 1 + context_lines).min(all_lines.len());
                        let context_after: Vec<String> =
                            all_lines[line_num + 1..end_after].iter().cloned().collect();

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

        Ok(results)
    }

    /// Search specific files using a regex pattern.
    ///
    /// Similar to `search_files`, but uses a compiled regex for pattern matching.
    ///
    /// # Arguments
    /// * `files` - List of file paths to search within
    /// * `regex` - Compiled regex pattern to search for
    /// * `options` - Search options (context lines, etc.)
    ///
    /// # Returns
    /// List of search results with file path, line number, and context
    ///
    /// # Example
    /// ```rust,ignore
    /// let files = vec![PathBuf::from("Module.Feature.cs")];
    /// let regex = regex::Regex::new(r"async\s+\w+")?;
    /// let results = MyCommand::search_files_regex(files, &regex, &options)?;
    /// ```
    fn search_files_regex(
        files: Vec<PathBuf>,
        regex: &regex::Regex,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        for file_path in files {
            if let Ok(file) = fs::File::open(&file_path) {
                let reader = BufReader::new(file);

                // Read all lines into a vector for context extraction
                let all_lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

                for (line_num, line) in all_lines.iter().enumerate() {
                    if let Some(mat) = regex.find(line) {
                        let context_lines = options.context_lines;

                        // Extract context before
                        let start_before = line_num.saturating_sub(context_lines);
                        let context_before: Vec<String> =
                            all_lines[start_before..line_num].iter().cloned().collect();

                        // Extract context after
                        let end_after = (line_num + 1 + context_lines).min(all_lines.len());
                        let context_after: Vec<String> =
                            all_lines[line_num + 1..end_after].iter().cloned().collect();

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

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    struct TestCommand;
    impl ContentSearchCapable for TestCommand {}

    #[test]
    fn test_search_files_basic() -> Result<()> {
        // Create a temporary file with test content
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "line 1")?;
        writeln!(temp_file, "line 2 with TODO")?;
        writeln!(temp_file, "line 3")?;

        let files = vec![temp_file.path().to_path_buf()];
        let options = SearchOptions {
            case_insensitive: false,
            context_lines: 0,
            ..Default::default()
        };

        let results = TestCommand::search_files(files, "TODO", &options)?;

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line_number, 2);
        assert!(results[0].line.contains("TODO"));

        Ok(())
    }

    #[test]
    fn test_search_files_case_insensitive() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "UPPERCASE TODO")?;
        writeln!(temp_file, "lowercase todo")?;

        let files = vec![temp_file.path().to_path_buf()];
        let options = SearchOptions {
            case_insensitive: true,
            context_lines: 0,
            ..Default::default()
        };

        let results = TestCommand::search_files(files, "todo", &options)?;

        assert_eq!(results.len(), 2);

        Ok(())
    }

    #[test]
    fn test_search_files_with_context() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "line 1")?;
        writeln!(temp_file, "line 2")?;
        writeln!(temp_file, "line 3 MATCH")?;
        writeln!(temp_file, "line 4")?;
        writeln!(temp_file, "line 5")?;

        let files = vec![temp_file.path().to_path_buf()];
        let options = SearchOptions {
            case_insensitive: false,
            context_lines: 1,
            ..Default::default()
        };

        let results = TestCommand::search_files(files, "MATCH", &options)?;

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].context_before.len(), 1);
        assert_eq!(results[0].context_after.len(), 1);
        assert!(results[0].context_before[0].contains("line 2"));
        assert!(results[0].context_after[0].contains("line 4"));

        Ok(())
    }

    #[test]
    fn test_search_files_regex() -> Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "function foo()")?;
        writeln!(temp_file, "function bar()")?;
        writeln!(temp_file, "not a function")?;

        let files = vec![temp_file.path().to_path_buf()];
        let options = SearchOptions {
            case_insensitive: false,
            context_lines: 0,
            ..Default::default()
        };

        let regex = regex::Regex::new(r"function \w+\(\)")?;
        let results = TestCommand::search_files_regex(files, &regex, &options)?;

        assert_eq!(results.len(), 2);

        Ok(())
    }
}
