use std::fs;
use std::path::Path;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Widget};

use crate::widgets::file_system_tree::config::FileSystemTreeConfig;
use crate::widgets::file_system_tree::entry::FileSystemEntry;
use crate::widgets::file_system_tree::state::FileSystemTreeState;
use crate::widgets::file_system_tree::tree_node::FileSystemTreeNode;
use devicons::{icon_for_file, Theme as DevIconTheme};

fn get_ayu_dark_color(filename: &str) -> Color {
    let lower = filename.to_lowercase();

    if lower.ends_with(".sh")
        || lower.ends_with(".bash")
        || lower.ends_with(".zsh")
        || lower.ends_with(".fish")
        || lower.ends_with(".py")
        || lower.ends_with(".rb")
    {
        return Color::Rgb(126, 147, 80);
    }

    if lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".gif")
        || lower.ends_with(".svg")
        || lower.ends_with(".ico")
        || lower.ends_with(".webp")
        || lower.ends_with(".bmp")
    {
        return Color::Rgb(194, 160, 92);
    }

    if lower.ends_with(".mp3")
        || lower.ends_with(".mp4")
        || lower.ends_with(".wav")
        || lower.ends_with(".avi")
        || lower.ends_with(".mkv")
        || lower.ends_with(".flac")
        || lower.ends_with(".ogg")
        || lower.ends_with(".webm")
    {
        return Color::Rgb(126, 147, 80);
    }

    if lower.ends_with(".zip")
        || lower.ends_with(".tar")
        || lower.ends_with(".gz")
        || lower.ends_with(".bz2")
        || lower.ends_with(".xz")
        || lower.ends_with(".7z")
        || lower.ends_with(".rar")
    {
        return Color::Rgb(168, 83, 97);
    }

    if lower.ends_with(".pdf")
        || lower.ends_with(".doc")
        || lower.ends_with(".docx")
        || lower.ends_with(".rtf")
        || lower.ends_with(".odt")
    {
        return Color::Rgb(31, 111, 136);
    }

    if lower.ends_with(".json")
        || lower.ends_with(".js")
        || lower.ends_with(".ts")
        || lower.ends_with(".jsx")
        || lower.ends_with(".tsx")
    {
        return Color::Rgb(194, 160, 92);
    }

    if lower.ends_with(".yml") || lower.ends_with(".yaml") {
        return Color::Rgb(31, 111, 136);
    }

    if lower.ends_with(".toml") {
        return Color::Rgb(148, 100, 182);
    }

    if lower.ends_with(".rs") {
        return Color::Rgb(194, 160, 92);
    }

    if lower.ends_with(".c")
        || lower.ends_with(".cpp")
        || lower.ends_with(".h")
        || lower.ends_with(".hpp")
    {
        return Color::Rgb(31, 111, 136);
    }

    if lower.ends_with(".go") {
        return Color::Rgb(31, 111, 136);
    }

    if lower.ends_with(".md") || lower.ends_with(".txt") || lower.ends_with(".log") {
        return Color::Rgb(230, 225, 207);
    }

    Color::Rgb(230, 225, 207)
}

fn get_custom_icon(filename: &str) -> Option<(char, Color)> {
    let lower = filename.to_lowercase();

    if lower.ends_with(".just") || lower == "justfile" || lower == ".justfile" {
        return Some(('\u{e779}', Color::Rgb(194, 160, 92)));
    }

    if lower == "makefile" || lower.starts_with("makefile.") || lower == "gnumakefile" {
        return Some(('\u{e779}', Color::Rgb(109, 128, 134)));
    }

    if lower == "gemfile" || lower == "gemfile.lock" {
        return Some(('\u{e21e}', Color::Rgb(112, 21, 22)));
    }

    if lower == ".env" || lower.starts_with(".env.") {
        return Some(('\u{f462}', Color::Rgb(251, 192, 45)));
    }

    if lower == "license"
        || lower == "license.txt"
        || lower == "license.md"
        || lower == "licence"
        || lower == "licence.txt"
        || lower == "copying"
    {
        return Some(('\u{f48a}', Color::Rgb(216, 187, 98)));
    }

    if lower == "jenkinsfile" || lower.starts_with("jenkinsfile.") {
        return Some(('\u{e767}', Color::Rgb(217, 69, 57)));
    }

    if lower == ".ds_store" {
        return Some(('\u{f179}', Color::Rgb(126, 142, 168)));
    }

    None
}

