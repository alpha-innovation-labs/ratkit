//! AI chat interface for ratatui

pub mod ai_chat;
mod components;
mod input;
mod message;

pub use ai_chat::{AIChat, AIChatEvent};
pub use input::InputState;
pub use message::{Message, MessageRole, MessageStore};
