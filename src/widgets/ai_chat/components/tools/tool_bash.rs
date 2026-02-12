//! Bash tool ($) display component.
//!
//! This module provides rendering for bash/shell command execution tools.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::Widget,
};

use super::block_tool::BlockTool;
use super::inline_tool::ToolStatus;
use crate::widgets::ai_chat::components::theme::ChatColors;

/// Bash tool display with command, output, and execution status.
///
/// Renders:
/// - Command display with $ prefix
/// - Working directory
/// - Execution spinner during running
/// - Expandable output (>10 lines)
pub struct ToolBash<'a> {
    /// The command to execute
    command: &'a str,
    /// Working directory for the command
    working_dir: Option<&'a str>,
    /// Output from the command
    output: Option<String>,
    /// Exit code of the command
    exit_code: Option<i32>,
    /// Whether the command is currently executing
    executing: bool,
    /// Whether output is expanded
    expanded: bool,
}

impl<'a> ToolBash<'a> {
    /// Create a new ToolBash.
    pub fn new(command: &'a str) -> Self {
        Self {
            command,
            working_dir: None,
            output: None,
            exit_code: None,
            executing: false,
            expanded: false,
        }
    }

    /// Set the working directory.
    pub fn working_dir(mut self, dir: Option<&'a str>) -> Self {
        self.working_dir = dir;
        self
    }

    /// Set the output.
    pub fn output(mut self, output: Option<String>) -> Self {
        self.output = output;
        self
    }

    /// Set the exit code.
    pub fn exit_code(mut self, code: Option<i32>) -> Self {
        self.exit_code = code;
        self
    }

    /// Set executing state.
    pub fn executing(mut self, executing: bool) -> Self {
        self.executing = executing;
        self
    }

    /// Set expanded state.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Get the status based on current state.
    fn get_status(&self) -> ToolStatus {
        if self.executing {
            ToolStatus::Pending
        } else if let Some(code) = self.exit_code {
            if code == 0 {
                ToolStatus::Complete
            } else {
                ToolStatus::Error
            }
        } else {
            ToolStatus::Pending
        }
    }

    /// Render the bash tool to a buffer.
    pub fn render(&self, area: Rect, buf: &mut Buffer, colors: &ChatColors) {
        if area.height < 1 {
            return;
        }

        let max_y = area.y + area.height;
        let mut y = area.y;
        let border_color = if self.executing {
            colors.warning
        } else if let Some(code) = self.exit_code {
            if code == 0 {
                colors.success
            } else {
                colors.error
            }
        } else {
            colors.primary
        };

        // Draw left border
        for y_pos in area.y..max_y {
            buf.get_mut(area.x, y_pos)
                .set_char('│')
                .set_style(Style::default().fg(border_color));
        }

        // === Header ===
        let header = self.render_header(colors);
        buf.set_span(area.x + 2, y, &header, area.width.saturating_sub(3));
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

        // === Command ===
        if y < max_y {
            let cmd_span = Span::styled(
                format!("$ {}", self.command),
                Style::default()
                    .fg(colors.text)
                    .add_modifier(Modifier::BOLD),
            );
            buf.set_span(area.x + 2, y, &cmd_span, area.width.saturating_sub(3));
            y += 1;
        }

        // === Working Directory ===
        if let Some(dir) = self.working_dir {
            if y < max_y {
                let dir_span = Span::styled(
                    format!("📁 {}", dir),
                    Style::default().fg(colors.text_muted),
                );
                buf.set_span(area.x + 2, y, &dir_span, area.width.saturating_sub(3));
                y += 1;
            }
        }

        // === Output ===
        if let Some(output) = &self.output {
            let output_lines: Vec<&str> = output.lines().collect();
            let needs_expansion = output_lines.len() > 10;

            // Show expand/collapse indicator
            if y < max_y {
                let expand_text = if self.expanded {
                    "▼ Output"
                } else if needs_expansion {
                    "▶ Output (click to expand)"
                } else {
                    "▶ Output"
                };
                let expand_span = Span::styled(
                    expand_text,
                    Style::default()
                        .fg(colors.primary)
                        .add_modifier(Modifier::BOLD),
                );
                buf.set_span(area.x + 2, y, &expand_span, area.width.saturating_sub(3));
                y += 1;
            }

            // Render output content
            if self.expanded || !needs_expansion {
                let display_lines = if self.expanded {
                    output_lines.len()
                } else {
                    output_lines.len().min(10)
                };

                for (i, line) in output_lines.iter().take(display_lines).enumerate() {
                    if y >= max_y {
                        break;
                    }

                    let line_text = if line.len() > area.width as usize - 4 {
                        format!("{}...", &line[..area.width as usize - 7])
                    } else {
                        line.to_string()
                    };

                    buf.set_span(
                        area.x + 2,
                        y,
                        &Span::styled(line_text, Style::default().fg(colors.text_muted)),
                        area.width.saturating_sub(3),
                    );
                    y += 1;
                }

                if self.expanded && output_lines.len() > display_lines && y < max_y {
                    buf.set_span(
                        area.x + 2,
                        y,
                        &Span::styled(
                            format!("... {} more lines", output_lines.len() - display_lines),
                            Style::default()
                                .fg(colors.text_muted)
                                .add_modifier(Modifier::ITALIC),
                        ),
                        area.width.saturating_sub(3),
                    );
                    y += 1;
                }
            }
        }

        // === Exit Code ===
        if let Some(code) = self.exit_code {
            if y < max_y {
                let (code_text, code_color) = if code == 0 {
                    ("✓ Success", colors.success)
                } else {
                    (format!("✗ Exit code: {}", code), colors.error)
                };
                let code_span = Span::styled(
                    code_text,
                    Style::default().fg(code_color).add_modifier(Modifier::BOLD),
                );
                buf.set_span(area.x + 2, y, &code_span, area.width.saturating_sub(3));
            }
        }
    }

    /// Render the header with status.
    fn render_header(&self, colors: &ChatColors) -> Span<'static> {
        let icon = '$';
        let status_text = if self.executing {
            let spinner_frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let frame = spinner_frames[(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                / 100) as usize
                % spinner_frames.len()];
            format!("{} Executing...", frame)
        } else if self.exit_code.is_some() {
            "✓ Done".to_string()
        } else {
            "Bash".to_string()
        };

        let style = if self.executing {
            Style::default()
                .fg(colors.warning)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(colors.text)
                .add_modifier(Modifier::BOLD)
        };

        Span::styled(format!("{} {}", icon, status_text), style)
    }

    /// Toggle expanded state.
    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }
}

impl Widget for ToolBash<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let colors = ChatColors::default();
        self.render(area, buf, &colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_bash_basic() {
        let bash = ToolBash::new("ls -la");
        assert_eq!(bash.command, "ls -la");
    }

    #[test]
    fn test_tool_bash_with_options() {
        let bash = ToolBash::new("cargo build")
            .working_dir(Some("/home/user"))
            .output(Some("Compiling...".to_string()))
            .exit_code(Some(0));

        assert!(bash.working_dir.is_some());
        assert!(bash.output.is_some());
    }

    #[test]
    fn test_tool_bash_status() {
        let pending = ToolBash::new("sleep 1").executing(true);
        assert_eq!(pending.get_status(), ToolStatus::Pending);

        let success = ToolBash::new("echo hello").exit_code(Some(0));
        assert_eq!(success.get_status(), ToolStatus::Complete);

        let error = ToolBash::new("false").exit_code(Some(1));
        assert_eq!(error.get_status(), ToolStatus::Error);
    }
}
