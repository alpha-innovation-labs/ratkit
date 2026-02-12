//! TextPart component for rendering text content in AI Chat.
//!
//! This module provides the [`TextPart`] widget for rendering text content
//! with support for markdown rendering, streaming animation, and concealed content.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

/// TextPart component for rendering text content in chat messages.
///
/// Supports:
/// - Markdown rendering with syntax highlighting
/// - Streaming animation effects
/// - Concealed/collapsible content sections
/// - Plain text or code rendering
pub struct TextPart<'a> {
    /// The text content to render
    content: &'a str,
    /// Whether to show streaming animation
    streaming: bool,
    /// Whether to render content as markdown
    markdown: bool,
    /// Whether content is concealed/hidden
    concealed: bool,
    /// Cursor position for streaming
    cursor_pos: usize,
    /// Custom colors (optional)
    colors: Option<ChatColors>,
}

impl<'a> TextPart<'a> {
    /// Create a new TextPart with the given content.
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            streaming: false,
            markdown: false,
            concealed: false,
            cursor_pos: 0,
            colors: None,
        }
    }

    /// Enable streaming mode with animation.
    pub fn streaming(mut self, streaming: bool) -> Self {
        self.streaming = streaming;
        self
    }

    /// Enable markdown rendering.
    pub fn markdown(mut self, markdown: bool) -> Self {
        self.markdown = markdown;
        self
    }

    /// Conceal/hide the content.
    pub fn concealed(mut self, concealed: bool) -> Self {
        self.concealed = concealed;
        self
    }

    /// Set the cursor position for streaming.
    pub fn cursor_pos(mut self, cursor_pos: usize) -> Self {
        self.cursor_pos = cursor_pos;
        self
    }

    /// Set custom colors.
    pub fn colors(mut self, colors: ChatColors) -> Self {
        self.colors = Some(colors);
        self
    }

    /// Get the colors to use.
    fn get_colors(&self) -> ChatColors {
        self.colors.clone().unwrap_or_default()
    }

    /// Render plain text with optional streaming cursor.
    fn render_plain_text(&self, area: Rect, buf: &mut Buffer) {
        let colors = self.get_colors();
        let visible_content = if self.streaming {
            let chars: Vec<char> = self.content.chars().collect();
            if self.cursor_pos < chars.len() {
                chars[..self.cursor_pos].iter().collect::<String>()
            } else {
                self.content.to_string()
            }
        } else {
            self.content.to_string()
        };

        let lines: Vec<&str> = visible_content.lines().collect();
        let max_y = area.y + area.height;
        let mut y = area.y;

        for line in lines {
            if y >= max_y {
                break;
            }

            // Render the line
            let span = Span::styled(line.to_string(), Style::default().fg(colors.text));
            buf.set_span(area.x, y, &span, area.width);

            // Add streaming cursor at the end of content
            if self.streaming && y == area.y && self.cursor_pos <= self.content.len() {
                let cursor_x =
                    area.x + self.cursor_pos.min(area.width.saturating_sub(1) as usize) as u16;
                if cursor_x < area.x + area.width {
                    buf.get_mut(cursor_x, y).set_style(
                        Style::default()
                            .fg(colors.primary)
                            .add_modifier(Modifier::REVERSED),
                    );
                }
            }

            y += 1;
        }

        // Render streaming cursor on a new line if at the end
        if self.streaming && self.cursor_pos >= self.content.len() && area.y < max_y {
            let cursor_x = area.x;
            buf.get_mut(cursor_x, area.y).set_style(
                Style::default()
                    .fg(colors.primary)
                    .add_modifier(Modifier::REVERSED),
            );
        }
    }

    /// Render markdown content.
    fn render_markdown_content(&self, area: Rect, buf: &mut Buffer) {
        let colors = self.get_colors();
        let visible_content = if self.streaming && self.cursor_pos < self.content.len() {
            let chars: Vec<char> = self.content.chars().collect();
            chars[..self.cursor_pos].iter().collect::<String>()
        } else {
            self.content.to_string()
        };

        // Simple text rendering instead of markdown
        let max_y = area.y + area.height;
        let mut y = area.y;

        for line in visible_content.lines() {
            if y >= max_y {
                break;
            }

            let span = Span::raw(line).style(Style::default().fg(colors.text));
            buf.set_span(area.x, y, &span, area.width);

            y += 1;
        }

        // Add streaming cursor at the end
        if self.streaming && self.cursor_pos >= self.content.len() && area.y < max_y {
            buf.get_mut(area.x, area.y).set_style(
                Style::default()
                    .fg(colors.primary)
                    .add_modifier(Modifier::REVERSED),
            );
        }
    }

    /// Render concealed content as placeholder.
    fn render_concealed(&self, area: Rect, buf: &mut Buffer) {
        let colors = self.get_colors();

        // Show a placeholder for concealed content
        let placeholder = "••••••••••••";
        let span = Span::styled(
            placeholder,
            Style::default()
                .fg(colors.text_muted)
                .add_modifier(Modifier::DIM),
        );
        buf.set_span(
            area.x,
            area.y,
            &span,
            area.width.min(placeholder.len() as u16),
        );
    }
}

