//! Interactive markdown preview demo with TOC hover and dev bar.
//!
//! Run with:
//! `cargo run --example markdown_preview_markdown_preview_demo --features markdown-preview`

use std::env;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crossterm::event::{
    KeyCode, KeyEvent as CrosstermKeyEvent, KeyEventState, MouseEvent as CrosstermMouseEvent,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
    Frame,
};
use ratkit::prelude::{
    run, CoordinatorAction, CoordinatorApp, CoordinatorEvent, LayoutResult, RunnerConfig,
};
use ratkit::widgets::markdown_preview::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    MarkdownEvent, MarkdownWidget, ScrollState, SelectionState, SourceState, VimState,
};

struct MarkdownPreviewDemo {
    widget: MarkdownWidget<'static>,
    markdown_area: Rect,
    mouse_x: u16,
    mouse_y: u16,
    redraws: u64,
    frames_this_second: u32,
    fps: u16,
    fps_window_start: Instant,
    last_move_processed: Instant,
    toast_message: Option<String>,
    toast_expires_at: Option<Instant>,
}

impl MarkdownPreviewDemo {
    fn new(markdown_content: String, frontmatter_collapsed: bool) -> Self {
        let mut source = SourceState::default();
        source.set_source_string(markdown_content.clone());

        let mut scroll = ScrollState::default();
        scroll.update_total_lines(markdown_content.lines().count().max(1));

        let mut display = DisplaySettings::default();
        display.set_show_document_line_numbers(true);

        let widget = MarkdownWidget::new(
            markdown_content,
            scroll,
            source,
            CacheState::default(),
            display,
            CollapseState::default(),
            ExpandableState::default(),
            GitStatsState::default(),
            VimState::default(),
            SelectionState::default(),
            DoubleClickState::default(),
        )
        .with_has_pane(true)
        .with_frontmatter_collapsed(frontmatter_collapsed)
        .show_toc(true)
        .show_scrollbar(true)
        .show_statusline(true);

        Self {
            widget,
            markdown_area: Rect::default(),
            mouse_x: 0,
            mouse_y: 0,
            redraws: 0,
            frames_this_second: 0,
            fps: 0,
            fps_window_start: Instant::now(),
            last_move_processed: Instant::now(),
            toast_message: None,
            toast_expires_at: None,
        }
    }

    fn show_toast(&mut self, message: impl Into<String>) {
        self.toast_message = Some(message.into());
        self.toast_expires_at = Some(Instant::now() + Duration::from_secs(2));
    }

    fn clear_expired_toast(&mut self) -> bool {
        if let Some(expires_at) = self.toast_expires_at {
            if Instant::now() >= expires_at {
                self.toast_message = None;
                self.toast_expires_at = None;
                return true;
            }
        }
        false
    }

    fn update_fps(&mut self) {
        self.frames_this_second = self.frames_this_second.saturating_add(1);
        let elapsed = self.fps_window_start.elapsed();
        if elapsed >= Duration::from_secs(1) {
            let elapsed_ms = elapsed.as_millis().max(1) as u32;
            self.fps = ((self.frames_this_second.saturating_mul(1000)) / elapsed_ms) as u16;
            self.frames_this_second = 0;
            self.fps_window_start = Instant::now();
        }
    }
}

impl CoordinatorApp for MarkdownPreviewDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(key) => {
                if !key.is_key_down() {
                    return Ok(CoordinatorAction::Continue);
                }

                if key.key_code == KeyCode::Char('q')
                    || (key.key_code == KeyCode::Char('c')
                        && key
                            .modifiers
                            .contains(crossterm::event::KeyModifiers::CONTROL))
                {
                    return Ok(CoordinatorAction::Quit);
                }

                if key.key_code == KeyCode::Char(']') {
                    let toc_visible = self.widget.toggle_toc();
                    self.show_toast(if toc_visible {
                        "TOC enabled"
                    } else {
                        "TOC disabled"
                    });
                    return Ok(CoordinatorAction::Redraw);
                }

                let key_event = CrosstermKeyEvent {
                    code: key.key_code,
                    modifiers: key.modifiers,
                    kind: key.kind,
                    state: KeyEventState::NONE,
                };

