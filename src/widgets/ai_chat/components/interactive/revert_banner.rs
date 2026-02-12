//! Revert Banner component for AI Chat widget.
//!
//! This module provides the [`RevertBanner`] widget for displaying reverted
//! messages in the chat interface with diff statistics and restore functionality.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

/// Diff statistics for the revert banner.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DiffStats {
    /// Number of lines added
    pub additions: u32,
    /// Number of lines removed
    pub deletions: u32,
}

impl DiffStats {
    /// Create new diff stats.
    pub fn new(additions: u32, deletions: u32) -> Self {
        Self {
            additions,
            deletions,
        }
    }
}

/// Revert Banner widget for showing reverted messages.
///
/// This widget displays:
/// - Number of reverted messages
/// - Diff summary (additions/deletions)
/// - Hover-to-restore interaction
/// - Keybind hint for redo action
pub struct RevertBanner {
    /// Number of reverted messages
    reverted_count: u32,
    /// Diff statistics (additions/deletions)
    diff_stats: DiffStats,
    /// Whether the banner has been interacted with (restored)
    restored: bool,
    /// Hover state
    hovered: bool,
}

impl RevertBanner {
    /// Create a new revert banner.
    pub fn new() -> Self {
        Self {
            reverted_count: 0,
            diff_stats: DiffStats::default(),
            restored: false,
            hovered: false,
        }
    }

    /// Create a banner with reverted message count.
    pub fn with_reverted_count(mut self, count: u32) -> Self {
        self.reverted_count = count;
        self
    }

    /// Create a banner with diff statistics.
    pub fn with_diff_stats(mut self, stats: DiffStats) -> Self {
        self.diff_stats = stats;
        self
    }

    /// Create a banner with specific additions and deletions.
    pub fn with_diff(mut self, additions: u32, deletions: u32) -> Self {
        self.diff_stats = DiffStats::new(additions, deletions);
        self
    }

    /// Set the restored state.
    pub fn restored(mut self, restored: bool) -> Self {
        self.restored = restored;
        self
    }

    /// Set hover state.
    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    /// Check if the given point is within the banner's interactive area.
    pub fn is_hovered(&self, x: u16, y: u16, area: Rect) -> bool {
        x >= area.x && x < area.x + area.width && y >= area.y && y < area.y + area.height
    }

    /// Handle mouse click at the given position.
    /// Returns true if the click was within the banner and triggers restore.
    pub fn handle_click(&mut self, x: u16, y: u16, area: Rect) -> bool {
        if self.is_hovered(x, y, area) && !self.restored {
            self.restored = true;
            true
        } else {
            false
        }
    }

    /// Mark the banner as restored.
    pub fn mark_restored(&mut self) {
        self.restored = true;
    }

    /// Check if the banner has been restored.
    pub fn is_restored(&self) -> bool {
        self.restored
    }

    /// Render the widget.
    fn render_widget(&self, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
        if area.width < 10 || area.height < 1 {
            return;
        }

        // Determine styles based on state
        let bg_color = if self.restored {
            // Restored state - subtle background
            colors.background_panel
        } else if self.hovered {
            // Hovered state - highlight background
            colors.background_element
        } else {
            // Default state
            colors.background_element
        };

        // Fill background
        for x in area.x..(area.x + area.width) {
            buf.get_mut(x, area.y).set_bg(bg_color);
        }

        // Build the content based on state
        let (content, content_style) = if self.restored {
            // Restored indicator with checkmark
            let text = format!(
                " ✓ Restored {} message{}",
                self.reverted_count,
                if self.reverted_count == 1 { "" } else { "s" }
            );
            (
                text,
                Style::default()
                    .fg(colors.success)
                    .add_modifier(Modifier::DIM),
            )
        } else {
            // Revert indicator with diff stats and keybind hint
            let revert_icon = "↩";
            let message_text = format!(
                "{} {} message{} reverted",
                revert_icon,
                self.reverted_count,
                if self.reverted_count == 1 { "" } else { "s" }
            );

            let diff_text = if self.diff_stats.additions > 0 || self.diff_stats.deletions > 0 {
                format!(
                    "(+{} -{})",
                    self.diff_stats.additions, self.diff_stats.deletions
                )
            } else {
                String::new()
            };

            let keybind_hint = if self.hovered {
                " [Ctrl+Z] to redo "
            } else {
                " "
            };

            let full_text = format!("{}{}{}", message_text, diff_text, keybind_hint);

            let style = if self.hovered {
                Style::default()
                    .fg(colors.primary)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(colors.text)
            };

            (full_text, style)
        };

        // Center the content horizontally
        let content_width = content.chars().count() as u16;
        let x_offset = if area.width > content_width {
            (area.width - content_width) / 2
        } else {
            0
        };

        // Render the styled content
        let span = Span::styled(content, content_style);
        buf.set_span(
            area.x + x_offset,
            area.y,
            &span,
            content_width.min(area.width),
        );
    }
}

