//! Output formatting (terminal, JSON, etc.).

use std::io::Write;
use std::path::{Path, PathBuf};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::search::{SearchResult, CallerResult, CalleeResult, TraceResult, TraceNode, TraceDirection};

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

    pub fn print_callee_result(&mut self, result: &CalleeResult) {
        // Reuse caller formatting since CalleeResult is a type alias
        self.print_caller_result(result);
    }

    pub fn print_callee_results(&mut self, results: &[CalleeResult]) {
        for result in results {
            self.print_callee_result(result);
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

    pub fn format_callee_results(results: &[CalleeResult]) -> String {
        // Reuse caller formatting since CalleeResult is a type alias
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

    pub fn format_trace_result(result: &TraceResult) -> String {
        fn node_to_json(node: &TraceNode) -> serde_json::Value {
            serde_json::json!({
                "function": node.function,
                "path": node.path.display().to_string(),
                "line_number": node.line_number,
                "is_hierarchical": node.is_hierarchical,
                "depth": node.depth,
                "is_cycle": node.is_cycle,
                "children": node.children.iter().map(node_to_json).collect::<Vec<_>>(),
            })
        }

        serde_json::json!({
            "root": node_to_json(&result.root),
            "direction": format!("{:?}", result.direction),
            "stats": {
                "total_nodes": result.stats.total_nodes,
                "direct_callees": result.stats.direct_callees,
                "transitive_callees": result.stats.transitive_callees,
                "max_depth_reached": result.stats.max_depth_reached,
                "cycles_detected": result.stats.cycles_detected,
            }
        }).to_string()
    }
}

/// Output format for trace results
#[derive(Debug, Clone, Copy)]
pub enum TraceFormat {
    Tree,   // Tree with box-drawing characters
    Flat,   // Flat indented format
    Graph,  // Graph format (future)
}

impl TerminalFormatter {
    /// Print trace result in specified format
    pub fn print_trace_result(&mut self, result: &TraceResult, format: TraceFormat) -> anyhow::Result<()> {
        match format {
            TraceFormat::Tree => self.print_trace_tree(result),
            TraceFormat::Flat => self.print_trace_flat(result),
            TraceFormat::Graph => {
                // For now, graph format is same as tree
                self.print_trace_tree(result)
            }
        }
    }

    fn print_trace_tree(&mut self, result: &TraceResult) -> anyhow::Result<()> {
        // Print root node
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_bold(true));
        }
        let _ = write!(self.stdout, "{}", result.root.function);
        if self.color {
            let _ = self.stdout.reset();
        }

        let _ = write!(self.stdout, " (");
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
        }
        let _ = write!(self.stdout, "{}:{}", result.root.path.display(), result.root.line_number);
        if self.color {
            let _ = self.stdout.reset();
        }
        let _ = write!(self.stdout, ")");

        // Print hierarchical marker
        let marker = if result.root.is_hierarchical {
            format!(" [h:{}]", result.root.depth)
        } else {
            " [flat]".to_string()
        };
        if self.color {
            let color = if result.root.is_hierarchical {
                Color::Green
            } else {
                Color::Red
            };
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(color)));
        }
        let _ = writeln!(self.stdout, "{}", marker);
        if self.color {
            let _ = self.stdout.reset();
        }

        let _ = writeln!(self.stdout);

        // Print children
        for (i, child) in result.root.children.iter().enumerate() {
            let is_last = i == result.root.children.len() - 1;
            self.print_trace_node(child, "", is_last, 0)?;
        }

        // Print stats
        let _ = writeln!(self.stdout);
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
        }
        let _ = writeln!(
            self.stdout,
            "Summary: {} direct callees, {} transitive callees (depth {})",
            result.stats.direct_callees,
            result.stats.transitive_callees,
            result.stats.max_depth_reached
        );
        if self.color {
            let _ = self.stdout.reset();
        }

        Ok(())
    }

    fn print_trace_node(&mut self, node: &TraceNode, prefix: &str, is_last: bool, _subsection_index: usize) -> anyhow::Result<()> {
        // Print tree lines
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(128, 128, 128))));
        }

        let branch = if is_last { "└─ " } else { "├─ " };
        let _ = write!(self.stdout, "{}{}", prefix, branch);

        if self.color {
            let _ = self.stdout.reset();
        }

        // Print function name
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_bold(true));
        }
        let _ = write!(self.stdout, "{}", node.function);
        if self.color {
            let _ = self.stdout.reset();
        }

        // Print path
        let _ = write!(self.stdout, " (");
        if self.color {
            let color = if node.is_hierarchical {
                Color::Cyan
            } else {
                Color::Magenta
            };
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(color)));
        }

        // Show abbreviated path if parent exists
        let path_str = node.path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let _ = write!(self.stdout, "{}:{}", path_str, node.line_number);

        if self.color {
            let _ = self.stdout.reset();
        }
        let _ = write!(self.stdout, ")");

        // Print hierarchical marker or cycle marker
        if node.is_cycle {
            if self.color {
                let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
            }
            let _ = writeln!(self.stdout, " [cycle detected]");
            if self.color {
                let _ = self.stdout.reset();
            }
            return Ok(());
        } else {
            let marker = if node.is_hierarchical {
                format!(" [h:{}]", node.depth)
            } else {
                " [flat]".to_string()
            };
            if self.color {
                let color = if node.is_hierarchical {
                    Color::Green
                } else {
                    Color::Red
                };
                let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(color)));
            }
            let _ = writeln!(self.stdout, "{}", marker);
            if self.color {
                let _ = self.stdout.reset();
            }
        }

        // Print children recursively
        for (i, child) in node.children.iter().enumerate() {
            let is_last_child = i == node.children.len() - 1;
            let child_prefix = if is_last { "   " } else { "│  " };
            self.print_trace_node(child, &format!("{}{}", prefix, child_prefix), is_last_child, 0)?;
        }

        Ok(())
    }

    fn print_trace_flat(&mut self, result: &TraceResult) -> anyhow::Result<()> {
        // Print root
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_bold(true));
        }
        let _ = writeln!(
            self.stdout,
            "{} ({}:{}) [h:{}]",
            result.root.function,
            result.root.path.display(),
            result.root.line_number,
            result.root.depth
        );
        if self.color {
            let _ = self.stdout.reset();
        }

        // Print children with indentation
        for child in &result.root.children {
            self.print_trace_node_flat(child, 1)?;
        }

        Ok(())
    }

    fn print_trace_node_flat(&mut self, node: &TraceNode, indent_level: usize) -> anyhow::Result<()> {
        let indent = "  ".repeat(indent_level);

        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_bold(true));
        }
        let _ = write!(self.stdout, "{}{}", indent, node.function);
        if self.color {
            let _ = self.stdout.reset();
        }

        let _ = write!(self.stdout, " (");
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
        }
        let _ = write!(self.stdout, "{}:{}", node.path.display(), node.line_number);
        if self.color {
            let _ = self.stdout.reset();
        }
        let _ = write!(self.stdout, ")");

        if node.is_cycle {
            if self.color {
                let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
            }
            let _ = writeln!(self.stdout, " [cycle]");
            if self.color {
                let _ = self.stdout.reset();
            }
            return Ok(());
        }

        let marker = if node.is_hierarchical {
            format!(" [h:{}]", node.depth)
        } else {
            " [flat]".to_string()
        };
        let _ = writeln!(self.stdout, "{}", marker);

        // Print children
        for child in &node.children {
            self.print_trace_node_flat(child, indent_level + 1)?;
        }

        Ok(())
    }
}