                let markdown_event = self.widget.handle_key(key_event);
                let copied_chars = match &markdown_event {
                    MarkdownEvent::Copied { text } => Some(text.chars().count()),
                    _ => None,
                };
                if let Some(copied_chars) = copied_chars {
                    self.show_toast(format!("Copied {} chars to clipboard", copied_chars));
                }
                if matches!(markdown_event, MarkdownEvent::None) {
                    Ok(CoordinatorAction::Continue)
                } else {
                    Ok(CoordinatorAction::Redraw)
                }
            }
            CoordinatorEvent::Mouse(mouse) => {
                let is_moved = matches!(mouse.kind, crossterm::event::MouseEventKind::Moved);
                self.mouse_x = mouse.x();
                self.mouse_y = mouse.y();

                if is_moved {
                    // Coalesce high-frequency motion events to avoid queue backlog.
                    if self.last_move_processed.elapsed() < Duration::from_millis(24) {
                        return Ok(CoordinatorAction::Continue);
                    }
                    self.last_move_processed = Instant::now();
                }

                let mouse_event = CrosstermMouseEvent {
                    kind: mouse.kind,
                    column: mouse.column,
                    row: mouse.row,
                    modifiers: mouse.modifiers,
                };

                let markdown_area = self.markdown_area;
                let markdown_event = self.widget.handle_mouse(mouse_event, markdown_area);
                let copied_chars = match &markdown_event {
                    MarkdownEvent::Copied { text } => Some(text.chars().count()),
                    _ => None,
                };
                if let Some(copied_chars) = copied_chars {
                    self.show_toast(format!("Copied {} chars to clipboard", copied_chars));
                }
                if let Some((_line_number, _line_kind, content)) =
                    self.widget.take_last_double_click()
                {
                    self.show_toast(content);
                }

                if is_moved {
                    if matches!(markdown_event, MarkdownEvent::TocHoverChanged { .. }) {
                        Ok(CoordinatorAction::Redraw)
                    } else {
                        Ok(CoordinatorAction::Continue)
                    }
                } else {
                    Ok(CoordinatorAction::Redraw)
                }
            }
            CoordinatorEvent::Tick(_) => {
                if self.clear_expired_toast() {
                    Ok(CoordinatorAction::Redraw)
                } else {
                    Ok(CoordinatorAction::Continue)
                }
            }
            CoordinatorEvent::Resize(_) => Ok(CoordinatorAction::Redraw),
            _ => Ok(CoordinatorAction::Continue),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        self.redraws = self.redraws.saturating_add(1);
        self.update_fps();

        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);

        let dev_bar_area = chunks[0];
        let markdown_area = chunks[1];
        self.markdown_area = markdown_area;

        let dev_text = format!(
            " DEV | FPS {:>3} | REDRAWS {:>7} | MOUSE {:>4},{:<4} | q quit | ] TOC | wheel scroll | hover TOC | click TOC jump ",
            self.fps, self.redraws, self.mouse_x, self.mouse_y
        );
        frame.render_widget(
            Paragraph::new(Line::from(dev_text))
                .style(Style::default().fg(Color::Black).bg(Color::Cyan)),
            dev_bar_area,
        );

        frame.render_widget(&mut self.widget, markdown_area);

        if let Some(message) = &self.toast_message {
            if markdown_area.height > 1 {
                let toast_width =
                    (message.chars().count() as u16 + 2).min(markdown_area.width.max(1));
                let toast_area = Rect {
                    x: markdown_area.x + markdown_area.width.saturating_sub(toast_width) / 2,
                    y: markdown_area.y + markdown_area.height.saturating_sub(2),
                    width: toast_width,
                    height: 1,
                };
                frame.render_widget(
                    Paragraph::new(Line::from(format!(" {}", message)))
                        .style(Style::default().fg(Color::Black).bg(Color::LightGreen)),
                    toast_area,
                );
            }
        }
    }
}

fn load_demo_markdown() -> io::Result<String> {
    if let Ok(path) = env::var("RATKIT_MD_DEMO_FILE") {
        return std::fs::read_to_string(path);
    }

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("skills");
    path.push("ratkit");
    path.push("SKILL.md");
    std::fs::read_to_string(path)
}

fn main() -> io::Result<()> {
    let frontmatter_collapsed = env::args().any(|arg| arg == "--frontmatter-collapsed");
    let markdown = load_demo_markdown()?;
    let app = MarkdownPreviewDemo::new(markdown, frontmatter_collapsed);
    let config = RunnerConfig {
        tick_rate: Duration::from_millis(250),
        ..RunnerConfig::default()
    };
    run(app, config)
}
