//! Edit tool (←) display component.
//!
//! This module provides rendering for file editing tools with diff view
//! (split vs unified based on width).

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

use super::inline_tool::ToolStatus;
use super::tool_write::{Diagnostic, DiagnosticSeverity};

/// Edit mode for displaying diffs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffMode {
    /// Unified diff view (side by side)
    Unified,
    /// Split diff view (old | new)
    Split,
}

/// Edit tool display for file modifications.
///
/// Renders:
/// - File path
/// - Diff view (unified or split based on width)
/// - Line numbers
/// - Diff highlighting (additions/deletions)
pub struct ToolEdit<'a> {
    /// Path to the file being edited
    file_path: &'a str,
    /// Original content being replaced
    old_content: &'a str,
    /// New content replacing the old
    new_content: &'a str,
    /// Diagnostic messages
    diagnostics: Vec<Diagnostic>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether output is expanded
    expanded: bool,
    /// Current diff mode
    diff_mode: DiffMode,
}

impl<'a> ToolEdit<'a> {
    /// Create a new ToolEdit.
    pub fn new(file_path: &'a str, old_content: &'a str, new_content: &'a str) -> Self {
        Self {
            file_path,
            old_content,
            new_content,
            diagnostics: Vec::new(),
            status: ToolStatus::Pending,
            expanded: false,
            diff_mode: DiffMode::Unified,
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

    /// Set diff mode.
    pub fn diff_mode(mut self, mode: DiffMode) -> Self {
        self.diff_mode = mode;
        self
    }

    /// Determine diff mode based on available width.
    pub fn auto_diff_mode(&mut self, width: u16) {
        self.diff_mode = if width >= 80 {
            DiffMode::Split
        } else {
            DiffMode::Unified
        };
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

    /// Compute simple line-by-line diff.
    fn compute_diff(&self) -> Vec<DiffLine> {
        let old_lines: Vec<&str> = self.old_content.lines().collect();
        let new_lines: Vec<&str> = self.new_content.lines().collect();
        let mut diff = Vec::new();

        // Simple LCS-based diff algorithm
        let old_len = old_lines.len();
        let new_len = new_lines.len();

        // Very simple diff: find matching lines
        let mut old_idx = 0;
        let mut new_idx = 0;

        while old_idx < old_len || new_idx < new_len {
            if old_idx >= old_len {
                // Remaining new lines are additions
                diff.push(DiffLine {
                    line_num_old: None,
                    line_num_new: Some(new_idx + 1),
                    content: new_lines[new_idx].to_string(),
                    diff_type: DiffLineType::Addition,
                });
                new_idx += 1;
            } else if new_idx >= new_len {
                // Remaining old lines are deletions
                diff.push(DiffLine {
                    line_num_old: Some(old_idx + 1),
                    line_num_new: None,
                    content: old_lines[old_idx].to_string(),
                    diff_type: DiffLineType::Deletion,
                });
                old_idx += 1;
            } else if old_lines[old_idx] == new_lines[new_idx] {
                // Matching line
                diff.push(DiffLine {
                    line_num_old: Some(old_idx + 1),
                    line_num_new: Some(new_idx + 1),
                    content: old_lines[old_idx].to_string(),
                    diff_type: DiffLineType::Context,
                });
                old_idx += 1;
                new_idx += 1;
            } else {
                // Check if it's a simple replacement
                diff.push(DiffLine {
                    line_num_old: Some(old_idx + 1),
                    line_num_new: None,
                    content: old_lines[old_idx].to_string(),
                    diff_type: DiffLineType::Deletion,
                });
                diff.push(DiffLine {
                    line_num_old: None,
                    line_num_new: Some(new_idx + 1),
                    content: new_lines[new_idx].to_string(),
                    diff_type: DiffLineType::Addition,
                });
                old_idx += 1;
                new_idx += 1;
            }
        }

        diff
    }

    /// Render the edit tool to a buffer.
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
        let expand_icon = if self.expanded { "▼" } else { "▶" };
        let mode_icon = match self.diff_mode {
            DiffMode::Unified => "⬚",
            DiffMode::Split => "⬚⬚",
        };
        let header = format!("← Edit {} {} [{}]", expand_icon, mode_icon, self.file_path);
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

        // === Diff Content ===
        let diff = self.compute_diff();
        let max_lines = if self.expanded {
            diff.len()
                .min(area.height.saturating_sub(y - area.y) as usize)
        } else {
            diff.len().min(15)
        };

        for (i, line) in diff.iter().take(max_lines).enumerate() {
            if y >= max_y {
                break;
            }

            let (prefix, style) = match line.diff_type {
                DiffLineType::Addition => ("+ ", colors.diff_added_style()),
                DiffLineType::Deletion => ("- ", colors.diff_removed_style()),
                DiffLineType::Context => ("  ", Style::default().fg(colors.text)),
            };

            // Line numbers
            let old_num = line
                .line_num_old
                .map(|n| format!("{:4}", n))
                .unwrap_or_else(|| "    ".to_string());
            let new_num = line
                .line_num_new
                .map(|n| format!("{:4}", n))
                .unwrap_or_else(|| "    ".to_string());

            let line_num_span = Span::styled(
                format!("{}{}│ ", old_num, new_num),
                Style::default()
                    .fg(colors.text_muted)
                    .add_modifier(Modifier::DIM),
            );
            buf.set_span(area.x + 2, y, &line_num_span, 11);

            // Content
            let display_line = if line.content.len() > area.width as usize - 14 {
                format!("{}...", &line.content[..area.width as usize - 17])
            } else {
                line.content.clone()
            };

            buf.set_span(
                area.x + 13,
                y,
                &Span::styled(format!("{}{}", prefix, display_line), style),
                area.width.saturating_sub(14),
            );
            y += 1;
        }

        // Show more indicator
        if !self.expanded && diff.len() > 15 && y < max_y {
            buf.set_span(
                area.x + 2,
                y,
                &Span::styled(
                    format!("... {} more changes (click to expand)", diff.len() - 15),
                    Style::default()
                        .fg(colors.text_muted)
                        .add_modifier(Modifier::ITALIC),
                ),
                area.width.saturating_sub(3),
            );
            y += 1;
        }

        // === Stats ===
        if y < max_y {
            let additions = diff
                .iter()
                .filter(|l| l.diff_type == DiffLineType::Addition)
                .count();
            let deletions = diff
                .iter()
                .filter(|l| l.diff_type == DiffLineType::Deletion)
                .count();
            let stats = format!("{} additions, {} deletions", additions, deletions);
            let stats_span = Span::styled(stats, Style::default().fg(colors.text_muted));
            buf.set_span(area.x + 2, y, &stats_span, area.width.saturating_sub(3));
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

            let diag_text = format!("{} {}", symbol, diag.message);
            let diag_span = Span::styled(diag_text, Style::default().fg(diag_color));
            buf.set_span(area.x + 2, y, &diag_span, area.width.saturating_sub(3));
            y += 1;
        }

        // === Status ===
        if y < max_y {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Pending...",
                ToolStatus::Complete => "✓ Edited successfully",
                ToolStatus::Error => "✗ Edit failed",
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

    /// Toggle expanded state.
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }

    /// Cycle through diff modes.
    pub fn cycle_diff_mode(&mut self) {
        self.diff_mode = match self.diff_mode {
            DiffMode::Unified => DiffMode::Split,
            DiffMode::Split => DiffMode::Unified,
        };
    }
}

/// Represents a single line in a diff.
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// Line number in old file (if applicable)
    line_num_old: Option<u32>,
    /// Line number in new file (if applicable)
    line_num_new: Option<u32>,
    /// Content of the line
    content: String,
    /// Type of diff line
    diff_type: DiffLineType,
}

/// Type of diff line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLineType {
    /// Line was added
    Addition,
    /// Line was deleted
    Deletion,
    /// Context (unchanged) line
    Context,
}

impl Widget for ToolEdit<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_edit_basic() {
        let edit = ToolEdit::new("src/main.rs", "old content", "new content");
        assert_eq!(edit.file_path, "src/main.rs");
    }

    #[test]
    fn test_tool_edit_diff() {
        let edit = ToolEdit::new("test.rs", "line 1\nline 2", "line 1\nmodified");
        let diff = edit.compute_diff();
        assert!(!diff.is_empty());
    }

    #[test]
    fn test_tool_edit_status() {
        let pending = ToolEdit::new("test.rs", "old", "new").status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);
    }
}
