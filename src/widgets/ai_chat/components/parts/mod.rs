//! Parts module for AI Chat widget components.
//!
//! This module contains UI part components for rendering different parts of chat messages:
//! - [`TextPart`]: Text content with markdown, streaming, and concealed content support
//! - [`ReasoningPart`]: AI thinking/reasoning content with distinct visual styling
//! - [`ToolPart`]: Tool call dispatcher with tool-specific rendering
//!
//! # Example
//!
//! ```rust,ignore
//! use ratkit::widgets::ai_chat::components::parts::{TextPart, ReasoningPart, ToolPart};
//!
//! // Render text content
//! let text = TextPart::new("Hello, world!").markdown(true);
//!
//! // Render reasoning
//! let reasoning = ReasoningPart::new("Let me think...");
//!
//! // Render tool call
//! let tool_call = ToolCall::new("bash".to_string(), "ls".to_string());
//! let tool = ToolPart::new(&tool_call);
//! ```

pub mod reasoning_part;
pub mod text_part;
pub mod tool_part;

pub use reasoning_part::{ReasoningPart, ReasoningPartRenderer};
pub use text_part::{TextPart, TextPartRenderer};
pub use tool_part::{ToolCall, ToolPart, ToolPartRenderer, ToolType};

// Re-export for convenience
pub use tool_part::{get_renderer_for_tool, PART_MAPPING};
