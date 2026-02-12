//! Write tool (←) display component.
//!
//! This module provides rendering for file writing tools with syntax highlighting
//! and code preview.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

use super::block_tool::BlockTool;
use super::inline_tool::ToolStatus;

/// Diagnostic message from tool execution.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic {
    /// Severity level
    pub severity: DiagnosticSeverity,
    /// Message text
    pub message: String,
    /// Line number (optional)
    pub line: Option<u32>,
    /// Column number (optional)
    pub column: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
}

/// Write tool display for file creation.
///
/// Renders:
/// - File path
/// - Syntax-highlighted code preview
/// - Line numbers
/// - Diagnostic messages
pub struct ToolWrite<'a> {
    /// Path to the file being written
    file_path: &'a str,
    /// Content being written
    content: &'a str,
    /// Diagnostic messages
    diagnostics: Vec<Diagnostic>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether output is expanded
    expanded: bool,
}

impl<'a> ToolWrite<'a> {
    /// Create a new ToolWrite.
    pub fn new(file_path: &'a str, content: &'a str) -> Self {
        Self {
            file_path,
            content,
            diagnostics: Vec::new(),
            status: ToolStatus::Pending,
            expanded: false,
        }
    }

    /// Add a diagnostic message.
    pub fn add_diagnostic(mut self, diagnostic: Diagnostic) -> Self {
        self.diagnostics.push(diagnostic);
        self
    }

    /// Set the status.
    pub fn status(mut self, status: ToolStatus) -> Self {
        self.status = status;
        self
    }

    /// Set expanded state.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Get border color based on status.
    fn border_color(&self, colors: &ChatColors) -> Color {
        match self.status {
            ToolStatus::Pending => colors.warning,
            ToolStatus::Complete => colors.success,
            ToolStatus::Error => colors.error,
            ToolStatus::PermissionPending => Color::Rgb(255, 165, 0),
        }
    }

