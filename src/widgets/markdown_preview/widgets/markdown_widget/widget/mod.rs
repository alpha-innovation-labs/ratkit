//! A scrollable, interactive markdown widget.

mod builder;
mod features;
mod input;
mod line_info;
mod render;
mod state_sync;

pub use crate::widgets::markdown_preview::widgets::markdown_widget::widget::features::filter::element_to_plain_text_for_filter;
pub use crate::widgets::markdown_preview::widgets::markdown_widget::widget::features::selection::apply_selection_highlighting;
pub use crate::widgets::markdown_preview::widgets::markdown_widget::widget::state_sync::WidgetStateSync;

use crate::primitives::pane::Pane;
use crate::widgets::markdown_preview::services::theme::AppTheme;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::scrollbar::ScrollbarConfig;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::toc::TocConfig;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::events::MarkdownEvent;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::types::GitStats;
use crate::widgets::markdown_preview::widgets::markdown_widget::state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    ScrollState, SelectionState, SourceState, TocState, VimState,
};
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;

pub(crate) const FRONTMATTER_SECTION_ID: usize = 0;
pub(crate) const CURRENT_LINE_BG: ratatui::style::Color = ratatui::style::Color::Rgb(38, 52, 63);
pub(crate) const CURRENT_LINE_DRAG_BG: ratatui::style::Color =
    ratatui::style::Color::Rgb(70, 80, 100);

pub struct MarkdownWidget<'a> {
    pub(crate) content: String,
    pub(crate) scroll: ScrollState,
    pub(crate) source: SourceState,
    pub(crate) cache: CacheState,
    pub(crate) display: DisplaySettings,
    pub(crate) collapse: CollapseState,
    pub(crate) expandable: ExpandableState,
    pub(crate) git_stats_state: GitStatsState,
    pub(crate) vim: VimState,
    pub(crate) selection: SelectionState,
    pub(crate) double_click: DoubleClickState,
    pub(crate) toc_state: Option<TocState>,
    pub(crate) is_resizing: bool,
    pub(crate) mode: MarkdownWidgetMode,
    pub(crate) show_statusline: bool,
    pub(crate) show_scrollbar: bool,
    pub(crate) scrollbar_config: ScrollbarConfig,
    pub(crate) selection_active: bool,
    pub(crate) git_stats: Option<GitStats>,
    pub(crate) show_toc: bool,
    pub(crate) toc_config: TocConfig,
    pub(crate) toc_hovered: bool,
    pub(crate) toc_hovered_entry: Option<usize>,
    pub(crate) toc_scroll_offset: usize,
    pub(crate) rendered_lines: Vec<ratatui::text::Line<'static>>,
    pub(crate) app_theme: Option<AppTheme>,
    pub(crate) last_double_click: Option<(usize, String, String)>,
    pub(crate) filter: Option<String>,
    pub(crate) filter_mode: bool,
    pub(crate) bordered: bool,
    pub(crate) has_pane: bool,
    pub(crate) pane: Option<Pane<'a>>,
    pub(crate) pane_title: Option<String>,
    pub(crate) pane_color: Option<ratatui::style::Color>,
    pub inner_area: Option<Rect>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkdownWidgetMode {
    #[default]
    Normal,
    Drag,
    Filter,
}

impl MarkdownWidget<'_> {
    pub fn handle_key(&mut self, key: KeyEvent) -> MarkdownEvent {
        self.handle_key_event(key)
    }

    pub fn handle_mouse(&mut self, event: MouseEvent, area: Rect) -> MarkdownEvent {
        self.handle_mouse_internal(&event, area)
    }

    pub fn update_git_stats(&mut self) {
        self.git_stats_state.update(self.source.source_path());
    }
}

impl<'a> MarkdownWidget<'a> {
    pub(crate) fn parse_elements(&self) -> Vec<crate::widgets::markdown_preview::MarkdownElement> {
        crate::widgets::markdown_preview::widgets::markdown_widget::foundation::parser::render_markdown_to_elements(
            &self.content,
            true,
        )
    }

    pub(crate) fn exit_filter_mode_with_focus(&mut self) -> MarkdownEvent {
        let focused_line = self.scroll.current_line;
        self.filter_mode = false;
        self.filter = None;
        self.mode = MarkdownWidgetMode::Normal;
        self.scroll.filter_mode = false;
        self.scroll.filter = None;
        self.cache.render = None;
        MarkdownEvent::FilterModeExited { line: focused_line }
    }

    pub(crate) fn copy_text_to_clipboard(
        &mut self,
        text: String,
        remember_in_selection_state: bool,
    ) -> Option<MarkdownEvent> {
        if text.is_empty() {
            return None;
        }

        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            if clipboard.set_text(&text).is_ok() {
                if remember_in_selection_state {
                    self.selection.last_copied_text = Some(text.clone());
                }
                return Some(MarkdownEvent::Copied { text });
            }
        }
        None
    }
}
