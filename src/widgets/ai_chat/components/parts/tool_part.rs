//! ToolPart component for rendering tool calls in AI Chat.
//!
//! This module provides the [`ToolPart`] widget that dispatches to the correct
//! tool-specific component based on the tool type. It uses a PART_MAPPING pattern
//! to map tool types to their corresponding renderable components.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

/// Enum representing different tool types that can be called.
///
/// Each variant corresponds to a specific tool with its own rendering requirements:
/// - Simple tools render inline
/// - Complex tools render as blocks with detailed information
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolType {
    /// Bash/shell command execution
    Bash,
    /// File writing operation
    Write,
    /// File editing operation
    Edit,
    /// File reading operation
    Read,
    /// File glob pattern matching
    Glob,
    /// Content search
    Grep,
    /// Directory listing
    List,
    /// Web URL fetching
    WebFetch,
    /// Web search
    WebSearch,
    /// Code search
    CodeSearch,
    /// Task/subagent delegation
    Task,
    /// Patch application
    ApplyPatch,
    /// Todo/note writing
    TodoWrite,
    /// Question to user
    Question,
    /// Skill invocation
    Skill,
    /// Generic/unknown tool
    Generic,
}

impl ToolType {
    /// Convert a tool name string to ToolType.
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "bash" | "shell" | "sh" | "zsh" => ToolType::Bash,
            "write" | "filesystem_write" | "write_file" => ToolType::Write,
            "edit" | "filesystem_edit" | "edit_file" => ToolType::Edit,
            "read" | "filesystem_read" | "read_file" => ToolType::Read,
            "glob" | "filesystem_glob" => ToolType::Glob,
            "grep" | "search" | "content_search" => ToolType::Grep,
            "list" | "ls" | "directory_list" => ToolType::List,
            "webfetch" | "fetch" | "http_get" => ToolType::WebFetch,
            "websearch" | "search_web" => ToolType::WebSearch,
            "codesearch" | "search_code" => ToolType::CodeSearch,
            "task" | "subagent" | "delegate" => ToolType::Task,
            "applypatch" | "patch" | "apply_patch" => ToolType::ApplyPatch,
            "todowrite" | "todo" | "note" => ToolType::TodoWrite,
            "question" | "ask" | "clarify" => ToolType::Question,
            "skill" | "invoke" | "call" => ToolType::Skill,
            _ => ToolType::Generic,
        }
    }

    /// Check if this tool type should render as inline (simple) or block (complex).
    pub fn is_inline(&self) -> bool {
        matches!(
            self,
            ToolType::Read
                | ToolType::Glob
                | ToolType::List
                | ToolType::TodoWrite
                | ToolType::Question
        )
    }

    /// Get the display name for this tool type.
    pub fn display_name(&self) -> &'static str {
        match self {
            ToolType::Bash => "Bash",
            ToolType::Write => "Write",
            ToolType::Edit => "Edit",
            ToolType::Read => "Read",
            ToolType::Glob => "Glob",
            ToolType::Grep => "Grep",
            ToolType::List => "List",
            ToolType::WebFetch => "WebFetch",
            ToolType::WebSearch => "WebSearch",
            ToolType::CodeSearch => "CodeSearch",
            ToolType::Task => "Task",
            ToolType::ApplyPatch => "ApplyPatch",
            ToolType::TodoWrite => "TodoWrite",
            ToolType::Question => "Question",
            ToolType::Skill => "Skill",
            ToolType::Generic => "Tool",
        }
    }

    /// Get the icon for this tool type.
    pub fn icon(&self) -> &'static str {
        match self {
            ToolType::Bash => "⚡",
            ToolType::Write => "✏️",
            ToolType::Edit => "🔧",
            ToolType::Read => "📖",
            ToolType::Glob => "🔍",
            ToolType::Grep => "🔎",
            ToolType::List => "📁",
            ToolType::WebFetch => "🌐",
            ToolType::WebSearch => "🔎",
            ToolType::CodeSearch => "💻",
            ToolType::Task => "🎯",
            ToolType::ApplyPatch => "🩹",
            ToolType::TodoWrite => "📝",
            ToolType::Question => "❓",
            ToolType::Skill => "⚙️",
            ToolType::Generic => "🔧",
        }
    }
}

