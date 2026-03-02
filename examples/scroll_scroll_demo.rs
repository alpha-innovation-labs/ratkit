use std::io;

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::primitives::scroll::calculate_scroll_offset;
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, KeyboardEvent,
    RunnerConfig,
};

struct ScrollDemo {
    selected: usize,
    items: Vec<String>,
}

impl ScrollDemo {
    fn new() -> Self {
        let items = (1..=50).map(|i| format!("Item {}", i)).collect();
        Self { selected: 0, items }
    }
}

impl CoordinatorApp for ScrollDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                match keyboard.key_code {
                    KeyCode::Char('q') => return Ok(CoordinatorAction::Quit),
                    KeyCode::Up => {
                        if self.selected > 0 {
                            self.selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.selected + 1 < self.items.len() {
                            self.selected += 1;
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
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let header = Paragraph::new(Line::from("Scroll offset demo (Up/Down, q to quit)"))
            .block(Block::default().borders(Borders::ALL).title(" Header "));
        frame.render_widget(header, chunks[0]);

        let visible_count = chunks[1].height.saturating_sub(2) as usize;
        let offset = calculate_scroll_offset(self.selected, visible_count.max(1), self.items.len());

        let mut lines = Vec::new();
        for (idx, item) in self
            .items
            .iter()
            .enumerate()
            .skip(offset)
            .take(visible_count)
        {
            if idx == self.selected {
                lines.push(Line::from(format!("> {}", item)));
            } else {
                lines.push(Line::from(format!("  {}", item)));
            }
        }

        let body =
            Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Items "));
        frame.render_widget(body, chunks[1]);
    }
}

fn main() -> io::Result<()> {
    let app = ScrollDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
