//! A scrollable, interactive markdown widget.

use crate::primitives::pane::Pane;
use crate::widgets::markdown_preview::services::theme::AppTheme;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::scrollbar::CustomScrollbar;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::scrollbar::ScrollbarConfig;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::selection::should_render_line as should_render_collapsed_line;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::toc::Toc;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::toc::TocConfig;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::types::GitStats;
use crate::widgets::markdown_preview::widgets::markdown_widget::state::{
    CacheState, CollapseState, DisplaySettings, DoubleClickState, ExpandableState, GitStatsState,
    MarkdownState, ScrollState, SelectionState, SourceState, TocState, VimState,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::widgets::{Block, Borders, Widget};

/// A scrollable, interactive markdown widget.
///
/// This widget renders markdown content with:
/// - Scroll support (keyboard and mouse)
/// - Click-to-highlight line selection
/// - Clickable headings to collapse/expand sections
/// - Clickable frontmatter to collapse/expand
/// - Expandable content blocks ("Show more"/"Show less")
/// - Text selection and copy support (drag to select)
/// - Double-click detection
/// - Statusline showing mode and scroll position
///
/// The widget handles ALL event processing internally and returns `MarkdownEvent`
/// variants so the parent application can react appropriately.
///
/// # Mouse Capture Requirement
///
/// For click events to work (line highlighting, TOC navigation, text selection),
/// you must enable mouse capture in your terminal setup:
///
/// ```rust,ignore,ignore
/// use crossterm::{
///     event::{EnableMouseCapture, DisableMouseCapture},
///     execute,
///     terminal::{EnterAlternateScreen, LeaveAlternateScreen},
/// };
///
/// // On startup:
/// execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
///
/// // On cleanup:
/// execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
/// ```
///
/// Without `EnableMouseCapture`, scroll wheel events may still work (terminal-dependent),
/// but click events will not be received by the application.
pub struct MarkdownWidget<'a> {
    /// The markdown content to render.
    pub(crate) content: String,
    /// Scroll state (position, viewport, current line).
    pub(crate) scroll: ScrollState,
    /// Content source state.
    pub(crate) source: SourceState,
    /// Render cache state.
    pub(crate) cache: CacheState,
    /// Display settings (line numbers, themes).
    pub(crate) display: DisplaySettings,
    /// Section collapse state.
    pub(crate) collapse: CollapseState,
    /// Expandable content state.
    pub(crate) expandable: ExpandableState,
    /// Git stats state.
    pub(crate) git_stats_state: GitStatsState,
    /// Vim keybinding state.
    pub(crate) vim: VimState,
    /// Selection state for text selection/copy.
    pub(crate) selection: SelectionState,
    /// Double-click state for double-click detection.
    pub(crate) double_click: DoubleClickState,
    /// Optional TOC state for table of contents.
    pub(crate) toc_state: Option<TocState>,
    /// When true, use stale cache for smoother resize during drag operations.
    pub(crate) is_resizing: bool,
    /// Current mode for the statusline.
    pub(crate) mode: MarkdownWidgetMode,
    /// Whether to show the statusline.
    pub(crate) show_statusline: bool,
    /// Whether to show the scrollbar.
    pub(crate) show_scrollbar: bool,
    /// Configuration for the scrollbar.
    pub(crate) scrollbar_config: ScrollbarConfig,
    /// Whether selection mode is active (affects statusline mode display).
    pub(crate) selection_active: bool,
    /// Git statistics for the file (optional, from git_stats_state).
    pub(crate) git_stats: Option<GitStats>,
    /// Whether to show the TOC.
    pub(crate) show_toc: bool,
    /// Configuration for the TOC.
    pub(crate) toc_config: TocConfig,
    /// Whether the TOC is currently hovered (expands to show text).
    pub(crate) toc_hovered: bool,
    /// Index of the hovered TOC entry.
    pub(crate) toc_hovered_entry: Option<usize>,
    /// Scroll offset for the TOC list.
    pub(crate) toc_scroll_offset: usize,
    /// Cached rendered lines for selection text extraction.
    pub(crate) rendered_lines: Vec<ratatui::text::Line<'static>>,
    /// Optional application theme for styling.
    pub(crate) app_theme: Option<AppTheme>,
    /// Last double-click info (line number, kind, content) for app to retrieve.
    pub(crate) last_double_click: Option<(usize, String, String)>,
    /// Current filter text (when in filter mode).
    pub(crate) filter: Option<String>,
    /// Whether filter mode is currently active.
    pub(crate) filter_mode: bool,
    /// Whether to render a border around the widget.
    pub(crate) bordered: bool,
    /// Whether to wrap the widget in a Pane.
    pub(crate) has_pane: bool,
    /// Optional Pane configuration for wrapping the widget.
    pub(crate) pane: Option<Pane<'a>>,
    /// Title to use for the default Pane (e.g., filename).
    pub(crate) pane_title: Option<String>,
    /// Color for the Pane border.
    pub(crate) pane_color: Option<ratatui::style::Color>,
    /// Inner area calculated during render (for mouse event handling).
    pub inner_area: Option<ratatui::layout::Rect>,
}

/// Construct MarkdownWidget from a unified MarkdownState.

impl<'a> MarkdownWidget<'a> {
    /// Create a new MarkdownWidget from a unified MarkdownState.
    ///
    /// This constructor clones the state into the widget, allowing the widget
    /// to own its state internally without holding references to the original state.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to render
    /// * `state` - The unified markdown state containing all component states
    ///
    /// # Returns
    ///
    /// A new `MarkdownWidget` instance that owns its state.
    ///
    /// # Example
    ///
    /// ```rust,ignore,ignore
    /// use ratatui_toolkit::markdown_widget::state::MarkdownState;
    /// use ratatui_toolkit::MarkdownWidget;
    ///
    /// let mut state = MarkdownState::default();
    /// state.source.set_content("# Hello World");
    ///
    /// let widget = MarkdownWidget::from_state(&state);
    /// ```
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
}

/// Constructor for has_pane option.

impl<'a> MarkdownWidget<'a> {
    /// Set whether to wrap the widget in a Pane.
    ///
    /// When `has_pane` is true (default), the widget is wrapped in a styled Pane
    /// with title, border, and padding. Set to false for raw markdown rendering.
    ///
    /// # Arguments
    ///
    /// * `has_pane` - Whether to wrap in a Pane (default: true)
    ///
    /// # Returns
    ///
    /// The modified `MarkdownWidget` instance.
    pub fn with_has_pane(mut self, has_pane: bool) -> Self {
        self.has_pane = has_pane;
        self
    }
}

/// Constructor for MarkdownWidget.
impl<'a> MarkdownWidget<'a> {
    /// Create a new MarkdownWidget with the given content and state managers.
    ///
    /// This constructor takes owned state values, allowing the widget to own
    /// its state internally.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to render
    /// * `scroll` - Scroll state (position, viewport, current line)
    /// * `source` - Content source state
    /// * `cache` - Render cache state
    /// * `display` - Display settings (line numbers, themes)
    /// * `collapse` - Section collapse state
    /// * `expandable` - Expandable content state
    /// * `git_stats_state` - Git stats state
    /// * `vim` - Vim keybinding state
    /// * `selection` - Selection state for text selection/copy
    /// * `double_click` - Double-click state for detection
    ///
    /// # Returns
    ///
    /// A new `MarkdownWidget` instance.
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
}

/// Constructor for pane configuration.

impl<'a> MarkdownWidget<'a> {
    /// Configure the Pane that wraps the widget.
    ///
    /// When `has_pane` is true (default), the widget is wrapped in this Pane.
    /// Use this to customize the pane's title, icon, padding, border style, etc.
    ///
    /// # Arguments
    ///
    /// * `pane` - The Pane configuration to use
    ///
    /// # Returns
    ///
    /// The modified `MarkdownWidget` instance.
    pub fn with_pane(mut self, pane: Pane<'a>) -> Self {
        self.pane = Some(pane);
        self
    }
}

/// Constructor for pane color.

impl<'a> MarkdownWidget<'a> {
    /// Set the border color for the Pane that wraps the widget.
    ///
    /// Only used when `has_pane` is true (default).
    ///
    /// # Arguments
    ///
    /// * `color` - The color to use for the pane's border
    ///
    /// # Returns
    ///
    /// The modified `MarkdownWidget` instance.
    pub fn with_pane_color(mut self, color: impl Into<ratatui::style::Color>) -> Self {
        self.pane_color = Some(color.into());
        self
    }
}

/// Constructor for pane title.

impl<'a> MarkdownWidget<'a> {
    /// Set the title for the Pane that wraps the widget.
    ///
    /// This is typically the filename being displayed.
    /// Only used when `has_pane` is true (default).
    ///
    /// # Arguments
    ///
    /// * `title` - The title to display in the pane's title bar
    ///
    /// # Returns
    ///
    /// The modified `MarkdownWidget` instance.
    pub fn with_pane_title(mut self, title: impl Into<String>) -> Self {
        self.pane_title = Some(title.into());
        self
    }
}

/// Set the custom scrollbar configuration.

impl<'a> MarkdownWidget<'a> {
    /// Set the custom scrollbar configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The scrollbar configuration to use
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust,ignore,ignore
    /// use ratatui::style::{Color, Style};
    /// use ratatui_toolkit::markdown_widget::extensions::scrollbar::ScrollbarConfig;
    ///
    /// let config = ScrollbarConfig {
    ///     thumb_style: Style::default().fg(Color::Cyan),
    ///     ..Default::default()
    /// };
    ///
    /// let widget = MarkdownWidget::from_state(&content, &mut state)
    ///     .show_custom_scrollbar(true)
    ///     .scrollbar_config(config);
    /// ```
    pub fn scrollbar_config(mut self, config: ScrollbarConfig) -> Self {
        self.scrollbar_config = config;
        self
    }
}

/// Set whether selection mode is active.

impl<'a> MarkdownWidget<'a> {
    /// Set whether selection mode is active.
    ///
    /// This affects the mode displayed in the statusline (Normal vs Drag).
    ///
    /// # Arguments
    ///
    /// * `active` - Whether selection is active
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn selection_active(mut self, active: bool) -> Self {
        self.selection_active = active;
        self
    }
}

/// Enable or disable the scrollbar.

impl<'a> MarkdownWidget<'a> {
    /// Enable or disable the scrollbar.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the scrollbar
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }
}

/// Enable or disable the statusline.

impl<'a> MarkdownWidget<'a> {
    /// Set whether to show the statusline.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the statusline
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_statusline(mut self, show: bool) -> Self {
        self.show_statusline = show;
        self
    }
}

/// Enable or disable the TOC (Table of Contents).

impl<'a> MarkdownWidget<'a> {
    /// Enable or disable the TOC (Table of Contents).
    ///
    /// When enabled, shows heading navigation in the top-right corner.
    /// Compact mode shows lines, expanded mode (on hover) shows text.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether to show the TOC
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn show_toc(mut self, show: bool) -> Self {
        self.show_toc = show;
        self
    }

