use ratatui::{
    style::{Color, Style},
    text::Line,
    Frame,
};
use ratkit::primitives::pane::Pane;
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct PaneDemo {
    ticks: u64,
}

impl CoordinatorApp for PaneDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Tick(_) => {
                self.ticks += 1;
                Ok(CoordinatorAction::Redraw)
            }
            CoordinatorEvent::Keyboard(keyboard)
                if keyboard.key_code == crossterm::event::KeyCode::Char('q') =>
            {
                Ok(CoordinatorAction::Quit)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let pane = Pane::new("Pane")
            .with_icon("■")
            .with_uniform_padding(1)
            .border_style(Style::default().fg(Color::Cyan));

        pane.render_paragraph(
            frame,
            area,
            vec![
                Line::from("Minimal pane demo"),
                Line::from(format!("Ticks: {}", self.ticks)),
                Line::from("Press q to quit"),
            ],
        );
    }
}

fn main() -> std::io::Result<()> {
    let app = PaneDemo { ticks: 0 };
    run_with_diagnostics(app, RunnerConfig::default())
}