#[derive(Clone)]
pub struct FileSystemTree<'a> {
    pub root_path: std::path::PathBuf,
    pub nodes: Vec<FileSystemTreeNode>,
    pub config: FileSystemTreeConfig,
    pub block: Option<Block<'a>>,
}

impl<'a> FileSystemTree<'a> {
    pub fn new(root_path: std::path::PathBuf) -> std::io::Result<Self> {
        let config = FileSystemTreeConfig::default();
        let root_entry = FileSystemEntry::new(root_path.clone())?;
        let root_children = if root_entry.is_dir {
            Self::load_directory(&root_path, &config)?
        } else {
            Vec::new()
        };
        let nodes = vec![FileSystemTreeNode {
            data: root_entry,
            children: root_children,
            expandable: root_path.is_dir(),
        }];

        Ok(Self {
            root_path,
            nodes,
            config,
            block: None,
        })
    }

    pub fn with_config(
        root_path: std::path::PathBuf,
        config: FileSystemTreeConfig,
    ) -> std::io::Result<Self> {
        let root_entry = FileSystemEntry::new(root_path.clone())?;
        let root_children = if root_entry.is_dir {
            Self::load_directory(&root_path, &config)?
        } else {
            Vec::new()
        };
        let nodes = vec![FileSystemTreeNode {
            data: root_entry,
            children: root_children,
            expandable: root_path.is_dir(),
        }];

        Ok(Self {
            root_path,
            nodes,
            config,
            block: None,
        })
    }

    fn load_directory(
        path: &Path,
        config: &FileSystemTreeConfig,
    ) -> std::io::Result<Vec<FileSystemTreeNode>> {
        let mut entries = Vec::new();

        let read_dir = fs::read_dir(path)?;

        for entry in read_dir {
            let entry = entry?;
            let path = entry.path();

            let fs_entry = FileSystemEntry::new(path.clone())?;

            if fs_entry.is_hidden && !config.show_hidden {
                continue;
            }

            let node = if fs_entry.is_dir {
                FileSystemTreeNode {
                    data: fs_entry,
                    children: Vec::new(),
                    expandable: true,
                }
            } else {
                FileSystemTreeNode::new(fs_entry)
            };

            entries.push(node);
        }

        entries.sort_by(|a, b| match (a.data.is_dir, b.data.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.data.name.to_lowercase().cmp(&b.data.name.to_lowercase()),
        });

        Ok(entries)
    }

    pub fn expand_directory(&mut self, path: &[usize]) -> std::io::Result<()> {
        fn find_and_expand(
            nodes: &mut [FileSystemTreeNode],
            path: &[usize],
            config: &FileSystemTreeConfig,
        ) -> std::io::Result<()> {
            if path.is_empty() {
                return Ok(());
            }

            if path.len() == 1 {
                if let Some(node) = nodes.get_mut(path[0]) {
                    if node.data.is_dir && node.children.is_empty() {
                        node.children = FileSystemTree::load_directory(&node.data.path, config)?;
                    }
                }
                return Ok(());
            }

            if let Some(node) = nodes.get_mut(path[0]) {
                find_and_expand(&mut node.children, &path[1..], config)?;
            }

            Ok(())
        }

        find_and_expand(&mut self.nodes, path, &self.config)
    }

    pub fn get_entry_at_path(&self, path: &[usize]) -> Option<&FileSystemEntry> {
        fn find_entry<'a>(
            nodes: &'a [FileSystemTreeNode],
            path: &[usize],
        ) -> Option<&'a FileSystemEntry> {
            if path.is_empty() {
                return None;
            }

