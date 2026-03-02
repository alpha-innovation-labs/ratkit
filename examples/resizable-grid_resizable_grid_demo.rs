use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;
use ratkit::primitives::resizable_grid::{
    ResizableGrid, ResizableGridWidget, ResizableGridWidgetState,
};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct ResizableGridDemo {
    layout: ResizableGrid,
    state: ResizableGridWidgetState,
    last_area: Rect,
}

impl ResizableGridDemo {
    fn new() -> Self {
        let mut layout = ResizableGrid::new(0);
        let bottom_pane = layout.split_pane_horizontally(0).unwrap_or(0);
        let _top_right = layout.split_pane_vertically(0).unwrap_or(0);
        let _bottom_right = layout.split_pane_vertically(bottom_pane).unwrap_or(0);
        let _ = layout.resize_divider(0, 60);
        let _ = layout.resize_divider(bottom_pane, 40);

        Self {
            layout,
            state: ResizableGridWidgetState::default(),
            last_area: Rect::default(),
        }
    }

    fn handle_mouse(&mut self, mouse: ratkit::MouseEvent) {
        let crossterm_mouse = crossterm::event::MouseEvent {
            kind: mouse.kind,
            column: mouse.column,
            row: mouse.row,
            modifiers: mouse.modifiers,
        };
        let mut widget = ResizableGridWidget::new(self.layout.clone()).with_state(self.state);
        widget.handle_mouse(crossterm_mouse, self.last_area);
        self.state = widget.state();
        self.layout = widget.layout().clone();
    }
}

impl CoordinatorApp for ResizableGridDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard)
                if keyboard.key_code == crossterm::event::KeyCode::Char('q') =>
            {
                Ok(CoordinatorAction::Quit)
            }
            CoordinatorEvent::Mouse(mouse) => {
                self.handle_mouse(mouse);
                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Resizable Grid ");
        let inner = block.inner(area);
        self.last_area = inner;
        frame.render_widget(block, area);

        let widget = ResizableGridWidget::new(self.layout.clone()).with_state(self.state);
        self.state = widget.state();
        self.layout = widget.layout().clone();
        frame.render_widget(widget, inner);
    }
}

fn main() -> std::io::Result<()> {
    let app = ResizableGridDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