    /// Render the write tool to a buffer.
    pub fn render(&self, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
        if area.height < 1 {
            return;
        }

        let max_y = area.y + area.height;
        let mut y = area.y;
        let border_color = self.border_color(colors);

        // Draw left border
        for y_pos in area.y..max_y {
            buf.get_mut(area.x, y_pos)
                .set_char('│')
                .set_style(Style::default().fg(border_color));
        }

        // === Header ===
        let header = format!("← Write {}", if self.expanded { "▼" } else { "▶" });
        let header_span = Span::styled(
            header,
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        );
        buf.set_span(area.x + 2, y, &header_span, area.width.saturating_sub(3));
        y += 1;

        // Separator
        if y < max_y {
            buf.set_span(
                area.x + 2,
                y,
                &Span::styled(
                    "─".repeat(30),
                    Style::default()
                        .fg(border_color)
                        .add_modifier(Modifier::DIM),
                ),
                area.width.saturating_sub(3),
            );
            y += 1;
        }

        // === File Path ===
        if y < max_y {
            let path_span = Span::styled(
                format!("📄 {}", self.file_path),
                Style::default()
                    .fg(colors.primary)
                    .add_modifier(Modifier::BOLD),
            );
            buf.set_span(area.x + 2, y, &path_span, area.width.saturating_sub(3));
            y += 1;
        }

        // === Code Preview ===
        let content_lines: Vec<&str> = self.content.lines().collect();
        let max_lines = if self.expanded {
            content_lines
                .len()
                .min(area.height.saturating_sub(y - area.y) as usize)
        } else {
            content_lines.len().min(10)
        };

        // Render code with line numbers
        for (i, line) in content_lines.iter().take(max_lines).enumerate() {
            if y >= max_y {
                break;
            }

            let line_num = format!("{:4} │ ", i + 1);
            let line_num_span = Span::styled(
                line_num,
                Style::default()
                    .fg(colors.text_muted)
                    .add_modifier(Modifier::DIM),
            );
            buf.set_span(area.x + 2, y, &line_num_span, 7);

            // Simple syntax highlighting based on common patterns
            let line_style = self.syntax_style_for_line(line, colors);
            let display_line = if line.len() > area.width as usize - 10 {
                format!("{}...", &line[..area.width as usize - 13])
            } else {
                line.to_string()
            };

            buf.set_span(
                area.x + 9,
                y,
                &Span::styled(display_line, line_style),
                area.width.saturating_sub(10),
            );
            y += 1;
        }

        // Show more indicator
        if !self.expanded && content_lines.len() > 10 && y < max_y {
            buf.set_span(
                area.x + 2,
                y,
                &Span::styled(
                    format!(
                        "... {} more lines (click to expand)",
                        content_lines.len() - 10
                    ),
                    Style::default()
                        .fg(colors.text_muted)
                        .add_modifier(Modifier::ITALIC),
                ),
                area.width.saturating_sub(3),
            );
            y += 1;
        }

        // === Diagnostics ===
        for diag in &self.diagnostics {
            if y >= max_y {
                break;
            }

            let (symbol, diag_color) = match diag.severity {
                DiagnosticSeverity::Error => ("✗", colors.error),
                DiagnosticSeverity::Warning => ("⚠", colors.warning),
                DiagnosticSeverity::Info => ("ℹ", colors.primary),
            };

            let location = if let (Some(line), Some(col)) = (diag.line, diag.column) {
                format!(":{}:{}", line, col)
            } else {
                String::new()
            };

            let diag_text = format!("{} {}{}", symbol, diag.message, location);
            let diag_span = Span::styled(diag_text, Style::default().fg(diag_color));
            buf.set_span(area.x + 2, y, &diag_span, area.width.saturating_sub(3));
            y += 1;
        }

        // === Status ===
        if y < max_y {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Pending...",
                ToolStatus::Complete => "✓ Written successfully",
                ToolStatus::Error => "✗ Write failed",
                ToolStatus::PermissionPending => "⚠ Permission required",
            };
            let status_span = Span::styled(
                status_text,
                Style::default()
                    .fg(border_color)
                    .add_modifier(Modifier::BOLD),
            );
            buf.set_span(area.x + 2, y, &status_span, area.width.saturating_sub(3));
        }
    }

    /// Apply simple syntax highlighting based on line content.
    fn syntax_style_for_line(&self, line: &str, colors: &ChatColors) -> Style {
        let trimmed = line.trim();

        // Comments
        if trimmed.starts_with("//") || trimmed.starts_with("#") || trimmed.starts_with("/*") {
            return Style::default()
                .fg(colors.text_muted)
                .add_modifier(Modifier::ITALIC);
        }

        // Strings
        if trimmed.starts_with('"') || trimmed.starts_with('\'') {
            return Style::default().fg(colors.diff_added);
        }

        // Keywords
        let keywords = [
            "fn", "let", "const", "mut", "pub", "struct", "enum", "impl", "use", "mod", "if",
            "else", "for", "while", "return", "async", "await",
        ];
        for kw in keywords {
            if trimmed.split_whitespace().next() == Some(kw) {
                return Style::default()
                    .fg(colors.secondary)
                    .add_modifier(Modifier::BOLD);
            }
        }

        // Default
        Style::default().fg(colors.text)
    }

    /// Toggle expanded state.
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}

impl Widget for ToolWrite<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_write_basic() {
        let write = ToolWrite::new("src/main.rs", "fn main() {}");
        assert_eq!(write.file_path, "src/main.rs");
    }

    #[test]
    fn test_tool_write_with_diagnostics() {
        let write = ToolWrite::new("src/main.rs", "fn main() {}").add_diagnostic(Diagnostic {
            severity: DiagnosticSeverity::Warning,
            message: "Unused variable".to_string(),
            line: Some(1),
            column: Some(5),
        });

        assert_eq!(write.diagnostics.len(), 1);
    }

    #[test]
    fn test_tool_write_status() {
        let pending = ToolWrite::new("test.rs", "content").status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);

        let complete = ToolWrite::new("test.rs", "content").status(ToolStatus::Complete);
        assert_eq!(complete.status, ToolStatus::Complete);
    }
}
