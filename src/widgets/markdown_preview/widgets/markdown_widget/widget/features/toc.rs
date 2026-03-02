use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::toc::{Toc, TocConfig};
use crate::widgets::markdown_preview::widgets::markdown_widget::state::TocState;
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::MarkdownWidget;
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

impl<'a> MarkdownWidget<'a> {
    pub fn show_toc(mut self, show: bool) -> Self {
        self.show_toc = show;
        self
    }

    pub fn toggle_toc(&mut self) -> bool {
        self.show_toc = !self.show_toc;
        if !self.show_toc {
            self.toc_hovered = false;
            self.toc_hovered_entry = None;
        }
        self.show_toc
    }

    pub fn toc_config(mut self, config: TocConfig) -> Self {
        self.toc_config = config;
        self
    }

    pub fn toc_hovered(mut self, hovered: bool) -> Self {
        self.toc_hovered = hovered;
        self
    }

    pub fn toc_hovered_entry(mut self, index: Option<usize>) -> Self {
        self.toc_hovered_entry = index;
        self
    }

    pub fn toc_scroll_offset(mut self, offset: usize) -> Self {
        self.toc_scroll_offset = offset;
        self
    }

    pub fn calculate_toc_area(&self, total_area: Rect) -> Option<Rect> {
        if !self.show_toc {
            return None;
        }

        let main_area = if self.show_statusline && total_area.height > 1 {
            Rect {
                height: total_area.height.saturating_sub(1),
                ..total_area
            }
        } else {
            total_area
        };

        let padding_right: u16 = 2;
        let padding_top: u16 = 1;

        let toc_width = if self.toc_hovered {
            Toc::required_expanded_width(&self.content, self.toc_config.show_border)
                .min(main_area.width.saturating_sub(padding_right + 4))
        } else {
            self.toc_config.compact_width
        };

        let toc_height = if self.toc_hovered {
            Toc::required_height(&self.content, self.toc_config.show_border)
                .min(main_area.height.saturating_sub(1))
        } else {
            Toc::required_compact_height(
                &self.content,
                self.toc_config.line_spacing,
                self.toc_config.show_border,
            )
            .min(main_area.height.saturating_sub(1))
        };

        if main_area.width <= toc_width + padding_right + 2 {
            return None;
        }

        Some(Rect {
            x: main_area.x + main_area.width.saturating_sub(toc_width + padding_right),
            y: main_area.y + padding_top,
            width: toc_width,
            height: toc_height,
        })
    }

    pub(crate) fn resolved_toc_state<'s>(&'s self, auto_state: &'s TocState) -> &'s TocState {
        if let Some(provided) = &self.toc_state {
            if provided.entries.is_empty() {
                auto_state
            } else {
                provided
            }
        } else {
            auto_state
        }
    }

    pub fn handle_toc_click(&mut self, event: &MouseEvent, area: Rect) -> bool {
        if !matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
            return false;
        }

        let toc_area = match self.calculate_toc_area(area) {
            Some(t_area) => t_area,
            None => return false,
        };

