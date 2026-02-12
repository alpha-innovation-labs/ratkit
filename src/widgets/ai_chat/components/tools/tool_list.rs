//! List tool (→) display component.
//!
//! This module provides rendering for directory listing tools.

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

/// List tool display for directory listing.
///
/// Renders:
/// - Directory path
/// - List of entries
pub struct ToolList<'a> {
    /// Path being listed
    path: &'a str,
    /// Entries in the directory
    entries: Vec<DirectoryEntry>,
    /// Status of the operation
    status: ToolStatus,
    /// Whether to show all entries
    expanded: bool,
}

/// A single directory entry.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectoryEntry {
    /// Name of the entry
    pub name: String,
    /// Whether it's a directory
    pub is_dir: bool,
    /// File size (if not a directory)
    pub size: Option<u64>,
}

impl<'a> ToolList<'a> {
    /// Create a new ToolList.
    pub fn new(path: &'a str) -> Self {
        Self {
            path,
            entries: Vec::new(),
            status: ToolStatus::Pending,
            expanded: false,
        }
    }

    /// Add an entry.
    pub fn add_entry(mut self, entry: DirectoryEntry) -> Self {
        self.entries.push(entry);
        self
    }

    /// Set entries.
    pub fn entries(mut self, entries: Vec<DirectoryEntry>) -> Self {
        self.entries = entries;
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

    /// Get border color based on status.
    fn border_color(&self, colors: &ChatColors) -> Color {
        match self.status {
            ToolStatus::Pending => colors.warning,
            ToolStatus::Complete => colors.success,
            ToolStatus::Error => colors.error,
            ToolStatus::PermissionPending => Color::Rgb(255, 165, 0),
        }
    }

    /// Render the list tool to a buffer.
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
        let expand_icon = if self.expanded { "▼" } else { "▶" };
        let entry_count = self.entries.len();
        let count_text = if self.status == ToolStatus::Complete {
            format!(" ({} items)", entry_count)
        } else {
            String::new()
        };
        let header = format!("{} List {} {}{}", icon, self.path, expand_icon, count_text);
        let header_span = Span::styled(
            header,
            Style::default()
                .fg(colors.text)
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

        // === Entries ===
        if y < max_y {
            let display_count = if self.expanded {
                self.entries.len()
            } else {
                self.entries.len().min(10)
            };

            for entry in self.entries.iter().take(display_count) {
                if y >= max_y {
                    break;
                }

                // Icon based on type
                let icon = if entry.is_dir { "📁" } else { "📄" };

                // Size for files
                let size_str = if let Some(size) = entry.size {
                    format!(" ({})", Self::format_size(size))
                } else {
                    String::new()
                };

                let entry_span = Span::styled(
                    format!("  {} {}{}", icon, entry.name, size_str),
                    if entry.is_dir {
                        Style::default().fg(colors.primary)
                    } else {
                        Style::default().fg(colors.text_muted)
                    },
                );
                buf.set_span(area.x + 2, y, &entry_span, area.width.saturating_sub(3));
                y += 1;
            }

            // Show more indicator
            if !self.expanded && self.entries.len() > 10 && y < max_y {
                buf.set_span(
                    area.x + 2,
                    y,
                    &Span::styled(
                        format!(
                            "  ... and {} more (click to expand)",
                            self.entries.len() - 10
                        ),
                        Style::default()
                            .fg(colors.text_muted)
                            .add_modifier(Modifier::ITALIC),
                    ),
                    area.width.saturating_sub(3),
                );
                y += 1;
            }
        }

        // === Empty directory ===
        if self.entries.is_empty() && self.status == ToolStatus::Complete && y < max_y {
            let empty_span = Span::styled(
                "  (empty directory)",
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
                ToolStatus::Pending => "⏳ Listing...",
                ToolStatus::Complete => "✓ Listing complete",
                ToolStatus::Error => "✗ Listing failed",
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

    /// Format file size in human-readable form.
    fn format_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit_idx = 0;

        while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }

        format!("{:.1}{}", size, UNITS[unit_idx])
    }

    /// Toggle expanded state.
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}

impl Widget for ToolList<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_list_basic() {
        let list = ToolList::new("/home/user");
        assert_eq!(list.path, "/home/user");
    }

    #[test]
    fn test_tool_list_with_entries() {
        let list = ToolList::new("/test")
            .add_entry(DirectoryEntry {
                name: "file.txt".to_string(),
                is_dir: false,
                size: Some(1024),
            })
            .add_entry(DirectoryEntry {
                name: "subdir".to_string(),
                is_dir: true,
                size: None,
            });

        assert_eq!(list.entries.len(), 2);
    }

    #[test]
    fn test_tool_list_status() {
        let pending = ToolList::new("/test").status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);
    }
}
