//! CodeSearch tool (◇) display component.
//!
//! This module provides rendering for code search tools.

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
use super::tool_grep::GrepContext;

/// CodeSearch tool display for code searching.
///
/// Renders:
/// - Search query
/// - Results count
/// - Code match previews
pub struct ToolCodeSearch<'a> {
    /// Search query
    query: &'a str,
    /// Number of results
    results_count: usize,
    /// Match contexts
    contexts: Vec<GrepContext>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether to show all results
    expanded: bool,
}

impl<'a> ToolCodeSearch<'a> {
    /// Create a new ToolCodeSearch.
    pub fn new(query: &'a str) -> Self {
        Self {
            query,
            results_count: 0,
            contexts: Vec::new(),
            status: ToolStatus::Pending,
            expanded: false,
        }
    }

    /// Set results count.
    pub fn results_count(mut self, count: usize) -> Self {
        self.results_count = count;
        self
    }

    /// Add a context.
    pub fn add_context(mut self, context: GrepContext) -> Self {
        self.results_count += 1;
        self.contexts.push(context);
        self
    }

    /// Set contexts.
    pub fn contexts(mut self, contexts: Vec<GrepContext>) -> Self {
        self.contexts = contexts;
        self.results_count = contexts.len();
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

    /// Render the codesearch tool to a buffer.
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
        let icon = '◇';
        let expand_icon = if self.expanded { "▼" } else { "▶" };
        let count_text = if self.status == ToolStatus::Complete {
            format!(" ({} matches)", self.results_count)
        } else {
            String::new()
        };
        let header = format!(
            "{} CodeSearch \"{}\" {}{}",
            icon, self.query, expand_icon, count_text
        );
        let header_span = Span::styled(
            header,
            Style::default()
                .fg(colors.secondary)
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

        // === Results ===
        if y < max_y && !self.contexts.is_empty() {
            let display_count = if self.expanded {
                self.contexts.len()
            } else {
                self.contexts.len().min(5)
            };

            for ctx in self.contexts.iter().take(display_count) {
                if y >= max_y {
                    break;
                }

                // File:line
                let location = format!("{}:{}", ctx.file_path, ctx.line_number);
                let location_span = Span::styled(
                    format!("  💻 {}", location),
                    Style::default()
                        .fg(colors.secondary)
                        .add_modifier(Modifier::BOLD),
                );
                buf.set_span(area.x + 2, y, &location_span, area.width.saturating_sub(3));
                y += 1;

                // Code line
                if y < max_y {
                    let code_span = Span::styled(
                        format!("    {}", ctx.line),
                        Style::default().fg(colors.text_muted),
                    );
                    buf.set_span(area.x + 2, y, &code_span, area.width.saturating_sub(3));
                    y += 1;
                }
            }

            // Show more indicator
            if !self.expanded && self.contexts.len() > 5 && y < max_y {
                buf.set_span(
                    area.x + 2,
                    y,
                    &Span::styled(
                        format!(
                            "  ... and {} more (click to expand)",
                            self.contexts.len() - 5
                        ),
                        Style::default()
                            .fg(colors.text_muted)
                            .add_modifier(Modifier::ITALIC),
                    ),
                    area.width.saturating_sub(3),
                );
                y += 1;
            }
        }

        // === Status ===
        if y < max_y {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Searching code...",
                ToolStatus::Complete => "✓ Search complete",
                ToolStatus::Error => "✗ Search failed",
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
}

impl Widget for ToolCodeSearch<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_codesearch_basic() {
        let search = ToolCodeSearch::new("fn main");
        assert_eq!(search.query, "fn main");
    }

    #[test]
    fn test_tool_codesearch_status() {
        let pending = ToolCodeSearch::new("test").status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);
    }
}
