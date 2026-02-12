//! Agent-specific color definitions for the AI Chat widget.
//!
//! This module provides color definitions for different agent types that can
//! appear in chat messages, allowing visual distinction between various
//! assistant agents, tools, and system components.
//!
//! # Agent Types
//!
//! - **Default**: General-purpose assistant messages
//! - **Code**: Code execution or analysis agents
//! - **Browse**: Web browsing or information retrieval agents
//! - **Memory**: Memory/knowledge base agents
//! - **General**: General conversation agents
//! - **Architect**: Architecture or design planning agents
//!
//! # Example
//!
//! ```rust,ignore
//! use ratatui::style::Color;
//! use ratkit::widgets::ai_chat::components::theme::{AgentColors, AgentType};
//!
//! let colors = AgentColors::default();
//! let code_color = colors.color_for(AgentType::Code);
//! ```
use ratatui::style::{Color, Style};

/// Agent type enumeration for categorizing different assistant agents.
///
/// Each agent type has an associated color for visual identification
/// in the chat interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AgentType {
    /// Default/general-purpose agent
    #[default]
    Default,
    /// Code-related agent (code review, generation, execution)
    Code,
    /// Web browsing or information retrieval agent
    Browse,
    /// Memory or knowledge base agent
    Memory,
    /// General conversation agent
    General,
    /// Architecture or design planning agent
    Architect,
}

impl AgentType {
    /// Returns a human-readable name for the agent type.
    pub fn name(&self) -> &'static str {
        match self {
            AgentType::Default => "Assistant",
            AgentType::Code => "Code",
            AgentType::Browse => "Browse",
            AgentType::Memory => "Memory",
            AgentType::General => "General",
            AgentType::Architect => "Architect",
        }
    }
}

/// Color definitions for different agent types.
///
/// This struct maps each agent type to specific colors for:
/// - Border/accent color for message containers
/// - Icon or label color
/// - Background tint (optional)
///
/// # Example
///
/// ```rust,ignore
/// use ratkit::widgets::ai_chat::components::theme::{AgentColors, AgentType};
///
/// let colors = AgentColors::default();
/// let code_agent_colors = colors.for_type(AgentType::Code);
/// // Use code_agent_colors.border for the left border
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentColors {
    pub default: AgentColorScheme,
    pub code: AgentColorScheme,
    pub browse: AgentColorScheme,
    pub memory: AgentColorScheme,
    pub general: AgentColorScheme,
    pub architect: AgentColorScheme,
}

/// A complete color scheme for a single agent type.
///
/// Contains all color variants needed to render an agent's messages
/// with proper visual distinction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentColorScheme {
    /// Primary border color (left border of message container).
    pub border: Color,
    /// Color for agent icon or label.
    pub icon: Color,
    /// Optional background tint for message elements.
    pub background_tint: Color,
}

impl AgentColorScheme {
    /// Creates a new agent color scheme.
    pub const fn new(border: Color, icon: Color, background_tint: Color) -> Self {
        Self {
            border,
            icon,
            background_tint,
        }
    }

    /// Creates a [`Style`] with the border color as foreground.
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Creates a [`Style`] with the icon color as foreground.
    pub fn icon_style(&self) -> Style {
        Style::default().fg(self.icon)
    }

    /// Creates a [`Style`] with the background tint.
    pub fn background_style(&self) -> Style {
        Style::default().bg(self.background_tint)
    }
}

impl Default for AgentColors {
    /// Creates a default agent color scheme with distinct colors for each agent type.
    ///
    /// The default colors are chosen for good contrast on dark backgrounds
    /// and to clearly distinguish between different agent types.
    fn default() -> Self {
        Self {
            // Default agent - Green theme
            default: AgentColorScheme::new(
                Color::Green,           // border
                Color::LightGreen,      // icon
                Color::Rgb(30, 50, 30), // background tint
            ),
            // Code agent - Blue theme
            code: AgentColorScheme::new(
                Color::Blue,            // border
                Color::LightBlue,       // icon
                Color::Rgb(30, 30, 60), // background tint
            ),
            // Browse agent - Cyan theme
            browse: AgentColorScheme::new(
                Color::Cyan,            // border
                Color::LightCyan,       // icon
                Color::Rgb(30, 50, 50), // background tint
            ),
            // Memory agent - Magenta theme
            memory: AgentColorScheme::new(
                Color::Magenta,         // border
                Color::LightMagenta,    // icon
                Color::Rgb(50, 30, 50), // background tint
            ),
            // General agent - Yellow theme
            general: AgentColorScheme::new(
                Color::Yellow,          // border
                Color::LightYellow,     // icon
                Color::Rgb(50, 50, 30), // background tint
            ),
            // Architect agent - White theme
            architect: AgentColorScheme::new(
                Color::White,           // border
                Color::Gray,            // icon
                Color::Rgb(45, 45, 45), // background tint
            ),
        }
    }
}

