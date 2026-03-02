use crossterm::event::KeyCode;
use ratatui::{widgets::Block, Frame};
use ratkit::widgets::code_diff::CodeDiff;
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct CodeDiffDemo {
    diff: CodeDiff,
}

impl CodeDiffDemo {
    fn new() -> Self {
        let diff = CodeDiff::from_unified_diff("@@ -1,3 +1,3 @@\n-old line\n+new line\n unchanged")
            .with_file_path("src/lib.rs");
        Self { diff }
    }
}

impl CoordinatorApp for CodeDiffDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) if keyboard.key_code == KeyCode::Char('q') => {
                Ok(CoordinatorAction::Quit)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::default().title(" Code Diff ");
        let inner = block.inner(area);
        frame.render_widget(block, area);
        frame.render_widget(self.diff.clone(), inner);
    }
}

fn main() -> std::io::Result<()> {
    let app = CodeDiffDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
