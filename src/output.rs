//! Output formatting (terminal, JSON, etc.).

use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::search::{SearchResult, CallerResult};

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
        // Print context before (with line numbers)
        if !result.context_before.is_empty() {
            let start_line = result.line_number - result.context_before.len();
            for (i, line) in result.context_before.iter().enumerate() {
                if self.color {
                    let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
                }
                let _ = write!(self.stdout, "{}:", result.path.display());

                if self.color {
                    let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                }
                let _ = write!(self.stdout, "{}-", start_line + i);

                if self.color {
                    let _ = self.stdout.reset();
                }
                let _ = writeln!(self.stdout, "{}", line);
            }
        }

        // Print the matching line
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

        // Print context after (with line numbers)
        if !result.context_after.is_empty() {
            let start_line = result.line_number + 1;
            for (i, line) in result.context_after.iter().enumerate() {
                if self.color {
                    let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
                }
                let _ = write!(self.stdout, "{}:", result.path.display());

                if self.color {
                    let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                }
                let _ = write!(self.stdout, "{}-", start_line + i);

                if self.color {
                    let _ = self.stdout.reset();
                }
                let _ = writeln!(self.stdout, "{}", line);
            }
        }
    }

    pub fn print_search_results(&mut self, results: &[SearchResult]) {
        for result in results {
            self.print_search_result(result);
        }
    }

    pub fn print_caller_result(&mut self, result: &CallerResult) {
        // Print context before
        if !result.context_before.is_empty() {
            let start_line = result.line_number - result.context_before.len();
            for (i, line) in result.context_before.iter().enumerate() {
                if self.color {
                    let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                }
                let _ = write!(self.stdout, "{}-", start_line + i);
                if self.color {
                    let _ = self.stdout.reset();
                }
                let _ = writeln!(self.stdout, "{}", line);
            }
        }

        // Print file:line with hierarchical marker
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
        }
        let _ = write!(self.stdout, "{}:", result.path.display());
        if self.color {
            let _ = self.stdout.reset();
        }

        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)));
        }
        let _ = write!(self.stdout, "{}:", result.line_number);
        if self.color {
            let _ = self.stdout.reset();
        }

        // Print line with hierarchical marker
        let hierarchy_marker = if result.is_hierarchical {
            format!(" [hierarchical, depth={}]", result.depth)
        } else {
            " [flat]".to_string()
        };

        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
        }
        let _ = write!(self.stdout, "{}", result.line);
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
        }
        let _ = writeln!(self.stdout, "{}", hierarchy_marker);
        if self.color {
            let _ = self.stdout.reset();
        }

        // Print context after
        if !result.context_after.is_empty() {
            let start_line = result.line_number + 1;
            for (i, line) in result.context_after.iter().enumerate() {
                if self.color {
                    let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
                }
                let _ = write!(self.stdout, "{}-", start_line + i);
                if self.color {
                    let _ = self.stdout.reset();
                }
                let _ = writeln!(self.stdout, "{}", line);
            }
        }
    }

    pub fn print_caller_results(&mut self, results: &[CallerResult]) {
        for result in results {
            self.print_caller_result(result);
            let _ = writeln!(self.stdout); // Blank line between results
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
                "context_before": r.context_before,
                "context_after": r.context_after,
            }))
            .collect();
        serde_json::to_string_pretty(&items).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn format_caller_results(results: &[CallerResult]) -> String {
        let items: Vec<_> = results
            .iter()
            .map(|r| serde_json::json!({
                "path": r.path.display().to_string(),
                "line_number": r.line_number,
                "line": r.line,
                "is_hierarchical": r.is_hierarchical,
                "depth": r.depth,
                "context_before": r.context_before,
                "context_after": r.context_after,
            }))
            .collect();
        serde_json::to_string_pretty(&items).unwrap_or_else(|_| "[]".to_string())
    }
}