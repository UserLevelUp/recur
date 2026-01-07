//! Output formatting (terminal, JSON, etc.).

use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::search::SearchResult;

/// Formats output for terminal display with colors.
pub struct TerminalFormatter {
    stdout: StandardStream,
    color: bool,
}

impl TerminalFormatter {
    pub fn new(color: bool) -> Self {
        let choice = if color {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        };
        Self {
            stdout: StandardStream::stdout(choice),
            color,
        }
    }

    pub fn print_file(&mut self, path: &PathBuf) {
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
        }
        let _ = writeln!(self.stdout, "{}", path.display());
        if self.color {
            let _ = self.stdout.reset();
        }
    }

    pub fn print_file_list(&mut self, paths: &[PathBuf]) {
        for path in paths {
            self.print_file(path);
        }
    }

    pub fn print_search_result(&mut self, result: &SearchResult) {
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
        }
        let _ = write!(self.stdout, "{}:", result.path.display());
        
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
        }
        let _ = write!(self.stdout, "{}:", result.line_number);
        
        if self.color {
            let _ = self.stdout.reset();
        }
        let _ = writeln!(self.stdout, "{}", result.line);
    }

    pub fn print_search_results(&mut self, results: &[SearchResult]) {
        for result in results {
            self.print_search_result(result);
        }
    }
}

/// Formats output as JSON.
pub struct JsonFormatter;

impl JsonFormatter {
    pub fn format_file_list(paths: &[PathBuf]) -> String {
        serde_json::to_string_pretty(paths).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn format_search_results(results: &[SearchResult]) -> String {
        let items: Vec<_> = results
            .iter()
            .map(|r| serde_json::json!({
                "path": r.path.display().to_string(),
                "line_number": r.line_number,
                "line": r.line,
            }))
            .collect();
        serde_json::to_string_pretty(&items).unwrap_or_else(|_| "[]".to_string())
    }
}