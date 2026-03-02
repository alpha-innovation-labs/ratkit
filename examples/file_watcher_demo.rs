use std::io;
use std::path::PathBuf;

use crossterm::event::KeyCode;
use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratkit::services::file_watcher::FileWatcher;
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

struct FileWatcherDemo {
    watcher: FileWatcher,
    last_change: String,
}

impl FileWatcherDemo {
    fn new() -> io::Result<Self> {
        let mut watcher = FileWatcher::for_directory()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        watcher
            .watch(&path)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

        Ok(Self {
            watcher,
            last_change: "No changes yet".to_string(),
        })
    }
}

impl CoordinatorApp for FileWatcherDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Tick(_) => {
                if self.watcher.check_for_changes() {
                    let paths = self.watcher.get_changed_paths();
                    if let Some(path) = paths.first() {
                        self.last_change = format!("Changed: {}", path.display());
                    } else {
                        self.last_change = "Changes detected".to_string();
                    }
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
            Line::from("Watching current directory"),
            Line::from(self.last_change.clone()),
            Line::from("Press q to quit"),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" FileWatcher "),
        );
        frame.render_widget(body, area);
    }
}

fn main() -> io::Result<()> {
    let app = FileWatcherDemo::new()?;
    run_with_diagnostics(app, RunnerConfig::default())
}
