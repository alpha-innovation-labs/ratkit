use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders},
    Frame,
};
use ratkit::primitives::tree_view::{TreeNavigator, TreeNode, TreeView, TreeViewState};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct TreeViewDemo {
    nodes: Vec<TreeNode<&'static str>>,
    state: TreeViewState,
    navigator: TreeNavigator,
}

impl TreeViewDemo {
    fn new() -> Self {
        let nodes = vec![
            TreeNode::with_children(
                "src",
                vec![
                    TreeNode::new("lib.rs"),
                    TreeNode::new("main.rs"),
                    TreeNode::with_children(
                        "widgets",
                        vec![TreeNode::new("button.rs"), TreeNode::new("dialog.rs")],
                    ),
                ],
            ),
            TreeNode::with_children("tests", vec![TreeNode::new("smoke.rs")]),
        ];

        Self {
            nodes,
            state: TreeViewState::new(),
            navigator: TreeNavigator::new(),
        }
    }

    fn build_tree(&self) -> TreeView<'static, &'static str> {
        TreeView::new(self.nodes.clone())
            .render_fn(|data, state| {
                if state.is_selected {
                    Line::from(format!("> {}", data))
                } else {
                    Line::from(*data)
                }
            })
            .highlight_style(Style::default().bg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title(" Tree View "))
            .with_filter_ui(true)
    }
}

impl CoordinatorApp for TreeViewDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                let key = keyboard.key_code;
                match key {
                    KeyCode::Char('q') => return Ok(CoordinatorAction::Quit),
                    KeyCode::Char('/') => {
                        self.state.enter_filter_mode();
                    }
                    _ => {
                        let mut tree = self.build_tree();
                        let key_event = crossterm::event::KeyEvent {
                            code: key,
                            modifiers: keyboard.modifiers,
                            kind: keyboard.kind,
                            state: crossterm::event::KeyEventState::empty(),
                        };
                        let _ = tree.handle_key_event(key_event, &self.navigator, &mut self.state);
                    }
                }
                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area: Rect = frame.area();
        let tree = self.build_tree();
        frame.render_stateful_widget(tree, area, &mut self.state);
    }
}

fn main() -> std::io::Result<()> {
    let app = TreeViewDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}
