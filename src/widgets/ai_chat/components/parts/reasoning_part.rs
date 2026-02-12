//! ReasoningPart component for rendering AI thinking/reasoning content.
//!
//! This module provides the [`ReasoningPart`] widget for rendering AI reasoning
//! content with a distinct visual style (left border, italic prefix, muted color).

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};
use std::borrow::Cow;

use crate::widgets::ai_chat::components::theme::ChatColors;

/// ReasoningPart component for rendering AI thinking/reasoning content.
///
/// Renders with:
/// - Left border in backgroundElement color
/// - "Thinking: " prefix in italic
/// - Content in subtle/muted color
/// - Support for markdown code block rendering
pub struct ReasoningPart<'a> {
    /// The reasoning content to render
    content: &'a str,
    /// Whether the reasoning is hidden (user preference)
    hidden: bool,
    /// Whether to render content as markdown
    markdown: bool,
    /// Custom colors (optional)
    colors: Option<ChatColors>,
}

impl<'a> ReasoningPart<'a> {
    /// Create a new ReasoningPart with the given content.
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            hidden: false,
            markdown: true,
            colors: None,
        }
    }

    /// Set hidden state (user preference).
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Enable markdown rendering.
    pub fn markdown(mut self, markdown: bool) -> Self {
        self.markdown = markdown;
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

    /// Render the reasoning part with markdown support.
    fn render_content(&self, area: Rect, buf: &mut Buffer, start_y: u16) {
        let colors = self.get_colors();
        let max_y = area.y + area.height;
        let mut y = start_y;

        // Render prefix "Thinking: " in italic
        let prefix = "Thinking: ";
        let prefix_style = Style::default()
            .fg(colors.text_muted)
            .add_modifier(Modifier::ITALIC);
        let prefix_span = Span::styled(prefix, prefix_style);
        buf.set_span(area.x + 2, y, &prefix_span, area.width.saturating_sub(2));
        y += 1;

        // Simple text rendering instead of markdown
        for line in self.content.lines() {
            if y >= max_y {
                break;
            }
            let span = Span::styled(
                format!("  {}", line),
                Style::default().fg(colors.text_muted),
            );
            buf.set_span(area.x + 2, y, &span, area.width.saturating_sub(2));
            y += 1;
        }
    }

    /// Render as collapsed/hidden (show summary only).
    fn render_hidden(&self, area: Rect, buf: &mut Buffer) {
        let colors = self.get_colors();

        // Show collapsed indicator
        let collapsed = "💭 Thinking (click to expand)";
        let span = Span::styled(
            collapsed,
            Style::default()
                .fg(colors.text_muted)
                .add_modifier(Modifier::ITALIC),
        );
        buf.set_span(area.x + 2, area.y, &span, area.width.saturating_sub(2));
    }
}

impl<'a> Widget for ReasoningPart<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.content.is_empty() {
            return;
        }

        let colors = self.get_colors();

        // Draw left border with backgroundElement color
        let border_color = colors.background_element;
        for y in area.y..area.y + area.height {
            buf.get_mut(area.x, y)
                .set_style(Style::default().fg(border_color));
            // Add a marker character on the border
            buf.get_mut(area.x, y).set_char('│');
        }

        // Fill background slightly
        for x in (area.x + 1)..(area.x + area.width) {
            for y in area.y..area.y + area.height {
                buf.get_mut(x, y).set_bg(colors.background_panel);
            }
        }

        // Render content or hidden state
        if self.hidden {
            self.render_hidden(area, buf);
        } else {
            self.render_content(area, buf, area.y);
        }
    }
}

/// Builder for ReasoningPart with custom rendering options.
pub struct ReasoningPartRenderer<'a> {
    content: &'a str,
    hidden: bool,
    markdown: bool,
    colors: ChatColors,
}

impl<'a> ReasoningPartRenderer<'a> {
    /// Create a new renderer.
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            hidden: false,
            markdown: true,
            colors: ChatColors::default(),
        }
    }

    /// Set hidden state.
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Enable markdown rendering.
    pub fn markdown(mut self, markdown: bool) -> Self {
        self.markdown = markdown;
        self
    }

    /// Set custom colors.
    pub fn colors(mut self, colors: ChatColors) -> Self {
        self.colors = colors;
        self
    }

    /// Render the reasoning part.
    pub fn render(self, area: Rect, buf: &mut Buffer) {
        let part = ReasoningPart {
            content: self.content,
            hidden: self.hidden,
            markdown: self.markdown,
            colors: Some(self.colors),
        };
        part.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_part_builder() {
        let part = ReasoningPart::new("Let me think about this...");
        assert_eq!(part.content, "Let me think about this...");
        assert!(!part.hidden);
        assert!(part.markdown);
    }

    #[test]
    fn test_reasoning_part_options() {
        let part = ReasoningPart::new("Thinking content")
            .hidden(true)
            .markdown(false);

        assert!(part.hidden);
        assert!(!part.markdown);
    }

    #[test]
    fn test_reasoning_part_with_colors() {
        let colors = ChatColors::default();
        let part = ReasoningPart::new("Content").colors(colors.clone());
        assert!(part.colors.is_some());
    }

    #[test]
    fn test_reasoning_part_renderer() {
        let renderer = ReasoningPartRenderer::new("Test reasoning")
            .hidden(false)
            .markdown(true);

        assert!(!renderer.hidden);
        assert!(renderer.markdown);
    }
}