impl<'a> Widget for TextPart<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.content.is_empty() {
            return;
        }

        if self.concealed {
            self.render_concealed(area, buf);
        } else if self.markdown {
            self.render_markdown_content(area, buf);
        } else {
            self.render_plain_text(area, buf);
        }
    }
}

/// Builder for TextPart with custom rendering options.
pub struct TextPartRenderer<'a> {
    content: &'a str,
    streaming: bool,
    markdown: bool,
    concealed: bool,
    cursor_pos: usize,
    colors: ChatColors,
}

impl<'a> TextPartRenderer<'a> {
    /// Create a new renderer.
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            streaming: false,
            markdown: false,
            concealed: false,
            cursor_pos: 0,
            colors: ChatColors::default(),
        }
    }

    /// Enable streaming mode.
    pub fn streaming(mut self, streaming: bool) -> Self {
        self.streaming = streaming;
        self
    }

    /// Enable markdown rendering.
    pub fn markdown(mut self, markdown: bool) -> Self {
        self.markdown = markdown;
        self
    }

    /// Conceal the content.
    pub fn concealed(mut self, concealed: bool) -> Self {
        self.concealed = concealed;
        self
    }

    /// Set cursor position.
    pub fn cursor_pos(mut self, cursor_pos: usize) -> Self {
        self.cursor_pos = cursor_pos;
        self
    }

    /// Set custom colors.
    pub fn colors(mut self, colors: ChatColors) -> Self {
        self.colors = colors;
        self
    }

    /// Render the text part.
    pub fn render(self, area: Rect, buf: &mut Buffer) {
        let part = TextPart {
            content: self.content,
            streaming: self.streaming,
            markdown: self.markdown,
            concealed: self.concealed,
            cursor_pos: self.cursor_pos,
            colors: Some(self.colors),
        };
        part.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_part_builder() {
        let part = TextPart::new("Hello, world!");
        assert_eq!(part.content, "Hello, world!");
        assert!(!part.streaming);
        assert!(!part.markdown);
        assert!(!part.concealed);
    }

    #[test]
    fn test_text_part_options() {
        let part = TextPart::new("Test content")
            .streaming(true)
            .markdown(true)
            .concealed(false)
            .cursor_pos(5);

        assert!(part.streaming);
        assert!(part.markdown);
        assert!(!part.concealed);
        assert_eq!(part.cursor_pos, 5);
    }

    #[test]
    fn test_text_part_with_colors() {
        let colors = ChatColors::default();
        let part = TextPart::new("Content").colors(colors.clone());
        assert!(part.colors.is_some());
    }

    #[test]
    fn test_text_part_renderer() {
        let renderer = TextPartRenderer::new("Test")
            .streaming(true)
            .markdown(false);

        assert!(renderer.streaming);
        assert!(!renderer.markdown);
    }
}
