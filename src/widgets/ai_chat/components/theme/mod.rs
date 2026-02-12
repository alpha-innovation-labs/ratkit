//! Theme module for AI Chat widget components.
//!
//! This module provides color definitions and styling utilities for the AI Chat
//! widget, including chat-specific colors, agent-specific colors, and helper
//! methods for creating styled UI elements.
//!
//! # Modules
//!
//! - [`colors`]: Chat-specific color definitions
//! - [`agent_colors`]: Agent-specific color definitions
//!
//! # Example
//!
//! ```rust,ignore
//! use ratkit::widgets::ai_chat::components::theme::{ChatColors, AgentColors, AgentType};
//!
//! // Use chat colors for general styling
//! let chat_colors = ChatColors::default();
//! let style = chat_colors.primary_style();
//!
//! // Use agent colors for distinguishing agent types
//! let agent_colors = AgentColors::default();
//! let code_border = agent_colors.color_for(AgentType::Code);
//! ```
pub mod agent_colors;
pub mod colors;

pub use agent_colors::{AgentColorScheme, AgentColors, AgentType};
pub use colors::ChatColors;
