use std::io;

use crossterm::event::{MouseButton, MouseEventKind};
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::primitives::button::Button;
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct ButtonDemo {
    button: Button,
    clicks: u32,
}

impl ButtonDemo {
    fn new() -> Self {
        Self {
            button: Button::new("Action"),
            clicks: 0,
        }
    }
}

impl CoordinatorApp for ButtonDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                if keyboard.is_key_down() && keyboard.is_char('q') {
                    Ok(CoordinatorAction::Quit)
                } else {
                    Ok(CoordinatorAction::Redraw)
                }
            }
            CoordinatorEvent::Mouse(mouse) => {
                match mouse.kind {
                    MouseEventKind::Moved => {
                        self.button.update_hover(mouse.column, mouse.row);
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        self.button.update_hover(mouse.column, mouse.row);
                        if self.button.is_clicked(mouse.column, mouse.row) {
                            self.clicks += 1;
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
        let block = Block::default().borders(Borders::ALL).title(" Button ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let button_area = button_demo_area(inner, &self.button);
        self.button.set_area(button_area);
        let button_text = format!(" [{}] ", self.button.text());
        let button_style = if self.button.hovered() {
            self.button.hover()
        } else {
            self.button.normal()
        };
        let button = Paragraph::new(Line::from(Span::styled(button_text, button_style)));
        frame.render_widget(button, button_area);

        let body_area = Rect {
            x: inner.x,
            y: inner.y + 2,
            width: inner.width,
            height: inner.height.saturating_sub(2),
        };
        let body = Paragraph::new(Line::from(format!(
            "Click the button (mouse) or press q to quit. Clicks: {}",
            self.clicks
        )));
        frame.render_widget(body, body_area);
    }
}

fn button_demo_area(inner: Rect, button: &Button) -> Rect {
    let button_width = format!(" [{}] ", button.text()).len() as u16;
    let x = inner.x + 2;
    let y = inner.y + 1;

    Rect {
        x,
        y,
        width: button_width,
        height: 1,
    }
}

fn main() -> io::Result<()> {
    let app = ButtonDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
