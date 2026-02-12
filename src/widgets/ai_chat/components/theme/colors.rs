//! Chat-specific color definitions for the AI Chat widget.
//!
//! This module provides [`ChatColors`] which contains all the colors needed
//! for rendering chat messages, input areas, status indicators, and diff views
//! within the chat interface.
//!
//! # Color Categories
//!
//! The chat color scheme includes:
//! - **Status colors**: Primary, secondary, accent, success, warning, error
//! - **Text colors**: Primary text and muted text
//! - **Background colors**: Panel, element, and menu backgrounds
//! - **Diff colors**: Added, removed, and context colors for inline code diffs
//! - **Agent border colors**: Left border colors for distinguishing agent types
//!
//! # Example
//!
//! ```rust,ignore
//! use ratatui::style::Color;
//! use ratkit::widgets::ai_chat::components::theme::ChatColors;
//!
//! let colors = ChatColors::default();
//! // Use colors.primary for main accent
//! // Use colors.text for main message text
//! ```
use ratatui::style::{Color, Modifier, Style};

/// Chat-specific color definitions for the AI Chat widget.
///
/// This struct contains all the colors needed for rendering chat messages,
/// input fields, status indicators, and inline diff views within the chat
/// interface.
///
/// # Fields
///
/// ## Status Colors
/// - `primary`: Main accent color for interactive elements
/// - `secondary`: Secondary accent for less prominent elements
/// - `accent`: Highlight color for emphasis
/// - `success`: Color for successful operations
/// - `warning`: Color for warnings
/// - `error`: Color for errors
///
/// ## Text Colors
/// - `text`: Primary text color for messages
/// - `text_muted`: Muted text for timestamps, metadata
///
/// ## Background Colors
/// - `background_panel`: Main chat panel background
/// - `background_element`: Background for individual messages
/// - `background_menu`: Background for dropdown menus
///
/// ## Diff Colors
/// - `diff_added`: Text color for added lines in code
/// - `diff_removed`: Text color for removed lines in code
/// - `diff_added_bg`: Background for added lines
/// - `diff_removed_bg`: Background for removed lines
/// - `diff_context_bg`: Background for context lines
///
/// ## Agent Border Colors
/// - `user_border`: Left border color for user messages
/// - `assistant_border`: Left border color for assistant messages
/// - `system_border`: Left border color for system messages
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatColors {
    // Status colors
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    // Text colors
    pub text: Color,
    pub text_muted: Color,
    // Background colors
    pub background_panel: Color,
    pub background_element: Color,
    pub background_menu: Color,
    // Diff colors
    pub diff_added: Color,
    pub diff_removed: Color,
    pub diff_added_bg: Color,
    pub diff_removed_bg: Color,
    pub diff_context_bg: Color,
    // Agent border colors
    pub user_border: Color,
    pub assistant_border: Color,
    pub system_border: Color,
}

impl Default for ChatColors {
    /// Creates a default chat color scheme with a dark theme palette.
    ///
    /// The default colors are inspired by modern dark terminal themes
    /// with good contrast for readability.
    fn default() -> Self {
        Self {
            // Status colors - Cyan-based primary theme
            primary: Color::Cyan,
            secondary: Color::Magenta,
            accent: Color::Yellow,
            success: Color::Green,
            warning: Color::LightYellow,
            error: Color::LightRed,
            // Text colors
            text: Color::White,
            text_muted: Color::DarkGray,
            // Background colors - Dark theme
            background_panel: Color::Rgb(30, 30, 30),
            background_element: Color::Rgb(40, 40, 40),
            background_menu: Color::Rgb(25, 25, 25),
            // Diff colors - Gruvbox-inspired
            diff_added: Color::Rgb(152, 151, 26),
            diff_removed: Color::Rgb(204, 36, 29),
            diff_added_bg: Color::Rgb(50, 48, 47),
            diff_removed_bg: Color::Rgb(50, 41, 41),
            diff_context_bg: Color::Rgb(40, 40, 40),
            // Agent border colors - Distinct agent identification
            user_border: Color::Cyan,
            assistant_border: Color::Green,
            system_border: Color::Yellow,
        }
    }
}