        self.handle_toc_click_in_area(event, toc_area)
    }

    pub fn handle_toc_click_in_area(&mut self, event: &MouseEvent, toc_area: Rect) -> bool {
        if !matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
            return false;
        }

        if event.column < toc_area.x
            || event.column >= toc_area.x + toc_area.width
            || event.row < toc_area.y
        {
            return false;
        }

        let mut auto_state = TocState::from_content(&self.content);
        auto_state.scroll_offset = self.toc_scroll_offset;
        auto_state.hovered = self.toc_hovered;
        auto_state.hovered_entry = self.toc_hovered_entry;
        let toc_state = self.resolved_toc_state(&auto_state);
        let toc = Toc::new(toc_state)
            .expanded(self.toc_hovered)
            .config(self.toc_config.clone());

        if let Some(entry_idx) = toc.entry_at_position(event.column, event.row, toc_area) {
            if let Some(target_line) = toc.click_to_line(entry_idx) {
                let new_offset = target_line.saturating_sub(2);
                let max_offset = self
                    .scroll
                    .total_lines
                    .saturating_sub(self.scroll.viewport_height);
                self.scroll.scroll_offset = new_offset.min(max_offset);
                self.scroll.current_line = target_line.saturating_add(1);
                self.toc_hovered_entry = Some(entry_idx);
                return true;
            }
        }

        false
    }

    pub fn handle_toc_hover(&mut self, event: &MouseEvent, area: Rect) -> bool {
        if !matches!(event.kind, MouseEventKind::Moved) {
            return false;
        }

        let toc_area = match self.calculate_toc_area(area) {
            Some(t_area) => t_area,
            None => {
                let changed = self.toc_hovered || self.toc_hovered_entry.is_some();
                if changed {
                    self.toc_hovered = false;
                    self.toc_hovered_entry = None;
                }
                return changed;
            }
        };

        let is_potentially_over_toc = event.column >= toc_area.x
            && event.column < toc_area.x + toc_area.width
            && event.row >= toc_area.y
            && event.row < toc_area.y + toc_area.height;

        let prev_hovered = self.toc_hovered;
        let prev_entry = self.toc_hovered_entry;

        if is_potentially_over_toc {
            let hovered_entry = {
                let mut auto_state = TocState::from_content(&self.content);
                auto_state.scroll_offset = self.toc_scroll_offset;
                auto_state.hovered = true;
                auto_state.hovered_entry = self.toc_hovered_entry;
                let toc_state = self.resolved_toc_state(&auto_state);
                let toc = Toc::new(toc_state)
                    .expanded(true)
                    .config(self.toc_config.clone());
                toc.entry_at_position(event.column, event.row, toc_area)
            };
            self.toc_hovered = true;
            self.toc_hovered_entry = hovered_entry;
        } else {
            self.toc_hovered = false;
            self.toc_hovered_entry = None;
        }

        prev_hovered != self.toc_hovered || prev_entry != self.toc_hovered_entry
    }

    pub fn is_toc_hovered(&self) -> bool {
        self.toc_hovered
    }

    pub fn get_toc_hovered_entry(&self) -> Option<usize> {
        self.toc_hovered_entry
    }

    pub fn set_toc_hovered(&mut self, hovered: bool) {
        self.toc_hovered = hovered;
        if !hovered {
            self.toc_hovered_entry = None;
        }
    }

    pub fn get_toc_scroll_offset(&self) -> usize {
        self.toc_scroll_offset
    }

    pub fn set_toc_scroll_offset(&mut self, offset: usize) {
        self.toc_scroll_offset = offset;
    }

    pub fn update_toc_hovered_entry(&mut self, x: u16, y: u16, toc_area: Rect) {
        let mut auto_state = TocState::from_content(&self.content);
        auto_state.scroll_offset = self.toc_scroll_offset;
        auto_state.hovered = true;
        auto_state.hovered_entry = self.toc_hovered_entry;
        let toc_state = self.resolved_toc_state(&auto_state);
        let toc = Toc::new(toc_state)
            .expanded(true)
            .config(self.toc_config.clone());

        self.toc_hovered_entry = toc.entry_at_position(x, y, toc_area);
    }

    pub(crate) fn handle_toc_hover_internal(&mut self, event: &MouseEvent, toc_area: Rect) {
        let hovered_entry = {
            let mut auto_state = TocState::from_content(&self.content);
            auto_state.scroll_offset = self.toc_scroll_offset;
            auto_state.hovered = true;
            auto_state.hovered_entry = self.toc_hovered_entry;
            let toc_state = self.resolved_toc_state(&auto_state);
            let toc = Toc::new(toc_state)
                .expanded(true)
                .config(self.toc_config.clone());
            toc.entry_at_position(event.column, event.row, toc_area)
        };
        self.toc_hovered = true;
        self.toc_hovered_entry = hovered_entry;
    }

    pub(crate) fn handle_toc_click_internal(&mut self, event: &MouseEvent, toc_area: Rect) -> bool {
        self.handle_toc_click_in_area(event, toc_area)
    }
}