/// Tool call data containing the tool name and arguments.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolCall {
    /// The tool name
    pub name: String,
    /// The tool arguments (JSON string)
    pub arguments: String,
    /// Optional result from tool execution
    pub result: Option<String>,
    /// Whether the tool is currently executing
    pub executing: bool,
}

impl ToolCall {
    /// Create a new tool call.
    pub fn new(name: String, arguments: String) -> Self {
        Self {
            name,
            arguments,
            result: None,
            executing: false,
        }
    }

    /// Set the result.
    pub fn with_result(mut self, result: String) -> Self {
        self.result = Some(result);
        self
    }

    /// Set executing state.
    pub fn executing(mut self, executing: bool) -> Self {
        self.executing = executing;
        self
    }

    /// Get the tool type.
    pub fn tool_type(&self) -> ToolType {
        ToolType::from_name(&self.name)
    }
}

/// ToolPart component that dispatches to the correct tool renderer.
///
/// Uses PART_MAPPING to map ToolType to the appropriate render function.
pub struct ToolPart<'a> {
    /// The tool call data
    tool_call: &'a ToolCall,
    /// Whether to show detailed information
    show_details: bool,
    /// Custom colors (optional)
    colors: Option<ChatColors>,
}

impl<'a> ToolPart<'a> {
    /// Create a new ToolPart with the given tool call.
    pub fn new(tool_call: &'a ToolCall) -> Self {
        Self {
            tool_call,
            show_details: true,
            colors: None,
        }
    }

    /// Set show_details flag.
    pub fn show_details(mut self, show_details: bool) -> Self {
        self.show_details = show_details;
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

    /// Render inline tool (simple display).
    fn render_inline(&self, area: Rect, buf: &mut Buffer) {
        let colors = self.get_colors();
        let tool_type = self.tool_call.tool_type();

        // Status icon and tool info
        let status_icon = if self.tool_call.executing {
            "⚙"
        } else if self.tool_call.result.is_some() {
            "✓"
        } else {
            tool_type.icon()
        };

        let status_style = if self.tool_call.executing {
            Style::default()
                .fg(colors.warning)
                .add_modifier(Modifier::BOLD)
        } else if self.tool_call.result.is_some() {
            Style::default()
                .fg(colors.success)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(colors.primary)
                .add_modifier(Modifier::BOLD)
        };

        let content = format!("{} {}", status_icon, tool_type.display_name());
        let span = Span::styled(content, status_style);
        buf.set_span(area.x, area.y, &span, area.width);
    }

    /// Render block tool (detailed display).
    fn render_block(&self, area: Rect, buf: &mut Buffer) {
        let colors = self.get_colors();
        let tool_type = self.tool_call.tool_type();
        let max_y = area.y + area.height;
        let mut y = area.y;

        // Draw left border
        let border_color = if self.tool_call.executing {
            colors.warning
        } else if self.tool_call.result.is_some() {
            colors.success
        } else {
            colors.primary
        };

        for y_pos in area.y..max_y {
            buf.get_mut(area.x, y_pos)
                .set_style(Style::default().fg(border_color));
        }

        // Header with tool info
        let status_icon = if self.tool_call.executing {
            "⚙️  Executing"
        } else if self.tool_call.result.is_some() {
            "✓  Done"
        } else {
            "🔧  Tool"
        };

        let header_style = Style::default()
            .fg(border_color)
            .add_modifier(Modifier::BOLD);

        let header = format!("{} {}", status_icon, tool_type.display_name());
        buf.set_span(
            area.x + 2,
            y,
            &Span::styled(header, header_style),
            area.width - 2,
        );
        y += 1;

        // Tool name in arguments
        if y < max_y {
            let args_label = Span::styled("  name: ", Style::default().fg(colors.text_muted));
            buf.set_span(area.x + 2, y, &args_label, area.width - 2);

            let name_value = Span::styled(
                self.tool_call.name.clone(),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            );
            buf.set_span(area.x + 2 + 7, y, &name_value, area.width.saturating_sub(9));
            y += 1;
        }

        // Arguments (truncated for block display)
        if y < max_y && !self.tool_call.arguments.is_empty() {
            let args_preview = if self.tool_call.arguments.len() > 50 {
                format!("{}...", &self.tool_call.arguments[..50])
            } else {
                self.tool_call.arguments.clone()
            };

            let args_span = Span::styled(
                format!("  args: {}", args_preview),
                Style::default().fg(colors.text_muted),
            );
            buf.set_span(area.x + 2, y, &args_span, area.width - 2);
            y += 1;
        }

        // Result if present
        if let Some(result) = &self.tool_call.result {
            if y < max_y {
                let result_preview = if result.len() > 80 {
                    format!("{}...", &result[..80])
                } else {
                    result.clone()
                };

                let result_span = Span::styled(
                    format!("  → {}", result_preview),
                    Style::default().fg(colors.success),
                );
                buf.set_span(area.x + 2, y, &result_span, area.width - 2);
            }
        }
    }
}

impl<'a> Widget for ToolPart<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // If show_details is false, don't render anything (hidden)
        if !self.show_details {
            return;
        }

