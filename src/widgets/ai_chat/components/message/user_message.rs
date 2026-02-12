//! User message component for AI Chat widget.
//!
//! This module provides the [`UserMessage`] widget for rendering user messages
//! in the chat interface with support for attachments, timestamps, and queued state.

use std::time::SystemTime;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

/// File attachment associated with a message.
#[derive(Debug, Clone, PartialEq)]
pub struct Attachment {
    /// File path
    pub path: String,
    /// MIME type of the attachment
    pub mime_type: String,
}

impl Attachment {
    /// Create a new attachment.
    pub fn new(path: String, mime_type: String) -> Self {
        Self { path, mime_type }
    }

    /// Get the icon for this attachment based on MIME type.
    pub fn icon(&self) -> &'static str {
        if self.mime_type.starts_with("image/") {
            "🖼"
        } else if self.mime_type == "application/pdf" {
            "📄"
        } else if self.mime_type.starts_with("text/") || self.mime_type == "application/json" {
            "📝"
        } else if self.mime_type == "application/x-directory"
            || self.mime_type.contains("directory")
        {
            "📁"
        } else {
            "📎"
        }
    }

    /// Get a short label for the MIME type.
    pub fn label(&self) -> &'static str {
        if self.mime_type.starts_with("image/") {
            "img"
        } else if self.mime_type == "application/pdf" {
            "pdf"
        } else if self.mime_type.starts_with("text/") {
            "txt"
        } else if self.mime_type == "application/json" {
            "json"
        } else if self.mime_type.contains("directory") {
            "dir"
        } else {
            "file"
        }
    }
}

/// User message widget for displaying user-originated chat messages.
///
/// Renders messages with:
/// - Left border colored by agent
/// - Text content in theme colors
/// - File attachment badges
/// - Optional "QUEUED" badge for pending messages
/// - Optional timestamp
/// - Hover effect when focused
pub struct UserMessage<'a> {
    /// Message content
    content: &'a str,
    /// File attachments
    attachments: &'a [Attachment],
    /// Optional timestamp
    timestamp: Option<SystemTime>,
    /// Whether message is queued/pending
    queued: bool,
    /// Agent-specific border color
    agent_color: ratatui::style::Color,
    /// Whether this message is compacted
    compacted: bool,
    /// Whether the widget has focus (for hover effect)
    focused: bool,
}

impl<'a> UserMessage<'a> {
    /// Create a new user message widget.
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            attachments: &[],
            timestamp: None,
            queued: false,
            agent_color: ratatui::style::Color::Cyan,
            compacted: false,
            focused: false,
        }
    }

    /// Set attachments.
    pub fn attachments(mut self, attachments: &'a [Attachment]) -> Self {
        self.attachments = attachments;
        self
    }

    /// Set timestamp.
    pub fn timestamp(mut self, timestamp: SystemTime) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Set queued state.
    pub fn queued(mut self, queued: bool) -> Self {
        self.queued = queued;
        self
    }

    /// Set agent color.
    pub fn agent_color(mut self, color: ratatui::style::Color) -> Self {
        self.agent_color = color;
        self
    }

    /// Set compacted state.
    pub fn compacted(mut self, compacted: bool) -> Self {
        self.compacted = compacted;
        self
    }

    /// Set focus state.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Format timestamp as HH:MM.
    fn format_timestamp(&self) -> String {
        if let Some(ts) = self.timestamp {
            let duration = ts
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let hours = (duration / 3600) % 24;
            let minutes = (duration / 60) % 60;
            format!("{:02}:{:02}", hours, minutes)
        } else {
            String::new()
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

        // Draw left border with agent color
        for y in area.y..area.y + area.height {
            buf.get_mut(area.x, y)
                .set_style(Style::default().fg(self.agent_color));
        }

        // Determine background style based on focus
        let bg_color = if self.focused {
            colors.background_element
        } else {
            colors.background_panel
        };

        // Fill background
        for x in (area.x + 1)..(area.x + area.width) {
            for y in area.y..area.y + area.height {
                buf.get_mut(x, y).set_bg(bg_color);
            }
        }

        // Render content
        let mut y_offset = 0;
        let max_y = area.y + area.height;

        // Render text content
        let text_style = Style::default().fg(colors.text);
        for line in self.content.lines() {
            if area.y + y_offset >= max_y {
                break;
            }
            let span = Span::styled(line, text_style);
            buf.set_span(area.x + 2, area.y + y_offset, &span, content_area.width - 2);
            y_offset += 1;
        }

        // Render attachment badges if present
        if !self.attachments.is_empty() {
            if area.y + y_offset < max_y {
                let badge_parts: Vec<Span> = self
                    .attachments
                    .iter()
                    .map(|att| {
                        let icon = att.icon();
                        let label = att.label();
                        Span::styled(
                            format!(" {} {} ", icon, label),
                            Style::default()
                                .fg(colors.text)
                                .bg(colors.background_element)
                                .add_modifier(ratatui::style::Modifier::DIM),
                        )
                    })
                    .collect();

                // Render badges on the same line or wrap
                let mut x_pos = area.x + 2;
                for span in badge_parts {
                    let span_width = span.content.chars().count();
                    if x_pos + span_width as u16 > area.x + content_area.width {
                        y_offset += 1;
                        x_pos = area.x + 2;
                        if area.y + y_offset >= max_y {
                            break;
                        }
                    }
                    buf.set_span(x_pos, area.y + y_offset, &span, span_width as u16);
                    x_pos += span_width as u16 + 1;
                }
                y_offset += 1;
            }
        }

        // Render QUEUED badge
        if self.queued {
            if area.y + y_offset < max_y {
                let queued_badge = Span::styled(
                    " QUEUED ",
                    Style::default()
                        .fg(ratatui::style::Color::Black)
                        .bg(colors.warning)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                );
                buf.set_span(area.x + 2, area.y + y_offset, &queued_badge, 7);
                y_offset += 1;
            }
        }

        // Render timestamp if present
        if let Some(_) = self.timestamp {
            let ts = self.format_timestamp();
            if !ts.is_empty() && area.y + y_offset < max_y {
                let ts_span = Span::styled(ts.as_str(), Style::default().fg(colors.text_muted));
                buf.set_span(area.x + 2, area.y + y_offset, &ts_span, ts.len() as u16);
            }
        }
    }
}

