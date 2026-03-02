use std::io;

use crossterm::event::KeyCode;
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::primitives::toast::{render_toasts, ToastManager};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct ToastDemo {
    toasts: ToastManager,
}

impl ToastDemo {
    fn new() -> Self {
        Self {
            toasts: ToastManager::new(),
        }
    }
}

impl CoordinatorApp for ToastDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Tick(_) => {
                self.toasts.remove_expired();
                Ok(CoordinatorAction::Redraw)
            }
            CoordinatorEvent::Keyboard(keyboard) => {
                match keyboard.key_code {
                    KeyCode::Char('q') => return Ok(CoordinatorAction::Quit),
                    KeyCode::Char('t') => self.toasts.info("Background task finished"),
                    KeyCode::Char('e') => self.toasts.error("Something went wrong"),
                    KeyCode::Char('c') => self.toasts.clear(),
                    _ => {}
                }
                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let body = Paragraph::new(vec![
            Line::from("t: info toast"),
            Line::from("e: error toast"),
            Line::from("c: clear"),
            Line::from("q: quit"),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Toasts "));
        frame.render_widget(body, area);

        render_toasts(frame, &self.toasts);
    }
}

fn main() -> io::Result<()> {
    let app = ToastDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
