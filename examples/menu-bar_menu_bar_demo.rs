use crossterm::event::{KeyCode, MouseButton, MouseEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::primitives::menu_bar::{MenuBar, MenuItem};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, MouseEvent,
    RunnerConfig,
};

struct MenuBarDemo {
    menu: MenuBar,
    last_selected: Option<usize>,
}

impl MenuBarDemo {
    fn new() -> Self {
        let menu = MenuBar::new(vec![
            MenuItem::new("File", 0),
            MenuItem::new("Edit", 1),
            MenuItem::new("View", 2),
        ])
        .with_selected(0);

        Self {
            menu,
            last_selected: Some(0),
        }
    }

    fn select_index(&mut self, index: usize) {
        for (i, item) in self.menu.items.iter_mut().enumerate() {
            item.selected = i == index;
        }
        self.last_selected = Some(index);
    }
}

impl CoordinatorApp for MenuBarDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                match keyboard.key_code {
                    KeyCode::Char('q') => return Ok(CoordinatorAction::Quit),
                    KeyCode::Left => {
                        let current = self.menu.selected().unwrap_or(0);
                        let next = current.saturating_sub(1);
                        self.select_index(next);
                    }
                    KeyCode::Right => {
                        let current = self.menu.selected().unwrap_or(0);
                        let next = (current + 1).min(self.menu.items.len().saturating_sub(1));
                        self.select_index(next);
                    }
                    _ => {}
                }
                Ok(CoordinatorAction::Redraw)
            }
            CoordinatorEvent::Mouse(mouse) => {
                match mouse.kind {
                    MouseEventKind::Moved => self.menu.update_hover(mouse.column, mouse.row),
                    MouseEventKind::Down(MouseButton::Left) => {
                        if let Some(index) = self.menu.handle_click(mouse.column, mouse.row) {
                            self.last_selected = Some(index);
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

        self.menu.render(frame, chunks[0]);

        let info = Paragraph::new(Line::from(format!(
            "Selected: {:?}  |  Use mouse or arrows  |  q to quit",
            self.last_selected
        )))
        .block(Block::default().borders(Borders::ALL).title(" Menu Bar "));
        frame.render_widget(info, chunks[1]);
    }
}

fn main() -> std::io::Result<()> {
    let app = MenuBarDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
