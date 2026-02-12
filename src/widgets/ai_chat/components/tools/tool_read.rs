//! Read tool (→) display component.
//!
//! This module provides rendering for file reading tools with dependency tracking.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};

use crate::widgets::ai_chat::components::theme::ChatColors;

use super::block_tool::BlockTool;
use super::inline_tool::ToolStatus;

/// Read tool display for file content retrieval.
///
/// Renders:
/// - File path
/// - Loaded dependencies list with ↳ prefix
pub struct ToolRead<'a> {
    /// Path to the file being read
    file_path: &'a str,
    /// Dependencies (other files that were loaded as part of this read)
    dependencies: Vec<&'a str>,
    /// Status of the operation
    status: ToolStatus,
}

impl<'a> ToolRead<'a> {
    /// Create a new ToolRead.
    pub fn new(file_path: &'a str) -> Self {
        Self {
            file_path,
            dependencies: Vec::new(),
            status: ToolStatus::Pending,
        }
    }

    /// Add a dependency (file that was also loaded).
    pub fn add_dependency(mut self, path: &'a str) -> Self {
        self.dependencies.push(path);
        self
    }

    /// Set dependencies.
    pub fn dependencies(mut self, deps: Vec<&'a str>) -> Self {
        self.dependencies = deps;
        self
    }

    /// Set the status.
    pub fn status(mut self, status: ToolStatus) -> Self {
        self.status = status;
        self
    }

    /// Get border color based on status.
    fn border_color(&self, colors: &ChatColors) -> Color {
        match self.status {
            ToolStatus::Pending => colors.warning,
            ToolStatus::Complete => colors.success,
            ToolStatus::Error => colors.error,
            ToolStatus::PermissionPending => Color::Rgb(255, 165, 0),
        }
    }

    /// Render the read tool to a buffer.
    pub fn render(&self, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
        if area.height < 1 {
            return;
        }

        let max_y = area.y + area.height;
        let mut y = area.y;
        let border_color = self.border_color(colors);

        // Draw left border
        for y_pos in area.y..max_y {
            buf.get_mut(area.x, y_pos)
                .set_char('│')
                .set_style(Style::default().fg(border_color));
        }

        // === Header ===
        let icon = '→';
        let status_text = match self.status {
            ToolStatus::Pending => "Reading...",
            ToolStatus::Complete => "Read",
            ToolStatus::Error => "Failed",
            ToolStatus::PermissionPending => "Permission required",
        };
        let header = format!("{} {}", icon, status_text);
        let header_span = Span::styled(
            header,
            Style::default()
                .fg(border_color)
                .add_modifier(Modifier::BOLD),
        );
        buf.set_span(area.x + 2, y, &header_span, area.width.saturating_sub(3));
        y += 1;

        // Separator
        if y < max_y {
            buf.set_span(
                area.x + 2,
                y,
                &Span::styled(
                    "─".repeat(20),
                    Style::default()
                        .fg(border_color)
                        .add_modifier(Modifier::DIM),
                ),
                area.width.saturating_sub(3),
            );
            y += 1;
        }

        // === File Path ===
        if y < max_y {
            let path_span = Span::styled(
                format!("📄 {}", self.file_path),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            );
            buf.set_span(area.x + 2, y, &path_span, area.width.saturating_sub(3));
            y += 1;
        }

        // === Dependencies ===
        if !self.dependencies.is_empty() && y < max_y {
            let dep_header = Span::styled(
                "  Dependencies:",
                Style::default()
                    .fg(colors.text_muted)
                    .add_modifier(Modifier::ITALIC),
            );
            buf.set_span(area.x + 2, y, &dep_header, area.width.saturating_sub(3));
            y += 1;

            for dep in self
                .dependencies
                .iter()
                .take(area.height.saturating_sub(y - area.y) as usize)
            {
                if y >= max_y {
                    break;
                }

                let dep_span = Span::styled(
                    format!("  ↳ {}", dep),
                    Style::default().fg(colors.text_muted),
                );
                buf.set_span(area.x + 2, y, &dep_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

        // === Status ===
        if y < max_y && self.status != ToolStatus::Complete {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Loading...",
                ToolStatus::Error => "✗ Failed to read file",
                ToolStatus::PermissionPending => "⚠ Permission required",
                _ => "",
            };

            if !status_text.is_empty() {
                let status_span = Span::styled(status_text, Style::default().fg(border_color));
                buf.set_span(area.x + 2, y, &status_span, area.width.saturating_sub(3));
            }
        }
    }
}

impl Widget for ToolRead<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_read_basic() {
        let read = ToolRead::new("src/main.rs");
        assert_eq!(read.file_path, "src/main.rs");
        assert!(read.dependencies.is_empty());
    }

    #[test]
    fn test_tool_read_with_dependencies() {
        let read = ToolRead::new("src/lib.rs")
            .add_dependency("src/utils.rs")
            .add_dependency("src/types.rs");

        assert_eq!(read.dependencies.len(), 2);
    }

    #[test]
    fn test_tool_read_status() {
        let pending = ToolRead::new("test.rs").status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);

        let complete = ToolRead::new("test.rs").status(ToolStatus::Complete);
        assert_eq!(complete.status, ToolStatus::Complete);
    }
}