    /// Toggle TOC visibility at runtime.
    ///
    /// Returns the new visibility state.
    pub fn toggle_toc(&mut self) -> bool {
        self.show_toc = !self.show_toc;
        if !self.show_toc {
            self.toc_hovered = false;
            self.toc_hovered_entry = None;
        }
        self.show_toc
    }
}

/// Set the TOC configuration.

impl<'a> MarkdownWidget<'a> {
    /// Set the TOC configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The TOC configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_config(mut self, config: TocConfig) -> Self {
        self.toc_config = config;
        self
    }
}

/// Set the TOC hovered state.

impl<'a> MarkdownWidget<'a> {
    /// Set the TOC hovered state.
    ///
    /// When hovered, the TOC expands to show heading text.
    ///
    /// # Arguments
    ///
    /// * `hovered` - Whether the TOC is hovered
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_hovered(mut self, hovered: bool) -> Self {
        self.toc_hovered = hovered;
        self
    }
}

/// Set the hovered TOC entry index.

impl<'a> MarkdownWidget<'a> {
    /// Set the hovered TOC entry index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the hovered entry, or None
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_hovered_entry(mut self, index: Option<usize>) -> Self {
        self.toc_hovered_entry = index;
        self
    }
}

/// Set the TOC scroll offset.

impl<'a> MarkdownWidget<'a> {
    /// Set the TOC scroll offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset for the TOC list
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_scroll_offset(mut self, offset: usize) -> Self {
        self.toc_scroll_offset = offset;
        self
    }
}

/// Theme application constructor for MarkdownWidget.

impl<'a> MarkdownWidget<'a> {
    /// Applies an application theme to the widget.
    ///
    /// When a theme is applied, the widget will use theme colors for:
    /// - Statusline (mode colors, background, text)
    /// - TOC (text, active, hover, background, border colors)
    /// - Selection highlighting
    ///
    /// If no theme is set, the widget falls back to default hardcoded colors.
    ///
    /// # Arguments
    ///
    /// * `theme` - The application theme to use for styling
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_run
    /// use ratatui_toolkit::{MarkdownWidget, theme::AppTheme};
    ///
    /// let theme = AppTheme::default();
    /// // let widget = MarkdownWidget::new(content, scroll, selection, double_click)
    /// //     .with_theme(&theme);
    /// ```
    pub fn with_theme(mut self, theme: &AppTheme) -> Self {
        self.app_theme = Some(theme.clone());
        // Apply theme colors to TOC config
        self.toc_config = self.toc_config.with_theme(theme);
        self
    }
}

/// TOC state constructor for MarkdownWidget.

impl<'a> MarkdownWidget<'a> {
    /// Set the TOC state for the widget.
    ///
    /// When a TOC state is provided, the widget can use it for TOC rendering
    /// and navigation.
    ///
    /// # Arguments
    ///
    /// * `toc_state` - The TOC state containing entries and hover information
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn with_toc_state(mut self, toc_state: TocState) -> Self {
        self.toc_state = Some(toc_state);
        self
    }
}

/// Mode enum for the markdown widget statusline.

/// Mode for the markdown widget statusline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkdownWidgetMode {
    /// Normal viewing mode.
    #[default]
    Normal,
    /// Drag/selection mode.
    Drag,
    /// Filter mode (search/filter document).
    Filter,
}

/// Apply selection highlighting to rendered lines.
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::types::SelectionPos;

/// Selection highlight background color (dark blue for better visibility).
const SELECTION_BG: Color = Color::Rgb(55, 75, 120);

/// Apply selection highlighting to visible lines.
///
/// # Arguments
///
/// * `lines` - The visible lines to highlight.
/// * `selection` - The selection state.
/// * `scroll_offset` - The current scroll offset.
///
/// # Returns
///
/// Lines with selection highlighting applied.
pub fn apply_selection_highlighting(
    lines: Vec<Line<'static>>,
    selection: &SelectionState,
    scroll_offset: usize,
) -> Vec<Line<'static>> {
    // If no active selection, return as-is
    if !selection.active {
        return lines;
    }

    let Some((start, end)) = selection.get_selection() else {
        return lines;
    };

    let selection_style = Style::new().bg(SELECTION_BG);

    lines
        .into_iter()
        .enumerate()
        .map(|(visible_idx, line)| {
            // Convert visible index to document index
            let doc_y = (scroll_offset + visible_idx) as i32;

            // Check if this line is in selection range
            if doc_y < start.y || doc_y > end.y {
                return line;
            }

            // This line is at least partially selected
            apply_selection_to_line(line, doc_y, &start, &end, selection_style)
        })
        .collect()
}

/// Apply selection highlighting to a single line.
fn apply_selection_to_line(
    line: Line<'static>,
    doc_y: i32,
    start: &SelectionPos,
    end: &SelectionPos,
    _selection_style: Style,
) -> Line<'static> {
    // Calculate the character range to highlight on this line
    let line_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
    let line_len = line_text.chars().count() as i32;

    let (sel_start, sel_end) = if start.y == end.y {
        // Single line selection
        (start.x.max(0), end.x.min(line_len - 1))
    } else if doc_y == start.y {
        // First line of multi-line selection
        (start.x.max(0), line_len - 1)
    } else if doc_y == end.y {
        // Last line of multi-line selection
        (0, end.x.min(line_len - 1))
    } else {
        // Middle line - entire line selected
        (0, line_len - 1)
    };

    if sel_start > sel_end || sel_start >= line_len {
        return line;
    }

    // Rebuild spans with selection highlighting
    // Skip line bar (│) and blockquote markers (▋) from selection highlighting
    let mut new_spans = Vec::new();
    let mut current_pos = 0i32;

    for span in line.spans {
        let span_text = span.content.to_string();
        let span_len = span_text.chars().count() as i32;
        let span_end = current_pos + span_len;

        // Skip line numbers, line bar, and blockquote markers from selection
        let is_line_number =
            current_pos == 0 && span_text.chars().all(|c| c.is_ascii_digit() || c == ' ');
        if is_line_number || span_text.contains('│') || span_text.contains('▋') {
            new_spans.push(span);
            current_pos = span_end;
            continue;
        }

        if span_end <= sel_start || current_pos > sel_end {
            // Span is entirely outside selection
            new_spans.push(span);
        } else if current_pos >= sel_start && span_end <= sel_end + 1 {
            // Span is entirely inside selection
            new_spans.push(Span::styled(span_text, span.style.bg(SELECTION_BG)));
        } else {
            // Span is partially selected - split it
            let chars: Vec<char> = span_text.chars().collect();

            // Before selection
            if current_pos < sel_start {
                let before_count = (sel_start - current_pos) as usize;
                let before: String = chars[..before_count].iter().collect();
                new_spans.push(Span::styled(before, span.style));
            }

            // Selected part
            let sel_local_start = (sel_start - current_pos).max(0) as usize;
            let sel_local_end = ((sel_end - current_pos + 1) as usize).min(chars.len());
            if sel_local_start < sel_local_end {
                let selected: String = chars[sel_local_start..sel_local_end].iter().collect();
                new_spans.push(Span::styled(selected, span.style.bg(SELECTION_BG)));
            }

            // After selection
            let after_start = (sel_end - current_pos + 1) as usize;
            if after_start < chars.len() {
                let after: String = chars[after_start..].iter().collect();
                new_spans.push(Span::styled(after, span.style));
            }
        }

        current_pos = span_end;
    }

    Line::from(new_spans)
}

