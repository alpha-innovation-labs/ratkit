//! WebSearch tool (◈) display component.
//!
//! This module provides rendering for web search tools.

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

/// Search result entry.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    /// Title of the result
    pub title: String,
    /// URL of the result
    pub url: String,
    /// Snippet/description
    pub snippet: String,
}

/// WebSearch tool display for web search queries.
///
/// Renders:
/// - Search query
/// - Results count
/// - List of results
pub struct ToolWebSearch<'a> {
    /// Search query
    query: &'a str,
    /// Number of results
    results_count: usize,
    /// Search results
    results: Vec<SearchResult>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether to show all results
    expanded: bool,
}

impl<'a> ToolWebSearch<'a> {
    /// Create a new ToolWebSearch.
    pub fn new(query: &'a str) -> Self {
        Self {
            query,
            results_count: 0,
            results: Vec::new(),
            status: ToolStatus::Pending,
            expanded: false,
        }
    }

    /// Set results count.
    pub fn results_count(mut self, count: usize) -> Self {
        self.results_count = count;
        self
    }

    /// Add a search result.
    pub fn add_result(mut self, result: SearchResult) -> Self {
        self.results_count += 1;
        self.results.push(result);
        self
    }

    /// Set results.
    pub fn results(mut self, results: Vec<SearchResult>) -> Self {
        self.results = results;
        self.results_count = results.len();
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

    /// Render the websearch tool to a buffer.
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
        let icon = '◈';
        let expand_icon = if self.expanded { "▼" } else { "▶" };
        let count_text = if self.status == ToolStatus::Complete {
            format!(" ({} results)", self.results_count)
        } else {
            String::new()
        };
        let header = format!(
            "{} WebSearch \"{}\" {}{}",
            icon, self.query, expand_icon, count_text
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

        // === Results ===
        if y < max_y && !self.results.is_empty() {
            let display_count = if self.expanded {
                self.results.len()
            } else {
                self.results.len().min(5)
            };

            for result in self.results.iter().take(display_count) {
                if y >= max_y {
                    break;
                }

                // Title
                let title_span = Span::styled(
                    format!("  🔗 {}", result.title),
                    Style::default()
                        .fg(colors.primary)
                        .add_modifier(Modifier::BOLD),
                );
                buf.set_span(area.x + 2, y, &title_span, area.width.saturating_sub(3));
                y += 1;

                // URL (truncated)
                if y < max_y {
                    let url_display = if result.url.len() > 50 {
                        format!("    {}...", &result.url[..50])
                    } else {
                        result.url.clone()
                    };
                    let url_span = Span::styled(
                        url_display,
                        Style::default()
                            .fg(colors.text_muted)
                            .add_modifier(Modifier::ITALIC),
                    );
                    buf.set_span(area.x + 2, y, &url_span, area.width.saturating_sub(3));
                    y += 1;
                }

                // Snippet
                if y < max_y && !result.snippet.is_empty() {
                    let snippet_display = if result.snippet.len() > 60 {
                        format!("    {}...", &result.snippet[..60])
                    } else {
                        result.snippet.clone()
                    };
                    let snippet_span =
                        Span::styled(snippet_display, Style::default().fg(colors.text_muted));
                    buf.set_span(area.x + 2, y, &snippet_span, area.width.saturating_sub(3));
                    y += 1;
                }
            }

            // Show more indicator
            if !self.expanded && self.results.len() > 5 && y < max_y {
                buf.set_span(
                    area.x + 2,
                    y,
                    &Span::styled(
                        format!(
                            "  ... and {} more (click to expand)",
                            self.results.len() - 5
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

        // === No results ===
        if self.results.is_empty() && self.status == ToolStatus::Complete && y < max_y {
            let no_result_span = Span::styled(
                "  No results found",
                Style::default()
                    .fg(colors.text_muted)
                    .add_modifier(Modifier::ITALIC),
            );
            buf.set_span(area.x + 2, y, &no_result_span, area.width.saturating_sub(3));
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

impl Widget for ToolWebSearch<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_websearch_basic() {
        let search = ToolWebSearch::new("rust programming");
        assert_eq!(search.query, "rust programming");
    }

    #[test]
    fn test_tool_websearch_with_results() {
        let search = ToolWebSearch::new("test").add_result(SearchResult {
            title: "Test Result".to_string(),
            url: "https://example.com".to_string(),
            snippet: "A test result".to_string(),
        });

        assert_eq!(search.results_count, 1);
    }
}
