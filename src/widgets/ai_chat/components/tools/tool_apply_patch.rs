//! ApplyPatch tool (%) display component.
//!
//! This module provides rendering for patch application tools.

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

/// A single file patch.
#[derive(Debug, Clone, PartialEq)]
pub struct FilePatch {
    /// Path to the file
    pub file_path: String,
    /// Operation type
    pub operation: PatchOperation,
    /// Diff content
    pub diff: String,
}

/// Type of patch operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatchOperation {
    /// File was created
    Created,
    /// File was deleted
    Deleted,
    /// File was moved
    Moved,
    /// File was patched/modified
    Patched,
}

/// Statistics about applied patches.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PatchStats {
    /// Number of files created
    pub created: usize,
    /// Number of files deleted
    pub deleted: usize,
    /// Number of files moved
    pub moved: usize,
    /// Number of files patched
    pub patched: usize,
    /// Number of additions
    pub additions: usize,
    /// Number of deletions
    pub deletions: usize,
}

/// ApplyPatch tool display for patch application.
///
/// Renders:
/// - Per-file diffs
/// - File operation indicators
/// - Diff statistics
pub struct ToolApplyPatch<'a> {
    /// Patches to apply
    patches: Vec<FilePatch>,
    /// Statistics about the patch
    stats: PatchStats,
    /// Status of the operation
    status: ToolStatus,
    /// Whether to show all patches
    expanded: bool,
}

impl<'a> ToolApplyPatch<'a> {
    /// Create a new ToolApplyPatch.
    pub fn new() -> Self {
        Self {
            patches: Vec::new(),
            stats: PatchStats::default(),
            status: ToolStatus::Pending,
            expanded: false,
        }
    }

    /// Add a patch.
    pub fn add_patch(mut self, patch: FilePatch) -> Self {
        match patch.operation {
            PatchOperation::Created => self.stats.created += 1,
            PatchOperation::Deleted => self.stats.deleted += 1,
            PatchOperation::Moved => self.stats.moved += 1,
            PatchOperation::Patched => self.stats.patched += 1,
        }
        self.patches.push(patch);
        self
    }

    /// Set patches.
    pub fn patches(mut self, patches: Vec<FilePatch>) -> Self {
        self.patches = patches;
        self
    }

    /// Set statistics.
    pub fn stats(mut self, stats: PatchStats) -> Self {
        self.stats = stats;
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

    /// Get icon for operation type.
    fn operation_icon(op: &PatchOperation) -> &'static str {
        match op {
            PatchOperation::Created => "✚",
            PatchOperation::Deleted => "✗",
            PatchOperation::Moved => "→",
            PatchOperation::Patched => "◐",
        }
    }

    /// Get color for operation type.
    fn operation_color(op: &PatchOperation, colors: &ChatColors) -> Color {
        match op {
            PatchOperation::Created => colors.success,
            PatchOperation::Deleted => colors.error,
            PatchOperation::Moved => colors.warning,
            PatchOperation::Patched => colors.primary,
        }
    }

    /// Render the applypatch tool to a buffer.
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
        let icon = '%';
        let expand_icon = if self.expanded { "▼" } else { "▶" };
        let patch_count = self.patches.len();
        let header = format!(
            "{} ApplyPatch [{} files] {}",
            icon, patch_count, expand_icon
        );
        let header_span = Span::styled(
            header,
            Style::default()
                .fg(colors.warning)
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
                    "─".repeat(30),
                    Style::default()
                        .fg(border_color)
                        .add_modifier(Modifier::DIM),
                ),
                area.width.saturating_sub(3),
            );
            y += 1;
        }

        // === Stats ===
        if y < max_y {
            let mut stat_parts = Vec::new();
            if self.stats.created > 0 {
                stat_parts.push(format!("{} created", self.stats.created));
            }
            if self.stats.deleted > 0 {
                stat_parts.push(format!("{} deleted", self.stats.deleted));
            }
            if self.stats.moved > 0 {
                stat_parts.push(format!("{} moved", self.stats.moved));
            }
            if self.stats.patched > 0 {
                stat_parts.push(format!("{} patched", self.stats.patched));
            }

            if !stat_parts.is_empty() {
                let stats_text = format!("  📊 {}", stat_parts.join(", "));
                let stats_span = Span::styled(stats_text, Style::default().fg(colors.text_muted));
                buf.set_span(area.x + 2, y, &stats_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

        // === Patches ===
        if y < max_y && !self.patches.is_empty() {
            let display_count = if self.expanded {
                self.patches.len()
            } else {
                self.patches.len().min(5)
            };

            for patch in self.patches.iter().take(display_count) {
                if y >= max_y {
                    break;
                }

                let op_icon = Self::operation_icon(&patch.operation);
                let op_color = Self::operation_color(&patch.operation, colors);

                // File with operation indicator
                let file_span = Span::styled(
                    format!("  {} {}", op_icon, patch.file_path),
                    Style::default().fg(op_color).add_modifier(Modifier::BOLD),
                );
                buf.set_span(area.x + 2, y, &file_span, area.width.saturating_sub(3));
                y += 1;

                // Diff preview (if expanded)
                if self.expanded && !patch.diff.is_empty() && y < max_y {
                    let diff_lines: Vec<&str> = patch.diff.lines().take(3).collect();
                    for line in diff_lines {
                        if y >= max_y {
                            break;
                        }

                        let line_display = if line.len() > area.width as usize - 6 {
                            format!("    {}...", &line[..area.width as usize - 9])
                        } else {
                            format!("    {}", line)
                        };

                        let line_style = if line.starts_with('+') {
                            colors.diff_added_style()
                        } else if line.starts_with('-') {
                            colors.diff_removed_style()
                        } else {
                            Style::default().fg(colors.text_muted)
                        };

                        buf.set_span(
                            area.x + 2,
                            y,
                            &Span::styled(line_display, line_style),
                            area.width.saturating_sub(3),
                        );
                        y += 1;
                    }
                }
            }

            // Show more indicator
            if !self.expanded && self.patches.len() > 5 && y < max_y {
                buf.set_span(
                    area.x + 2,
                    y,
                    &Span::styled(
                        format!(
                            "  ... and {} more (click to expand)",
                            self.patches.len() - 5
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

        // === Status ===
        if y < max_y {
            let status_text = match self.status {
                ToolStatus::Pending => "⏳ Applying patches...",
                ToolStatus::Complete => "✓ Patches applied",
                ToolStatus::Error => "✗ Failed to apply patches",
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

impl Default for ToolApplyPatch<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ToolApplyPatch<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_apply_patch_basic() {
        let patch = ToolApplyPatch::new();
        assert!(patch.patches.is_empty());
    }

    #[test]
    fn test_tool_apply_patch_with_patches() {
        let patch = ToolApplyPatch::new().add_patch(FilePatch {
            file_path: "src/main.rs".to_string(),
            operation: PatchOperation::Created,
            diff: "+ fn main() {}".to_string(),
        });

        assert_eq!(patch.patches.len(), 1);
        assert_eq!(patch.stats.created, 1);
    }

    #[test]
    fn test_tool_apply_patch_status() {
        let pending = ToolApplyPatch::new().status(ToolStatus::Pending);
        assert_eq!(pending.status, ToolStatus::Pending);

        let complete = ToolApplyPatch::new().status(ToolStatus::Complete);
        assert_eq!(complete.status, ToolStatus::Complete);
    }
}
