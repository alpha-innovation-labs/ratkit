use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::widgets::theme_picker::{ThemePicker, ThemePickerEvent};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct ThemePickerDemo {
    picker: ThemePicker,
    last_event: String,
}

impl ThemePickerDemo {
    fn new() -> Self {
        let mut picker = ThemePicker::new();
        picker.show();
        Self {
            picker,
            last_event: "Previewing themes".to_string(),
        }
    }
}

impl CoordinatorApp for ThemePickerDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) if keyboard.is_key_down() => {
                use crossterm::event::KeyCode;

                if keyboard.key_code == KeyCode::Char('q') {
                    return Ok(CoordinatorAction::Quit);
                }

                if keyboard.key_code == KeyCode::Char('t') {
                    if self.picker.is_visible() {
                        self.picker.hide();
                    } else {
                        self.picker.show();
                    }
                    return Ok(CoordinatorAction::Redraw);
                }

                if let Some(event) = self.picker.handle_key(&keyboard.key_code) {
                    self.last_event = match event {
                        ThemePickerEvent::Selected(name) => format!("Selected: {}", name),
                        ThemePickerEvent::Cancelled => "Cancelled".to_string(),
                        ThemePickerEvent::PreviewChanged(name) => {
                            format!("Preview: {}", name)
                        }
                    };
                }
                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Continue),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let body = Paragraph::new(vec![
            Line::from("t: toggle picker"),
            Line::from("q: quit"),
            Line::from(self.last_event.clone()),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Theme Picker "),
        );
        frame.render_widget(body, area);
        self.picker.render(frame, area);
    }
}

fn main() -> std::io::Result<()> {
    let app = ThemePickerDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
