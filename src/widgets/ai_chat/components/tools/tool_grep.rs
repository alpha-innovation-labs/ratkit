//! Grep tool (✱) display component.
//!
//! This module provides rendering for content search (grep) tools.

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

/// Match context for grep results.
#[derive(Debug, Clone, PartialEq)]
pub struct GrepContext {
    /// File path where match was found
    pub file_path: String,
    /// Line number of match
    pub line_number: u32,
    /// The matching line content
    pub line: String,
}

/// Grep tool display for content searching.
///
/// Renders:
/// - Search pattern
/// - Path being searched
/// - Match count and preview of matches
pub struct ToolGrep<'a> {
    /// Search pattern
    pattern: &'a str,
    /// Number of matches found
    matches: usize,
    /// Match contexts (file, line, content)
    contexts: Vec<GrepContext>,
    /// Path being searched
    path: Option<&'a str>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether to show all matches
    expanded: bool,
}

impl<'a> ToolGrep<'a> {
    /// Create a new ToolGrep.
    pub fn new(pattern: &'a str) -> Self {
        Self {
            pattern,
            matches: 0,
            contexts: Vec::new(),
            path: None,
            status: ToolStatus::Pending,
            expanded: false,
        }
    }

    /// Set match count.
    pub fn matches(mut self, count: usize) -> Self {
        self.matches = count;
        self
    }

    /// Add a match context.
    pub fn add_context(mut self, context: GrepContext) -> Self {
        self.matches += 1;
        self.contexts.push(context);
        self
    }

    /// Set contexts.
    pub fn contexts(mut self, contexts: Vec<GrepContext>) -> Self {
        self.contexts = contexts;
        self.matches = contexts.len();
        self
    }

    /// Set the search path.
    pub fn path(mut self, path: Option<&'a str>) -> Self {
        self.path = path;
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

    /// Render the grep tool to a buffer.
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
        let icon = '✱';
        let expand_icon = if self.expanded { "▼" } else { "▶" };
        let count_text = if self.status == ToolStatus::Complete {
            format!(" ({} matches)", self.matches)
        } else {
            String::new()
        };
        let header = format!(
            "{} Grep \"{}\" {}{}",
            icon, self.pattern, expand_icon, count_text
        );
        let header_span = Span::styled(
            header,
            Style::default()
                .fg(colors.text)
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

        // === Path ===
        if let Some(path) = self.path {
            if y < max_y {
                let path_span = Span::styled(
                    format!("📁 in: {}", path),
                    Style::default().fg(colors.text_muted),
                );
                buf.set_span(area.x + 2, y, &path_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

        // === Match Contexts ===
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

                // File:line indicator
                let location = format!("{}:{}", ctx.file_path, ctx.line_number);
                let location_span = Span::styled(
                    format!("  📄 {}", location),
                    Style::default()
                        .fg(colors.primary)
                        .add_modifier(Modifier::BOLD),
                );
                buf.set_span(area.x + 2, y, &location_span, area.width.saturating_sub(3));
                y += 1;

                // Match line content
                if y < max_y {
                    // Highlight the matching pattern
                    let highlighted = Self::highlight_match(&ctx.line, self.pattern, colors);
                    buf.set_span(area.x + 2, y, &highlighted, area.width.saturating_sub(3));
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

        // === No matches ===
        if self.matches == 0 && self.status == ToolStatus::Complete && y < max_y {
            let no_match_span = Span::styled(
                "  No matches found",
                Style::default()
                    .fg(colors.text_muted)
                    .add_modifier(Modifier::ITALIC),
            );
            buf.set_span(area.x + 2, y, &no_match_span, area.width.saturating_sub(3));
            y += 1;
        }

        // === Status ===
        if y < max_y {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Searching...",
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

    /// Highlight the matching pattern in the line.
    fn highlight_match(line: &str, pattern: &str, colors: &ChatColors) -> Span<'static> {
        // Simple highlighting - just return the line with accent color for the pattern
        // In a real implementation, you'd use regex to find matches
        let styled = line.replace(pattern, &format!("│{}│", pattern));

        let mut spans = Vec::new();
        let parts: Vec<&str> = line.split(pattern).collect();

        for (i, part) in parts.iter().enumerate() {
            if !part.is_empty() {
                spans.push(Span::styled(
                    part.to_string(),
                    Style::default().fg(colors.text_muted),
                ));
            }
            if i < parts.len() - 1 {
                spans.push(Span::styled(
                    pattern.to_string(),
                    Style::default()
                        .fg(colors.accent)
                        .add_modifier(Modifier::BOLD),
                ));
            }
        }

        // Combine spans into a single Span with line breaks not supported in single Span
        // Just return the line with a style
        Span::styled(line.to_string(), Style::default().fg(colors.text_muted))
    }

    /// Toggle expanded state.
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}

impl Widget for ToolGrep<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_grep_basic() {
        let grep = ToolGrep::new("fn main");
        assert_eq!(grep.pattern, "fn main");
    }

    #[test]
    fn test_tool_grep_with_contexts() {
        let grep = ToolGrep::new("test").add_context(GrepContext {
            file_path: "src/main.rs".to_string(),
            line_number: 10,
            line: "fn test() {}".to_string(),
        });

        assert_eq!(grep.matches, 1);
    }

    #[test]
    fn test_tool_grep_status() {
        let pending = ToolGrep::new("test").status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);
    }
}
