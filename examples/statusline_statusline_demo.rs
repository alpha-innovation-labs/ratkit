use std::io;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::primitives::statusline::{OperationalMode, StyledStatusLine};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct StatusLineDemo {
    renders: usize,
    events: usize,
}

impl CoordinatorApp for StatusLineDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        self.events += 1;
        match event {
            CoordinatorEvent::Keyboard(keyboard)
                if keyboard.key_code == crossterm::event::KeyCode::Char('q') =>
            {
                Ok(CoordinatorAction::Quit)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        self.renders += 1;
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let body = Paragraph::new(Line::from("Statusline renders at the bottom."))
            .block(Block::default().borders(Borders::ALL).title(" Statusline "));
        frame.render_widget(body, chunks[0]);

        let status = StyledStatusLine::new()
            .title(" RATKIT ")
            .mode(OperationalMode::Operational)
            .center_text("Runner loop active")
            .render_metrics(self.renders, 120)
            .event_metrics(self.events, 60)
            .message_count(self.events as u32)
            .build();

        frame.render_widget(status, chunks[1]);
    }
}

fn main() -> io::Result<()> {
    let app = StatusLineDemo {
        renders: 0,
        events: 0,
    };
    run_with_diagnostics(app, RunnerConfig::default())
}
