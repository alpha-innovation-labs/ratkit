//! TodoWrite tool (⚙) display component.
//!
//! This module provides rendering for todo/note writing tools.

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

/// A single todo item.
#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    /// Whether the item is completed
    pub completed: bool,
    /// The todo text
    pub text: String,
}

/// TodoWrite tool display for task list management.
///
/// Renders:
/// - Todo list with status checkboxes
pub struct ToolTodoWrite<'a> {
    /// Todo items
    todos: Vec<TodoItem>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether to show all items
    expanded: bool,
    /// Optional file path where todos are saved
    file_path: Option<&'a str>,
}

impl<'a> ToolTodoWrite<'a> {
    /// Create a new ToolTodoWrite.
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            status: ToolStatus::Pending,
            expanded: false,
            file_path: None,
        }
    }

    /// Add a todo item.
    pub fn add_todo(mut self, item: TodoItem) -> Self {
        self.todos.push(item);
        self
    }

    /// Set todos.
    pub fn todos(mut self, items: Vec<TodoItem>) -> Self {
        self.todos = items;
        self
    }

    /// Set the status.
    pub fn status(mut self, status: ToolStatus) -> Self {
        self.status = status;
        self
    }

    /// Set expanded state.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set file path.
    pub fn file_path(mut self, path: Option<&'a str>) -> Self {
        self.file_path = path;
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

    /// Render the todowrite tool to a buffer.
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
        let icon = '⚙';
        let expand_icon = if self.expanded { "▼" } else { "▶" };
        let todo_count = self.todos.len();
        let completed_count = self.todos.iter().filter(|t| t.completed).count();
        let header = format!(
            "{} TodoWrite [{}] {}{}",
            icon,
            todo_count,
            expand_icon,
            if completed_count > 0 {
                format!(" ({} done)", completed_count)
            } else {
                String::new()
            }
        );
        let header_span = Span::styled(
            header,
            Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD),
        );
        buf.set_span(area.x + 2, y, &header_span, area.width.saturating_sub(3));
        y += 1;

        // File path if set
        if let Some(path) = self.file_path {
            if y < max_y {
                let path_span = Span::styled(
                    format!("  📝 {}", path),
                    Style::default().fg(colors.text_muted),
                );
                buf.set_span(area.x + 2, y, &path_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

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

        // === Todo Items ===
        if y < max_y && !self.todos.is_empty() {
            let display_count = if self.expanded {
                self.todos.len()
            } else {
                self.todos.len().min(8)
            };

            for item in self.todos.iter().take(display_count) {
                if y >= max_y {
                    break;
                }

                let checkbox = if item.completed { "☑" } else { "☐" };
                let text_style = if item.completed {
                    Style::default()
                        .fg(colors.text_muted)
                        .add_modifier(Modifier::DIM)
                } else {
                    Style::default().fg(colors.text)
                };

                let todo_span = Span::styled(format!("  {} {}", checkbox, item.text), text_style);
                buf.set_span(area.x + 2, y, &todo_span, area.width.saturating_sub(3));
                y += 1;
            }

            // Show more indicator
            if !self.expanded && self.todos.len() > 8 && y < max_y {
                buf.set_span(
                    area.x + 2,
                    y,
                    &Span::styled(
                        format!("  ... and {} more (click to expand)", self.todos.len() - 8),
                        Style::default()
                            .fg(colors.text_muted)
                            .add_modifier(Modifier::ITALIC),
                    ),
                    area.width.saturating_sub(3),
                );
                y += 1;
            }
        }

        // === Empty ===
        if self.todos.is_empty() && self.status == ToolStatus::Complete && y < max_y {
            let empty_span = Span::styled(
                "  (no todos)",
                Style::default()
                    .fg(colors.text_muted)
                    .add_modifier(Modifier::ITALIC),
            );
            buf.set_span(area.x + 2, y, &empty_span, area.width.saturating_sub(3));
            y += 1;
        }

        // === Status ===
        if y < max_y {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Saving...",
                ToolStatus::Complete => "✓ Saved",
                ToolStatus::Error => "✗ Failed to save",
                ToolStatus::PermissionPending => "⚠ Permission required",
            };
            let status_span = Span::styled(
                status_text,
                Style::default()
                    .fg(border_color)
                    .add_modifier(Modifier::BOLD),
            );
            buf.set_span(area.x + 2, y, &status_span, area.width.saturating_sub(3));
        }
    }

    /// Toggle expanded state.
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}

impl Default for ToolTodoWrite<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ToolTodoWrite<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_todo_write_basic() {
        let todo = ToolTodoWrite::new();
        assert!(todo.todos.is_empty());
    }

    #[test]
    fn test_tool_todo_write_with_items() {
        let todo = ToolTodoWrite::new()
            .add_todo(TodoItem {
                completed: false,
                text: "Buy groceries".to_string(),
            })
            .add_todo(TodoItem {
                completed: true,
                text: "Walk the dog".to_string(),
            });

        assert_eq!(todo.todos.len(), 2);
    }

    #[test]
    fn test_tool_todo_write_status() {
        let pending = ToolTodoWrite::new().status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);
    }
}
