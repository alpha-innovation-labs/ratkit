//! Task tool (#) display component.
//!
//! This module provides rendering for task/subagent delegation tools.

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

/// A tool call made by a subagent.
#[derive(Debug, Clone, PartialEq)]
pub struct SubAgentToolCall {
    /// Name of the tool
    pub tool_name: String,
    /// Tool arguments
    pub arguments: String,
}

/// Task tool display for subagent delegation.
///
/// Renders:
/// - Subagent type title
/// - Tool call count
/// - Current operation status
/// - Clickable navigation
/// - Error state
pub struct ToolTask<'a> {
    /// Type of subagent being invoked
    subagent_type: &'a str,
    /// Tool calls made by this subagent
    tool_calls: Vec<SubAgentToolCall>,
    /// Current operation being performed
    current_operation: Option<String>,
    /// Error message if any
    error: Option<String>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether to show details
    expanded: bool,
}

impl<'a> ToolTask<'a> {
    /// Create a new ToolTask.
    pub fn new(subagent_type: &'a str) -> Self {
        Self {
            subagent_type,
            tool_calls: Vec::new(),
            current_operation: None,
            error: None,
            status: ToolStatus::Pending,
            expanded: false,
        }
    }

    /// Add a tool call.
    pub fn add_tool_call(mut self, call: SubAgentToolCall) -> Self {
        self.tool_calls.push(call);
        self
    }

    /// Set tool calls.
    pub fn tool_calls(mut self, calls: Vec<SubAgentToolCall>) -> Self {
        self.tool_calls = calls;
        self
    }

    /// Set current operation.
    pub fn current_operation(mut self, op: Option<String>) -> Self {
        self.current_operation = op;
        self
    }

    /// Set error message.
    pub fn error(mut self, err: Option<String>) -> Self {
        self.error = err;
        if err.is_some() {
            self.status = ToolStatus::Error;
        }
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

    /// Render the task tool to a buffer.
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
        let icon = '#';
        let expand_icon = if self.expanded { "▼" } else { "▶" };
        let tool_count = self.tool_calls.len();
        let status_text = match self.status {
            ToolStatus::Pending => "Working...",
            ToolStatus::Complete => "Complete",
            ToolStatus::Error => "Failed",
            ToolStatus::PermissionPending => "Waiting...",
        };
        let header = format!(
            "{} Task: {} [{}] {}",
            icon, self.subagent_type, tool_count, expand_icon
        );
        let header_span = Span::styled(
            header,
            Style::default()
                .fg(colors.accent)
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

        // === Current Operation ===
        if let Some(op) = &self.current_operation {
            if y < max_y {
                let op_span = Span::styled(
                    format!("  ⚙  {}", op),
                    Style::default()
                        .fg(colors.warning)
                        .add_modifier(Modifier::ITALIC),
                );
                buf.set_span(area.x + 2, y, &op_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

        // === Error ===
        if let Some(err) = &self.error {
            if y < max_y {
                let err_span = Span::styled(
                    format!("  ✗ Error: {}", err),
                    Style::default()
                        .fg(colors.error)
                        .add_modifier(Modifier::BOLD),
                );
                buf.set_span(area.x + 2, y, &err_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

        // === Tool Calls ===
        if self.expanded && !self.tool_calls.is_empty() && y < max_y {
            let calls_header = Span::styled(
                "  Tool calls:",
                Style::default()
                    .fg(colors.text_muted)
                    .add_modifier(Modifier::BOLD),
            );
            buf.set_span(area.x + 2, y, &calls_header, area.width.saturating_sub(3));
            y += 1;

            for call in self
                .tool_calls
                .iter()
                .take(area.height.saturating_sub(y - area.y) as usize)
            {
                if y >= max_y {
                    break;
                }

                let call_span = Span::styled(
                    format!("    → {}: {}", call.tool_name, call.arguments),
                    Style::default().fg(colors.text_muted),
                );
                buf.set_span(area.x + 2, y, &call_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

        // === Status ===
        if y < max_y {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Subagent is working...",
                ToolStatus::Complete => "✓ Task completed",
                ToolStatus::Error => "✗ Task failed",
                ToolStatus::PermissionPending => "⚠ Waiting for approval",
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

    /// Check if a point is clickable.
    pub fn is_clickable(&self, x: u16, y: u16, area: Rect) -> bool {
        // Header is clickable
        y == area.y && x >= area.x + 2
    }
}

impl Widget for ToolTask<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_task_basic() {
        let task = ToolTask::new("explore");
        assert_eq!(task.subagent_type, "explore");
    }

    #[test]
    fn test_tool_task_with_calls() {
        let task = ToolTask::new("code").add_tool_call(SubAgentToolCall {
            tool_name: "read".to_string(),
            arguments: "src/main.rs".to_string(),
        });

        assert_eq!(task.tool_calls.len(), 1);
    }

    #[test]
    fn test_tool_task_error() {
        let task = ToolTask::new("test").error(Some("Connection failed".to_string()));
        assert!(task.error.is_some());
        assert_eq!(task.status, ToolStatus::Error);
    }
}