impl ChatColors {
    /// Creates a new [`ChatColors`] instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a default dark theme chat color scheme.
    pub fn default_dark() -> Self {
        Self::default()
    }

    /// Creates a light-themed [`ChatColors`] instance.
    pub fn light() -> Self {
        Self {
            // Status colors
            primary: Color::Blue,
            secondary: Color::Magenta,
            accent: Color::Yellow,
            success: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            // Text colors
            text: Color::Black,
            text_muted: Color::Gray,
            // Background colors - Light theme
            background_panel: Color::Rgb(250, 250, 250),
            background_element: Color::White,
            background_menu: Color::Rgb(245, 245, 245),
            // Diff colors - Light theme
            diff_added: Color::Green,
            diff_removed: Color::Red,
            diff_added_bg: Color::Rgb(230, 245, 230),
            diff_removed_bg: Color::Rgb(245, 230, 230),
            diff_context_bg: Color::Rgb(240, 240, 240),
            // Agent border colors
            user_border: Color::Blue,
            assistant_border: Color::Green,
            system_border: Color::Yellow,
        }
    }

    // --- Builder-style setters ---

    /// Sets the primary color.
    pub fn with_primary(mut self, color: Color) -> Self {
        self.primary = color;
        self
    }

    /// Sets the secondary color.
    pub fn with_secondary(mut self, color: Color) -> Self {
        self.secondary = color;
        self
    }

    /// Sets the accent color.
    pub fn with_accent(mut self, color: Color) -> Self {
        self.accent = color;
        self
    }

    /// Sets the success color.
    pub fn with_success(mut self, color: Color) -> Self {
        self.success = color;
        self
    }

    /// Sets the warning color.
    pub fn with_warning(mut self, color: Color) -> Self {
        self.warning = color;
        self
    }

    /// Sets the error color.
    pub fn with_error(mut self, color: Color) -> Self {
        self.error = color;
        self
    }

    /// Sets the text color.
    pub fn with_text(mut self, color: Color) -> Self {
        self.text = color;
        self
    }

    /// Sets the muted text color.
    pub fn with_text_muted(mut self, color: Color) -> Self {
        self.text_muted = color;
        self
    }

    // --- Helper methods for creating styled objects ---

    /// Creates a [`Style`] with the primary color as foreground.
    pub fn primary_style(&self) -> Style {
        Style::default().fg(self.primary)
    }

