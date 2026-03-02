use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::primitives::widget_event::WidgetEvent;
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct WidgetEventDemo {
    last: WidgetEvent,
}

impl WidgetEventDemo {
    fn new() -> Self {
        Self {
            last: WidgetEvent::None,
        }
    }
}

impl CoordinatorApp for WidgetEventDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                match keyboard.key_code {
                    ratatui::crossterm::event::KeyCode::Char('q') => {
                        return Ok(CoordinatorAction::Quit)
                    }
                    ratatui::crossterm::event::KeyCode::Char('s') => {
                        self.last = WidgetEvent::Selected { path: vec![0, 1] };
                    }
                    ratatui::crossterm::event::KeyCode::Char('t') => {
                        self.last = WidgetEvent::Toggled {
                            path: vec![0, 2],
                            expanded: true,
                        };
                    }
                    ratatui::crossterm::event::KeyCode::Char('f') => {
                        self.last = WidgetEvent::FilterModeChanged {
                            active: true,
                            filter: "name".to_string(),
                        };
                    }
                    _ => {
                        self.last = WidgetEvent::None;
                    }
                }
                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let body = Paragraph::new(vec![
            Line::from("s: Selected"),
            Line::from("t: Toggled"),
            Line::from("f: FilterModeChanged"),
            Line::from(format!("Last: {:?}", self.last)),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" WidgetEvent "),
        );
        frame.render_widget(body, area);
    }
}

fn main() -> std::io::Result<()> {
    let app = WidgetEventDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
