//! Output formatting (terminal, JSON, etc.).

use crate::search::{
    CalleeResult, CallerResult, SearchResult, TraceCallKind, TraceDirection, TraceNode,
    TraceResult, TraceStopReason,
};
use std::io::{IsTerminal, Write};
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Formats output for terminal display with colors.
pub struct TerminalFormatter {
    stdout: StandardStream,
    color: bool,
}

impl TerminalFormatter {
    pub fn new(color: bool) -> Self {
        // Only enable colors if both requested AND stdout is a terminal
        let is_tty = std::io::stdout().is_terminal();
        let should_color = color && is_tty;

        let choice = if should_color {
            ColorChoice::Always
        } else {
            ColorChoice::Never
        };
        Self {
            stdout: StandardStream::stdout(choice),
            color: should_color,
        }
    }

    pub fn print_file(&mut self, path: &PathBuf) {
        if self.color {
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
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
                    let _ = self
                        .stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
                }
                let _ = write!(self.stdout, "{}:", result.path.display());

                if self.color {
                    let _ = self
                        .stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
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
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
        }
        let _ = write!(self.stdout, "{}:", result.path.display());

        if self.color {
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)));
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
                    let _ = self
                        .stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
                }
                let _ = write!(self.stdout, "{}:", result.path.display());

                if self.color {
                    let _ = self
                        .stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
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
                    let _ = self
                        .stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
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
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
        }
        let _ = write!(self.stdout, "{}:", result.path.display());
        if self.color {
            let _ = self.stdout.reset();
        }

        if self.color {
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Green)));
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
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
        }
        let _ = write!(self.stdout, "{}", result.line);
        if self.color {
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
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
                    let _ = self
                        .stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
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
            .map(|r| {
                serde_json::json!({
                    "path": r.path.display().to_string(),
                    "line_number": r.line_number,
                    "line": r.line,
                    "context_before": r.context_before,
                    "context_after": r.context_after,
                })
            })
            .collect();
        serde_json::to_string_pretty(&items).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn format_caller_results(results: &[CallerResult]) -> String {
        let items: Vec<_> = results
            .iter()
            .map(|r| {
                serde_json::json!({
                    "path": r.path.display().to_string(),
                    "line_number": r.line_number,
                    "line": r.line,
                    "is_hierarchical": r.is_hierarchical,
                    "depth": r.depth,
                    "context_before": r.context_before,
                    "context_after": r.context_after,
                })
            })
            .collect();
        serde_json::to_string_pretty(&items).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn format_callee_results(results: &[CalleeResult]) -> String {
        // Reuse caller formatting since CalleeResult is a type alias
        let items: Vec<_> = results
            .iter()
            .map(|r| {
                serde_json::json!({
                    "path": r.path.display().to_string(),
                    "line_number": r.line_number,
                    "line": r.line,
                    "is_hierarchical": r.is_hierarchical,
                    "depth": r.depth,
                    "context_before": r.context_before,
                    "context_after": r.context_after,
                })
            })
            .collect();
        serde_json::to_string_pretty(&items).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn format_trace_result(result: &TraceResult) -> String {
        fn node_to_json(node: &TraceNode) -> serde_json::Value {
            let call_site = node.call_site.as_ref().map(|site| {
                serde_json::json!({
                    "path": site.path.display().to_string(),
                    "line_number": site.line_number,
                    "line": site.line,
                    "call_kind": format!("{:?}", site.call_kind),
                })
            });

            serde_json::json!({
                "function": node.function,
                "path": node.path.display().to_string(),
                "line_number": node.line_number,
                "is_hierarchical": node.is_hierarchical,
                "depth": node.depth,
                "is_cycle": node.is_cycle,
                "stop_reason": node.stop_reason.map(|r| format!("{:?}", r)),
                "ambiguous_matches": node.ambiguous_matches,
                "truncated_children": node.truncated_children,
                "call_site": call_site,
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
                "depth_limited": result.stats.depth_limited,
                "unresolved_symbols": result.stats.unresolved_symbols,
                "ambiguous_symbols": result.stats.ambiguous_symbols,
                "width_limited": result.stats.width_limited,
            }
        })
        .to_string()
    }

    pub fn format_trace_result_both(callers: &TraceResult, callees: &TraceResult) -> String {
        fn node_to_json(node: &TraceNode) -> serde_json::Value {
            let call_site = node.call_site.as_ref().map(|site| {
                serde_json::json!({
                    "path": site.path.display().to_string(),
                    "line_number": site.line_number,
                    "line": site.line,
                    "call_kind": format!("{:?}", site.call_kind),
                })
            });

            serde_json::json!({
                "function": node.function,
                "path": node.path.display().to_string(),
                "line_number": node.line_number,
                "is_hierarchical": node.is_hierarchical,
                "depth": node.depth,
                "is_cycle": node.is_cycle,
                "stop_reason": node.stop_reason.map(|r| format!("{:?}", r)),
                "ambiguous_matches": node.ambiguous_matches,
                "truncated_children": node.truncated_children,
                "call_site": call_site,
                "children": node.children.iter().map(node_to_json).collect::<Vec<_>>(),
            })
        }

        serde_json::json!({
            "root": node_to_json(&callees.root),
            "callers": callers.root.children.iter().map(node_to_json).collect::<Vec<_>>(),
            "callees": callees.root.children.iter().map(node_to_json).collect::<Vec<_>>(),
            "stats": {
                "callers": {
                    "total_nodes": callers.stats.total_nodes,
                    "direct_callees": callers.stats.direct_callees,
                    "transitive_callees": callers.stats.transitive_callees,
                    "max_depth_reached": callers.stats.max_depth_reached,
                    "cycles_detected": callers.stats.cycles_detected,
                    "depth_limited": callers.stats.depth_limited,
                    "unresolved_symbols": callers.stats.unresolved_symbols,
                    "ambiguous_symbols": callers.stats.ambiguous_symbols,
                    "width_limited": callers.stats.width_limited,
                },
                "callees": {
                    "total_nodes": callees.stats.total_nodes,
                    "direct_callees": callees.stats.direct_callees,
                    "transitive_callees": callees.stats.transitive_callees,
                    "max_depth_reached": callees.stats.max_depth_reached,
                    "cycles_detected": callees.stats.cycles_detected,
                    "depth_limited": callees.stats.depth_limited,
                    "unresolved_symbols": callees.stats.unresolved_symbols,
                    "ambiguous_symbols": callees.stats.ambiguous_symbols,
                    "width_limited": callees.stats.width_limited,
                }
            }
        })
        .to_string()
    }
}

/// Output format for trace results
#[derive(Debug, Clone, Copy)]
pub enum TraceFormat {
    Tree,  // Tree with box-drawing characters
    Flat,  // Flat indented format
    Graph, // Graph format (future)
}

impl TerminalFormatter {
    /// Print trace result in specified format
    pub fn print_trace_result(
        &mut self,
        result: &TraceResult,
        format: TraceFormat,
        verbose: bool,
    ) -> anyhow::Result<()> {
        match format {
            TraceFormat::Tree => self.print_trace_tree(result, verbose),
            TraceFormat::Flat => self.print_trace_flat(result, verbose),
            TraceFormat::Graph => {
                // For now, graph format is same as tree
                self.print_trace_tree(result, verbose)
            }
        }
    }

    pub fn print_trace_both(
        &mut self,
        callers: &TraceResult,
        callees: &TraceResult,
        verbose: bool,
    ) -> anyhow::Result<()> {
        // Callers section
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_bold(true));
        }
        let _ = writeln!(
            self.stdout,
            "Callers (who calls {}):",
            callees.root.function
        );
        if self.color {
            let _ = self.stdout.reset();
        }

        for (i, child) in callers.root.children.iter().enumerate() {
            let is_last = i == callers.root.children.len() - 1;
            self.print_trace_node(child, "", is_last, verbose)?;
        }

        let _ = writeln!(self.stdout);

        // Root line
        self.print_trace_root_line(&callees.root, verbose)?;
        let _ = writeln!(self.stdout);

        // Callees section
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_bold(true));
        }
        let _ = writeln!(
            self.stdout,
            "Callees (what {} calls):",
            callees.root.function
        );
        if self.color {
            let _ = self.stdout.reset();
        }

        for (i, child) in callees.root.children.iter().enumerate() {
            let is_last = i == callees.root.children.len() - 1;
            self.print_trace_node(child, "", is_last, verbose)?;
        }

        let _ = writeln!(self.stdout);
        if self.color {
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
        }
        let _ = writeln!(
            self.stdout,
            "Summary: {} callers, {} callees",
            callers.stats.direct_callees, callees.stats.direct_callees
        );
        if self.color {
            let _ = self.stdout.reset();
        }

        Ok(())
    }

    fn print_trace_tree(&mut self, result: &TraceResult, verbose: bool) -> anyhow::Result<()> {
        self.print_trace_root_line(&result.root, verbose)?;

        let _ = writeln!(self.stdout);

        // Print children
        for (i, child) in result.root.children.iter().enumerate() {
            let is_last = i == result.root.children.len() - 1;
            self.print_trace_node(child, "", is_last, verbose)?;
        }

        // Print stats
        let _ = writeln!(self.stdout);
        if self.color {
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)));
        }
        let (direct_label, transitive_label) = match result.direction {
            TraceDirection::Callers => ("callers", "callers"),
            _ => ("callees", "callees"),
        };
        let _ = writeln!(
            self.stdout,
            "Summary: {} direct {}, {} transitive {} (depth {})",
            result.stats.direct_callees,
            direct_label,
            result.stats.transitive_callees,
            transitive_label,
            result.stats.max_depth_reached
        );
        if result.stats.depth_limited > 0
            || result.stats.unresolved_symbols > 0
            || result.stats.cycles_detected > 0
            || result.stats.ambiguous_symbols > 0
            || result.stats.width_limited > 0
        {
            let _ = writeln!(
                self.stdout,
                "Stops: depth limit={}, unresolved={}, cycles={}, ambiguous={}, max width={}",
                result.stats.depth_limited,
                result.stats.unresolved_symbols,
                result.stats.cycles_detected,
                result.stats.ambiguous_symbols,
                result.stats.width_limited
            );
        }
        if self.color {
            let _ = self.stdout.reset();
        }

        Ok(())
    }

    fn print_trace_root_line(&mut self, node: &TraceNode, verbose: bool) -> anyhow::Result<()> {
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_bold(true));
        }
        let _ = write!(self.stdout, "{}", node.function);
        if self.color {
            let _ = self.stdout.reset();
        }

        let _ = write!(self.stdout, " (");
        if self.color {
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
        }
        let path_label = trace_display_path(node, verbose);
        let _ = write!(self.stdout, "{}:{}", path_label, node.line_number);
        if self.color {
            let _ = self.stdout.reset();
        }
        let _ = write!(self.stdout, ")");

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
        let _ = write!(self.stdout, "{}", marker);
        if let Some(stop_reason) = node.stop_reason {
            let label = format_stop_reason(node, stop_reason);
            let _ = write!(self.stdout, " [{}]", label);
        }
        let _ = writeln!(self.stdout);
        if self.color {
            let _ = self.stdout.reset();
        }

        Ok(())
    }

    fn print_trace_node(
        &mut self,
        node: &TraceNode,
        prefix: &str,
        is_last: bool,
        verbose: bool,
    ) -> anyhow::Result<()> {
        // Print tree lines
        if self.color {
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Rgb(128, 128, 128))));
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

        let path_label = trace_display_path(node, verbose);
        let _ = write!(self.stdout, "{}:{}", path_label, node.line_number);

        if self.color {
            let _ = self.stdout.reset();
        }
        let _ = write!(self.stdout, ")");

        // Print hierarchical marker or cycle marker
        if node.is_cycle {
            if self.color {
                let _ = self
                    .stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)));
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
            let _ = write!(self.stdout, "{}", marker);
            if self.color {
                let _ = self.stdout.reset();
            }
            if let Some(stop_reason) = node.stop_reason {
                let label = format_stop_reason(node, stop_reason);
                let _ = write!(self.stdout, " [{}]", label);
            }
            if let Some(call_site) = &node.call_site {
                let snippet = truncate_snippet(call_site.line.trim(), 120);
                let _ = write!(
                    self.stdout,
                    " [call:{}] {}",
                    call_kind_label(call_site.call_kind),
                    snippet
                );
            }
            let _ = writeln!(self.stdout);
        }

        // Print children recursively
        for (i, child) in node.children.iter().enumerate() {
            let is_last_child = i == node.children.len() - 1;
            let child_prefix = if is_last { "   " } else { "│  " };
            self.print_trace_node(
                child,
                &format!("{}{}", prefix, child_prefix),
                is_last_child,
                verbose,
            )?;
        }

        Ok(())
    }

    fn print_trace_flat(&mut self, result: &TraceResult, verbose: bool) -> anyhow::Result<()> {
        self.print_trace_node_flat(&result.root, 0, verbose)
    }

    fn print_trace_node_flat(
        &mut self,
        node: &TraceNode,
        indent_level: usize,
        verbose: bool,
    ) -> anyhow::Result<()> {
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
            let _ = self
                .stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)));
        }
        let path_label = trace_display_path(node, verbose);
        let _ = write!(self.stdout, "{}:{}", path_label, node.line_number);
        if self.color {
            let _ = self.stdout.reset();
        }
        let _ = write!(self.stdout, ")");

        if node.is_cycle {
            if self.color {
                let _ = self
                    .stdout
                    .set_color(ColorSpec::new().set_fg(Some(Color::Red)));
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
        let _ = write!(self.stdout, "{}", marker);
        if let Some(stop_reason) = node.stop_reason {
            let label = format_stop_reason(node, stop_reason);
            let _ = write!(self.stdout, " [{}]", label);
        }
        if let Some(call_site) = &node.call_site {
            let snippet = truncate_snippet(call_site.line.trim(), 120);
            let _ = write!(
                self.stdout,
                " [call:{}] {}",
                call_kind_label(call_site.call_kind),
                snippet
            );
        }
        let _ = writeln!(self.stdout);

        // Print children
        for child in &node.children {
            self.print_trace_node_flat(child, indent_level + 1, verbose)?;
        }

        Ok(())
    }
}