    /// Creates a [`Style`] with the text color as foreground.
    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text)
    }

    /// Creates a [`Style`] with the muted text color as foreground.
    pub fn text_muted_style(&self) -> Style {
        Style::default().fg(self.text_muted)
    }

    /// Creates a [`Style`] with the success color as foreground.
    pub fn success_style(&self) -> Style {
        Style::default().fg(self.success)
    }

    /// Creates a [`Style`] with the warning color as foreground.
    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.warning)
    }

    /// Creates a [`Style`] with the error color as foreground.
    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error)
    }

    /// Creates a [`Style`] with a colored border.
    ///
    /// # Arguments
    ///
    /// * `color` - The border color to use
    ///
    /// # Returns
    ///
    /// A [`Style`] with the border modifier applied.
    pub fn border_with_color(&self, color: Color) -> Style {
        Style::default().fg(color)
    }

    /// Creates a [`Style`] with the user message border color.
    pub fn user_border_style(&self) -> Style {
        Style::default().fg(self.user_border)
    }

    /// Creates a [`Style`] with the assistant message border color.
    pub fn assistant_border_style(&self) -> Style {
        Style::default().fg(self.assistant_border)
    }

    /// Creates a [`Style`] with the system message border color.
    pub fn system_border_style(&self) -> Style {
        Style::default().fg(self.system_border)
    }

    /// Creates a [`Style`] with bold modifier.
    pub fn bold(&self) -> Style {
        Style::default().add_modifier(Modifier::BOLD)
    }

    /// Creates a [`Style`] with italic modifier.
    pub fn italic(&self) -> Style {
        Style::default().add_modifier(Modifier::ITALIC)
    }

    /// Creates a [`Style`] with dim modifier.
    pub fn dim(&self) -> Style {
        Style::default().add_modifier(Modifier::DIM)
    }

    /// Creates a [`Style`] with cross-out modifier.
    pub fn crossed(&self) -> Style {
        Style::default().add_modifier(Modifier::CROSSED_OUT)
    }

    /// Creates a [`Style`] with reversed (inverse) modifier.
    pub fn reversed(&self) -> Style {
        Style::default().add_modifier(Modifier::REVERSED)
    }

    /// Creates a [`Style`] with underline modifier.
    pub fn underlined(&self) -> Style {
        Style::default().add_modifier(Modifier::UNDERLINED)
    }

    /// Creates a [`Style`] for a highlighted (selected) item.
    pub fn highlighted(&self) -> Style {
        Style::default()
            .fg(self.text)
            .bg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Creates a [`Style`] for a disabled/muted item.
    pub fn disabled(&self) -> Style {
        Style::default()
            .fg(self.text_muted)
            .add_modifier(Modifier::DIM)
    }

    /// Creates a [`Style`] for diff added text.
    pub fn diff_added_style(&self) -> Style {
        Style::default().fg(self.diff_added)
    }

    /// Creates a [`Style`] for diff removed text.
    pub fn diff_removed_style(&self) -> Style {
        Style::default().fg(self.diff_removed)
    }

    /// Creates a [`Style`] for diff added background.
    pub fn diff_added_bg_style(&self) -> Style {
        Style::default().bg(self.diff_added_bg)
    }

    /// Creates a [`Style`] for diff removed background.
    pub fn diff_removed_bg_style(&self) -> Style {
        Style::default().bg(self.diff_removed_bg)
    }

    /// Creates a [`Style`] for diff context background.
    pub fn diff_context_bg_style(&self) -> Style {
        Style::default().bg(self.diff_context_bg)
    }

    /// Creates a complete diff added style (foreground + background).
    pub fn diff_added_full_style(&self) -> Style {
        Style::default().fg(self.diff_added).bg(self.diff_added_bg)
    }

    /// Creates a complete diff removed style (foreground + background).
    pub fn diff_removed_full_style(&self) -> Style {
        Style::default()
            .fg(self.diff_removed)
            .bg(self.diff_removed_bg)
    }

    /// Creates a complete diff context style (foreground + background).
    pub fn diff_context_full_style(&self) -> Style {
        Style::default().fg(self.text).bg(self.diff_context_bg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_colors() {
        let colors = ChatColors::default();
        assert_eq!(colors.primary, Color::Cyan);
        assert_eq!(colors.text, Color::White);
    }

    #[test]
    fn test_light_colors() {
        let colors = ChatColors::light();
        assert_eq!(colors.primary, Color::Blue);
        assert_eq!(colors.text, Color::Black);
    }

    #[test]
    fn test_builder_setters() {
        let colors = ChatColors::default()
            .with_primary(Color::Red)
            .with_text(Color::Gray);
        assert_eq!(colors.primary, Color::Red);
        assert_eq!(colors.text, Color::Gray);
    }

    #[test]
    fn test_helper_methods() {
        let colors = ChatColors::default();
        assert_eq!(colors.primary_style().fg, Some(Color::Cyan));
        assert_eq!(colors.text_style().fg, Some(Color::White));
        assert_eq!(colors.error_style().fg, Some(Color::LightRed));
    }
}