            if let Some(node) = nodes.get(path[0]) {
                if path.len() == 1 {
                    return Some(&node.data);
                }
                return find_entry(&node.children, &path[1..]);
            }
            None
        }

        find_entry(&self.nodes, path)
    }

    pub fn get_selected_entry(&self, state: &FileSystemTreeState) -> Option<&FileSystemEntry> {
        state
            .selected_path
            .as_ref()
            .and_then(|path| self.get_entry_at_path(path))
    }

    pub fn get_visible_paths(&self, state: &FileSystemTreeState) -> Vec<Vec<usize>> {
        let mut paths = Vec::new();

        fn traverse(
            nodes: &[FileSystemTreeNode],
            current_path: Vec<usize>,
            state: &FileSystemTreeState,
            paths: &mut Vec<Vec<usize>>,
        ) {
            for (idx, node) in nodes.iter().enumerate() {
                let mut path = current_path.clone();
                path.push(idx);
                paths.push(path.clone());

                if state.is_expanded(&path) && !node.children.is_empty() {
                    traverse(&node.children, path, state, paths);
                }
            }
        }

        traverse(&self.nodes, Vec::new(), state, &mut paths);
        paths
    }

    pub fn select_next(&mut self, state: &mut FileSystemTreeState) {
        let visible_paths = self.get_visible_paths(state);
        if visible_paths.is_empty() {
            return;
        }

        if let Some(current_path) = &state.selected_path {
            if let Some(current_idx) = visible_paths.iter().position(|p| p == current_path) {
                if current_idx < visible_paths.len() - 1 {
                    state.select(visible_paths[current_idx + 1].clone());
                }
            }
        } else {
            state.select(visible_paths[0].clone());
        }
    }

    pub fn select_previous(&mut self, state: &mut FileSystemTreeState) {
        let visible_paths = self.get_visible_paths(state);
        if visible_paths.is_empty() {
            return;
        }

        if let Some(current_path) = &state.selected_path {
            if let Some(current_idx) = visible_paths.iter().position(|p| p == current_path) {
                if current_idx > 0 {
                    state.select(visible_paths[current_idx - 1].clone());
                }
            }
        } else {
            state.select(visible_paths[0].clone());
        }
    }

    pub fn toggle_selected(&mut self, state: &mut FileSystemTreeState) -> std::io::Result<()> {
        if let Some(path) = state.selected_path.clone() {
            if let Some(entry) = self.get_entry_at_path(&path) {
                if entry.is_dir {
                    if !state.is_expanded(&path) {
                        self.expand_directory(&path)?;
                    }
                    state.toggle_expansion(path);
                }
            }
        }
        Ok(())
    }

    pub fn expand_selected(&mut self, state: &mut FileSystemTreeState) -> std::io::Result<bool> {
        let Some(path) = state.selected_path.clone() else {
            return Ok(false);
        };

        let Some(entry) = self.get_entry_at_path(&path) else {
            return Ok(false);
        };

        if !entry.is_dir {
            return Ok(false);
        }

        if !state.is_expanded(&path) {
            self.expand_directory(&path)?;
            state.expand(path);
            return Ok(true);
        }

        Ok(false)
    }

    pub fn collapse_selected(&mut self, state: &mut FileSystemTreeState) -> bool {
        let Some(path) = state.selected_path.clone() else {
            return false;
        };

        if state.is_expanded(&path) {
            state.collapse(path);
            return true;
        }

        if path.len() > 1 {
            let mut parent = path;
            parent.pop();
            state.select(parent);
            return true;
        }

        false
    }

    pub fn handle_navigation_key(
        &mut self,
        key: crossterm::event::KeyCode,
        state: &mut FileSystemTreeState,
    ) -> std::io::Result<bool> {
        match key {
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('j') => {
                self.select_next(state);
                Ok(true)
            }
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('k') => {
                self.select_previous(state);
                Ok(true)
            }
            crossterm::event::KeyCode::Enter => {
                self.toggle_selected(state)?;
                Ok(true)
            }
            crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('l') => {
                if self.expand_selected(state)? {
                    return Ok(true);
                }

                if let Some(path) = state.selected_path.clone() {
                    let mut first_child = path.clone();
                    first_child.push(0);
                    if self.get_entry_at_path(&first_child).is_some() {
                        state.select(first_child);
                        return Ok(true);
                    }
                }

                Ok(false)
            }
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('h') => {
                Ok(self.collapse_selected(state))
            }
            _ => Ok(false),
        }
    }

    pub fn enter_filter_mode(&self, state: &mut FileSystemTreeState) {
        state.enter_filter_mode();
    }

    pub fn is_filter_mode(&self, state: &FileSystemTreeState) -> bool {
        state.is_filter_mode()
    }

    pub fn filter_text<'s>(&self, state: &'s FileSystemTreeState) -> Option<&'s str> {
        state.filter_text()
    }

    pub fn clear_filter(&self, state: &mut FileSystemTreeState) {
        state.clear_filter();
    }

    pub fn handle_filter_key(
        &self,
        key: crossterm::event::KeyCode,
        state: &mut FileSystemTreeState,
    ) -> bool {
        match key {
            crossterm::event::KeyCode::Esc => {
                state.exit_filter_mode();
                true
            }
            crossterm::event::KeyCode::Enter => {
                state.exit_filter_mode();
                true
            }
            crossterm::event::KeyCode::Backspace => {
                state.pop_filter();
                true
            }
            crossterm::event::KeyCode::Char(c) => {
                state.push_filter(c);
                true
            }
            _ => false,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> ratatui::widgets::StatefulWidget for FileSystemTree<'a> {
    type State = FileSystemTreeState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.expanded.is_empty() && !self.nodes.is_empty() {
            state.expand(vec![0]);
        }

        let config = self.config;
        let block = self.block.take();
        let filter_mode = state.filter_mode;
        let has_filter = state.filter.as_ref().is_some_and(|f| !f.is_empty());
        let show_filter_line = filter_mode || has_filter;

        let tree_area = if show_filter_line && area.height > 1 {
            Rect {
                height: area.height - 1,
                ..area
            }
        } else {
            area
        };

        let visible_paths = self.get_visible_paths(state);
        let visible_count = visible_paths.len();
        let offset = state.offset.min(visible_count.saturating_sub(1));
        let visible_paths: Vec<_> = visible_paths.into_iter().skip(offset).collect();

        let tree_area = if let Some(block) = block {
            let inner = block.inner(tree_area);
            block.render(tree_area, buf);
            inner
        } else {
            tree_area
        };

        for (row, path) in visible_paths.iter().enumerate() {
            let y = tree_area.y + row as u16;
            if y >= tree_area.y + tree_area.height {
                break;
            }

            if let Some(entry) = self.get_entry_at_path(path) {
                let is_selected = state.selected_path.as_ref() == Some(path);

                let (icon_glyph, icon_color) = if entry.is_dir {
                    if state.is_expanded(path) {
                        ('\u{f07c}', Color::Rgb(31, 111, 136))
                    } else {
                        ('\u{f07b}', Color::Rgb(31, 111, 136))
                    }
                } else {
                    let theme = if config.use_dark_theme {
                        DevIconTheme::Dark
                    } else {
                        DevIconTheme::Light
                    };
                    let icon_char = if let Some((custom_icon, _)) = get_custom_icon(&entry.name) {
                        custom_icon
                    } else {
                        let file_icon = icon_for_file(&entry.name, &Some(theme));
                        file_icon.icon
                    };
                    let color = get_ayu_dark_color(&entry.name);
                    (icon_char, color)
                };

                let style = if is_selected {
                    config.selected_style
                } else if entry.is_dir {
                    config.dir_style
                } else {
                    config.file_style
                };

                let depth = path.len().saturating_sub(1);
                let indent = "  ".repeat(depth);

                let line = Line::from(vec![
                    Span::raw(indent),
                    Span::styled(format!("{} ", icon_glyph), Style::default().fg(icon_color)),
                    Span::styled(entry.name.clone(), style),
                ]);

                buf.set_line(tree_area.x, y, &line, tree_area.width);
            }
        }

        if show_filter_line && area.height > 1 {
            let y = area.y + area.height - 1;
            let filter_str = state.filter_text().unwrap_or("");
            let cursor = if filter_mode { "_" } else { "" };

            let line = Line::from(vec![
                Span::styled(
                    "/ ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(filter_str, Style::default().fg(Color::White)),
                Span::styled(
                    cursor,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::SLOW_BLINK),
                ),
            ]);

            let bg_style = Style::default().bg(Color::Rgb(15, 25, 40));
            for x in area.x..(area.x + area.width) {
                buf[(x, y)].set_style(bg_style);
            }

            buf.set_line(area.x, y, &line, area.width);
        }
    }
}
