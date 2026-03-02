use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::widgets::{HotkeyFooter, HotkeyItem};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct HotkeyFooterDemo;

impl CoordinatorApp for HotkeyFooterDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
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
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let body = Paragraph::new(Line::from("Footer on the bottom. Press q to quit.")).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Hotkey Footer "),
        );
        frame.render_widget(body, chunks[0]);

        let footer = HotkeyFooter::new(vec![
            HotkeyItem::new("q", "quit"),
            HotkeyItem::new("?", "help"),
            HotkeyItem::new("/", "search"),
        ]);
        footer.render(frame, chunks[1]);
    }
}

fn main() -> std::io::Result<()> {
    let app = HotkeyFooterDemo;
    run_with_diagnostics(app, RunnerConfig::default())
}
