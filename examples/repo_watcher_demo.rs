use std::io;
use std::path::PathBuf;

use crossterm::event::KeyCode;
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::services::repo_watcher::RepoWatcher;
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct RepoWatcherDemo {
    watcher: RepoWatcher,
    last_summary: String,
}

impl RepoWatcherDemo {
    fn new() -> io::Result<Self> {
        let mut watcher =
            RepoWatcher::new().map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        watcher
            .watch(&path)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(Self {
            watcher,
            last_summary: "No repo changes yet".to_string(),
        })
    }
}

impl CoordinatorApp for RepoWatcherDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Tick(_) => {
                if self.watcher.check_for_changes() {
                    let changes = self.watcher.get_change_set();
                    let summary = format!(
                        "A:{} M:{} D:{} R:{} U:{}",
                        changes.added.len(),
                        changes.modified.len(),
                        changes.deleted.len(),
                        changes.renamed.len(),
                        changes.untracked.len()
                    );
                    self.last_summary = summary;
                    Ok(CoordinatorAction::Redraw)
                } else {
                    Ok(CoordinatorAction::Continue)
                }
            }
            CoordinatorEvent::Keyboard(keyboard) if keyboard.key_code == KeyCode::Char('q') => {
                Ok(CoordinatorAction::Quit)
            }
            _ => Ok(CoordinatorAction::Redraw),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let body = Paragraph::new(vec![
            Line::from("Watching repo changes"),
            Line::from(self.last_summary.clone()),
            Line::from("Press q to quit"),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" RepoWatcher "),
        );
        frame.render_widget(body, area);
    }
}

fn main() -> io::Result<()> {
    let app = RepoWatcherDemo::new()?;
    run_with_diagnostics(app, RunnerConfig::default())
}
