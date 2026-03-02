use std::io;
use std::path::PathBuf;

use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::services::git_watcher::GitWatcher;
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct GitWatcherDemo {
    watcher: GitWatcher,
    last_change: String,
}

impl GitWatcherDemo {
    fn new() -> io::Result<Self> {
        let mut watcher =
            GitWatcher::new().map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        watcher
            .watch(&path)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(Self {
            watcher,
            last_change: "No git changes yet".to_string(),
        })
    }
}

impl CoordinatorApp for GitWatcherDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Tick(_) => {
                if self.watcher.check_for_changes() {
                    self.last_change = "Git state changed".to_string();
                    Ok(CoordinatorAction::Redraw)
                } else {
                    Ok(CoordinatorAction::Continue)
                }
            }
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
        let body = Paragraph::new(vec![
            Line::from("Watching git directory"),
            Line::from(self.last_change.clone()),
            Line::from("Press q to quit"),
        ])
        .block(Block::default().borders(Borders::ALL).title(" GitWatcher "));
        frame.render_widget(body, area);
    }
}

fn main() -> std::io::Result<()> {
    let app = GitWatcherDemo::new()?;
    run_with_diagnostics(app, RunnerConfig::default())
}
