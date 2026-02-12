//! Message components for AI Chat widget.
//!
//! This module provides widgets for rendering different types of chat messages:
//! - [`UserMessage`] - User-originated messages
//! - [`AssistantMessage`] - AI/assistant responses
//! - [`CompactDivider`] - Collapsed message sections
//!
//! # Usage
//!
//! ```rust,ignore
//! use ratatui::{Frame, buffer::Buffer, layout::Rect};
//! use ratkit::widgets::ai_chat::components::message::{
//!     UserMessage, AssistantMessage, CompactDivider, Attachment, MessagePart, ToolCall,
//! };
//! use ratkit::widgets::ai_chat::components::theme::ChatColors;
//!
//! // Render a user message
//! let user_msg = UserMessage::new("Hello, world!")
//!     .queued(false)
//!     .agent_color(ratatui::style::Color::Cyan);
//! user_msg.render(area, buf);
//!
//! // Render an assistant message
//! let parts = vec![
//!     MessagePart::Text("Hello!".to_string()),
//!     MessagePart::Reasoning("Thinking...".to_string()),
//! ];
//! let assistant_msg = AssistantMessage::new(&parts)
//!     .agent_name("claude")
//!     .model_id("claude-3")
//!     .duration_ms(1500);
//! assistant_msg.render(area, buf);
//!
//! // Render a compact divider
//! let divider = CompactDivider::new().with_hidden_count(5);
//! divider.render(area, buf);
//! ```

pub mod assistant_message;
pub mod compact_divider;
pub mod user_message;

pub use assistant_message::{AssistantMessage, AssistantMessageRenderer, MessagePart, ToolCall};
pub use compact_divider::{CompactDivider, CompactDividerRenderer};
pub use user_message::{Attachment, UserMessage, UserMessageRenderer};
