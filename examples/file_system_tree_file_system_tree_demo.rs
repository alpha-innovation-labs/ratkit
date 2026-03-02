use std::io;
use std::path::PathBuf;

use crossterm::event::KeyCode;
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::widgets::file_system_tree::{FileSystemTree, FileSystemTreeState};
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct FileSystemTreeDemo {
    tree: FileSystemTree<'static>,
    state: FileSystemTreeState,
    last_selection: String,
}

impl FileSystemTreeDemo {
    fn new() -> io::Result<Self> {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let tree =
            FileSystemTree::new(root).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let mut state = FileSystemTreeState::new();
        state.select(vec![0]);

        Ok(Self {
            tree,
            state,
            last_selection: "No selection".to_string(),
        })
    }
}

impl CoordinatorApp for FileSystemTreeDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                match keyboard.key_code {
                    KeyCode::Char('q') => return Ok(CoordinatorAction::Quit),
                    KeyCode::Down
                    | KeyCode::Up
                    | KeyCode::Char('j')
                    | KeyCode::Char('k')
                    | KeyCode::Enter
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Char('h')
                    | KeyCode::Char('l') => {
                        let _ = self
                            .tree
                            .handle_navigation_key(keyboard.key_code, &mut self.state);
                    }
                    KeyCode::Char('/') => {
                        if !self.tree.is_filter_mode(&self.state) {
                            self.tree.enter_filter_mode(&mut self.state);
                        }
                    }
                    _ => {
                        if self.tree.is_filter_mode(&self.state) {
                            let _ = self
                                .tree
                                .handle_filter_key(keyboard.key_code, &mut self.state);
                        }
                    }
                }

                if let Some(entry) = self.tree.get_selected_entry(&self.state) {
                    self.last_selection = entry.path.display().to_string();
                }

                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Min(0),
                ratatui::layout::Constraint::Length(3),
            ])
            .split(area);

        let tree = self.tree.clone().block(
            Block::default()
                .borders(Borders::ALL)
                .title(" File System "),
        );
        frame.render_stateful_widget(tree, layout[0], &mut self.state);

        let footer = Paragraph::new(vec![
            Line::from("j/k or Up/Down move, Enter toggle, h/l collapse/expand, / filter"),
            Line::from(format!("Selected: {}", self.last_selection)),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Status "));
        frame.render_widget(footer, layout[1]);
    }
}

fn main() -> std::io::Result<()> {
    let app = FileSystemTreeDemo::new()?;
    run_with_diagnostics(app, RunnerConfig::default())
}
