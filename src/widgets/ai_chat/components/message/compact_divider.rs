//! Compact divider component for AI Chat widget.
//!
//! This module provides the [`CompactDivider`] widget for indicating
//! collapsed/compacted message threads in the chat interface.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

/// Compact divider widget for collapsed message sections.
///
/// Renders a visual divider indicating that messages have been compacted.
pub struct CompactDivider {
    /// Text indicator for the divider
    indicator: String,
    /// Number of hidden messages (optional)
    hidden_count: Option<usize>,
    /// Whether the divider is expandable (has hidden messages)
    expandable: bool,
    /// Whether the widget has focus
    focused: bool,
}

impl CompactDivider {
    /// Create a new compact divider.
    pub fn new() -> Self {
        Self {
            indicator: "· · ·".to_string(),
            hidden_count: None,
            expandable: true,
            focused: false,
        }
    }

    /// Create a divider with hidden message count.
    pub fn with_hidden_count(mut self, count: usize) -> Self {
        self.hidden_count = Some(count);
        self
    }

    /// Set whether the divider is expandable.
    pub fn expandable(mut self, expandable: bool) -> Self {
        self.expandable = expandable;
        self
    }

    /// Set focus state.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set custom indicator text.
    pub fn indicator(mut self, indicator: String) -> Self {
        self.indicator = indicator;
        self
    }

    /// Render the widget.
    fn render_widget(&self, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
        if area.width < 4 || area.height < 1 {
            return;
        }

        let border_width = 1;
        let content_area = Rect {
            x: area.x + border_width,
            y: area.y,
            width: area.width.saturating_sub(border_width * 2),
            height: area.height,
        };

        // Draw left border in muted color
        buf.get_mut(area.x, area.y)
            .set_style(Style::default().fg(colors.text_muted));

        // Fill background
        let bg_color = if self.focused {
            colors.background_element
        } else {
            colors.background_panel
        };
        for x in (area.x + 1)..(area.x + area.width) {
            buf.get_mut(x, area.y).set_bg(bg_color);
        }

        // Build the display text
        let display_text = if let Some(count) = self.hidden_count {
            if self.expandable {
                format!(
                    " {} {} {} message{} ",
                    if self.focused { "▶" } else { "▸" },
                    count,
                    if count == 1 { "hidden" } else { "hidden" },
                    if count == 1 { "" } else { "s" }
                )
            } else {
                format!(" {} {} hidden ", self.indicator, count)
            }
        } else if self.expandable {
            format!(
                " {} {} ",
                if self.focused { "▶" } else { "▸" },
                self.indicator
            )
        } else {
            format!(" {} ", self.indicator)
        };

        // Determine style based on state
        let text_style = if self.focused {
            Style::default()
                .fg(colors.primary)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(colors.text_muted)
                .add_modifier(Modifier::DIM)
        };

        // Center the text horizontally
        let text_width = display_text.chars().count() as u16;
        let x_offset = if content_area.width > text_width {
            (content_area.width - text_width) / 2
        } else {
            0
        };

        let span = Span::styled(display_text, text_style);
        buf.set_span(
            content_area.x + x_offset,
            area.y,
            &span,
            text_width.min(content_area.width),
        );
    }
}

impl Default for CompactDivider {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for CompactDivider {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let colors = ChatColors::default();
        self.render_widget(area, buf, &colors);
    }
}

/// Builder for CompactDivider with custom colors.
pub struct CompactDividerRenderer {
    indicator: String,
    hidden_count: Option<usize>,
    expandable: bool,
    focused: bool,
    colors: ChatColors,
}

impl CompactDividerRenderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self {
            indicator: "· · ·".to_string(),
            hidden_count: None,
            expandable: true,
            focused: false,
            colors: ChatColors::default(),
        }
    }

    /// Set hidden message count.
    pub fn hidden_count(mut self, count: usize) -> Self {
        self.hidden_count = Some(count);
        self
    }

    /// Set expandable state.
    pub fn expandable(mut self, expandable: bool) -> Self {
        self.expandable = expandable;
        self
    }

    /// Set focus state.
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set custom indicator.
    pub fn indicator(mut self, indicator: String) -> Self {
        self.indicator = indicator;
        self
    }

    /// Set custom colors.
    pub fn colors(mut self, colors: ChatColors) -> Self {
        self.colors = colors;
        self
    }

    /// Render the divider.
    pub fn render(self, area: Rect, buf: &mut Buffer) {
        let divider = CompactDivider {
            indicator: self.indicator,
            hidden_count: self.hidden_count,
            expandable: self.expandable,
            focused: self.focused,
        };
        divider.render_widget(area, buf, &self.colors);
    }
}

impl Default for CompactDividerRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_divider_default() {
        let divider = CompactDivider::new();
        assert_eq!(divider.indicator, "· · ·");
        assert!(divider.hidden_count.is_none());
        assert!(divider.expandable);
        assert!(!divider.focused);
    }

    #[test]
    fn test_compact_divider_with_count() {
        let divider = CompactDivider::new().with_hidden_count(5);
        assert_eq!(divider.hidden_count, Some(5));
    }

    #[test]
    fn test_compact_divider_builder() {
        let divider = CompactDivider::new()
            .with_hidden_count(3)
            .expandable(true)
            .focused(true)
            .indicator("---".to_string());

        assert_eq!(divider.hidden_count, Some(3));
        assert!(divider.expandable);
        assert!(divider.focused);
        assert_eq!(divider.indicator, "---");
    }

    #[test]
    fn test_compact_divider_not_expandable() {
        let divider = CompactDivider::new().with_hidden_count(2).expandable(false);

        assert!(!divider.expandable);
        assert_eq!(divider.hidden_count, Some(2));
    }

    #[test]
    fn test_compact_divider_renderer() {
        let renderer = CompactDividerRenderer::new()
            .hidden_count(10)
            .expandable(true)
            .focused(true);

        assert!(renderer.expandable);
        assert_eq!(renderer.hidden_count, Some(10));
    }
}
