use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::Result;
use crossterm::{cursor::SetCursorStyle, event::KeyCode, event::KeyModifiers, execute};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;
use ratkit::primitives::termtui::{render_screen, CursorStyle, Parser, VtEvent};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, KeyboardEvent,
    RedrawSignal, ResizeEvent, RunnerConfig,
};

struct TermMprocsTerminal {
    parser: Arc<Mutex<Parser>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    redraw_signal: RedrawSignal,
    _child: Box<dyn Child + Send + Sync>,
}

impl TermMprocsTerminal {
    fn spawn_shell(rows: u16, cols: u16) -> Result<Self> {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string());
        Self::spawn_with_command(&shell, &[], rows, cols)
    }

    fn spawn_with_command(command: &str, args: &[&str], rows: u16, cols: u16) -> Result<Self> {
        let pty_system = native_pty_system();
        let pty_size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };
        let pair = pty_system.openpty(pty_size)?;

        let mut cmd = CommandBuilder::new(command);
        for arg in args {
            cmd.arg(arg);
        }
        cmd.env("TERM", "xterm-256color");
        cmd.cwd(std::env::current_dir()?);

        let child = pair.slave.spawn_command(cmd)?;

        let mut master = pair.master;
        let mut reader = master.try_clone_reader()?;
        let writer = master.take_writer()?;

        let writer = Arc::new(Mutex::new(writer));
        let parser = Arc::new(Mutex::new(Parser::new(rows, cols, 10000)));
        let redraw_signal = RedrawSignal::new();
        redraw_signal.request_redraw();

        let parser_clone = Arc::clone(&parser);
        let writer_clone = Arc::clone(&writer);
        let redraw_signal_clone = redraw_signal.clone();
        thread::spawn(move || {
            let mut buf = [0u8; 8192];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let mut events = Vec::new();
                        if let Ok(mut parser) = parser_clone.lock() {
                            parser.screen.process(&buf[..n], &mut events);
                            redraw_signal_clone.request_redraw();
                        }
                        if !events.is_empty() {
                            if let Ok(mut writer) = writer_clone.lock() {
                                for event in events {
                                    if let VtEvent::Reply(reply) = event {
                                        let _ = writer.write_all(reply.as_bytes());
                                        let _ = writer.flush();
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self {
            parser,
            writer,
            master: Arc::new(Mutex::new(master)),
            redraw_signal,
            _child: child,
        })
    }

    fn resize(&mut self, rows: u16, cols: u16) {
        if rows == 0 || cols == 0 {
            return;
        }
        if let Ok(mut parser) = self.parser.lock() {
            parser.set_size(rows, cols);
        }
        if let Ok(mut master) = self.master.lock() {
            let _ = master.resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            });
        }
        self.redraw_signal.request_redraw();
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        if let Ok(parser) = self.parser.lock() {
            render_screen(parser.screen(), area, frame.buffer_mut());
        }
    }

    fn write_input(&self, bytes: &[u8]) {
        if let Ok(mut writer) = self.writer.lock() {
            let _ = writer.write_all(bytes);
            let _ = writer.flush();
        }
    }

    fn take_needs_redraw(&self) -> bool {
        self.redraw_signal.take_redraw_request()
    }
}

struct TermMprocsDemo {
    terminal: TermMprocsTerminal,
    last_area: Rect,
    terminal_focused: bool,
}

impl TermMprocsDemo {
    fn new() -> Result<Self> {
        Ok(Self {
            terminal: TermMprocsTerminal::spawn_shell(24, 80)?,
            last_area: Rect::default(),
            terminal_focused: true,
        })
    }
}

impl CoordinatorApp for TermMprocsDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Resize(ResizeEvent { width, height }) => {
                self.terminal.resize(height, width);
                Ok(CoordinatorAction::Redraw)
            }
            CoordinatorEvent::Keyboard(keyboard) => {
                if !keyboard.is_key_down() {
                    return Ok(CoordinatorAction::Continue);
                }
                if keyboard.is_char('x') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
                    if self.terminal_focused {
                        self.terminal_focused = false;
                        return Ok(CoordinatorAction::Redraw);
                    }
                    return Ok(CoordinatorAction::Continue);
                }
                if keyboard.is_char('q') && keyboard.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(CoordinatorAction::Quit);
                }
                if !self.terminal_focused {
                    if keyboard.is_char('q') && keyboard.modifiers.is_empty() {
                        return Ok(CoordinatorAction::Quit);
                    }
                    return Ok(CoordinatorAction::Continue);
                }
                if let Some(bytes) = encode_key_event(&keyboard) {
                    self.terminal.write_input(&bytes);
                }
                Ok(CoordinatorAction::Redraw)
            }
            CoordinatorEvent::Mouse(mouse) => {
                if mouse.is_click() && mouse.is_inside(self.last_area) && !self.terminal_focused {
                    self.terminal_focused = true;
                    return Ok(CoordinatorAction::Redraw);
                }
                Ok(CoordinatorAction::Continue)
            }
            CoordinatorEvent::Tick(_) => {
                if self.terminal.take_needs_redraw() {
                    Ok(CoordinatorAction::Redraw)
                } else {
                    Ok(CoordinatorAction::Continue)
                }
            }
            _ => Ok(CoordinatorAction::Continue),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let title = if self.terminal_focused {
            " termtui demo (terminal focused) "
        } else {
            " termtui demo (wrapper focused) "
        };
        let border_style = if self.terminal_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner != self.last_area {
            self.terminal.resize(inner.height, inner.width);
            self.last_area = inner;
        }

        self.terminal.render(frame, inner);

        if self.terminal_focused {
            if let Ok(parser) = self.terminal.parser.lock() {
                let screen = parser.screen();
                if !screen.hide_cursor() {
                    let (row, col) = screen.cursor_position();
                    let x = inner.x.saturating_add(col);
                    let y = inner.y.saturating_add(row);
                    if x < inner.x + inner.width && y < inner.y + inner.height {
                        frame.set_cursor_position((x, y));
                        apply_cursor_style(screen.cursor_style());
                    }
                }
            }
        }
    }
}

