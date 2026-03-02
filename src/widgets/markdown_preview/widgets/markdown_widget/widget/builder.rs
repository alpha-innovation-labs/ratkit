use crate::primitives::pane::Pane;
use crate::widgets::markdown_preview::services::theme::AppTheme;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::scrollbar::ScrollbarConfig;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::toc::TocConfig;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::types::GitStats;
use crate::widgets::markdown_preview::widgets::markdown_widget::state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    MarkdownState, ScrollState, SelectionState, SourceState, TocState, VimState,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::{
    MarkdownWidget, MarkdownWidgetMode, FRONTMATTER_SECTION_ID,
};
use ratatui::layout::Rect;

impl<'a> MarkdownWidget<'a> {
    pub fn from_state(state: &'a MarkdownState) -> Self {
        let content = state.content().to_string();
        let rendered_lines = state
            .cache
            .render
            .as_ref()
            .map(|c| c.lines.clone())
            .unwrap_or_else(|| state.rendered_lines.clone());

        let mode = if state.filter_mode {
            MarkdownWidgetMode::Filter
        } else {
            MarkdownWidgetMode::Normal
        };

        Self {
            content,
            scroll: state.scroll.clone(),
            source: state.source.clone(),
            cache: state.cache.clone(),
            display: state.display.clone(),
            collapse: state.collapse.clone(),
            expandable: state.expandable.clone(),
            git_stats_state: state.git_stats.clone(),
            vim: state.vim.clone(),
            selection: state.selection.clone(),
            double_click: state.double_click.clone(),
            toc_state: None,
            is_resizing: false,
            mode,
            show_statusline: true,
            show_scrollbar: false,
            scrollbar_config: ScrollbarConfig::default(),
            selection_active: state.selection_active,
            git_stats: state.cached_git_stats,
            show_toc: false,
            toc_config: TocConfig::default(),
            toc_hovered: state.toc_hovered,
            toc_hovered_entry: state.toc_hovered_entry,
            toc_scroll_offset: state.toc_scroll_offset,
            rendered_lines,
            app_theme: None,
            last_double_click: None,
            filter: state.filter.clone(),
            filter_mode: state.filter_mode,
            bordered: false,
            has_pane: true,
            pane: None,
            pane_title: None,
            pane_color: None,
            inner_area: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        content: String,
        scroll: ScrollState,
        source: SourceState,
        cache: CacheState,
        display: DisplaySettings,
        collapse: CollapseState,
        expandable: ExpandableState,
        git_stats_state: GitStatsState,
        vim: VimState,
        selection: SelectionState,
        double_click: DoubleClickState,
    ) -> Self {
        Self {
            content,
            scroll,
            source,
            cache,
            display,
            collapse,
            expandable,
            git_stats_state,
            vim,
            selection,
            double_click,
            toc_state: None,
            is_resizing: false,
            mode: MarkdownWidgetMode::Normal,
            show_statusline: true,
            show_scrollbar: false,
            scrollbar_config: ScrollbarConfig::default(),
            selection_active: false,
            git_stats: None,
            show_toc: false,
            toc_config: TocConfig::default(),
            toc_hovered: false,
            toc_hovered_entry: None,
            toc_scroll_offset: 0,
            rendered_lines: Vec::new(),
            app_theme: None,
            last_double_click: None,
            filter: None,
            filter_mode: false,
            bordered: false,
            has_pane: true,
            pane: None,
            pane_title: None,
            pane_color: None,
            inner_area: None,
        }
    }

    pub fn with_has_pane(mut self, has_pane: bool) -> Self {
        self.has_pane = has_pane;
        self
    }

    pub fn with_pane(mut self, pane: Pane<'a>) -> Self {
        self.pane = Some(pane);
        self
    }

    pub fn with_pane_color(mut self, color: impl Into<ratatui::style::Color>) -> Self {
        self.pane_color = Some(color.into());
        self
    }

    pub fn with_pane_title(mut self, title: impl Into<String>) -> Self {
        self.pane_title = Some(title.into());
        self
    }

    pub fn scrollbar_config(mut self, config: ScrollbarConfig) -> Self {
        self.scrollbar_config = config;
        self
    }

    pub fn selection_active(mut self, active: bool) -> Self {
        self.selection_active = active;
        self
    }

    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    pub fn show_statusline(mut self, show: bool) -> Self {
        self.show_statusline = show;
        self
    }

    pub fn with_frontmatter_collapsed(mut self, collapsed: bool) -> Self {
        self.collapse
            .set_section_collapsed(FRONTMATTER_SECTION_ID, collapsed);
        self
    }

    pub fn set_frontmatter_collapsed(&mut self, collapsed: bool) {
        self.collapse
            .set_section_collapsed(FRONTMATTER_SECTION_ID, collapsed);
        self.cache.invalidate();
    }

    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.app_theme = Some(theme.clone());
        self.toc_config = self.toc_config.with_theme(theme);
        self
    }

    pub fn with_toc_state(mut self, toc_state: TocState) -> Self {
        self.toc_state = Some(toc_state);
        self
    }

    pub fn calculate_scrollbar_area(&self, area: Rect) -> Option<Rect> {
        let content_area = if self.show_statusline && area.height > 1 {
            Rect {
                height: area.height.saturating_sub(1),
                ..area
            }
        } else {
            area
        };

        if !self.show_scrollbar || self.scroll.total_lines <= content_area.height as usize {
            return None;
        }

        let scrollbar_width = self.scrollbar_config.width;

        Some(Rect {
            x: content_area.x + content_area.width.saturating_sub(scrollbar_width),
            y: content_area.y,
            width: scrollbar_width,
            height: content_area.height,
        })
    }

    pub fn git_stats(mut self, stats: GitStats) -> Self {
        self.git_stats = Some(stats);
        self
    }

    pub fn maybe_git_stats(mut self, stats: Option<GitStats>) -> Self {
        self.git_stats = stats;
        self
    }

    pub fn git_stats_tuple(mut self, additions: usize, modified: usize, deletions: usize) -> Self {
        self.git_stats = Some(GitStats {
            additions,
            modified,
            deletions,
        });
        self
    }

    pub fn is_resizing(mut self, resizing: bool) -> Self {
        self.is_resizing = resizing;
        self
    }

    pub fn take_last_double_click(&mut self) -> Option<(usize, String, String)> {
        self.last_double_click.take()
    }

    pub fn take_last_copied(&mut self) -> Option<String> {
        self.selection.last_copied_text.take()
    }

    pub fn mode(mut self, mode: MarkdownWidgetMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn rendered_lines(&self) -> &Vec<ratatui::text::Line<'static>> {
        &self.rendered_lines
    }
}
