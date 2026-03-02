use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::selection::should_render_line;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::events::MarkdownEvent;
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::features::filter::element_to_plain_text_for_filter;
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::{
    MarkdownWidget, MarkdownWidgetMode,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

impl<'a> MarkdownWidget<'a> {
    pub fn handle_key_event(&mut self, key: KeyEvent) -> MarkdownEvent {
        if self.filter_mode {
            return self.handle_filter_key(key);
        }

        if key.code == KeyCode::Esc && self.selection.is_active() {
            self.selection.exit();
            self.selection_active = false;
            self.mode = MarkdownWidgetMode::Normal;
            self.vim.clear_pending_g();
            return MarkdownEvent::SelectionEnded;
        }

        if key.code == KeyCode::Char('y') && self.selection.has_selection() {
            if let Some(text) = self.selection.get_selected_text() {
                if let Some(event) = self.copy_text_to_clipboard(text, false) {
                    self.selection.exit();
                    self.selection_active = false;
                    self.mode = MarkdownWidgetMode::Normal;
                    self.vim.clear_pending_g();
                    return event;
                }
            }
        }

        if key.code == KeyCode::Char('C')
            && key.modifiers.contains(KeyModifiers::CONTROL)
            && key.modifiers.contains(KeyModifiers::SHIFT)
        {
            if let Some(text) = self.selection.get_selected_text() {
                if let Some(event) = self.copy_text_to_clipboard(text, false) {
                    self.selection.exit();
                    self.selection_active = false;
                    self.mode = MarkdownWidgetMode::Normal;
                    self.vim.clear_pending_g();
                    return event;
                }
            }
        }

        if key.code == KeyCode::Char('g') {
            if self.vim.check_pending_gg() {
                self.scroll.scroll_to_top();
                return MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                };
            }
            self.vim.set_pending_g();
            return MarkdownEvent::None;
        }

        self.vim.clear_pending_g();

        match key.code {
            KeyCode::Char('/') => {
                self.filter_mode = true;
                self.filter = Some(String::new());
                self.mode = MarkdownWidgetMode::Filter;
                MarkdownEvent::FilterModeChanged {
                    active: true,
                    filter: String::new(),
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.scroll.line_down();
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.scroll.line_up();
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::PageDown => {
                let old_offset = self.scroll.scroll_offset;
                self.scroll.scroll_down(self.scroll.viewport_height);
                MarkdownEvent::Scrolled {
                    offset: self.scroll.scroll_offset,
                    direction: self.scroll.scroll_offset.saturating_sub(old_offset) as i32,
                }
            }
            KeyCode::PageUp => {
                let old_offset = self.scroll.scroll_offset;
                self.scroll.scroll_up(self.scroll.viewport_height);
                MarkdownEvent::Scrolled {
                    offset: self.scroll.scroll_offset,
                    direction: -(old_offset.saturating_sub(self.scroll.scroll_offset) as i32),
                }
            }
            KeyCode::Home => {
                self.scroll.scroll_to_top();
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::End | KeyCode::Char('G') => {
                self.scroll.scroll_to_bottom();
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            _ => MarkdownEvent::None,
        }
    }

    fn handle_filter_key(&mut self, key: KeyEvent) -> MarkdownEvent {
        match key.code {
            KeyCode::Esc | KeyCode::Enter => self.exit_filter_mode_with_focus(),
            KeyCode::Backspace => {
                if let Some(filter) = &mut self.filter {
                    filter.pop();
                    return MarkdownEvent::FilterModeChanged {
                        active: true,
                        filter: filter.clone(),
                    };
                }
                MarkdownEvent::None
            }
            KeyCode::Char('j') | KeyCode::Down => {
                let filter = self.filter.clone().unwrap_or_default();
                let next_line = self.find_next_filter_match(filter);
                if let Some(line) = next_line {
                    self.scroll.current_line = line;
                }
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let filter = self.filter.clone().unwrap_or_default();
                let prev_line = self.find_prev_filter_match(filter);
                if let Some(line) = prev_line {
                    self.scroll.current_line = line;
                }
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let filter = self.filter.clone().unwrap_or_default();
                let next_line = self.find_next_filter_match(filter);
                if let Some(line) = next_line {
                    self.scroll.current_line = line;
                }
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                let filter = self.filter.clone().unwrap_or_default();
                let prev_line = self.find_prev_filter_match(filter);
                if let Some(line) = prev_line {
                    self.scroll.current_line = line;
                }
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char(c) => {
                if let Some(filter) = &mut self.filter {
                    filter.push(c);
                    return MarkdownEvent::FilterModeChanged {
                        active: true,
                        filter: filter.clone(),
                    };
                }
                MarkdownEvent::None
            }
            _ => MarkdownEvent::None,
        }
    }

    fn find_next_filter_match(&self, filter: String) -> Option<usize> {
        if filter.is_empty() {
            return None;
        }
        let filter_lower = filter.to_lowercase();
        let elements = self.parse_elements();
        let current = self.scroll.current_line;

        for (idx, element) in elements.iter().enumerate() {
            let line_num = idx + 1;
            if line_num <= current {
                continue;
            }
            if !should_render_line(element, idx, &self.collapse) {
                continue;
            }
            let text = element_to_plain_text_for_filter(&element.kind).to_lowercase();
            if text.contains(&filter_lower) {
                return Some(line_num);
            }
        }
        None
    }

    fn find_prev_filter_match(&self, filter: String) -> Option<usize> {
        if filter.is_empty() {
            return None;
        }
        let filter_lower = filter.to_lowercase();
        let elements = self.parse_elements();
        let current = self.scroll.current_line;

        for (idx, element) in elements.iter().enumerate().rev() {
            let line_num = idx + 1;
            if line_num >= current {
                continue;
            }
            if !should_render_line(element, idx, &self.collapse) {
                continue;
            }
            let text = element_to_plain_text_for_filter(&element.kind).to_lowercase();
            if text.contains(&filter_lower) {
                return Some(line_num);
            }
        }
        None
    }
}
