use crossterm::event::KeyCode;
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::services::hotkey_service::{Hotkey, HotkeyRegistry, HotkeyScope};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct HotkeyServiceDemo {
    registry: HotkeyRegistry,
    scope: HotkeyScope,
}

impl HotkeyServiceDemo {
    fn new() -> Self {
        let mut registry = HotkeyRegistry::new();
        registry.register(Hotkey::new("q", "Quit application").scope(HotkeyScope::Global));
        registry.register(Hotkey::new("j", "Move down").scope(HotkeyScope::Tab("Demo")));
        registry.register(Hotkey::new("k", "Move up").scope(HotkeyScope::Tab("Demo")));

        Self {
            registry,
            scope: HotkeyScope::Tab("Demo"),
        }
    }
}

impl CoordinatorApp for HotkeyServiceDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                if keyboard.key_code == KeyCode::Char('q') {
                    return Ok(CoordinatorAction::Quit);
                }
                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let mut lines = vec![Line::from("Hotkeys for Demo scope:"), Line::from("")];

        for hotkey in self.registry.get_hotkeys() {
            if hotkey.scope != self.scope && hotkey.scope != HotkeyScope::Global {
                continue;
            }
            lines.push(Line::from(format!(
                "{}  - {}",
                hotkey.key, hotkey.description
            )));
        }

        let body = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" HotkeyRegistry "),
        );
        frame.render_widget(body, area);
    }
}

fn main() -> std::io::Result<()> {
    let app = HotkeyServiceDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
