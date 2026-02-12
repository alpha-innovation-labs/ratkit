//! Glob tool (✱) display component.
//!
//! This module provides rendering for file glob pattern matching tools.

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

/// Glob tool display for pattern matching results.
///
/// Renders:
/// - Pattern display
/// - Path being searched
/// - Match count and list of matches
pub struct ToolGlob<'a> {
    /// Glob pattern (e.g., "**/*.rs")
    pattern: &'a str,
    /// Paths where matches were found
    matches: Vec<&'a str>,
    /// Base path for the search
    path: Option<&'a str>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether to show all matches
    expanded: bool,
}

impl<'a> ToolGlob<'a> {
    /// Create a new ToolGlob.
    pub fn new(pattern: &'a str) -> Self {
        Self {
            pattern,
            matches: Vec::new(),
            path: None,
            status: ToolStatus::Pending,
            expanded: false,
        }
    }

    /// Add a match.
    pub fn add_match(mut self, path: &'a str) -> Self {
        self.matches.push(path);
        self
    }

    /// Set matches.
    pub fn matches(mut self, matches: Vec<&'a str>) -> Self {
        self.matches = matches;
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

    /// Render the glob tool to a buffer.
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
        let match_count = self.matches.len();
        let count_text = if self.status == ToolStatus::Complete {
            format!(" ({} matches)", match_count)
        } else {
            String::new()
        };
        let header = format!(
            "{} Glob {} {}{}",
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
                    "─".repeat(25),
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

        // === Matches ===
        if y < max_y {
            let display_count = if self.expanded {
                self.matches.len()
            } else {
                self.matches.len().min(5)
            };

            for (i, m) in self.matches.iter().take(display_count).enumerate() {
                if y >= max_y {
                    break;
                }

                let match_span =
                    Span::styled(format!("  📄 {}", m), Style::default().fg(colors.primary));
                buf.set_span(area.x + 2, y, &match_span, area.width.saturating_sub(3));
                y += 1;
            }

            // Show more indicator
            if !self.expanded && self.matches.len() > 5 && y < max_y {
                buf.set_span(
                    area.x + 2,
                    y,
                    &Span::styled(
                        format!(
                            "  ... and {} more (click to expand)",
                            self.matches.len() - 5
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
        if self.matches.is_empty() && self.status == ToolStatus::Complete && y < max_y {
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

    /// Toggle expanded state.
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}

impl Widget for ToolGlob<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_glob_basic() {
        let glob = ToolGlob::new("**/*.rs");
        assert_eq!(glob.pattern, "**/*.rs");
    }

    #[test]
    fn test_tool_glob_with_matches() {
        let glob = ToolGlob::new("*.rs")
            .add_match("src/main.rs")
            .add_match("src/lib.rs");

        assert_eq!(glob.matches.len(), 2);
    }

    #[test]
    fn test_tool_glob_status() {
        let pending = ToolGlob::new("test.*").status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);

        let complete = ToolGlob::new("test.*").status(ToolStatus::Complete);
        assert_eq!(complete.status, ToolStatus::Complete);
    }
}