fn trace_display_path(node: &TraceNode, verbose: bool) -> String {
    if verbose {
        node.path.display().to_string()
    } else {
        node.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string()
    }
}

fn call_kind_label(kind: TraceCallKind) -> &'static str {
    match kind {
        TraceCallKind::Ctor => "ctor",
        TraceCallKind::Method => "method",
        TraceCallKind::Function => "function",
        TraceCallKind::Unknown => "unknown",
    }
}

fn stop_reason_label(reason: TraceStopReason) -> &'static str {
    match reason {
        TraceStopReason::DepthLimit => "depth limit",
        TraceStopReason::Unresolved => "unresolved",
        TraceStopReason::Ambiguous => "ambiguous",
        TraceStopReason::WidthLimit => "max width",
    }
}

fn format_stop_reason(node: &TraceNode, reason: TraceStopReason) -> String {
    let label = stop_reason_label(reason);
    match reason {
        TraceStopReason::Ambiguous if node.ambiguous_matches > 0 => {
            format!("{}: {} matches", label, node.ambiguous_matches)
        }
        TraceStopReason::WidthLimit if node.truncated_children > 0 => {
            format!("{}: {} omitted", label, node.truncated_children)
        }
        _ => label.to_string(),
    }
}

fn truncate_snippet(line: &str, max_len: usize) -> String {
    if line.len() <= max_len {
        return line.to_string();
    }
    let cut = max_len.saturating_sub(3);
    format!("{}...", &line[..cut])
}
