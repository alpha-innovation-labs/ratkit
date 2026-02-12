//! Skill tool (→) display component.
//!
//! This module provides rendering for skill invocation tools.

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

/// Skill tool display for skill invocations.
///
/// Renders:
/// - Skill name being invoked
/// - Status of invocation
pub struct ToolSkill<'a> {
    /// Name of the skill being invoked
    skill_name: &'a str,
    /// Arguments for the skill
    arguments: Option<String>,
    /// Status of the operation
    status: ToolStatus,
}

impl<'a> ToolSkill<'a> {
    /// Create a new ToolSkill.
    pub fn new(skill_name: &'a str) -> Self {
        Self {
            skill_name,
            arguments: None,
            status: ToolStatus::Pending,
        }
    }

    /// Set arguments.
    pub fn arguments(mut self, args: Option<String>) -> Self {
        self.arguments = args;
        self
    }

    /// Set the status.
    pub fn status(mut self, status: ToolStatus) -> Self {
        self.status = status;
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

    /// Render the skill tool to a buffer.
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
        let icon = '→';
        let status_text = match self.status {
            ToolStatus::Pending => "Invoking...",
            ToolStatus::Complete => "Invoked",
            ToolStatus::Error => "Failed",
            ToolStatus::PermissionPending => "Waiting...",
        };
        let header = format!("{} Skill: {} [{}]", icon, self.skill_name, status_text);
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
                    "─".repeat(20),
                    Style::default()
                        .fg(border_color)
                        .add_modifier(Modifier::DIM),
                ),
                area.width.saturating_sub(3),
            );
            y += 1;
        }

        // === Skill Info ===
        if y < max_y {
            let skill_span = Span::styled(
                format!("  ⚙️  {}", self.skill_name),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            );
            buf.set_span(area.x + 2, y, &skill_span, area.width.saturating_sub(3));
            y += 1;
        }

        // === Arguments ===
        if let Some(args) = &self.arguments {
            if y < max_y {
                let args_text = if args.len() > 50 {
                    format!("  Args: {}...", &args[..50])
                } else {
                    format!("  Args: {}", args)
                };
                let args_span = Span::styled(args_text, Style::default().fg(colors.text_muted));
                buf.set_span(area.x + 2, y, &args_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

        // === Status ===
        if y < max_y {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Invoking skill...",
                ToolStatus::Complete => "✓ Skill invoked successfully",
                ToolStatus::Error => "✗ Failed to invoke skill",
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
}

impl Widget for ToolSkill<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_skill_basic() {
        let skill = ToolSkill::new("skill-bash");
        assert_eq!(skill.skill_name, "skill-bash");
    }

    #[test]
    fn test_tool_skill_with_args() {
        let skill = ToolSkill::new("skill-git").arguments(Some("commit -m 'test'".to_string()));

        assert!(skill.arguments.is_some());
    }

    #[test]
    fn test_tool_skill_status() {
        let pending = ToolSkill::new("test").status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);

        let complete = ToolSkill::new("test").status(ToolStatus::Complete);
        assert_eq!(complete.status, ToolStatus::Complete);
    }
}