/// Filter navigation helpers for MarkdownWidget.
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::{
    render, ElementKind, TextSegment,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::parser::render_markdown_to_elements;

#[allow(dead_code)]
fn get_filtered_visual_lines(
    content: &str,
    filter_text: &str,
    collapse: &CollapseState,
    width: usize,
) -> Vec<usize> {
    let filter_lower = filter_text.to_lowercase();
    let elements = render_markdown_to_elements(content, true);
    let mut filtered_visual_lines: Vec<usize> = Vec::new();
    let mut visual_line_idx = 0;

    for (idx, element) in elements.iter().enumerate() {
        if !should_render_collapsed_line(element, idx, collapse) {
            continue;
        }

        let rendered = render(element, width);
        let line_count = rendered.len();

        let text = element_to_plain_text_for_filter(&element.kind);
        let text_lower = text.to_lowercase();

        if text_lower.contains(&filter_lower) || filter_lower.is_empty() {
            for offset in 0..line_count {
                filtered_visual_lines.push(visual_line_idx + offset + 1);
            }
        }

        visual_line_idx += line_count;
    }

    filtered_visual_lines
}

#[allow(dead_code)]
fn find_next_filtered_line(
    content: &str,
    filter_text: &str,
    collapse: &CollapseState,
    current_visual_line: usize,
    width: usize,
) -> Option<usize> {
    let filtered = get_filtered_visual_lines(content, filter_text, collapse, width);
    if filtered.is_empty() {
        return None;
    }

    let mut search_idx = 0;
    for (i, &line) in filtered.iter().enumerate() {
        if line >= current_visual_line {
            search_idx = i;
            break;
        }
        search_idx = i + 1;
    }

    filtered.get(search_idx).copied()
}

#[allow(dead_code)]
fn find_prev_filtered_line(
    content: &str,
    filter_text: &str,
    collapse: &CollapseState,
    current_visual_line: usize,
    width: usize,
) -> Option<usize> {
    let filtered = get_filtered_visual_lines(content, filter_text, collapse, width);
    if filtered.is_empty() {
        return None;
    }

    for (_i, &line) in filtered.iter().enumerate().rev() {
        if line < current_visual_line {
            return Some(line);
        }
    }

    filtered.last().copied()
}

fn text_segment_to_string(segment: &TextSegment) -> String {
    match segment {
        TextSegment::Plain(s) => s.clone(),
        TextSegment::Bold(s) => s.clone(),
        TextSegment::Italic(s) => s.clone(),
        TextSegment::BoldItalic(s) => s.clone(),
        TextSegment::InlineCode(s) => s.clone(),
        TextSegment::Link { text, .. } => text.clone(),
        TextSegment::Strikethrough(s) => s.clone(),
        TextSegment::Html(s) => s.clone(),
        TextSegment::Checkbox(_) => String::new(),
    }
}

pub fn element_to_plain_text_for_filter(kind: &ElementKind) -> String {
    match kind {
        ElementKind::Heading { text, .. } => text
            .iter()
            .map(text_segment_to_string)
            .collect::<Vec<_>>()
            .join(""),
        ElementKind::Paragraph(segments) => segments
            .iter()
            .map(text_segment_to_string)
            .collect::<Vec<_>>()
            .join(""),
        ElementKind::ListItem { content, .. } => content
            .iter()
            .map(text_segment_to_string)
            .collect::<Vec<_>>()
            .join(""),
        ElementKind::Blockquote { content, .. } => content
            .iter()
            .map(text_segment_to_string)
            .collect::<Vec<_>>()
            .join(""),
        ElementKind::CodeBlockContent { content, .. } => content.clone(),
        ElementKind::TableRow { cells, .. } => cells.join(" | "),
        ElementKind::FrontmatterField { key, value, .. } => format!("{}: {}", key, value),
        ElementKind::Expandable { .. } => String::new(),
        _ => String::new(),
    }
}

/// Calculate the scrollbar area for a given content area.
use ratatui::layout::Rect;

impl<'a> MarkdownWidget<'a> {
    /// Calculate the scrollbar area based on the content area.
    ///
    /// Returns `Some(Rect)` if the scrollbar should be shown, `None` otherwise.
    ///
    /// # Arguments
    ///
    /// * `area` - The main widget area
    pub fn calculate_scrollbar_area(&self, area: Rect) -> Option<Rect> {
        // Calculate content area (same logic as render)
        let content_area = if self.show_statusline && area.height > 1 {
            Rect {
                height: area.height.saturating_sub(1),
                ..area
            }
        } else {
            area
        };

        // Only show scrollbar if content exceeds viewport
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
}

/// Calculate the TOC area.

impl<'a> MarkdownWidget<'a> {
    /// Calculate the TOC area based on current widget configuration.
    ///
    /// Uses dynamic dimensions based on content:
    /// - Expanded mode: width fits all headers, height fits all entries
    /// - Compact mode: fixed width, height based on entry count and line spacing
    ///
    /// # Arguments
    ///
    /// * `total_area` - The total area available for the widget
    ///
    /// # Returns
    ///
    /// `Some(Rect)` with the TOC area if TOC is enabled, `None` otherwise.
    pub fn calculate_toc_area(&self, total_area: Rect) -> Option<Rect> {
        if !self.show_toc {
            return None;
        }

        // Account for statusline
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

        // Use dynamic dimensions matching the rendering code
        let toc_width = if self.toc_hovered {
            // Dynamic width based on content for expanded mode
            Toc::required_expanded_width(&self.content, self.toc_config.show_border)
                .min(main_area.width.saturating_sub(padding_right + 4))
        } else {
            self.toc_config.compact_width
        };

        let toc_height = if self.toc_hovered {
            // Expanded: one row per entry
            Toc::required_height(&self.content, self.toc_config.show_border)
                .min(main_area.height.saturating_sub(1))
        } else {
            // Compact: based on entries and line_spacing
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
}

/// Direct event handling methods for MarkdownWidget.
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::events::MarkdownEvent;

impl MarkdownWidget<'_> {
    /// Handle a keyboard event for navigation and actions.
    ///
    /// This is a convenience method that delegates to `handle_key_event`.
    /// The widget manages all state internally.
    ///
    /// # Arguments
    ///
    /// * `key` - The keyboard event to handle
    ///
    /// # Returns
    ///
    /// A `MarkdownEvent` indicating what action was taken.
    pub fn handle_key(&mut self, key: KeyEvent) -> MarkdownEvent {
        self.handle_key_event(key)
    }

    /// Handle a mouse event for all interactions.
    ///
    /// This is a convenience method that delegates to the internal handler.
    /// The widget manages all state internally.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event to handle
    /// * `area` - The area the widget occupies (for bounds checking)
    ///
    /// # Returns
    ///
    /// A `MarkdownEvent` indicating what action was taken.
    pub fn handle_mouse(&mut self, event: MouseEvent, area: Rect) -> MarkdownEvent {
        self.handle_mouse_internal(&event, area)
    }

    /// Update git stats for the current content.
    ///
    /// This method loads git stats for the file if the source is a file path.
    pub fn update_git_stats(&mut self) {
        self.git_stats_state.update(self.source.source_path());
    }
}

/// Git stats setter for MarkdownWidget.

impl<'a> MarkdownWidget<'a> {
    /// Set the git statistics to display in the statusline.
    ///
    /// # Arguments
    ///
    /// * `stats` - The git statistics (additions, modified, deletions)
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn git_stats(mut self, stats: GitStats) -> Self {
        self.git_stats = Some(stats);
        self
    }

    /// Set the git statistics from an optional value.
    ///
    /// This is useful when the git stats may or may not be available,
    /// such as when fetching from a scroll manager.
    ///
    /// # Arguments
    ///
    /// * `stats` - Optional git statistics
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn maybe_git_stats(mut self, stats: Option<GitStats>) -> Self {
        self.git_stats = stats;
        self
    }

    /// Set the git statistics from a tuple (additions, modified, deletions).
    ///
    /// # Arguments
    ///
    /// * `additions` - Lines added
    /// * `modified` - Files/lines modified
    /// * `deletions` - Lines deleted
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn git_stats_tuple(mut self, additions: usize, modified: usize, deletions: usize) -> Self {
        self.git_stats = Some(GitStats {
            additions,
            modified,
            deletions,
        });
        self
    }
}

/// Handle keyboard events for the markdown widget.

impl<'a> MarkdownWidget<'a> {
    /// Handle a keyboard event for navigation and actions.
    ///
    /// This method handles:
    /// - `j` / `Down`: Move focused line down (scrolls when near edge)
    /// - `k` / `Up`: Move focused line up (scrolls when near edge)
    /// - `PageDown`: Scroll down by viewport height
    /// - `PageUp`: Scroll up by viewport height
    /// - `Home` / `gg`: Go to top
    /// - `End` / `G`: Go to bottom
    /// - `/`: Enter filter mode
    /// - `Esc`: Exit selection mode or filter mode
    /// - `y`: Copy selection to clipboard (when selection active)
    /// - `Ctrl+Shift+C`: Copy selection to clipboard
    ///
    /// Returns a `MarkdownEvent` indicating what action was taken.
    pub fn handle_key_event(&mut self, key: KeyEvent) -> MarkdownEvent {
        // Handle filter mode first
        if self.filter_mode {
            return self.handle_filter_key(key);
        }

        // Handle selection-related keys first
        if key.code == KeyCode::Esc && self.selection.is_active() {
            self.selection.exit();
            self.selection_active = false;
            self.mode = MarkdownWidgetMode::Normal;
            self.vim.clear_pending_g();
            return MarkdownEvent::SelectionEnded;
        }

        // Copy selection with 'y' (vim-style)
        if key.code == KeyCode::Char('y') && self.selection.has_selection() {
            if let Some(text) = self.selection.get_selected_text() {
                if !text.is_empty() {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        if clipboard.set_text(&text).is_ok() {
                            self.selection.exit();
                            self.selection_active = false;
                            self.mode = MarkdownWidgetMode::Normal;
                            self.vim.clear_pending_g();
                            return MarkdownEvent::Copied { text };
                        }
                    }
                }
            }
        }

        // Copy selection with Ctrl+Shift+C
        if key.code == KeyCode::Char('C')
            && key.modifiers.contains(KeyModifiers::CONTROL)
            && key.modifiers.contains(KeyModifiers::SHIFT)
        {
            if let Some(text) = self.selection.get_selected_text() {
                if !text.is_empty() {
                    if let Ok(mut clipboard) = arboard::Clipboard::new() {
                        if clipboard.set_text(&text).is_ok() {
                            self.selection.exit();
                            self.selection_active = false;
                            self.mode = MarkdownWidgetMode::Normal;
                            self.vim.clear_pending_g();
                            return MarkdownEvent::Copied { text };
                        }
                    }
                }
            }
        }

        // Handle vim-style 'gg' for go to top
        if key.code == KeyCode::Char('g') {
            if self.vim.check_pending_gg() {
                // Second 'g' within timeout - go to top
                self.scroll.scroll_to_top();
                return MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                };
            }
            // First 'g' or timeout expired - set pending
            self.vim.set_pending_g();
            return MarkdownEvent::None;
        }

        // Any other key clears pending 'g'
        self.vim.clear_pending_g();

        // Handle navigation keys
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
                // Move focused line down (scrolls when near edge)
                self.scroll.line_down();
                MarkdownEvent::FocusedLine {
                    line: self.scroll.current_line,
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                // Move focused line up (scrolls when near edge)
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
                    direction: (self.scroll.scroll_offset.saturating_sub(old_offset) as i32),
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

    /// Handle a keyboard event in filter mode.
    ///
    /// This method handles:
    /// - `Esc`: Exit filter mode and keep filter text
    /// - `Enter`: Exit filter mode, clear filter, and jump to line
    /// - `Backspace`: Remove last character from filter
    /// - `Char(c)`: Add character to filter
    /// - `j` / `Down` / `Ctrl+n`: Move to next filtered line
    /// - `k` / `Up` / `Ctrl+p`: Move to previous filtered line
    fn handle_filter_key(&mut self, key: KeyEvent) -> MarkdownEvent {
        match key.code {
            KeyCode::Esc => {
                let focused_line = self.scroll.current_line;
                // Clear filter and exit filter mode
                self.filter_mode = false;
                self.filter = None;
                self.mode = MarkdownWidgetMode::Normal;
                // Sync to ScrollState
                self.scroll.filter_mode = false;
                self.scroll.filter = None;
                // Clear render cache so all content is shown again
                self.cache.render = None;
                MarkdownEvent::FilterModeExited { line: focused_line }
            }
            KeyCode::Enter => {
                let focused_line = self.scroll.current_line;
                // Clear filter and exit filter mode
                self.filter_mode = false;
                self.filter = None;
                self.mode = MarkdownWidgetMode::Normal;
                // Sync to ScrollState
                self.scroll.filter_mode = false;
                self.scroll.filter = None;
                // Clear render cache so all content is shown again
                self.cache.render = None;
                MarkdownEvent::FilterModeExited { line: focused_line }
            }
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

    /// Find the next line that matches the filter text (by original line number).
    fn find_next_filter_match(&self, filter: String) -> Option<usize> {
        if filter.is_empty() {
            return None;
        }
        let filter_lower = filter.to_lowercase();
        let elements =
            crate::widgets::markdown_preview::widgets::markdown_widget::foundation::parser::render_markdown_to_elements(
                &self.content,
                true,
            );
        let current = self.scroll.current_line;

        for (idx, element) in elements.iter().enumerate() {
            let line_num = idx + 1;
            if line_num <= current {
                continue;
            }
            if !crate::widgets::markdown_preview::widgets::markdown_widget::extensions::selection::should_render_line(
                element,
                idx,
                &self.collapse,
            ) {
                continue;
            }
            let text = element_to_plain_text_for_filter(&element.kind).to_lowercase();
            if text.contains(&filter_lower) {
                return Some(line_num);
            }
        }
        None
    }

    /// Find the previous line that matches the filter text (by original line number).
    fn find_prev_filter_match(&self, filter: String) -> Option<usize> {
        if filter.is_empty() {
            return None;
        }
        let filter_lower = filter.to_lowercase();
        let elements =
            crate::widgets::markdown_preview::widgets::markdown_widget::foundation::parser::render_markdown_to_elements(
                &self.content,
                true,
            );
        let current = self.scroll.current_line;

        for (idx, element) in elements.iter().enumerate().rev() {
            let line_num = idx + 1;
            if line_num >= current {
                continue;
            }
            if !crate::widgets::markdown_preview::widgets::markdown_widget::extensions::selection::should_render_line(
                element,
                idx,
                &self.collapse,
            ) {
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

/// Handle mouse events for the markdown widget.
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::scrollbar::{
    click_to_offset, is_in_scrollbar_area,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::selection::should_render_line;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::helpers::is_in_area;

impl<'a> MarkdownWidget<'a> {
    /// Internal mouse event handler with all logic.
    pub(crate) fn handle_mouse_internal(
        &mut self,
        event: &MouseEvent,
        area: Rect,
    ) -> MarkdownEvent {
        if !is_in_area(event.column, event.row, area) {
            // Click outside area exits selection mode
            if self.selection.is_active() {
                self.selection.exit();
                self.selection_active = false;
                return MarkdownEvent::SelectionEnded;
            }
            return MarkdownEvent::None;
        }

        let border_offset = if self.bordered { 1 } else { 0 };
        let relative_y = event.row.saturating_sub(area.y + border_offset) as usize;
        let relative_x = event.column.saturating_sub(area.x) as usize;
        let width = area.width as usize;

        // Document coordinates (accounting for scroll)
        let document_y = (relative_y + self.scroll.scroll_offset) as i32;
        let document_x = relative_x as i32;

        // Check if mouse is over TOC area - handle TOC scrolling if so
        if self.show_toc {
            if let Some(toc_area) = self.calculate_toc_area(area) {
                let is_over_toc = event.column >= toc_area.x
                    && event.column < toc_area.x + toc_area.width
                    && event.row >= toc_area.y
                    && event.row < toc_area.y + toc_area.height;

                if is_over_toc {
                    match event.kind {
                        MouseEventKind::Moved => {
                            let prev_hovered = self.toc_hovered;
                            let prev_entry = self.toc_hovered_entry;
                            self.handle_toc_hover_internal(event, toc_area);
                            if prev_hovered != self.toc_hovered
                                || prev_entry != self.toc_hovered_entry
                            {
                                return MarkdownEvent::TocHoverChanged {
                                    hovered: self.toc_hovered,
                                };
                            }
                            return MarkdownEvent::None;
                        }
                        MouseEventKind::Down(MouseButton::Left) => {
                            if self.handle_toc_click_internal(event, toc_area) {
                                return MarkdownEvent::Scrolled {
                                    offset: self.scroll.scroll_offset,
                                    direction: 0,
                                };
                            }
                            return MarkdownEvent::None;
                        }
                        MouseEventKind::ScrollUp => {
                            self.toc_scroll_offset = self.toc_scroll_offset.saturating_sub(1);
                            self.update_toc_hovered_entry(event.column, event.row, toc_area);
                            return MarkdownEvent::None;
                        }
                        MouseEventKind::ScrollDown => {
                            let entry_count = self
                                .toc_state
                                .as_ref()
                                .map(|s| s.entry_count())
                                .unwrap_or(0);
                            let visible_height = toc_area.height as usize;
                            let max_offset = entry_count.saturating_sub(visible_height);
                            if self.toc_scroll_offset < max_offset {
                                self.toc_scroll_offset += 1;
                            }
                            self.update_toc_hovered_entry(event.column, event.row, toc_area);
                            return MarkdownEvent::None;
                        }
                        _ => {}
                    }
                } else if matches!(event.kind, MouseEventKind::Moved) {
                    let was_hovered = self.toc_hovered || self.toc_hovered_entry.is_some();
                    self.toc_hovered = false;
                    self.toc_hovered_entry = None;
                    if was_hovered {
                        return MarkdownEvent::TocHoverChanged { hovered: false };
                    }
                }
            }
        }

        // Check if click is on scrollbar area (rightmost column(s) of content area)
        if let Some(scrollbar_area) = self.calculate_scrollbar_area(area) {
            if is_in_scrollbar_area(event.column, event.row, scrollbar_area) {
                match event.kind {
                    MouseEventKind::Down(MouseButton::Left)
                    | MouseEventKind::Drag(MouseButton::Left) => {
                        // Click or drag on scrollbar - jump to position
                        let new_offset = click_to_offset(event.row, scrollbar_area, &self.scroll);
                        self.scroll.scroll_offset = new_offset;
                        return MarkdownEvent::Scrolled {
                            offset: new_offset,
                            direction: 0,
                        };
                    }
                    MouseEventKind::ScrollUp => {
                        let old_offset = self.scroll.scroll_offset;
                        self.scroll.scroll_up(5);
                        return MarkdownEvent::Scrolled {
                            offset: self.scroll.scroll_offset,
                            direction: -(old_offset.saturating_sub(self.scroll.scroll_offset)
                                as i32),
                        };
                    }
                    MouseEventKind::ScrollDown => {
                        let old_offset = self.scroll.scroll_offset;
                        self.scroll.scroll_down(5);
                        return MarkdownEvent::Scrolled {
                            offset: self.scroll.scroll_offset,
                            direction: (self.scroll.scroll_offset.saturating_sub(old_offset)
                                as i32),
                        };
                    }
                    _ => {}
                }
            }
        }

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // Exit active selection on new click
                if self.selection.is_active() {
                    self.selection.exit();
                    self.selection_active = false;
                }

                // Process click for double-click detection
                // Pass current scroll_offset so it can be stored for accurate line calculation later
                let (is_double, _should_process_pending) = self.double_click.process_click(
                    event.column,
                    event.row,
                    self.scroll.scroll_offset,
                );

                if is_double {
                    // Double-click: store info for app to retrieve, return None
                    if let Some(evt) = self.get_line_info_at_position(relative_y, width) {
                        self.last_double_click = Some((evt.0, evt.1, evt.2));
                    }
                    return MarkdownEvent::None;
                }

                // Single click: highlight the clicked line (set as current line)
                let clicked_line = self.scroll.scroll_offset + relative_y + 1; // 1-indexed
                if clicked_line <= self.scroll.total_lines {
                    self.scroll.set_current_line(clicked_line);
                }

                MarkdownEvent::FocusedLine { line: clicked_line }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                let event_result = if !self.selection.is_active() {
                    // Start selection on drag
                    self.selection.enter(
                        document_x,
                        document_y,
                        self.rendered_lines.clone(),
                        width,
                    );
                    self.selection_active = true;
                    self.selection.anchor = Some(SelectionPos::new(document_x, document_y));
                    self.mode = MarkdownWidgetMode::Drag;
                    MarkdownEvent::SelectionStarted
                } else {
                    MarkdownEvent::None
                };

                // Update cursor position during drag
                self.selection.update_cursor(document_x, document_y);

                event_result
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if self.selection.is_active() {
                    // Selection complete - auto-copy to clipboard.
                    let copied_text = if self.selection.has_selection() {
                        self.selection.frozen_lines = Some(self.rendered_lines.clone());
                        self.selection.frozen_width = width;
                        self.selection.get_selected_text()
                    } else {
                        None
                    };

                    self.selection.exit();
                    self.selection_active = false;
                    self.mode = MarkdownWidgetMode::Normal;

                    if let Some(text) = copied_text {
                        if !text.is_empty() {
                            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                                if clipboard.set_text(&text).is_ok() {
                                    self.selection.last_copied_text = Some(text.clone());
                                    return MarkdownEvent::Copied { text };
                                }
                            }
                        }
                    }

                    return MarkdownEvent::SelectionEnded;
                }

                MarkdownEvent::None
            }
            MouseEventKind::ScrollUp => {
                let old_offset = self.scroll.scroll_offset;
                self.scroll.scroll_up(5);
                MarkdownEvent::Scrolled {
                    offset: self.scroll.scroll_offset,
                    direction: -(old_offset.saturating_sub(self.scroll.scroll_offset) as i32),
                }
            }
            MouseEventKind::ScrollDown => {
                let old_offset = self.scroll.scroll_offset;
                self.scroll.scroll_down(5);
                MarkdownEvent::Scrolled {
                    offset: self.scroll.scroll_offset,
                    direction: (self.scroll.scroll_offset.saturating_sub(old_offset) as i32),
                }
            }
            _ => MarkdownEvent::None,
        };

        self.check_pending_click_internal(area)
    }

    fn handle_toc_hover_internal(&mut self, event: &MouseEvent, toc_area: Rect) {
        let auto_state = TocState::from_content(&self.content);
        let toc_state = if let Some(provided) = &self.toc_state {
            if provided.entries.is_empty() {
                &auto_state
            } else {
                provided
            }
        } else {
            &auto_state
        };

        let toc = Toc::new(toc_state)
            .expanded(self.toc_hovered)
            .config(self.toc_config.clone());

        self.toc_hovered = true;
        self.toc_hovered_entry = toc.entry_at_position(event.column, event.row, toc_area);
    }

    fn handle_toc_click_internal(&mut self, event: &MouseEvent, toc_area: Rect) -> bool {
        let auto_state = TocState::from_content(&self.content);
        let toc_state = if let Some(provided) = &self.toc_state {
            if provided.entries.is_empty() {
                &auto_state
            } else {
                provided
            }
        } else {
            &auto_state
        };

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

    fn check_pending_click_internal(&mut self, area: Rect) -> MarkdownEvent {
        if let Some((x, y, click_scroll_offset)) = self.double_click.check_pending_timeout() {
            let relative_y = y.saturating_sub(area.y) as usize;
            let relative_x = x.saturating_sub(area.x) as usize;
            let width = area.width as usize;

            let clicked_line = click_scroll_offset + relative_y + 1;
            if clicked_line <= self.scroll.total_lines {
                self.scroll.set_current_line(clicked_line);
            }

            if self.handle_click_collapse(relative_x, relative_y, width) {
                if let Some((_, line_kind, text)) =
                    self.get_line_info_at_position(relative_y, width)
                {
                    if line_kind == "Heading" {
                        return MarkdownEvent::HeadingToggled {
                            level: 1,
                            text,
                            collapsed: true,
                        };
                    }
                }
            }

            return MarkdownEvent::FocusedLine { line: clicked_line };
        }

        MarkdownEvent::None
    }

    /// Handle click for collapse/expand functionality.
    ///
    /// Returns `true` if a collapsible element was toggled.
    fn handle_click_collapse(&mut self, _x: usize, y: usize, width: usize) -> bool {
        let elements = render_markdown_to_elements(&self.content, true);

        // Account for scroll offset - y is relative to visible area
        let document_y = y + self.scroll.scroll_offset;
        let mut line_idx = 0;

        for (idx, element) in elements.iter().enumerate() {
            // Skip elements that shouldn't be rendered (collapsed sections)
            if !should_render_line(element, idx, &self.collapse) {
                continue;
            }

            let rendered = render(element, width);
            let line_count = rendered.len();

            if document_y >= line_idx && document_y < line_idx + line_count {
                match &element.kind {
                    ElementKind::Heading { section_id, .. } => {
                        // Only collapse headings if show_heading_collapse is enabled
                        if self.display.show_heading_collapse {
                            self.collapse.toggle_section(*section_id);
                            self.cache.invalidate();
                            return true;
                        }
                    }
                    ElementKind::Frontmatter { .. } => {
                        self.collapse.toggle_section(0);
                        self.cache.invalidate();
                        return true;
                    }
                    ElementKind::FrontmatterStart { .. } => {
                        self.collapse.toggle_section(0);
                        self.cache.invalidate();
                        return true;
                    }
                    ElementKind::ExpandToggle { content_id, .. } => {
                        self.expandable.toggle(content_id);
                        self.cache.invalidate();
                        return true;
                    }
                    _ => {}
                }
            }

            line_idx += line_count;
        }

        false
    }

    /// Get line information at a given screen position.
    ///
    /// Returns (line_number, line_kind, content) if found.
    pub fn get_line_info_at_position(
        &self,
        y: usize,
        width: usize,
    ) -> Option<(usize, String, String)> {
        let elements = render_markdown_to_elements(&self.content, true);
        let document_y = y + self.scroll.scroll_offset;
        let mut visual_line_idx = 0;
        let mut logical_line_num = 0;

        for (idx, element) in elements.iter().enumerate() {
            if !should_render_line(element, idx, &self.collapse) {
                continue;
            }

            logical_line_num += 1;

            let rendered = render(element, width);
            let line_count = rendered.len();

            if document_y >= visual_line_idx && document_y < visual_line_idx + line_count {
                let line_kind = match &element.kind {
                    ElementKind::Heading { .. } => "Heading",
                    ElementKind::Paragraph(_) => "Paragraph",
                    ElementKind::CodeBlockHeader { .. } => "CodeBlockHeader",
                    ElementKind::CodeBlockContent { .. } => "CodeBlockContent",
                    ElementKind::CodeBlockBorder { .. } => "CodeBlockBorder",
                    ElementKind::ListItem { .. } => "ListItem",
                    ElementKind::Blockquote { .. } => "Blockquote",
                    ElementKind::Empty => "Empty",
                    ElementKind::HorizontalRule => "HorizontalRule",
                    ElementKind::Frontmatter { .. } => "Frontmatter",
                    ElementKind::FrontmatterStart { .. } => "FrontmatterStart",
                    ElementKind::FrontmatterField { .. } => "FrontmatterField",
                    ElementKind::FrontmatterEnd => "FrontmatterEnd",
                    ElementKind::Expandable { .. } => "Expandable",
                    ElementKind::ExpandToggle { .. } => "ExpandToggle",
                    ElementKind::TableRow { .. } => "TableRow",
                    ElementKind::TableBorder(_) => "TableBorder",
                    ElementKind::HeadingBorder { .. } => "HeadingBorder",
                };

                let text_content = self.get_element_text(&element.kind);

                return Some((logical_line_num, line_kind.to_string(), text_content));
            }

            visual_line_idx += line_count;
        }

        None
    }

    /// Extract plain text from an ElementKind.
    fn get_element_text(
        &self,
        kind: &crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::ElementKind,
    ) -> String {
        use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::{
            ElementKind, TextSegment,
        };

        fn segment_to_text(seg: &TextSegment) -> &str {
            match seg {
                TextSegment::Plain(s) => s,
                TextSegment::Bold(s) => s,
                TextSegment::Italic(s) => s,
                TextSegment::BoldItalic(s) => s,
                TextSegment::InlineCode(s) => s,
                TextSegment::Link { text, .. } => text,
                TextSegment::Strikethrough(s) => s,
                TextSegment::Html(s) => s,
                TextSegment::Checkbox(_) => "",
            }
        }

        match kind {
            ElementKind::Heading { text, .. } => text.iter().map(segment_to_text).collect(),
            ElementKind::Paragraph(segments) => segments.iter().map(segment_to_text).collect(),
            ElementKind::CodeBlockContent { content, .. } => content.clone(),
            ElementKind::CodeBlockHeader { language, .. } => language.clone(),
            ElementKind::ListItem { content, .. } => content.iter().map(segment_to_text).collect(),
            ElementKind::Blockquote { content, .. } => {
                content.iter().map(segment_to_text).collect()
            }
            ElementKind::Frontmatter { fields, .. } => fields
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join(", "),
            ElementKind::FrontmatterField { key, value } => format!("{}: {}", key, value),
            ElementKind::TableRow { cells, .. } => cells.join(" | "),
            _ => String::new(),
        }
    }

    /// Set the rendered lines for selection text extraction.
    ///
    /// Call this after rendering to update the cached lines.
    pub fn set_rendered_lines(&mut self, lines: Vec<ratatui::text::Line<'static>>) {
        self.rendered_lines = lines;
    }

    /// Check if selection mode is active.
    pub fn is_selection_active(&self) -> bool {
        self.selection.is_active()
    }

    /// Get the current selection state (for rendering).
    pub fn selection(
        &self,
    ) -> &crate::widgets::markdown_preview::widgets::markdown_widget::state::SelectionState {
        &self.selection
    }

    /// Get line information at the current highlighted line.
    ///
    /// Returns (line_number, line_kind, content) if found.
    pub fn get_current_line_info(&self, width: usize) -> Option<(usize, String, String)> {
        // current_line is 1-indexed document line, get_line_info_at_position expects
        // a relative viewport position, so we need to convert.
        // The document position of current_line is current_line - 1 (0-indexed).
        // Since get_line_info_at_position adds scroll_offset, we pass (current_line - 1).
        let document_y = self.scroll.current_line.saturating_sub(1);
        let elements = render_markdown_to_elements(&self.content, true);
        let mut visual_line_idx = 0;
        let mut logical_line_num = 0;

        for (idx, element) in elements.iter().enumerate() {
            if !should_render_line(element, idx, &self.collapse) {
                continue;
            }

            logical_line_num += 1;

            let rendered = render(element, width);
            let line_count = rendered.len();

            if document_y >= visual_line_idx && document_y < visual_line_idx + line_count {
                let line_kind = match &element.kind {
                    ElementKind::Heading { .. } => "Heading",
                    ElementKind::Paragraph(_) => "Paragraph",
                    ElementKind::CodeBlockHeader { .. } => "CodeBlockHeader",
                    ElementKind::CodeBlockContent { .. } => "CodeBlockContent",
                    ElementKind::CodeBlockBorder { .. } => "CodeBlockBorder",
                    ElementKind::ListItem { .. } => "ListItem",
                    ElementKind::Blockquote { .. } => "Blockquote",
                    ElementKind::Empty => "Empty",
                    ElementKind::HorizontalRule => "HorizontalRule",
                    ElementKind::Frontmatter { .. } => "Frontmatter",
                    ElementKind::FrontmatterStart { .. } => "FrontmatterStart",
                    ElementKind::FrontmatterField { .. } => "FrontmatterField",
                    ElementKind::FrontmatterEnd => "FrontmatterEnd",
                    ElementKind::Expandable { .. } => "Expandable",
                    ElementKind::ExpandToggle { .. } => "ExpandToggle",
                    ElementKind::TableRow { .. } => "TableRow",
                    ElementKind::TableBorder(_) => "TableBorder",
                    ElementKind::HeadingBorder { .. } => "HeadingBorder",
                };

                let text_content = self.get_element_text(&element.kind);

                return Some((logical_line_num, line_kind.to_string(), text_content));
            }

            visual_line_idx += line_count;
        }

        None
    }
}

/// Handle TOC click events for scroll-to-heading navigation.

impl<'a> MarkdownWidget<'a> {
    /// Handle a click on the TOC to scroll to the selected heading.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event
    /// * `area` - The total widget area
    ///
    /// # Returns
    ///
    /// `true` if the click was handled (was on a TOC entry), `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_run
    /// // In your event loop:
    /// if let Event::Mouse(mouse_event) = event {
    ///     if matches!(mouse_event.kind, MouseEventKind::Down(MouseButton::Left)) {
    ///         if widget.handle_toc_click(&mouse_event, area) {
    ///             // Click was handled - you may want to redraw
    ///         }
    ///     }
    /// }
    /// ```
    pub fn handle_toc_click(&mut self, event: &MouseEvent, area: Rect) -> bool {
        // Only handle left clicks
        if !matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
            return false;
        }

        // Get the TOC area
        let toc_area = match self.calculate_toc_area(area) {
            Some(t_area) => t_area,
            None => return false,
        };

        // Check horizontal bounds and if above TOC
        // Don't check lower vertical bound - let entry_at_position handle that
        // based on actual entry count
        if event.column < toc_area.x
            || event.column >= toc_area.x + toc_area.width
            || event.row < toc_area.y
        {
            return false;
        }

        // Create state from content with entries
        let auto_state = TocState::from_content(&self.content);
        let toc_state = if let Some(provided) = &self.toc_state {
            if provided.entries.is_empty() {
                &auto_state
            } else {
                provided
            }
        } else {
            &auto_state
        };

        // Create a TOC to find the clicked entry
        let toc = Toc::new(toc_state)
            .expanded(self.toc_hovered) // Use current expansion state
            .config(self.toc_config.clone());

        // Find which entry was clicked
        if let Some(entry_idx) = toc.entry_at_position(event.column, event.row, toc_area) {
            // Get the target line number
            if let Some(target_line) = toc.click_to_line(entry_idx) {
                // Scroll to the heading (a bit above for context)
                let new_offset = target_line.saturating_sub(2);

                // Clamp to valid range
                let max_offset = self
                    .scroll
                    .total_lines
                    .saturating_sub(self.scroll.viewport_height);
                self.scroll.scroll_offset = new_offset.min(max_offset);

                // Update current line
                self.scroll.current_line = target_line.saturating_add(1); // 1-indexed

                // Update hovered entry to match the clicked entry
                self.toc_hovered_entry = Some(entry_idx);

                return true;
            }
        }

        false
    }

    /// Handle a click on the TOC in a specific area (for when area is pre-calculated).
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event
    /// * `toc_area` - The pre-calculated TOC area
    ///
    /// # Returns
    ///
    /// `true` if the click was handled (was on a TOC entry), `false` otherwise.
    pub fn handle_toc_click_in_area(&mut self, event: &MouseEvent, toc_area: Rect) -> bool {
        // Only handle left clicks
        if !matches!(event.kind, MouseEventKind::Down(MouseButton::Left)) {
            return false;
        }

        // Check horizontal bounds and if above TOC
        // Don't check lower vertical bound - let entry_at_position handle that
        if event.column < toc_area.x
            || event.column >= toc_area.x + toc_area.width
            || event.row < toc_area.y
        {
            return false;
        }

        // Create state from content with entries
        let auto_state = TocState::from_content(&self.content);
        let toc_state = if let Some(provided) = &self.toc_state {
            if provided.entries.is_empty() {
                &auto_state
            } else {
                provided
            }
        } else {
            &auto_state
        };

        // Create a TOC to find the clicked entry
        let toc = Toc::new(toc_state)
            .expanded(self.toc_hovered)
            .config(self.toc_config.clone());

        // Find which entry was clicked
        if let Some(entry_idx) = toc.entry_at_position(event.column, event.row, toc_area) {
            // Get the target line number
            if let Some(target_line) = toc.click_to_line(entry_idx) {
                // Scroll to the heading
                let new_offset = target_line.saturating_sub(2);
                let max_offset = self
                    .scroll
                    .total_lines
                    .saturating_sub(self.scroll.viewport_height);
                self.scroll.scroll_offset = new_offset.min(max_offset);
                self.scroll.current_line = target_line.saturating_add(1);

                // Update hovered entry to match the clicked entry
                self.toc_hovered_entry = Some(entry_idx);

                return true;
            }
        }

        false
    }
}

/// Handle TOC hover events for interactive expansion and entry highlight.

impl<'a> MarkdownWidget<'a> {
    /// Handle mouse move events to detect TOC hover.
    ///
    /// Call this method with `MouseEventKind::Moved` events to track
    /// whether the mouse is hovering over the TOC area and which entry
    /// is being hovered.
    ///
    /// # Arguments
    ///
    /// * `event` - The mouse event (should be a Moved event)
    /// * `area` - The total widget area
    ///
    /// # Returns
    ///
    /// `true` if the hover state changed (entered/exited hover or hovered entry changed),
    /// `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_run
    /// // In your event loop:
    /// if let Event::Mouse(mouse_event) = event {
    ///     if matches!(mouse_event.kind, MouseEventKind::Moved) {
    ///         if widget.handle_toc_hover(&mouse_event, area) {
    ///             // Hover state changed - you may want to redraw
    ///         }
    ///     }
    /// }
    /// ```
    pub fn handle_toc_hover(&mut self, event: &MouseEvent, area: Rect) -> bool {
        // Only process move events
        if !matches!(event.kind, MouseEventKind::Moved) {
            return false;
        }

        // Get the TOC area
        let toc_area = match self.calculate_toc_area(area) {
            Some(t_area) => t_area,
            None => {
                // TOC not visible, ensure not hovered
                let changed = self.toc_hovered || self.toc_hovered_entry.is_some();
                if changed {
                    self.toc_hovered = false;
                    self.toc_hovered_entry = None;
                }
                return changed;
            }
        };

        // Check if mouse is within TOC area horizontally and at or below top
        // Don't check lower vertical bound - let entry_at_position handle that
        // based on actual entry count
        let is_potentially_over_toc = event.column >= toc_area.x
            && event.column < toc_area.x + toc_area.width
            && event.row >= toc_area.y;

        let prev_hovered = self.toc_hovered;
        let prev_entry = self.toc_hovered_entry;

        if is_potentially_over_toc {
            // Create state from content with entries
            let auto_state = TocState::from_content(&self.content);
            let toc_state = if let Some(provided) = &self.toc_state {
                if provided.entries.is_empty() {
                    &auto_state
                } else {
                    provided
                }
            } else {
                &auto_state
            };

            // Try to find an entry at this position
            // Use compact mode when not hovered, expanded mode when hovered
            let toc = Toc::new(toc_state)
                .expanded(self.toc_hovered)
                .config(self.toc_config.clone());

            self.toc_hovered = true;
            self.toc_hovered_entry = toc.entry_at_position(event.column, event.row, toc_area);
        } else {
            self.toc_hovered = false;
            self.toc_hovered_entry = None;
        }

        // Check if any state changed
        prev_hovered != self.toc_hovered || prev_entry != self.toc_hovered_entry
    }

    /// Check if the TOC is currently being hovered.
    ///
    /// # Returns
    ///
    /// `true` if the mouse is over the TOC, `false` otherwise.
    pub fn is_toc_hovered(&self) -> bool {
        self.toc_hovered
    }

    /// Get the currently hovered TOC entry index.
    ///
    /// # Returns
    ///
    /// The index of the hovered entry, or `None` if no entry is hovered.
    pub fn get_toc_hovered_entry(&self) -> Option<usize> {
        self.toc_hovered_entry
    }

    /// Set the TOC hover state directly.
    ///
    /// Useful for manually controlling hover state in tests or special scenarios.
    ///
    /// # Arguments
    ///
    /// * `hovered` - Whether the TOC should be considered hovered.
    pub fn set_toc_hovered(&mut self, hovered: bool) {
        self.toc_hovered = hovered;
        if !hovered {
            self.toc_hovered_entry = None;
        }
    }

    /// Get the current TOC scroll offset.
    ///
    /// # Returns
    ///
    /// The current scroll offset for the TOC list.
    pub fn get_toc_scroll_offset(&self) -> usize {
        self.toc_scroll_offset
    }

    /// Set the TOC scroll offset directly.
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset for the TOC list.
    pub fn set_toc_scroll_offset(&mut self, offset: usize) {
        self.toc_scroll_offset = offset;
    }

    /// Update the hovered entry based on current mouse position and scroll offset.
    ///
    /// Call this after scrolling the TOC to recalculate which entry is under the cursor.
    ///
    /// # Arguments
    ///
    /// * `x` - Mouse X coordinate
    /// * `y` - Mouse Y coordinate
    /// * `toc_area` - The TOC area rect
    pub fn update_toc_hovered_entry(&mut self, x: u16, y: u16, toc_area: Rect) {
        // Create state from content with entries
        let auto_state = TocState::from_content(&self.content);
        let toc_state = if let Some(provided) = &self.toc_state {
            if provided.entries.is_empty() {
                &auto_state
            } else {
                provided
            }
        } else {
            &auto_state
        };

        let toc = Toc::new(toc_state)
            .expanded(true) // Use expanded mode for entry detection when hovered
            .config(self.toc_config.clone()); // Use same config as rendering

        self.toc_hovered_entry = toc.entry_at_position(x, y, toc_area);
    }
}

/// Set resizing state method for MarkdownWidget.

impl<'a> MarkdownWidget<'a> {
    /// Set whether the widget is currently being resized (for smoother drag performance).
    ///
    /// # Arguments
    ///
    /// * `resizing` - Whether the widget is being resized
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn is_resizing(mut self, resizing: bool) -> Self {
        self.is_resizing = resizing;
        self
    }
}

/// Methods for retrieving the last double-click info and copied text.

impl<'a> MarkdownWidget<'a> {
    /// Get the last double-click info and clear it.
    ///
    /// Call this after processing events to check if a double-click occurred.
    ///
    /// # Returns
    ///
    /// `Some((line_number, line_kind, content))` if a double-click occurred, `None` otherwise.
    pub fn take_last_double_click(&mut self) -> Option<(usize, String, String)> {
        self.last_double_click.take()
    }

    /// Get the last copied text and clear it.
    ///
    /// Call this after processing events to check if text was copied to clipboard.
    /// Use this to show a toast notification when text is copied.
    ///
    /// # Returns
    ///
    /// `Some(text)` if text was copied, `None` otherwise.
    pub fn take_last_copied(&mut self) -> Option<String> {
        self.selection.last_copied_text.take()
    }
}

/// Set mode method for MarkdownWidget.

impl<'a> MarkdownWidget<'a> {
    /// Set the current mode for the statusline.
    ///
    /// # Arguments
    ///
    /// * `mode` - The mode to display (Normal or Drag)
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn mode(mut self, mode: MarkdownWidgetMode) -> Self {
        self.mode = mode;
        self
    }
}

/// Statusline rendering for MarkdownWidget.
use unicode_width::UnicodeWidthStr;

use crate::primitives::statusline::{StatusLineStacked, SLANT_BL_TR, SLANT_TL_BR};

impl<'a> MarkdownWidget<'a> {
    /// Render the statusline using StatusLineStacked (powerline style).
    ///
    /// The statusline displays:
    /// - Mode indicator (NORMAL/DRAG/FILTER) on the left with colored background
    /// - Filename with git stats (no background on git icons)
    /// - Scroll position (percentage/total lines) on the right
    ///
    /// If an app theme is set, uses theme colors. Otherwise falls back to defaults.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the statusline in
    /// * `buf` - The buffer to render to
    pub(crate) fn render_statusline(&self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Get theme-based or default colors
        let (mode_text, mode_color) = match self.mode {
            MarkdownWidgetMode::Normal => {
                let color = self
                    .app_theme
                    .as_ref()
                    .map(|t| t.info)
                    .unwrap_or(Color::Rgb(97, 175, 239)); // blue
                (" NORMAL ".to_string(), color)
            }
            MarkdownWidgetMode::Drag => {
                let color = self
                    .app_theme
                    .as_ref()
                    .map(|t| t.warning)
                    .unwrap_or(Color::Rgb(229, 192, 123)); // yellow/orange
                (" DRAG ".to_string(), color)
            }
            MarkdownWidgetMode::Filter => {
                let color = self
                    .app_theme
                    .as_ref()
                    .map(|t| t.success)
                    .unwrap_or(Color::Rgb(152, 195, 121)); // green
                let filter_text = self.filter.as_deref().unwrap_or("");
                let display_text = format!(" /{} ", filter_text);
                (display_text, color)
            }
        };

        // File segment background - use theme background_panel or default
        let file_bg = self
            .app_theme
            .as_ref()
            .map(|t| t.background_panel)
            .unwrap_or(Color::Rgb(58, 58, 58));

        // Mode text foreground - use theme background or default black
        let mode_fg = self
            .app_theme
            .as_ref()
            .map(|t| t.background)
            .unwrap_or(Color::Black);

        // File text color - use theme text or default white
        let file_fg = self
            .app_theme
            .as_ref()
            .map(|t| t.text)
            .unwrap_or(Color::White);

        // Get filename from source path
        let filename = self
            .source
            .source_path()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());

        // Position info - use source line count for accurate display
        let source_line_count = self.source.line_count();
        let display_total = if source_line_count > 0 {
            source_line_count
        } else {
            self.scroll.total_lines
        };
        let current_line = self.scroll.current_line;
        let percentage = if display_total == 0 {
            0
        } else {
            (current_line * 100) / display_total.max(1)
        };
        let position_text = format!(" {}%/{} ", percentage, display_total);

        // Position segment background - use theme text_muted or default
        let position_bg = self
            .app_theme
            .as_ref()
            .map(|t| t.text_muted)
            .unwrap_or(Color::Rgb(171, 178, 191));

        // Position text foreground - use theme background or default black
        let position_fg = self
            .app_theme
            .as_ref()
            .map(|t| t.background)
            .unwrap_or(Color::Black);

        // Calculate git stats start position
        let git_stats_start_x = {
            let mode_len = mode_text.len() as u16 + 1; // +1 for slant
            let file_len = filename.map(|n| n.len() + 2).unwrap_or(0) as u16 + 1; // +2 for spaces, +1 for slant
            area.x + mode_len + file_len
        };

        // Build the statusline
        let mut statusline = StatusLineStacked::new()
            // Mode segment (left)
            .start(
                Span::from(mode_text.clone()).style(
                    Style::new()
                        .fg(mode_fg)
                        .bg(mode_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::from(SLANT_TL_BR).style(Style::new().fg(mode_color).bg(file_bg)),
            );

        // Filename segment
        if let Some(name) = filename {
            let file_segment = format!(" {} ", name);
            statusline = statusline.start(
                Span::from(file_segment).style(Style::new().fg(file_fg).bg(file_bg)),
                Span::from(SLANT_TL_BR).style(Style::new().fg(file_bg)),
            );
        }

        // Position segment (right)
        statusline = statusline.end(
            Span::from(position_text).style(Style::new().fg(position_fg).bg(position_bg)),
            Span::from(SLANT_BL_TR).style(Style::new().fg(position_bg)),
        );

        // Render the statusline base
        ratatui::widgets::Widget::render(statusline, area, buf);

        // Now render git stats with colored icons (no background)
        // Icons from lvim: LineAdded (U+EADC), LineModified (U+EADE), LineRemoved (U+EADF)
        // Get git stats from git_stats_state (which manages the caching/updates)
        // or fall back to manually-set widget stats
        let git_stats = self.git_stats_state.git_stats().or(self.git_stats);
        if let Some(stats) = &git_stats {
            // Use theme colors for git stats or fall back to defaults
            let green = Style::new().fg(self
                .app_theme
                .as_ref()
                .map(|t| t.success)
                .unwrap_or(Color::Rgb(152, 195, 121)));
            let yellow = Style::new().fg(self
                .app_theme
                .as_ref()
                .map(|t| t.warning)
                .unwrap_or(Color::Rgb(229, 192, 123)));
            let red = Style::new().fg(self
                .app_theme
                .as_ref()
                .map(|t| t.error)
                .unwrap_or(Color::Rgb(224, 108, 117)));
            let dim = Style::new().fg(self
                .app_theme
                .as_ref()
                .map(|t| t.text_muted)
                .unwrap_or(Color::Rgb(92, 99, 112)));

            let mut x = git_stats_start_x;

            // Add margin after filename
            buf.set_string(x, area.y, "  ", dim);
            x += 2;

            // Added: icon space number space
            let add_icon = "\u{EADC}";
            let add_num = format!("{}", stats.additions);
            buf.set_string(x, area.y, add_icon, green);
            x += add_icon.width() as u16;
            buf.set_string(x, area.y, " ", green);
            x += 1;
            buf.set_string(x, area.y, &add_num, green);
            x += add_num.width() as u16;
            buf.set_string(x, area.y, " ", dim);
            x += 1;

            // Modified: icon space number space
            let mod_icon = "\u{EADE}";
            let mod_num = format!("{}", stats.modified);
            buf.set_string(x, area.y, mod_icon, yellow);
            x += mod_icon.width() as u16;
            buf.set_string(x, area.y, " ", yellow);
            x += 1;
            buf.set_string(x, area.y, &mod_num, yellow);
            x += mod_num.width() as u16;
            buf.set_string(x, area.y, " ", dim);
            x += 1;

            // Removed: icon space number
            let del_icon = "\u{EADF}";
            let del_num = format!("{}", stats.deletions);
            buf.set_string(x, area.y, del_icon, red);
            x += del_icon.width() as u16;
            buf.set_string(x, area.y, " ", red);
            x += 1;
            buf.set_string(x, area.y, &del_num, red);
        }
    }
}

impl<'a> MarkdownWidget<'a> {
    pub fn rendered_lines(&self) -> &Vec<ratatui::text::Line<'static>> {
        &self.rendered_lines
    }
}

/// Sync widget state back to MarkdownState.

/// State captured from MarkdownWidget that needs to be synced back to MarkdownState.
///
/// This struct holds the values that the widget may modify during event handling
/// that need to persist back to the application state.
#[derive(Debug, Clone)]
pub struct WidgetStateSync {
    /// The inner area calculated during rendering (inside borders).
    pub inner_area: Rect,
    /// Whether the TOC is currently hovered.
    pub toc_hovered: bool,
    /// Index of the hovered TOC entry.
    pub toc_hovered_entry: Option<usize>,
    /// Scroll offset for the TOC list.
    pub toc_scroll_offset: usize,
    /// Whether selection mode is active.
    pub selection_active: bool,
    /// Last double-click info (line number, kind, content).
    pub last_double_click: Option<(usize, String, String)>,
    /// Current filter text (when in filter mode).
    pub filter: Option<String>,
    /// Whether filter mode is currently active.
    pub filter_mode: bool,
    /// Current scroll offset in the document.
    pub scroll_offset: usize,
    /// Current focused line (1-indexed).
    pub current_line: usize,
}

impl WidgetStateSync {
    /// Create a new sync state with the given inner area.
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

    /// Apply this sync state to a MarkdownState.
    ///
    /// # Arguments
    ///
    /// * `state` - The MarkdownState to sync state to
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

    /// Check if there was a double-click and consume it.
    pub fn take_double_click(&mut self) -> Option<(usize, String, String)> {
        self.last_double_click.take()
    }
}

impl<'a> MarkdownWidget<'a> {
    /// Get the state that needs to be synced back to MarkdownState.
    ///
    /// This method captures the TOC and selection state from the widget
    /// so it can be synced back after the widget is dropped.
    ///
    /// # Returns
    ///
    /// A `WidgetStateSync` struct containing the state values to sync.
    ///
    /// # Example
    ///
    /// ```rust,ignore,ignore
    /// let sync_state = {
    ///     let mut widget = MarkdownWidget::from_state(&content, &mut state).show_toc(true);
    ///     widget.handle_toc_hover(&mouse, render_area);
    ///     widget.handle_toc_click(&mouse, render_area);
    ///     widget.handle_mouse_event(&mouse, render_area);
    ///     widget.get_state_sync()
    /// };
    /// sync_state.apply_to(&mut state);
    /// ```
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

    /// Sync widget state back to MarkdownState by consuming self.
    ///
    /// This method consumes the widget and syncs TOC and selection state back to
    /// the MarkdownState, ensuring state persistence between frames.
    ///
    /// Call this after handling mouse events to preserve hover and selection state.
    ///
    /// # Arguments
    ///
    /// * `state` - The MarkdownState to sync state back to
    ///
    /// # Example
    ///
    /// ```rust,ignore,ignore
    /// let mut widget = MarkdownWidget::from_state(&content, &mut state).show_toc(true);
    /// widget.handle_toc_hover(&mouse, render_area);
    /// widget.handle_toc_click(&mouse, render_area);
    /// widget.handle_mouse_event(&mouse, render_area);
    /// widget.sync_state_back(&mut state);
    /// ```
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

/// Widget trait implementation for MarkdownWidget.
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::{
    render_with_options, RenderOptions,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::helpers::hash_content;
use crate::widgets::markdown_preview::widgets::markdown_widget::state::{ParsedCache, RenderCache};
use unicode_width::UnicodeWidthChar;

/// Current line highlight color (normal - dark blue-gray)
const CURRENT_LINE_BG: Color = Color::Rgb(38, 52, 63);
/// Current line highlight color when dragging (selection is active)
const CURRENT_LINE_DRAG_BG: Color = Color::Rgb(70, 80, 100);

impl<'a> Widget for &mut MarkdownWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Handle pane wrapping if enabled
        let (area, _pane_footer_area) = if self.has_pane {
            let title = self
                .pane_title
                .clone()
                .unwrap_or_else(|| "Markdown".to_string());
            let pane = self.pane.take().unwrap_or_else(|| {
                let mut p = Pane::new(title);
                if let Some(color) = self.pane_color {
                    p = p.border_style(ratatui::style::Style::default().fg(color));
                }
                p
            });

            // Build the block
            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_type(pane.border_type)
                .border_style(pane.border_style)
                .title(pane.title);

            if let Some(icon) = &pane.icon {
                use ratatui::text::Span;
                let title = format!(" {} ", icon);
                block = block.title(Line::from(vec![Span::styled(title, pane.title_style)]));
            }

            if let Some(ref footer) = pane.text_footer {
                block = block.title_bottom(footer.clone().style(pane.footer_style));
            }

            // Get the inner area (inside the border) BEFORE rendering
            let inner = block.inner(area);

            // Render the block
            block.render(area, buf);

            // Calculate footer area if needed
            let (inner, pane_footer) = if pane.footer_height > 0 {
                let chunks = ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Vertical)
                    .constraints([
                        ratatui::layout::Constraint::Min(0),
                        ratatui::layout::Constraint::Length(pane.footer_height),
                    ])
                    .split(inner);
                (chunks[0], Some(chunks[1]))
            } else {
                (inner, None)
            };

            // Calculate padded inner area
            let padded = Rect {
                x: inner.x + pane.padding.3,
                y: inner.y + pane.padding.0,
                width: inner.width.saturating_sub(pane.padding.1 + pane.padding.3),
                height: inner.height.saturating_sub(pane.padding.0 + pane.padding.2),
            };

            // Render footer if exists
            if let Some(footer_area) = pane_footer {
                if let Some(ref footer) = pane.text_footer {
                    footer.render(footer_area, buf);
                }
            }

            (padded, None::<Rect>)
        } else {
            (area, None::<Rect>)
        };

        // Reserve space for statusline if enabled
        let (main_area, statusline_area) = if self.show_statusline && area.height > 1 {
            (
                Rect {
                    height: area.height.saturating_sub(1),
                    ..area
                },
                Some(Rect {
                    y: area.y + area.height.saturating_sub(1),
                    height: 1,
                    ..area
                }),
            )
        } else {
            (area, None)
        };

        let padding_right: u16 = 2;
        let padding_top: u16 = 1;
        let content_area = main_area;

        // Calculate overlay area for TOC
        let overlay_area = if self.show_toc {
            // TOC: compact when not hovered, expanded when hovered
            // Dynamic width based on content for expanded mode
            let toc_width = if self.toc_hovered {
                Toc::required_expanded_width(&self.content, self.toc_config.show_border)
                    .min(main_area.width.saturating_sub(padding_right + 4))
            } else {
                self.toc_config.compact_width
            };
            // Dynamic height based on content
            let toc_height = if self.toc_hovered {
                // Expanded: one row per entry
                Toc::required_height(&self.content, self.toc_config.show_border)
                    .min(main_area.height.saturating_sub(1))
            } else {
                // Compact: based on entries and line_spacing
                Toc::required_compact_height(
                    &self.content,
                    self.toc_config.line_spacing,
                    self.toc_config.show_border,
                )
                .min(main_area.height.saturating_sub(1))
            };

            if main_area.width > toc_width + padding_right + 2 {
                Some(Rect {
                    x: main_area.x + main_area.width.saturating_sub(toc_width + padding_right),
                    y: main_area.y + padding_top,
                    width: toc_width,
                    height: toc_height,
                })
            } else {
                None
            }
        } else {
            None
        };

        self.scroll.update_viewport(content_area);

        // Calculate line number width if document line numbers are enabled
        // Fixed width of 6 chars: "  1 │ " to "999 │ " covers most documents
        let line_num_width = if self.display.show_document_line_numbers {
            6
        } else {
            0
        };

        // Render markdown content (subtract line number width from available width)
        let width = (content_area.width as usize).saturating_sub(line_num_width);
        let content_hash = hash_content(&self.content);
        let show_line_numbers = self.display.show_line_numbers;
        let theme = self.display.code_block_theme;

        // Hash app theme for cache invalidation
        let app_theme_hash = self
            .app_theme
            .as_ref()
            .map(|t| {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                format!(
                    "{:?}{:?}{:?}{:?}{:?}",
                    t.primary, t.text, t.background, t.markdown.heading, t.markdown.code
                )
                .hash(&mut hasher);
                hasher.finish()
            })
            .unwrap_or(0);

        // Check if we can use fully cached rendered lines
        // Note: Never use cache when in filter mode (filter affects what's shown)
        let show_heading_collapse = self.display.show_heading_collapse;
        let render_cache_valid = !self.filter_mode
            && self
                .cache
                .render
                .as_ref()
                .map(|c| {
                    c.content_hash == content_hash
                        && c.width == width
                        && c.show_line_numbers == show_line_numbers
                        && c.theme == theme
                        && c.app_theme_hash == app_theme_hash
                        && c.show_heading_collapse == show_heading_collapse
                })
                .unwrap_or(false);

        // Get rendered lines and boundaries (from cache or fresh render)
        let (all_lines, line_boundaries): (Vec<Line<'static>>, Vec<(usize, usize)>) =
            if render_cache_valid {
                // Use fully cached rendered lines
                let cache = self.cache.render.as_ref().unwrap();
                (cache.lines.clone(), cache.line_boundaries.clone())
            } else {
                // Check if we can use cached parsed elements
                let parsed_cache_valid = self
                    .cache
                    .parsed
                    .as_ref()
                    .map(|c| c.content_hash == content_hash)
                    .unwrap_or(false);

                let elements = if parsed_cache_valid {
                    // Use cached parsed elements
                    self.cache.parsed.as_ref().unwrap().elements.clone()
                } else {
                    // Parse markdown and cache
                    let parsed = render_markdown_to_elements(&self.content, true);
                    self.cache.parsed = Some(ParsedCache {
                        content_hash,
                        elements: parsed.clone(),
                    });
                    parsed
                };

                let render_options = RenderOptions {
                    show_line_numbers,
                    theme,
                    app_theme: self.app_theme.as_ref(),
                    show_heading_collapse: self.display.show_heading_collapse,
                };

                // Get filter text if in filter mode
                let filter_lower = self
                    .filter_mode
                    .then(|| self.filter.as_deref().unwrap_or("").to_lowercase());

                // Build all rendered lines and track line boundaries
                let mut lines: Vec<Line<'static>> = Vec::new();
                let mut boundaries: Vec<(usize, usize)> = Vec::new();

                for (idx, element) in elements.iter().enumerate() {
                    if !should_render_line(element, idx, &self.collapse) {
                        continue;
                    }

                    // Check if element matches filter (skip if not matching)
                    if let Some(ref filter) = filter_lower {
                        let text = element_to_plain_text_for_filter(&element.kind).to_lowercase();
                        if !text.contains(filter) {
                            continue;
                        }
                    }

                    let start_idx = lines.len();
                    let rendered = render_with_options(element, width, render_options);
                    let line_count = rendered.len();
                    lines.extend(rendered);
                    boundaries.push((start_idx, line_count));
                }

                // Cache the rendered lines
                self.cache.render = Some(RenderCache {
                    content_hash,
                    width,
                    show_line_numbers,
                    theme,
                    app_theme_hash,
                    show_heading_collapse,
                    lines: lines.clone(),
                    line_boundaries: boundaries.clone(),
                });

                (lines, boundaries)
            };

        // Update total lines
        self.scroll.update_total_lines(all_lines.len());

        // Update cache for selection text extraction
        self.rendered_lines = all_lines.clone();

        // Extract visible portion
        let start = self.scroll.scroll_offset.min(all_lines.len());
        let end = (self.scroll.scroll_offset + content_area.height as usize).min(all_lines.len());
        let visible_lines: Vec<Line<'static>> = all_lines[start..end].to_vec();

        // Apply selection highlighting if selection is active
        let visible_lines = if self.selection_active {
            apply_selection_highlighting(visible_lines, &self.selection, self.scroll.scroll_offset)
        } else {
            visible_lines
        };

        // Current line for highlighting (0-indexed visual line)
        let current_visual_line = self.scroll.current_line.saturating_sub(1);

        // Add document line numbers if enabled and apply current line highlighting
        let final_lines: Vec<Line<'_>> = if self.display.show_document_line_numbers {
            // Get colors from the code block theme for consistency
            let theme_colors = self.display.code_block_theme.colors();
            let line_num_style = Style::default()
                .fg(theme_colors.line_number)
                .bg(theme_colors.background);
            let border_style = Style::default()
                .fg(theme_colors.border)
                .bg(theme_colors.background);

            // Build a map: visual_line_idx -> (logical_line_num, is_first_line_of_logical)
            let mut visual_to_logical: Vec<(usize, bool)> = Vec::with_capacity(all_lines.len());
            for (logical_idx, (_start_idx, count)) in line_boundaries.iter().enumerate() {
                for offset in 0..*count {
                    let is_first = offset == 0;
                    visual_to_logical.push((logical_idx + 1, is_first));
                }
            }

            visible_lines
                .into_iter()
                .enumerate()
                .map(|(i, mut line)| {
                    let visual_idx = start + i;
                    let is_current = visual_idx == current_visual_line;
                    let (logical_num, is_first) = visual_to_logical
                        .get(visual_idx)
                        .copied()
                        .unwrap_or((visual_idx + 1, true));

                    // Fixed width of 3 digits + " │ " = 6 chars total
                    let (num_str, border_str) = if is_first {
                        (format!("{:>3} ", logical_num), "│ ".to_string())
                    } else {
                        ("    ".to_string(), "│ ".to_string()) // Continuation line
                    };

                    let num_span = Span::styled(num_str, line_num_style);
                    let border_span = Span::styled(border_str, border_style);

                    let mut new_spans = vec![num_span, border_span];

                    // Apply current line highlighting to content
                    let highlight_bg = if self.selection_active {
                        CURRENT_LINE_DRAG_BG
                    } else {
                        CURRENT_LINE_BG
                    };
                    if is_current {
                        let mut content_width = 0usize;
                        for span in line.spans.drain(..) {
                            content_width += span.content.chars().count();
                            if span.content.contains('▋') {
                                // Keep blockquote marker without highlight
                                new_spans.push(span);
                            } else {
                                new_spans
                                    .push(Span::styled(span.content, span.style.bg(highlight_bg)));
                            }
                        }
                        // Add padding to fill the rest of the line
                        let total_content_width = line_num_width + content_width;
                        if total_content_width < content_area.width as usize {
                            let padding =
                                " ".repeat(content_area.width as usize - total_content_width);
                            new_spans
                                .push(Span::styled(padding, Style::default().bg(highlight_bg)));
                        }
                    } else {
                        new_spans.append(&mut line.spans);
                    }

                    Line::from(new_spans)
                })
                .collect()
        } else {
            // No line numbers, but still apply current line highlighting
            let highlight_bg = if self.selection_active {
                CURRENT_LINE_DRAG_BG
            } else {
                CURRENT_LINE_BG
            };
            visible_lines
                .into_iter()
                .enumerate()
                .map(|(i, mut line)| {
                    let visual_idx = start + i;
                    let is_current = visual_idx == current_visual_line;

                    if is_current {
                        let mut new_spans = Vec::new();
                        let mut content_width = 0usize;
                        for span in line.spans.drain(..) {
                            content_width += span.content.chars().count();
                            if span.content.contains('▋') {
                                new_spans.push(span);
                            } else {
                                new_spans
                                    .push(Span::styled(span.content, span.style.bg(highlight_bg)));
                            }
                        }
                        // Add padding to fill the rest of the line
                        if content_width < content_area.width as usize {
                            let padding = " ".repeat(content_area.width as usize - content_width);
                            new_spans
                                .push(Span::styled(padding, Style::default().bg(highlight_bg)));
                        }
                        Line::from(new_spans)
                    } else {
                        line
                    }
                })
                .collect()
        };

        // Render markdown content to buffer
        for (i, line) in final_lines.iter().enumerate() {
            if i < content_area.height as usize {
                let y = content_area.y + i as u16;
                let mut x = content_area.x;
                for span in line.spans.iter() {
                    let used = x.saturating_sub(content_area.x);
                    let remaining = content_area.width.saturating_sub(used) as usize;
                    if remaining == 0 {
                        break;
                    }

                    let mut clipped = String::new();
                    let mut clipped_width = 0usize;
                    for ch in span.content.chars() {
                        let ch_width = UnicodeWidthChar::width(ch).unwrap_or(0);
                        if clipped_width + ch_width > remaining {
                            break;
                        }
                        clipped.push(ch);
                        clipped_width += ch_width;
                    }

                    if !clipped.is_empty() {
                        buf.set_string(x, y, clipped, span.style);
                        x = x.saturating_add(clipped_width as u16);
                    }
                }
            }
        }

        // Render TOC overlay
        if let Some(ov_area) = overlay_area {
            // Create state from content with widget's hover state
            let mut auto_state = TocState::from_content(&self.content);
            auto_state.hovered = self.toc_hovered;
            auto_state.hovered_entry = self.toc_hovered_entry;
            auto_state.scroll_offset = self.toc_scroll_offset;

            // Use provided state if it has entries, otherwise use auto-extracted
            let final_state = if let Some(provided) = &self.toc_state {
                if provided.entries.is_empty() {
                    &auto_state
                } else {
                    provided
                }
            } else {
                &auto_state
            };

            let toc = Toc::new(final_state)
                .expanded(self.toc_hovered)
                .config(self.toc_config.clone());

            toc.render(ov_area, buf);
        }

        // Render statusline
        if let Some(sl_area) = statusline_area {
            self.render_statusline(sl_area, buf);
        }

        // Render scrollbar LAST so it's on top of everything
        if self.show_scrollbar && self.scroll.total_lines > content_area.height as usize {
            let scrollbar_width = self.scrollbar_config.width;
            let scrollbar_area = Rect {
                x: content_area.x + content_area.width.saturating_sub(scrollbar_width),
                y: content_area.y,
                width: scrollbar_width,
                height: content_area.height,
            };

            let scrollbar = CustomScrollbar::new(&self.scroll)
                .config(self.scrollbar_config.clone())
                .show_percentage(false);

            scrollbar.render(scrollbar_area, buf);
        }

        // Capture inner area for mouse event handling
        self.inner_area = Some(content_area);
    }
}

impl<'a> Widget for MarkdownWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        <&mut MarkdownWidget<'a> as Widget>::render(&mut self, area, buf);
    }
}