        if self.tool_call.tool_type().is_inline() {
            self.render_inline(area, buf);
        } else {
            self.render_block(area, buf);
        }
    }
}

/// Builder for ToolPart with custom rendering options.
pub struct ToolPartRenderer<'a> {
    tool_call: &'a ToolCall,
    show_details: bool,
    colors: ChatColors,
}

impl<'a> ToolPartRenderer<'a> {
    /// Create a new renderer.
    pub fn new(tool_call: &'a ToolCall) -> Self {
        Self {
            tool_call,
            show_details: true,
            colors: ChatColors::default(),
        }
    }

    /// Set show_details flag.
    pub fn show_details(mut self, show_details: bool) -> Self {
        self.show_details = show_details;
        self
    }

    /// Set custom colors.
    pub fn colors(mut self, colors: ChatColors) -> Self {
        self.colors = colors;
        self
    }

    /// Render the tool part.
    pub fn render(self, area: Rect, buf: &mut Buffer) {
        let part = ToolPart {
            tool_call: self.tool_call,
            show_details: self.show_details,
            colors: Some(self.colors),
        };
        part.render(area, buf);
    }
}

// === PART_MAPPING pattern ===

/// Type alias for tool part render functions.
type ToolPartRendererFn<'a> = fn(&'a ToolCall, Rect, &mut Buffer, &ChatColors);

/// Map of ToolType to its corresponding renderer function.
///
/// This PART_MAPPING pattern allows easy extension for new tool types.
pub const PART_MAPPING: &[(ToolType, ToolPartRendererFn)] = &[
    (ToolType::Bash, render_bash_tool),
    (ToolType::Write, render_write_tool),
    (ToolType::Edit, render_edit_tool),
    (ToolType::Read, render_read_tool),
    (ToolType::Glob, render_glob_tool),
    (ToolType::Grep, render_grep_tool),
    (ToolType::List, render_list_tool),
    (ToolType::WebFetch, render_webfetch_tool),
    (ToolType::WebSearch, render_websearch_tool),
    (ToolType::CodeSearch, render_codesearch_tool),
    (ToolType::Task, render_task_tool),
    (ToolType::ApplyPatch, render_applypatch_tool),
    (ToolType::TodoWrite, render_todowrite_tool),
    (ToolType::Question, render_question_tool),
    (ToolType::Skill, render_skill_tool),
    (ToolType::Generic, render_generic_tool),
];

