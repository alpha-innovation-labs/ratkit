//! WebFetch tool (%) display component.
//!
//! This module provides rendering for web URL fetching tools.

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

/// WebFetch tool display for URL content retrieval.
///
/// Renders:
/// - URL being fetched
/// - Content preview
pub struct ToolWebFetch<'a> {
    /// URL being fetched
    url: &'a str,
    /// Preview of fetched content
    content_preview: Option<String>,
    /// HTTP status code
    status_code: Option<u16>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether output is expanded
    expanded: bool,
}

impl<'a> ToolWebFetch<'a> {
    /// Create a new ToolWebFetch.
    pub fn new(url: &'a str) -> Self {
        Self {
            url,
            content_preview: None,
            status_code: None,
            status: ToolStatus::Pending,
            expanded: false,
        }
    }

    /// Set content preview.
    pub fn content_preview(mut self, preview: Option<String>) -> Self {
        self.content_preview = preview;
        self
    }

    /// Set HTTP status code.
    pub fn status_code(mut self, code: Option<u16>) -> Self {
        self.status_code = code;
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

    /// Render the webfetch tool to a buffer.
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
        let icon = '%';
        let status_text = match self.status {
            ToolStatus::Pending => "Fetching...",
            ToolStatus::Complete => "Fetched",
            ToolStatus::Error => "Failed",
            ToolStatus::PermissionPending => "Permission required",
        };
        let header = format!("{} WebFetch: {}", icon, status_text);
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
                    "─".repeat(25),
                    Style::default()
                        .fg(border_color)
                        .add_modifier(Modifier::DIM),
                ),
                area.width.saturating_sub(3),
            );
            y += 1;
        }

        // === URL ===
        if y < max_y {
            let url_span = Span::styled(
                format!("🌐 {}", self.url),
                Style::default()
                    .fg(colors.primary)
                    .add_modifier(Modifier::UNDERLINED),
            );
            buf.set_span(area.x + 2, y, &url_span, area.width.saturating_sub(3));
            y += 1;
        }

        // === HTTP Status ===
        if let Some(code) = self.status_code {
            if y < max_y {
                let (status_text, status_color) = if code >= 200 && code < 300 {
                    ("OK", colors.success)
                } else if code >= 300 && code < 400 {
                    ("Redirect", colors.warning)
                } else {
                    ("Error", colors.error)
                };
                let status_span = Span::styled(
                    format!("  HTTP {} {}", code, status_text),
                    Style::default().fg(status_color),
                );
                buf.set_span(area.x + 2, y, &status_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

        // === Content Preview ===
        if let Some(preview) = &self.content_preview {
            if y < max_y {
                let expand_text = if self.expanded {
                    "▼ Content"
                } else {
                    "▶ Content (click to expand)"
                };
                let expand_span = Span::styled(
                    expand_text,
                    Style::default()
                        .fg(colors.text_muted)
                        .add_modifier(Modifier::BOLD),
                );
                buf.set_span(area.x + 2, y, &expand_span, area.width.saturating_sub(3));
                y += 1;
            }

            if self.expanded {
                let lines: Vec<&str> = preview.lines().collect();
                let max_lines = area.height.saturating_sub(y - area.y) as usize;

                for line in lines.iter().take(max_lines) {
                    if y >= max_y {
                        break;
                    }

                    let display_line = if line.len() > area.width as usize - 4 {
                        format!("{}...", &line[..area.width as usize - 7])
                    } else {
                        line.to_string()
                    };

                    buf.set_span(
                        area.x + 2,
                        y,
                        &Span::styled(display_line, Style::default().fg(colors.text_muted)),
                        area.width.saturating_sub(3),
                    );
                    y += 1;
                }
            }
        }

        // === Status ===
        if y < max_y {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Fetching content...",
                ToolStatus::Complete => "✓ Content fetched successfully",
                ToolStatus::Error => "✗ Failed to fetch URL",
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

impl Widget for ToolWebFetch<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_webfetch_basic() {
        let fetch = ToolWebFetch::new("https://example.com");
        assert_eq!(fetch.url, "https://example.com");
    }

    #[test]
    fn test_tool_webfetch_with_status() {
        let fetch = ToolWebFetch::new("https://example.com")
            .status_code(Some(200))
            .content_preview(Some("<html>...".to_string()));

        assert_eq!(fetch.status_code, Some(200));
    }
}