impl<'a> Widget for UserMessage<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let colors = ChatColors::default();
        self.render_widget(area, buf, &colors);
    }
}

/// Builder for UserMessage with custom colors.
pub struct UserMessageRenderer<'a> {
    content: &'a str,
    attachments: &'a [Attachment],
    timestamp: Option<SystemTime>,
    queued: bool,
    agent_color: ratatui::style::Color,
    compacted: bool,
    focused: bool,
    colors: ChatColors,
}

impl<'a> UserMessageRenderer<'a> {
    /// Create a new renderer.
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            attachments: &[],
            timestamp: None,
            queued: false,
            agent_color: ratatui::style::Color::Cyan,
            compacted: false,
            focused: false,
            colors: ChatColors::default(),
        }
    }

    /// Set attachments.
    pub fn attachments(mut self, attachments: &'a [Attachment]) -> Self {
        self.attachments = attachments;
        self
    }

    /// Set timestamp.
    pub fn timestamp(mut self, timestamp: SystemTime) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Set queued state.
    pub fn queued(mut self, queued: bool) -> Self {
        self.queued = queued;
        self
    }

    /// Set agent color.
    pub fn agent_color(mut self, color: ratatui::style::Color) -> Self {
        self.agent_color = color;
        self
    }

    /// Set compacted state.
    pub fn compacted(mut self, compacted: bool) -> Self {
        self.compacted = compacted;
        self
    }

    /// Set focus state.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set custom colors.
    pub fn colors(mut self, colors: ChatColors) -> Self {
        self.colors = colors;
        self
    }

    /// Render the message.
    pub fn render(self, area: Rect, buf: &mut Buffer) {
        let widget = UserMessage {
            content: self.content,
            attachments: self.attachments,
            timestamp: self.timestamp,
            queued: self.queued,
            agent_color: self.agent_color,
            compacted: self.compacted,
            focused: self.focused,
        };
        widget.render_widget(area, buf, &self.colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attachment_icon() {
        let img = Attachment::new("test.png".to_string(), "image/png".to_string());
        assert_eq!(img.icon(), "🖼");

        let pdf = Attachment::new("doc.pdf".to_string(), "application/pdf".to_string());
        assert_eq!(pdf.icon(), "📄");

        let txt = Attachment::new("readme.txt".to_string(), "text/plain".to_string());
        assert_eq!(txt.icon(), "📝");

        let dir = Attachment::new("folder".to_string(), "application/x-directory".to_string());
        assert_eq!(dir.icon(), "📁");
    }

    #[test]
    fn test_attachment_label() {
        let img = Attachment::new("test.png".to_string(), "image/png".to_string());
        assert_eq!(img.label(), "img");

        let pdf = Attachment::new("doc.pdf".to_string(), "application/pdf".to_string());
        assert_eq!(pdf.label(), "pdf");

        let txt = Attachment::new("readme.txt".to_string(), "text/plain".to_string());
        assert_eq!(txt.label(), "txt");

        let json = Attachment::new("data.json".to_string(), "application/json".to_string());
        assert_eq!(json.label(), "json");

        let dir = Attachment::new("folder".to_string(), "application/x-directory".to_string());
        assert_eq!(dir.label(), "dir");
    }

    #[test]
    fn test_user_message_builder() {
        let msg = UserMessage::new("Hello, world!");
        assert_eq!(msg.content, "Hello, world!");
        assert!(msg.attachments.is_empty());
        assert!(!msg.queued);
        assert!(!msg.compacted);
    }

    #[test]
    fn test_user_message_with_options() {
        let attachments = vec![Attachment::new(
            "test.png".to_string(),
            "image/png".to_string(),
        )];
        let timestamp = SystemTime::now();

        let msg = UserMessage::new("Test message")
            .attachments(&attachments)
            .timestamp(timestamp)
            .queued(true)
            .agent_color(ratatui::style::Color::Magenta)
            .compacted(false)
            .focused(true);

        assert_eq!(msg.content, "Test message");
        assert_eq!(msg.attachments.len(), 1);
        assert!(msg.queued);
        assert!(msg.focused);
    }
}