fn apply_cursor_style(style: CursorStyle) {
    let style = match style {
        CursorStyle::Default => SetCursorStyle::DefaultUserShape,
        CursorStyle::BlinkingBlock => SetCursorStyle::BlinkingBlock,
        CursorStyle::SteadyBlock => SetCursorStyle::SteadyBlock,
        CursorStyle::BlinkingUnderline => SetCursorStyle::BlinkingUnderScore,
        CursorStyle::SteadyUnderline => SetCursorStyle::SteadyUnderScore,
        CursorStyle::BlinkingBar => SetCursorStyle::BlinkingBar,
        CursorStyle::SteadyBar => SetCursorStyle::SteadyBar,
    };
    let _ = execute!(std::io::stdout(), style);
}

fn encode_key_event(event: &KeyboardEvent) -> Option<Vec<u8>> {
    if !event.is_key_down() {
        return None;
    }
    match event.key_code {
        KeyCode::Char(mut c) => {
            if event.modifiers.contains(KeyModifiers::CONTROL) {
                if c == 'x' {
                    return None;
                }
                c = c.to_ascii_lowercase();
                if c.is_ascii() {
                    return Some(vec![(c as u8) & 0x1f]);
                }
                return None;
            }
            if event.modifiers.contains(KeyModifiers::ALT) {
                if c.is_ascii() {
                    return Some(vec![0x1b, c as u8]);
                }
                return None;
            }
            if c.is_ascii() {
                Some(vec![c as u8])
            } else {
                Some(c.to_string().into_bytes())
            }
        }
        KeyCode::Enter => Some(vec![b'\r']),
        KeyCode::Backspace => Some(vec![0x7f]),
        KeyCode::Tab => Some(vec![b'\t']),
        KeyCode::Esc => Some(vec![0x1b]),
        KeyCode::Up => Some(b"\x1b[A".to_vec()),
        KeyCode::Down => Some(b"\x1b[B".to_vec()),
        KeyCode::Right => Some(b"\x1b[C".to_vec()),
        KeyCode::Left => Some(b"\x1b[D".to_vec()),
        _ => None,
    }
}

fn main() -> Result<()> {
    let app = TermMprocsDemo::new()?;
    run_with_diagnostics(app, RunnerConfig::default())?;
    Ok(())
}
