//! Components module for AI Chat widget.
//!
//! This module contains submodules for various chat components:
//! - `theme`: Color definitions and styling utilities
//! - `message`: Message-related components (user, assistant, compact divider)
//! - `tools`: Tool-related components (temporarily disabled)
//! - `interactive`: Interactive UI components
//! - `parts`: UI part components (temporarily disabled)

pub mod interactive;
pub mod message;
pub mod theme;

// Temporarily disabled due to compilation issues
// pub mod parts;
// pub mod tools;

// Re-export commonly used types
pub use message::{MessagePart, ToolCall};
