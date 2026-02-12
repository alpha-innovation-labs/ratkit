//! Block-level tool display component.
//!
//! This module provides an expanded block representation of tool execution,
//! with interactive features like expand/collapse and hover effects.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

use super::inline_tool::{InlineTool, ToolStatus};

/// Block tool display with expanded information.
///
/// Renders a panel with:
/// - Left border indicating status
/// - Title with tool name and optional spinner
/// - Expandable/collapsible output
/// - Hover effect support
pub struct BlockTool<'a> {
    /// Tool name to display
    name: &'a str,
    /// Current execution status
    status: ToolStatus,
    /// Whether the output is expanded
    expanded: bool,
    /// Output content from the tool
    output: String,
    /// Icon character to display
    icon: char,
    /// Whether this block is being hovered
    hovered: bool,
}

impl<'a> BlockTool<'a> {
    /// Create a new BlockTool.
    pub fn new(name: &'a str, status: ToolStatus, output: String) -> Self {
        Self {
            name,
            status,
            expanded: false,
            output,
            icon: Self::default_icon(&status),
            hovered: false,
        }
    }

    /// Set expanded state.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set hover state.
    pub fn hovered(mut self, hovered: bool) -> Self {
        self.hovered = hovered;
        self
    }

    /// Set a custom icon.
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = icon;
        self
    }

    /// Get the default icon for a status.
    fn default_icon(status: &ToolStatus) -> char {
        match status {
            ToolStatus::Pending => '~',
            ToolStatus::Complete => '✓',
            ToolStatus::Error => '✗',
            ToolStatus::PermissionPending => '⚠',
        }
    }

    /// Get the border color for a status.
    fn border_color(&self, colors: &ChatColors) -> Color {
        match self.status {
            ToolStatus::Pending => colors.warning,
            ToolStatus::Complete => colors.success,
            ToolStatus::Error => colors.error,
            ToolStatus::PermissionPending => Color::Rgb(255, 165, 0), // Orange
        }
    }

    /// Render the block tool to a buffer.
    pub fn render(&self, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
        if area.height < 1 {
            return;
        }

        let border_color = self.border_color(colors);
        let max_y = area.y + area.height;
        let mut y = area.y;

        // Draw left border
        for y_pos in area.y..max_y {
            let cell = buf.get_mut(area.x, y_pos);
            cell.set_char('│')
                .set_style(Style::default().fg(border_color));
        }

        // Render header
        let header = self.render_header(colors);
        buf.set_span(area.x + 2, y, &header, area.width.saturating_sub(3));
        y += 1;

        // Render separator line
        if y < max_y {
            let sep = "─".repeat((area.width.saturating_sub(3)).min(50));
            let sep_span = Span::styled(
                sep,
                Style::default()
                    .fg(border_color)
                    .add_modifier(Modifier::DIM),
            );
            buf.set_span(area.x + 2, y, &sep_span, area.width.saturating_sub(3));
            y += 1;
        }

        // Render output if expanded and present
        if self.expanded && !self.output.is_empty() && y < max_y {
            let output_area = Rect::new(area.x + 2, y, area.width.saturating_sub(2), max_y - y);
            self.render_output(output_area, buf, colors);
        }
    }

    /// Render the header with title and status.
    fn render_header(&self, colors: &ChatColors) -> Span<'static> {
        // Status indicator
        let status_indicator = match self.status {
            ToolStatus::Pending => {
                // Animated spinner
                let spinner_frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
                let frame = spinner_frames[(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
                    / 100) as usize
                    % spinner_frames.len()];
                format!("{} ", frame)
            }
            ToolStatus::Complete => "✓ ".to_string(),
            ToolStatus::Error => "✗ ".to_string(),
            ToolStatus::PermissionPending => "⚠ ".to_string(),
        };

        // Expand/collapse indicator
        let expand_indicator = if self.expanded { "▼" } else { "▶" };

        // Build header text
        let header_text = format!(
            "{}{} {} {}{}",
            status_indicator,
            self.icon,
            self.name,
            expand_indicator,
            if self.hovered { " ◉" } else { "" }
        );

        // Style based on status
        let style = match self.status {
            ToolStatus::Error => Style::default()
                .fg(colors.error)
                .add_modifier(Modifier::BOLD),
            ToolStatus::Pending => Style::default()
                .fg(colors.warning)
                .add_modifier(Modifier::BOLD),
            _ => Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD),
        };

        Span::styled(header_text, style)
    }

    /// Render the output content.
    fn render_output(&self, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
        let lines: Vec<&str> = self.output.lines().collect();
        let max_lines = area.height as usize;
        let display_lines = lines.iter().take(max_lines).collect::<Vec<_>>();

        for (i, line) in display_lines.iter().enumerate() {
            if area.y + i >= area.y + area.height {
                break;
            }

            let line_text = if line.len() > area.width as usize {
                format!("{}...", &line[..area.width as usize - 3])
            } else {
                line.to_string()
            };

            let line_style = if self.status == ToolStatus::Error {
                Style::default().fg(colors.error)
            } else {
                Style::default().fg(colors.text_muted)
            };

            buf.set_span(
                area.x,
                area.y + i,
                &Span::styled(line_text, line_style),
                area.width,
            );
        }

        // Show "..." if output was truncated
        if lines.len() > max_lines && area.y + max_lines < area.y + area.height {
            let more_text = format!("... {} more lines", lines.len() - max_lines);
            buf.set_span(
                area.x,
                area.y + max_lines,
                &Span::styled(more_text, Style::default().fg(colors.text_muted)),
                area.width,
            );
        }
    }

    /// Toggle expanded state.
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }

    /// Check if a point is within the clickable header area.
    pub fn is_header_click(&self, x: u16, y: u16, area: Rect) -> bool {
        // Header is at area.y
        y == area.y && x >= area.x + 2
    }
}

impl Widget for BlockTool<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_tool_basic() {
        let tool = BlockTool::new("bash", ToolStatus::Complete, "output".to_string());
        assert_eq!(tool.name, "bash");
        assert_eq!(tool.status, ToolStatus::Complete);
        assert!(!tool.expanded);
    }

    #[test]
    fn test_block_tool_expanded() {
        let tool =
            BlockTool::new("read", ToolStatus::Complete, "content".to_string()).expanded(true);
        assert!(tool.expanded);
    }

    #[test]
    fn test_block_tool_toggle() {
        let mut tool = BlockTool::new("bash", ToolStatus::Pending, "".to_string());
        assert!(!tool.expanded);
        tool.toggle_expanded();
        assert!(tool.expanded);
    }
}
