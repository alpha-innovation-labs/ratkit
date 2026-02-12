//! Assistant message component for AI Chat widget.
//!
//! This module provides the [`AssistantMessage`] widget for rendering AI/assistant
//! responses in the chat interface with support for text, tool calls, and reasoning.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

/// A part of an assistant message.
#[derive(Debug, Clone, PartialEq)]
pub enum MessagePart {
    /// Plain text content
    Text(String),
    /// Tool call invocation
    Tool(ToolCall),
    /// Reasoning/thinking content
    Reasoning(String),
}

/// Tool call in a message.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolCall {
    /// Tool name
    pub name: String,
    /// Tool arguments (JSON string)
    pub arguments: String,
    /// Optional tool result
    pub result: Option<String>,
    /// Whether the tool is currently executing
    pub executing: bool,
}

impl ToolCall {
    /// Create a new tool call.
    pub fn new(name: String, arguments: String) -> Self {
        Self {
            name,
            arguments,
            result: None,
            executing: false,
        }
    }

    /// Set the result.
    pub fn with_result(mut self, result: String) -> Self {
        self.result = Some(result);
        self
    }

    /// Set executing state.
    pub fn executing(mut self, executing: bool) -> Self {
        self.executing = executing;
        self
    }
}

/// Assistant message widget for displaying AI responses.
///
/// Renders messages with:
/// - Dynamic part rendering (text, tools, reasoning)
/// - Metadata footer showing mode, agent, model, duration
/// - Error state with red left border
pub struct AssistantMessage<'a> {
    /// Message content parts
    parts: &'a [MessagePart],
    /// Agent name
    agent_name: &'a str,
    /// Model identifier
    model_id: &'a str,
    /// Generation duration in milliseconds
    duration_ms: u64,
    /// Whether generation was interrupted
    interrupted: bool,
    /// Whether this is an error message
    error: bool,
    /// Whether the widget has focus
    focused: bool,
    /// Agent-specific border color
    agent_color: Color,
}

impl<'a> AssistantMessage<'a> {
    /// Create a new assistant message widget.
    pub fn new(parts: &'a [MessagePart]) -> Self {
        Self {
            parts,
            agent_name: "assistant",
            model_id: "gpt-4",
            duration_ms: 0,
            interrupted: false,
            error: false,
            focused: false,
            agent_color: Color::Green,
        }
    }

    /// Set agent name.
    pub fn agent_name(mut self, name: &'a str) -> Self {
        self.agent_name = name;
        self
    }

    /// Set model ID.
    pub fn model_id(mut self, model_id: &'a str) -> Self {
        self.model_id = model_id;
        self
    }

