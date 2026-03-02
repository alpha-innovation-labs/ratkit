use std::io;

use crossterm::event::KeyCode;
use ratatui::Frame;
use ratkit::primitives::dialog::{Dialog, DialogWidget};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, KeyboardEvent,
    RunnerConfig,
};

struct DialogDemo {
    dialog: Dialog<'static>,
}

impl DialogDemo {
    fn new() -> Self {
        let dialog = Dialog::confirm("Delete file", "Remove README.md from disk?")
            .footer("Left/Right to change selection")
            .overlay(true)
            .title_inside(true);

        Self { dialog }
    }
}

impl CoordinatorApp for DialogDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                match keyboard.key_code {
                    KeyCode::Char('q') => return Ok(CoordinatorAction::Quit),
                    KeyCode::Left => {
                        if self.dialog.selected_button > 0 {
                            self.dialog.selected_button -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.dialog.selected_button + 1 < self.dialog.buttons.len() {
                            self.dialog.selected_button += 1;
                        }
                    }
                    _ => {}
                }
                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        frame.render_widget(DialogWidget::new(&mut self.dialog), area);
    }
}

fn main() -> io::Result<()> {
    let app = DialogDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
