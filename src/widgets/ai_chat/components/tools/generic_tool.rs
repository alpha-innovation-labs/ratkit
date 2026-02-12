//! Generic tool fallback component.
//!
//! This module provides a fallback display for unknown tool types.

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

/// GenericTool display for unknown/fallback tool types.
///
/// Renders:
/// - Tool name
/// - Input parameters
pub struct GenericTool<'a> {
    /// Name of the tool
    name: &'a str,
    /// Input parameters as key-value pairs
    input_params: Vec<(&'a str, &'a str)>,
    /// Raw arguments string
    arguments: Option<String>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether to show all parameters
    expanded: bool,
}

impl<'a> GenericTool<'a> {
    /// Create a new GenericTool.
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            input_params: Vec::new(),
            arguments: None,
            status: ToolStatus::Pending,
            expanded: false,
        }
    }

    /// Add an input parameter.
    pub fn add_param(mut self, key: &'a str, value: &'a str) -> Self {
        self.input_params.push((key, value));
        self
    }

    /// Set input parameters.
    pub fn input_params(mut self, params: Vec<(&'a str, &'a str)>) -> Self {
        self.input_params = params;
        self
    }

    /// Set raw arguments.
    pub fn arguments(mut self, args: Option<String>) -> Self {
        self.arguments = args;
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

    /// Render the generic tool to a buffer.
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
        let icon = '⚙';
        let expand_icon = if self.expanded { "▼" } else { "▶" };
        let header = format!("{} Tool: {} {}", icon, self.name, expand_icon);
        let header_span = Span::styled(
            header,
            Style::default()
                .fg(colors.text_muted)
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
                    "─".repeat(20),
                    Style::default()
                        .fg(border_color)
                        .add_modifier(Modifier::DIM),
                ),
                area.width.saturating_sub(3),
            );
            y += 1;
        }

        // === Parameters ===
        if self.expanded && !self.input_params.is_empty() && y < max_y {
            for (key, value) in &self.input_params {
                if y >= max_y {
                    break;
                }

                let param_span = Span::styled(
                    format!("  {}: {}", key, value),
                    Style::default().fg(colors.text_muted),
                );
                buf.set_span(area.x + 2, y, &param_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

        // === Raw Arguments ===
        if let Some(args) = &self.arguments {
            if y < max_y {
                let expand_text = if self.expanded {
                    "▼ Arguments"
                } else {
                    "▶ Arguments (click to expand)"
                };
                let expand_span = Span::styled(
                    expand_text,
                    Style::default()
                        .fg(colors.text_muted)
                        .add_modifier(Modifier::BOLD),
                );
                buf.set_span(area.x + 2, y, &expand_span, area.width.saturating_sub(3));
                y += 1;

                if self.expanded {
                    let arg_lines: Vec<&str> = args.lines().collect();
                    let max_lines = area.height.saturating_sub(y - area.y) as usize;

                    for line in arg_lines.iter().take(max_lines) {
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
        }

        // === Status ===
        if y < max_y {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Processing...",
                ToolStatus::Complete => "✓ Done",
                ToolStatus::Error => "✗ Failed",
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

impl Widget for GenericTool<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_tool_basic() {
        let tool = GenericTool::new("unknown_tool");
        assert_eq!(tool.name, "unknown_tool");
    }

    #[test]
    fn test_generic_tool_with_params() {
        let tool = GenericTool::new("custom")
            .add_param("arg1", "value1")
            .add_param("arg2", "value2");

        assert_eq!(tool.input_params.len(), 2);
    }

    #[test]
    fn test_generic_tool_status() {
        let pending = GenericTool::new("test").status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);

        let complete = GenericTool::new("test").status(ToolStatus::Complete);
        assert_eq!(complete.status, ToolStatus::Complete);
    }
}
