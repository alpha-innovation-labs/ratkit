//! Inline tool status display component.
//!
//! This module provides a compact inline representation of tool execution status,
//! showing the tool name, status indicator, and optional output.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

/// Status of a tool execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolStatus {
    /// Tool is pending execution
    Pending,
    /// Tool execution completed successfully
    Complete,
    /// Tool execution failed
    Error,
    /// Tool is waiting for user permission
    PermissionPending,
}

/// Inline tool display with compact status representation.
///
/// Shows a single-line representation of a tool with:
/// - Status-colored icon
/// - Tool name
/// - Optional output preview
pub struct InlineTool<'a> {
    /// Tool name to display
    name: &'a str,
    /// Current execution status
    status: ToolStatus,
    /// Optional output from the tool
    output: Option<String>,
    /// Icon character to display
    icon: char,
}

impl<'a> InlineTool<'a> {
    /// Create a new InlineTool.
    pub fn new(name: &'a str, status: ToolStatus) -> Self {
        Self {
            name,
            status: status.clone(),
            output: None,
            icon: Self::default_icon(&status),
        }
    }

    /// Set the output content.
    pub fn output(mut self, output: Option<String>) -> Self {
        self.output = output;
        self
    }

    /// Set a custom icon.
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = icon;
        self
    }

    /// Get the default icon for a status.
    fn default_icon(status: &ToolStatus) -> char {
        match status {
            ToolStatus::Pending => '~',
            ToolStatus::Complete => '✓',
            ToolStatus::Error => '✗',
            ToolStatus::PermissionPending => '⚠',
        }
    }

    /// Get the color for a status.
    fn status_color(&self, colors: &ChatColors) -> Color {
        match self.status {
            ToolStatus::Pending => colors.warning,
            ToolStatus::Complete => colors.success,
            ToolStatus::Error => colors.error,
            ToolStatus::PermissionPending => Color::Rgb(255, 165, 0), // Orange
        }
    }

    /// Get the style for a status.
    fn status_style(&self, colors: &ChatColors) -> Style {
        let color = self.status_color(colors);
        let mut style = Style::default().fg(color);

        if self.status == ToolStatus::Error {
            style = style.add_modifier(Modifier::CROSSED_OUT);
        }

        style
    }

    /// Render the inline tool to a buffer.
    pub fn render(&self, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
        let mut x = area.x;
        let y = area.y;

        // Render status icon
        let icon_span = Span::styled(
            self.icon.to_string(),
            self.status_style(colors).add_modifier(Modifier::BOLD),
        );
        buf.set_span(x, y, &icon_span, 1);
        x += 1;

        // Add space after icon
        buf.get_mut(x, y)
            .set_style(Style::default().fg(colors.text_muted));
        x += 1;

        // Render tool name
        let name_style = match self.status {
            ToolStatus::Error => Style::default()
                .fg(colors.text_muted)
                .add_modifier(Modifier::CROSSED_OUT),
            _ => Style::default().fg(colors.text),
        };

        let name_text = self.name.to_string();
        let name_span = Span::styled(name_text, name_style);
        buf.set_span(x, y, &name_span, area.width.saturating_sub(x - area.x));
        x += name_text.len();

        // Render output if present and complete
        if let Some(output) = &self.output {
            if self.status == ToolStatus::Complete && !output.is_empty() {
                // Add separator
                buf.get_mut(x, y)
                    .set_style(Style::default().fg(colors.text_muted));
                x += 1;

                // Truncate output if needed
                let available_width = area.width.saturating_sub(x - area.x);
                let output_preview = if output.len() > available_width as usize {
                    format!("{}...", &output[..available_width as usize - 3])
                } else {
                    output.clone()
                };

                let output_span =
                    Span::styled(output_preview, Style::default().fg(colors.text_muted));
                buf.set_span(x, y, &output_span, area.width.saturating_sub(x - area.x));
            }
        }

        // Render loading indicator for pending
        if self.status == ToolStatus::Pending {
            let spinner_frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let frame = spinner_frames[(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                / 100) as usize
                % spinner_frames.len()];

            buf.get_mut(area.x, y)
                .set_char(frame)
                .set_style(self.status_style(colors).add_modifier(Modifier::BOLD));
        }
    }
}

impl Widget for InlineTool<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_tool_pending() {
        let tool = InlineTool::new("bash", ToolStatus::Pending);
        assert_eq!(tool.status, ToolStatus::Pending);
    }

    #[test]
    fn test_inline_tool_complete() {
        let tool =
            InlineTool::new("read", ToolStatus::Complete).output(Some("content".to_string()));
        assert_eq!(tool.status, ToolStatus::Complete);
        assert!(tool.output.is_some());
    }

    #[test]
    fn test_inline_tool_error() {
        let tool = InlineTool::new("write", ToolStatus::Error);
        assert_eq!(tool.status, ToolStatus::Error);
    }

    #[test]
    fn test_inline_tool_permission() {
        let tool = InlineTool::new("bash", ToolStatus::PermissionPending);
        assert_eq!(tool.status, ToolStatus::PermissionPending);
    }
}