impl Default for RevertBanner {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for RevertBanner {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let colors = ChatColors::default();
        self.render_widget(area, buf, &colors);
    }
}

/// Builder for RevertBanner with custom colors.
pub struct RevertBannerRenderer {
    reverted_count: u32,
    diff_stats: DiffStats,
    restored: bool,
    hovered: bool,
    colors: ChatColors,
}

impl RevertBannerRenderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self {
            reverted_count: 0,
            diff_stats: DiffStats::default(),
            restored: false,
            hovered: false,
            colors: ChatColors::default(),
        }
    }

    /// Set reverted message count.
    pub fn reverted_count(mut self, count: u32) -> Self {
        self.reverted_count = count;
        self
    }

    /// Set diff statistics.
    pub fn diff_stats(mut self, stats: DiffStats) -> Self {
        self.diff_stats = stats;
        self
    }

    /// Set additions and deletions.
    pub fn diff(mut self, additions: u32, deletions: u32) -> Self {
        self.diff_stats = DiffStats::new(additions, deletions);
        self
    }

    /// Set restored state.
    pub fn restored(mut self, restored: bool) -> Self {
        self.restored = restored;
        self
    }

    /// Set hover state.
    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    /// Set custom colors.
    pub fn colors(mut self, colors: ChatColors) -> Self {
        self.colors = colors;
        self
    }

    /// Render the banner.
    pub fn render(self, area: Rect, buf: &mut Buffer) {
        let banner = RevertBanner {
            reverted_count: self.reverted_count,
            diff_stats: self.diff_stats,
            restored: self.restored,
            hovered: self.hovered,
        };
        banner.render_widget(area, buf, &self.colors);
    }
}

impl Default for RevertBannerRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revert_banner_default() {
        let banner = RevertBanner::new();
        assert_eq!(banner.reverted_count, 0);
        assert_eq!(banner.diff_stats.additions, 0);
        assert_eq!(banner.diff_stats.deletions, 0);
        assert!(!banner.restored);
        assert!(!banner.hovered);
    }

    #[test]
    fn test_revert_banner_with_count() {
        let banner = RevertBanner::new().with_reverted_count(5);
        assert_eq!(banner.reverted_count, 5);
    }

    #[test]
    fn test_revert_banner_with_diff() {
        let banner = RevertBanner::new().with_diff(10, 5);
        assert_eq!(banner.diff_stats.additions, 10);
        assert_eq!(banner.diff_stats.deletions, 5);
    }

    #[test]
    fn test_revert_banner_builder() {
        let banner = RevertBanner::new()
            .with_reverted_count(3)
            .with_diff(8, 2)
            .hovered(true)
            .restored(false);

        assert_eq!(banner.reverted_count, 3);
        assert_eq!(banner.diff_stats.additions, 8);
        assert_eq!(banner.diff_stats.deletions, 2);
        assert!(banner.hovered);
        assert!(!banner.restored);
    }

    #[test]
    fn test_diff_stats() {
        let stats = DiffStats::new(10, 5);
        assert_eq!(stats.additions, 10);
        assert_eq!(stats.deletions, 5);
    }

    #[test]
    fn test_diff_stats_default() {
        let stats = DiffStats::default();
        assert_eq!(stats.additions, 0);
        assert_eq!(stats.deletions, 0);
    }

    #[test]
    fn test_revert_banner_restored_state() {
        let mut banner = RevertBanner::new().with_reverted_count(2);
        assert!(!banner.is_restored());

        banner.mark_restored();
        assert!(banner.is_restored());
    }

    #[test]
    fn test_revert_banner_handle_click() {
        let mut banner = RevertBanner::new().with_reverted_count(1);

        // Click outside area should not restore
        let result = banner.handle_click(5, 5, Rect::new(0, 0, 3, 1));
        assert!(!result);
        assert!(!banner.is_restored());

        // Click inside area should restore
        let result = banner.handle_click(1, 0, Rect::new(0, 0, 10, 1));
        assert!(result);
        assert!(banner.is_restored());

        // Click after restored should not trigger again
        let result = banner.handle_click(1, 0, Rect::new(0, 0, 10, 1));
        assert!(!result);
    }

    #[test]
    fn test_revert_banner_renderer() {
        let renderer = RevertBannerRenderer::new()
            .reverted_count(7)
            .diff(15, 3)
            .hovered(true)
            .restored(false);

        assert_eq!(renderer.reverted_count, 7);
        assert_eq!(renderer.diff_stats.additions, 15);
        assert_eq!(renderer.diff_stats.deletions, 3);
        assert!(renderer.hovered);
        assert!(!renderer.restored);
    }
}
