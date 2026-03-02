use crate::widgets::markdown_preview::widgets::markdown_widget::state::MarkdownState;
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::MarkdownWidget;
use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct WidgetStateSync {
    pub inner_area: Rect,
    pub toc_hovered: bool,
    pub toc_hovered_entry: Option<usize>,
    pub toc_scroll_offset: usize,
    pub selection_active: bool,
    pub last_double_click: Option<(usize, String, String)>,
    pub filter: Option<String>,
    pub filter_mode: bool,
    pub scroll_offset: usize,
    pub current_line: usize,
}

impl WidgetStateSync {
    pub fn new(inner_area: Rect) -> Self {
        Self {
            inner_area,
            toc_hovered: false,
            toc_hovered_entry: None,
            toc_scroll_offset: 0,
            selection_active: false,
            last_double_click: None,
            filter: None,
            filter_mode: false,
            scroll_offset: 0,
            current_line: 1,
        }
    }

    pub fn apply_to(&self, state: &mut MarkdownState) {
        state.set_inner_area(self.inner_area);
        state.toc_hovered = self.toc_hovered;
        state.toc_hovered_entry = self.toc_hovered_entry;
        state.toc_scroll_offset = self.toc_scroll_offset;
        state.selection_active = self.selection_active;
        state.filter = self.filter.clone();
        state.filter_mode = self.filter_mode;
        state.scroll.scroll_offset = self.scroll_offset;
        state.scroll.current_line = self.current_line;
    }

    pub fn take_double_click(&mut self) -> Option<(usize, String, String)> {
        self.last_double_click.take()
    }
}

impl<'a> MarkdownWidget<'a> {
    pub fn get_state_sync(&mut self) -> WidgetStateSync {
        WidgetStateSync {
            inner_area: self.inner_area.unwrap_or_default(),
            toc_hovered: self.toc_hovered,
            toc_hovered_entry: self.toc_hovered_entry,
            toc_scroll_offset: self.toc_scroll_offset,
            selection_active: self.selection.is_active(),
            last_double_click: self.last_double_click.take(),
            filter: self.filter.clone(),
            filter_mode: self.filter_mode,
            scroll_offset: self.scroll.scroll_offset,
            current_line: self.scroll.current_line,
        }
    }

    pub fn sync_state_back(self, state: &mut MarkdownState) {
        state.set_inner_area(self.inner_area.unwrap_or_default());
        state.toc_hovered = self.toc_hovered;
        state.toc_hovered_entry = self.toc_hovered_entry;
        state.toc_scroll_offset = self.toc_scroll_offset;
        state.selection_active = self.selection.is_active();
        state.filter = self.filter;
        state.filter_mode = self.filter_mode;
    }
}