    /// Set duration.
    pub fn duration_ms(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Set interrupted state.
    pub fn interrupted(mut self, interrupted: bool) -> Self {
        self.interrupted = interrupted;
        self
    }

    /// Set error state.
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Set focus state.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set agent color.
    pub fn agent_color(mut self, color: Color) -> Self {
        self.agent_color = color;
        self
    }

    /// Format duration as human-readable string.
    fn format_duration(&self) -> String {
        if self.duration_ms < 1000 {
            format!("{}ms", self.duration_ms)
        } else {
            let secs = self.duration_ms / 1000;
            let ms = self.duration_ms % 1000;
            if secs < 60 {
                format!("{}.{:03}s", secs, ms)
            } else {
                let mins = secs / 60;
                let secs = secs % 60;
                format!("{}m {}s", mins, secs)
            }
        }
    }

    /// Render the widget.
    fn render_widget(&self, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
        let border_width = 1;
        let content_area = Rect {
            x: area.x + border_width,
            y: area.y,
            width: area.width.saturating_sub(border_width * 2),
            height: area.height,
        };

        // Determine border and background colors based on state
        let border_color = if self.error {
            colors.error
        } else {
            self.agent_color
        };

        // Draw left border
        for y in area.y..area.y + area.height {
            buf.get_mut(area.x, y)
                .set_style(Style::default().fg(border_color));
        }

        // Fill background
        let bg_color = if self.focused {
            colors.background_element
        } else {
            colors.background_panel
        };
        for x in (area.x + 1)..(area.x + area.width) {
            for y in area.y..area.y + area.height {
                buf.get_mut(x, y).set_bg(bg_color);
            }
        }

        let max_y = area.y + area.height;
        let mut y_offset = 0;

        // Render message parts
        for part in self.parts {
            if area.y + y_offset >= max_y {
                break;
            }

            match part {
                MessagePart::Text(text) => {
                    for line in text.lines() {
                        if area.y + y_offset >= max_y {
                            break;
                        }
                        let span = Span::styled(line, Style::default().fg(colors.text));
                        buf.set_span(area.x + 2, area.y + y_offset, &span, content_area.width - 2);
                        y_offset += 1;
                    }
                }
                MessagePart::Reasoning(reasoning) => {
                    // Render reasoning in italic/dim style
                    if area.y + y_offset < max_y {
                        let label = Span::styled(
                            "🤔 ",
                            Style::default()
                                .fg(colors.text_muted)
                                .add_modifier(Modifier::ITALIC),
                        );
                        buf.set_span(area.x + 2, area.y + y_offset, &label, 3);
                    }
                    y_offset += 1;

                    for line in reasoning.lines() {
                        if area.y + y_offset >= max_y {
                            break;
                        }
                        let span = Span::styled(
                            line,
                            Style::default()
                                .fg(colors.text_muted)
                                .add_modifier(Modifier::ITALIC),
                        );
                        buf.set_span(area.x + 2, area.y + y_offset, &span, content_area.width - 2);
                        y_offset += 1;
                    }
                }
                MessagePart::Tool(tool) => {
                    // Render tool call header
                    let tool_style = if tool.executing {
                        Style::default()
                            .fg(colors.warning)
                            .add_modifier(Modifier::BOLD)
                    } else if tool.result.is_some() {
                        Style::default()
                            .fg(colors.success)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .fg(colors.primary)
                            .add_modifier(Modifier::BOLD)
                    };

                    let status_icon = if tool.executing {
                        "⚙"
                    } else if tool.result.is_some() {
                        "✓"
                    } else {
                        "🔧"
                    };

                    if area.y + y_offset < max_y {
                        let header =
                            Span::styled(format!("{} {} ", status_icon, tool.name), tool_style);
                        buf.set_span(
                            area.x + 2,
                            area.y + y_offset,
                            &header,
                            content_area.width - 2,
                        );
                        y_offset += 1;
                    }

                    // Render tool arguments (indented)
                    if !tool.arguments.is_empty() && area.y + y_offset < max_y {
                        let args_span = Span::styled(
                            format!("  {}", tool.arguments),
                            Style::default()
                                .fg(colors.text_muted)
                                .add_modifier(Modifier::DIM),
                        );
                        buf.set_span(
                            area.x + 2,
                            area.y + y_offset,
                            &args_span,
                            content_area.width - 2,
                        );
                        y_offset += 1;
                    }

                    // Render tool result if present
                    if let Some(result) = &tool.result {
                        if area.y + y_offset < max_y {
                            let result_span = Span::styled(
                                format!("  → {}", result),
                                Style::default().fg(colors.success),
                            );
                            buf.set_span(
                                area.x + 2,
                                area.y + y_offset,
                                &result_span,
                                content_area.width - 2,
                            );
                            y_offset += 1;
                        }
                    }
                }
            }
        }

        // Add spacing before footer
        y_offset += 1;

        // Render metadata footer
        if area.y + y_offset < max_y {
            let footer_parts = vec![
                // Mode icon
                Span::styled("▣ ", Style::default().fg(colors.text_muted)),
                // Agent name
                Span::styled(
                    self.agent_name,
                    Style::default()
                        .fg(colors.text_muted)
                        .add_modifier(Modifier::DIM),
                ),
                // Separator
                Span::styled(" • ", Style::default().fg(colors.text_muted)),
                // Model ID
                Span::styled(
                    self.model_id,
                    Style::default()
                        .fg(colors.text_muted)
                        .add_modifier(Modifier::DIM),
                ),
                // Separator
                Span::styled(" • ", Style::default().fg(colors.text_muted)),
                // Duration
                Span::styled(
                    self.format_duration(),
                    Style::default()
                        .fg(colors.text_muted)
                        .add_modifier(Modifier::DIM),
                ),
            ];

            if self.interrupted {
                // Add interrupted badge
                footer_parts.iter().for_each(|span| {
                    buf.set_span(
                        area.x + 2,
                        area.y + y_offset,
                        span,
                        span.content.chars().count() as u16,
                    );
                });

                let interrupted_span = Span::styled(
                    " [interrupted]",
                    Style::default()
                        .fg(colors.warning)
                        .add_modifier(Modifier::ITALIC),
                );
                let x_offset = footer_parts
                    .iter()
                    .map(|s| s.content.chars().count())
                    .sum::<usize>() as u16;
                buf.set_span(
                    area.x + 2 + x_offset,
                    area.y + y_offset,
                    &interrupted_span,
                    13,
                );
            } else {
                let mut x_pos = area.x + 2;
                for span in footer_parts {
                    let width = span.content.chars().count() as u16;
                    buf.set_span(x_pos, area.y + y_offset, &span, width);
                    x_pos += width;
                }
            }
        }
    }
}

impl<'a> Widget for AssistantMessage<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let colors = ChatColors::default();
        self.render_widget(area, buf, &colors);
    }
}