impl AgentColors {
    /// Creates a new [`AgentColors`] instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the color scheme for a specific agent type.
    ///
    /// # Arguments
    ///
    /// * `agent_type` - The type of agent to get colors for
    ///
    /// # Returns
    ///
    /// The [`AgentColorScheme`] for the given agent type.
    pub fn for_type(&self, agent_type: AgentType) -> &AgentColorScheme {
        match agent_type {
            AgentType::Default => &self.default,
            AgentType::Code => &self.code,
            AgentType::Browse => &self.browse,
            AgentType::Memory => &self.memory,
            AgentType::General => &self.general,
            AgentType::Architect => &self.architect,
        }
    }

    /// Returns the color for a specific agent type (convenience method).
    ///
    /// This is a shortcut to get just the border color.
    ///
    /// # Arguments
    ///
    /// * `agent_type` - The type of agent to get the color for
    ///
    /// # Returns
    ///
    /// The border [`Color`] for the given agent type.
    pub fn color_for(&self, agent_type: AgentType) -> Color {
        self.for_type(agent_type).border
    }

    /// Returns the icon color for a specific agent type.
    ///
    /// # Arguments
    ///
    /// * `agent_type` - The type of agent to get the icon color for
    ///
    /// # Returns
    ///
    /// The icon [`Color`] for the given agent type.
    pub fn icon_color_for(&self, agent_type: AgentType) -> Color {
        self.for_type(agent_type).icon
    }

    /// Creates a [`Style`] with the agent's border color.
    ///
    /// # Arguments
    ///
    /// * `agent_type` - The type of agent
    ///
    /// # Returns
    ///
    /// A [`Style`] with the agent's border color as foreground.
    pub fn border_style_for(&self, agent_type: AgentType) -> Style {
        self.for_type(agent_type).border_style()
    }

    /// Creates a [`Style`] with the agent's icon color.
    ///
    /// # Arguments
    ///
    /// * `agent_type` - The type of agent
    ///
    /// # Returns
    ///
    /// A [`Style`] with the agent's icon color as foreground.
    pub fn icon_style_for(&self, agent_type: AgentType) -> Style {
        self.for_type(agent_type).icon_style()
    }

    // --- Builder-style customization ---

    /// Sets the default agent colors.
    pub fn with_default(mut self, scheme: AgentColorScheme) -> Self {
        self.default = scheme;
        self
    }

    /// Sets the code agent colors.
    pub fn with_code(mut self, scheme: AgentColorScheme) -> Self {
        self.code = scheme;
        self
    }

    /// Sets the browse agent colors.
    pub fn with_browse(mut self, scheme: AgentColorScheme) -> Self {
        self.browse = scheme;
        self
    }

    /// Sets the memory agent colors.
    pub fn with_memory(mut self, scheme: AgentColorScheme) -> Self {
        self.memory = scheme;
        self
    }

    /// Sets the general agent colors.
    pub fn with_general(mut self, scheme: AgentColorScheme) -> Self {
        self.general = scheme;
        self
    }

    /// Sets the architect agent colors.
    pub fn with_architect(mut self, scheme: AgentColorScheme) -> Self {
        self.architect = scheme;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_type_name() {
        assert_eq!(AgentType::Default.name(), "Assistant");
        assert_eq!(AgentType::Code.name(), "Code");
        assert_eq!(AgentType::Browse.name(), "Browse");
    }

    #[test]
    fn test_default_agent_colors() {
        let colors = AgentColors::default();
        assert_eq!(colors.default.border, Color::Green);
        assert_eq!(colors.code.border, Color::Blue);
        assert_eq!(colors.browse.border, Color::Cyan);
    }

    #[test]
    fn test_for_type() {
        let colors = AgentColors::default();
        assert_eq!(colors.for_type(AgentType::Memory).border, Color::Magenta);
        assert_eq!(colors.for_type(AgentType::Architect).border, Color::White);
    }

    #[test]
    fn test_color_for() {
        let colors = AgentColors::default();
        assert_eq!(colors.color_for(AgentType::Default), Color::Green);
        assert_eq!(colors.color_for(AgentType::General), Color::Yellow);
    }

    #[test]
    fn test_agent_color_scheme_styles() {
        let colors = AgentColors::default();
        let scheme = colors.for_type(AgentType::Code);
        assert_eq!(scheme.border_style().fg, Some(Color::Blue));
        assert_eq!(scheme.icon_style().fg, Some(Color::LightBlue));
    }

    #[test]
    fn test_builder_pattern() {
        let colors = AgentColors::default()
            .with_code(AgentColorScheme::new(
                Color::Red,
                Color::LightRed,
                Color::Rgb(60, 30, 30),
            ))
            .with_browse(AgentColorScheme::new(
                Color::Cyan,
                Color::Cyan,
                Color::Rgb(30, 60, 60),
            ));
        assert_eq!(colors.code.border, Color::Red);
        assert_eq!(colors.browse.border, Color::Cyan);
    }
}