/// Get the renderer function for a tool type.
pub fn get_renderer_for_tool(tool_type: &ToolType) -> Option<ToolPartRendererFn> {
    PART_MAPPING
        .iter()
        .find(|(t, _)| t == tool_type)
        .map(|(_, f)| *f)
}

// === Individual tool renderers ===

fn render_bash_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let status = if call.executing { "⚙" } else { "⚡" };
    let style = if call.executing {
        colors.warning
    } else {
        colors.primary
    };

    let content = format!("{} Bash: {}", status, call.arguments);
    let span = Span::styled(content, Style::default().fg(style));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_write_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("✏️  Write: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.primary));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_edit_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("🔧  Edit: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.secondary));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_read_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("📖  Read: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.text));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_glob_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("🔍  Glob: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.text));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_grep_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("🔎  Grep: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.accent));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_list_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("📁  List: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.text));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_webfetch_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("🌐  Fetch: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.primary));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_websearch_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("🔎  Search: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.primary));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_codesearch_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("💻  CodeSearch: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.secondary));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_task_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("🎯  Task: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.accent));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_applypatch_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("🩹  ApplyPatch: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.warning));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_todowrite_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("📝  Todo: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.text));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_question_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("❓  Question: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.warning));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_skill_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("⚙️  Skill: {}", call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.secondary));
    buf.set_span(area.x, area.y, &span, area.width);
}

fn render_generic_tool(call: &ToolCall, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
    let content = format!("🔧  {}: {}", call.name, call.arguments);
    let span = Span::styled(content, Style::default().fg(colors.text));
    buf.set_span(area.x, area.y, &span, area.width);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_type_from_name() {
        assert_eq!(ToolType::from_name("bash"), ToolType::Bash);
        assert_eq!(ToolType::from_name("Write"), ToolType::Write);
        assert_eq!(ToolType::from_name("filesystem_read"), ToolType::Read);
        assert_eq!(ToolType::from_name("unknown_tool"), ToolType::Generic);
    }

    #[test]
    fn test_tool_type_is_inline() {
        assert!(ToolType::Read.is_inline());
        assert!(ToolType::Glob.is_inline());
        assert!(!ToolType::Bash.is_inline());
        assert!(!ToolType::Write.is_inline());
    }

    #[test]
    fn test_tool_type_display_name() {
        assert_eq!(ToolType::Bash.display_name(), "Bash");
        assert_eq!(ToolType::Write.display_name(), "Write");
    }

    #[test]
    fn test_tool_type_icon() {
        assert_eq!(ToolType::Bash.icon(), "⚡");
        assert_eq!(ToolType::Read.icon(), "📖");
    }

    #[test]
    fn test_tool_call_builder() {
        let call = ToolCall::new("bash".to_string(), "ls -la".to_string());
        assert_eq!(call.name, "bash");
        assert_eq!(call.arguments, "ls -la");
        assert!(call.result.is_none());
        assert!(!call.executing);
    }

    #[test]
    fn test_tool_call_with_options() {
        let call = ToolCall::new("read".to_string(), "/test.txt".to_string())
            .with_result("file content".to_string())
            .executing(true);

        assert!(call.result.is_some());
        assert!(call.executing);
    }

    #[test]
    fn test_tool_part_builder() {
        let call = ToolCall::new("bash".to_string(), "echo hello".to_string());
        let part = ToolPart::new(&call);

        assert!(part.show_details);
    }

    #[test]
    fn test_tool_part_options() {
        let call = ToolCall::new("bash".to_string(), "echo hello".to_string());
        let part = ToolPart::new(&call).show_details(false);

        assert!(!part.show_details);
    }

    #[test]
    fn test_get_renderer_for_tool() {
        let renderer = get_renderer_for_tool(&ToolType::Bash);
        assert!(renderer.is_some());

        let renderer = get_renderer_for_tool(&ToolType::Generic);
        assert!(renderer.is_some());
    }

    #[test]
    fn test_part_mapping_length() {
        // Verify all tool types have renderers
        assert!(PART_MAPPING.len() >= 16);
    }
}