/// Builder for AssistantMessage with custom colors.
pub struct AssistantMessageRenderer<'a> {
    parts: &'a [MessagePart],
    agent_name: &'a str,
    model_id: &'a str,
    duration_ms: u64,
    interrupted: bool,
    error: bool,
    focused: bool,
    agent_color: Color,
    colors: ChatColors,
}

impl<'a> AssistantMessageRenderer<'a> {
    /// Create a new renderer.
    pub fn new(parts: &'a [MessagePart]) -> Self {
        Self {
            parts,
            agent_name: "assistant",
            model_id: "gpt-4",
            duration_ms: 0,
            interrupted: false,
            error: false,
            focused: false,
            agent_color: Color::Green,
            colors: ChatColors::default(),
        }
    }

    /// Set agent name.
    pub fn agent_name(mut self, name: &'a str) -> Self {
        self.agent_name = name;
        self
    }

    /// Set model ID.
    pub fn model_id(mut self, model_id: &'a str) -> Self {
        self.model_id = model_id;
        self
    }

    /// Set duration.
    pub fn duration_ms(mut self, duration_ms: u64) -> Self {
        self.duration_ms = duration_ms;
        self
    }

    /// Set interrupted state.
    pub fn interrupted(mut self, interrupted: bool) -> Self {
        self.interrupted = interrupted;
        self
    }

    /// Set error state.
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Set focus state.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set agent color.
    pub fn agent_color(mut self, color: Color) -> Self {
        self.agent_color = color;
        self
    }

    /// Set custom colors.
    pub fn colors(mut self, colors: ChatColors) -> Self {
        self.colors = colors;
        self
    }

    /// Render the message.
    pub fn render(self, area: Rect, buf: &mut Buffer) {
        let widget = AssistantMessage {
            parts: self.parts,
            agent_name: self.agent_name,
            model_id: self.model_id,
            duration_ms: self.duration_ms,
            interrupted: self.interrupted,
            error: self.error,
            focused: self.focused,
            agent_color: self.agent_color,
        };
        widget.render_widget(area, buf, &self.colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_call_builder() {
        let tool = ToolCall::new(
            "filesystem_read".to_string(),
            r#"{"path": "/test.txt"}"#.to_string(),
        );
        assert_eq!(tool.name, "filesystem_read");
        assert!(tool.result.is_none());
        assert!(!tool.executing);
    }

    #[test]
    fn test_tool_call_with_result() {
        let tool = ToolCall::new("echo".to_string(), r#"{"text": "hello"}"#.to_string())
            .with_result("hello".to_string())
            .executing(false);

        assert!(tool.result.is_some());
        assert_eq!(tool.result.as_ref().unwrap(), "hello");
    }

    #[test]
    fn test_assistant_message_builder() {
        let parts = vec![MessagePart::Text("Hello, world!".to_string())];
        let msg = AssistantMessage::new(&parts);

        assert_eq!(msg.parts.len(), 1);
        assert_eq!(msg.agent_name, "assistant");
        assert!(!msg.interrupted);
        assert!(!msg.error);
    }

    #[test]
    fn test_assistant_message_with_options() {
        let parts = vec![
            MessagePart::Text("Hello".to_string()),
            MessagePart::Reasoning("Let me think...".to_string()),
            MessagePart::Tool(ToolCall::new("test".to_string(), "{}".to_string())),
        ];

        let msg = AssistantMessage::new(&parts)
            .agent_name("claude")
            .model_id("claude-3-opus")
            .duration_ms(1500)
            .interrupted(false)
            .error(false)
            .focused(true)
            .agent_color(Color::Magenta);

        assert_eq!(msg.agent_name, "claude");
        assert_eq!(msg.model_id, "claude-3-opus");
        assert_eq!(msg.duration_ms, 1500);
        assert!(msg.focused);
    }

    #[test]
    fn test_duration_formatting() {
        // Test milliseconds
        let msg = AssistantMessage::new(&[]).duration_ms(500);
        assert_eq!(msg.format_duration(), "500ms");

        // Test seconds
        let msg = AssistantMessage::new(&[]).duration_ms(2500);
        assert_eq!(msg.format_duration(), "2.500s");

        // Test minutes
        let msg = AssistantMessage::new(&[]).duration_ms(125000);
        assert_eq!(msg.format_duration(), "2m 5s");
    }
}
