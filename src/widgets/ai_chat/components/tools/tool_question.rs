//! Question tool (→) display component.
//!
//! This module provides rendering for question/answer tools.

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

/// Question tool display for user queries.
///
/// Renders:
/// - Question being asked
/// - Answer if provided
pub struct ToolQuestion<'a> {
    /// Question being asked
    question: &'a str,
    /// Answer from the user (optional)
    answer: Option<String>,
    /// Status of the operation
    status: ToolStatus,
}

impl<'a> ToolQuestion<'a> {
    /// Create a new ToolQuestion.
    pub fn new(question: &'a str) -> Self {
        Self {
            question,
            answer: None,
            status: ToolStatus::Pending,
        }
    }

    /// Set the answer.
    pub fn answer(mut self, answer: Option<String>) -> Self {
        self.answer = answer;
        if answer.is_some() {
            self.status = ToolStatus::Complete;
        }
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

    /// Render the question tool to a buffer.
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
            ToolStatus::Pending => "Awaiting answer...",
            ToolStatus::Complete => "Answered",
            ToolStatus::Error => "Failed",
            ToolStatus::PermissionPending => "Waiting...",
        };
        let header = format!("{} Question: {}", icon, status_text);
        let header_span = Span::styled(
            header,
            Style::default()
                .fg(colors.warning)
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

        // === Question ===
        if y < max_y {
            let q_span = Span::styled(
                "  Q:",
                Style::default()
                    .fg(colors.primary)
                    .add_modifier(Modifier::BOLD),
            );
            buf.set_span(area.x + 2, y, &q_span, 4);

            // Question text (may wrap)
            let question_text = if self.question.len() > area.width as usize - 8 {
                format!("{}...", &self.question[..area.width as usize - 11])
            } else {
                self.question.to_string()
            };
            buf.set_span(
                area.x + 6,
                y,
                &Span::styled(question_text, Style::default().fg(colors.text)),
                area.width.saturating_sub(8),
            );
            y += 1;
        }

        // === Answer ===
        if let Some(answer) = &self.answer {
            if y < max_y {
                let a_span = Span::styled(
                    "  A:",
                    Style::default()
                        .fg(colors.success)
                        .add_modifier(Modifier::BOLD),
                );
                buf.set_span(area.x + 2, y, &a_span, 4);

                // Answer text (may wrap)
                let answer_text = if answer.len() > area.width as usize - 8 {
                    format!("{}...", &answer[..area.width as usize - 11])
                } else {
                    answer.clone()
                };
                buf.set_span(
                    area.x + 6,
                    y,
                    &Span::styled(answer_text, Style::default().fg(colors.text)),
                    area.width.saturating_sub(8),
                );
                y += 1;
            }
        }

        // === Waiting for answer ===
        if self.answer.is_none() && y < max_y {
            let waiting_span = Span::styled(
                "  ⏳ Please provide an answer...",
                Style::default()
                    .fg(colors.text_muted)
                    .add_modifier(Modifier::ITALIC),
            );
            buf.set_span(area.x + 2, y, &waiting_span, area.width.saturating_sub(3));
            y += 1;
        }
    }
}

impl Widget for ToolQuestion<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_question_basic() {
        let question = ToolQuestion::new("What is your name?");
        assert_eq!(question.question, "What is your name?");
    }

    #[test]
    fn test_tool_question_with_answer() {
        let question = ToolQuestion::new("What is 2+2?").answer(Some("4".to_string()));

        assert!(question.answer.is_some());
        assert_eq!(question.status, ToolStatus::Complete);
    }
}
