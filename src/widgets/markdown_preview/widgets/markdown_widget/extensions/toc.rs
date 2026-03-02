//! Table of Contents widget for markdown document navigation.
//!
//! Provides a compact or expanded view of document headings for quick navigation.
//! In compact mode, shows heading indicators as horizontal lines.
//! In expanded mode, shows full heading text with indentation.
//!
//! # Features
//!
//! - Compact mode: horizontal lines indicating heading positions and levels
//! - Expanded mode: full heading text with hierarchy indentation
//! - Current heading highlight (blue in expanded, bright in compact)
//! - Hover highlight for items
//! - Click-to-scroll navigation
//!
//! # Mouse Capture Requirement
//!
//! For TOC click navigation and hover interactions to work, you must enable
//! mouse capture with crossterm:
//!
//! ```rust,ignore,ignore
//! use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
//! execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//! ```
//!
//! Without `EnableMouseCapture`, click events will not be received.
//!
//! # Architecture
//!
//! The Toc extension is a UI widget only - it receives `&TocState` as a parameter
//! and ONLY handles rendering. State mutations happen through TocState methods.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::symbols::Marker;
use ratatui::widgets::{
    canvas::{Canvas, Line},
    Widget,
};
use unicode_width::UnicodeWidthStr;

use crate::primitives::pane::Pane;
use crate::widgets::markdown_preview::widgets::markdown_widget::state::TocState;

/// Table of Contents widget for markdown navigation.
///
/// Shows document headings in either compact (lines) or expanded (text) mode.
/// Supports hover interactions and click-to-scroll navigation.
///
/// This is a UI-only widget that receives `&TocState` for state access.
/// State mutations happen through `TocState` methods, not here.
///
/// # Example
///
/// ```rust,ignore,no_run
/// use ratatui_toolkit::markdown_widget::extensions::toc::{Toc, TocConfig};
/// use ratatui_toolkit::markdown_widget::state::TocState;
///
/// let mut toc_state = TocState::new();
/// let toc = Toc::new(&toc_state)
///     .config(TocConfig::default())
///     .expanded(true);
/// ```
#[derive(Debug)]
pub struct Toc<'a> {
    /// Reference to the TOC state (entries, scroll, hover).
    pub(crate) toc_state: &'a TocState,
    /// Configuration for appearance.
    pub(crate) config: TocConfig,
    /// Whether the TOC is in expanded mode.
    pub(crate) expanded: bool,
}

/// Hovered entry configuration for Toc widget.

impl<'a> Toc<'a> {
    fn panel_background_style(&self) -> Style {
        self.config.background_style
    }

    /// Set the hovered item index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the hovered heading, or None.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn hovered(self, _index: Option<usize>) -> Self {
        // Now managed by TocState, this is a no-op for compatibility
        self
    }
}

/// Constructor for Toc widget.

impl<'a> Toc<'a> {
    /// Create a new TOC widget from TocState.
    ///
    /// The Toc widget is a UI-only component that receives state via reference.
    /// State mutations happen through TocState methods, not the Toc widget.
    ///
    /// # Arguments
    ///
    /// * `toc_state` - Reference to the TocState containing entries, scroll, and hover state.
    ///
    /// # Returns
    ///
    /// A new `Toc` instance ready for rendering.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_run
    /// use ratatui_toolkit::markdown_widget::state::TocState;
    ///
    /// let toc_state = TocState::new();
    /// let toc = Toc::new(&toc_state);
    /// ```
    pub fn new(toc_state: &'a TocState) -> Self {
        Self {
            toc_state,
            config: TocConfig::default(),
            expanded: false,
        }
    }

    /// Set whether the TOC is expanded.
    ///
    /// # Arguments
    ///
    /// * `expanded` - True for expanded mode (full text), false for compact mode (lines).
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set the TOC visual style mode.
    ///
    /// # Arguments
    ///
    /// * `style` - The visual style mode (Normal or Clerk).
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn style(mut self, style: TocStyle) -> Self {
        self.config.style = style;
        self
    }

    /// Set the TOC configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The TOC configuration.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn config(mut self, config: TocConfig) -> Self {
        self.config = config;
        self
    }
}

/// Viewport configuration for Toc widget.

impl<'a> Toc<'a> {
    /// Set the current viewport information.
    ///
    /// # Arguments
    ///
    /// * `scroll_offset` - Current scroll offset.
    /// * `viewport_height` - Height of the visible viewport.
    /// * `total_lines` - Total number of lines in the document.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn viewport(
        self,
        _scroll_offset: usize,
        _viewport_height: usize,
        _total_lines: usize,
    ) -> Self {
        // These are now managed by TocState, this is a no-op for compatibility
        self
    }

    /// Set the TOC scroll offset (for scrolling within the TOC list).
    ///
    /// # Arguments
    ///
    /// * `offset` - The scroll offset for the TOC list.
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    pub fn toc_scroll(self, _offset: usize) -> Self {
        // Now managed by TocState, this is a no-op for compatibility
        self
    }
}

/// Theme application for TocConfig.

impl TocConfig {
    /// Creates a TocConfig with colors derived from the application theme.
    ///
    /// This applies theme colors to:
    /// - Text style (using markdown text color)
    /// - Active style (using theme primary color)
    /// - Hover style (using theme accent with background_element)
    /// - Background style (using theme background_panel)
    /// - Line style (using theme border color)
    /// - Active line style (using theme text color)
    /// - Border style (using theme border_active color)
    /// - Title style (using theme primary color)
    ///
    /// # Arguments
    ///
    /// * `theme` - The application theme to derive colors from
    ///
    /// # Returns
    ///
    /// Self for method chaining.
    ///
    /// # Example
    ///
    /// ```rust,ignore,no_run
    ///
    /// let theme = AppTheme::default();
    /// let config = TocConfig::default().with_theme(&theme);
    /// ```
    pub fn with_theme(
        mut self,
        theme: &crate::widgets::markdown_preview::services::theme::AppTheme,
    ) -> Self {
        self.text_style = Style::default().fg(theme.text_muted);
        self.active_style = Style::default().fg(theme.primary);
        self.hover_style = Style::default().fg(theme.text).bg(theme.background_element);
        self.background_style = Style::default().bg(theme.background_panel);
        self.line_style = Style::default().fg(theme.border);
        self.active_line_style = Style::default().fg(theme.text);
        self.border_style = Style::default().fg(theme.border_active);
        self.title_style = Style::default().fg(theme.primary);
        self
    }
}

/// Configuration for TOC appearance.

/// TOC visual style mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TocStyle {
    /// Standard style with border and text styling.
    #[default]
    Normal,
    /// Clerk-style with accent left border indicator.
    Clerk,
}

/// Configuration for TOC appearance.
#[derive(Debug, Clone)]
pub struct TocConfig {
    /// Width of the TOC in compact mode.
    pub compact_width: u16,
    /// Width of the TOC in expanded mode.
    pub expanded_width: u16,
    /// Height of the TOC.
    pub height: u16,
    /// Visual style mode for the TOC.
    pub style: TocStyle,
    /// Style for normal heading text.
    pub text_style: Style,
    /// Style for the current/active heading.
    pub active_style: Style,
    /// Style for hovered heading.
    pub hover_style: Style,
    /// Style for the accent bar in clerk mode (left border indicator).
    pub accent_style: Style,
    /// Style for the active accent bar in clerk mode.
    pub active_accent_style: Style,
    /// Background style for the TOC panel.
    pub background_style: Style,
    /// Style for the compact mode lines.
    pub line_style: Style,
    /// Style for the active line in compact mode.
    pub active_line_style: Style,
    /// Whether to show a border around the TOC (only in expanded mode).
    pub show_border: bool,
    /// Style for the border.
    pub border_style: Style,
    /// Style for the title text in the border.
    pub title_style: Style,
    /// Title text to show in the border header.
    pub title: String,
    /// Spacing between lines in compact mode (in 1/8 cell units).
    /// 1 = tightest (8 lines per row), 8 = one line per row.
    pub line_spacing: u8,
}

/// Line width calculation for TOC entries.

/// Calculate line width based on heading level.
///
/// # Arguments
///
/// * `canvas_width` - Total canvas width.
/// * `level` - Heading level (1-6).
///
/// # Returns
///
/// The line width in canvas units.
pub(crate) fn calculate_line_width(canvas_width: f64, level: u8) -> f64 {
    let width_ratio = match level {
        1 => 1.0,
        2 => 0.80,
        3 => 0.60,
        4 => 0.45,
        5 => 0.35,
        _ => 0.25,
    };
    (canvas_width * width_ratio).max(2.0)
}

/// Static method to count headings in markdown content.

impl<'a> Toc<'a> {
    /// Count the number of headings in markdown content.
    ///
    /// This is a static method useful for calculating dynamic TOC height
    /// without constructing a full Toc widget.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to scan.
    ///
    /// # Returns
    ///
    /// The number of headings found.
    pub fn count_headings(content: &str) -> usize {
        let mut count = 0;
        let mut in_code_block = false;

        for line in content.lines() {
            let trimmed = line.trim();

            // Track code blocks
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }

            if in_code_block {
                continue;
            }

            // Check for headings
            if trimmed.starts_with('#') {
                let level = trimmed.chars().take_while(|c| *c == '#').count();
                if level <= 6 {
                    let text = trimmed[level..].trim();
                    if !text.is_empty() {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

/// Entry accessor methods for TOC widget.
use crate::widgets::markdown_preview::widgets::markdown_widget::state::TocEntry;

impl<'a> Toc<'a> {
    /// Get the number of entries in the TOC.
    ///
    /// Delegates to the underlying TocState.
    pub fn entry_count(&self) -> usize {
        self.toc_state.entry_count()
    }

    /// Get all entries.
    ///
    /// Delegates to the underlying TocState.
    pub fn entries(&self) -> &[TocEntry] {
        self.toc_state.entries()
    }

    /// Get the target line number for a clicked entry.
    ///
    /// # Arguments
    ///
    /// * `entry_index` - The index of the clicked entry.
    ///
    /// # Returns
    ///
    /// The line number to scroll to, or None if the index is invalid.
    pub fn click_to_line(&self, entry_index: usize) -> Option<usize> {
        self.toc_state.get_entry(entry_index).map(|e| e.line_number)
    }

    /// Get the entry at a given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The entry index.
    ///
    /// # Returns
    ///
    /// The entry, or None if the index is invalid.
    pub fn get_entry(&self, index: usize) -> Option<&TocEntry> {
        self.toc_state.get_entry(index)
    }
}

/// Position-based entry lookup for hover and click detection.

impl<'a> Toc<'a> {
    /// Find the entry index at a given screen position.
    ///
    /// # Arguments
    ///
    /// * `x` - Screen X coordinate.
    /// * `y` - Screen Y coordinate.
    /// * `area` - The area the TOC is rendered in.
    ///
    /// # Returns
    ///
    /// The entry index at that position, or None if no entry is there.
    pub fn entry_at_position(&self, x: u16, y: u16, area: Rect) -> Option<usize> {
        // Check horizontal bounds - must be within the TOC width
        if x < area.x || x >= area.x + area.width {
            return None;
        }

        // Check if above the TOC area
        if y < area.y {
            return None;
        }

        let entries = self.toc_state.entries();
        if entries.is_empty() {
            return None;
        }

        if self.expanded {
            // Calculate content area based on actual entries rather than passed area height
            let content_area = get_expanded_content_area(area, &self.config, entries.len());
            find_entry_at_position_expanded(
                x,
                y,
                content_area,
                entries,
                self.toc_state.scroll_offset,
            )
        } else {
            find_entry_at_position_compact(y, area, &self.config, entries)
        }
    }
}

/// Find entry at position in compact mode.

/// Find entry at position in compact mode.
///
/// Compact mode uses Braille markers (4 dots per cell) with configurable line_spacing.
/// This maps terminal Y coordinate to entry index by converting to Braille dot coordinates.
///
/// # Arguments
///
/// * `y` - Screen Y coordinate.
/// * `area` - The outer TOC area.
/// * `config` - The TOC configuration.
/// * `entries` - The TOC entries.
///
/// # Returns
///
/// The entry index at that position, or None if no entry is there.
pub fn find_entry_at_position_compact(
    y: u16,
    area: Rect,
    config: &TocConfig,
    entries: &[TocEntry],
) -> Option<usize> {
    // Account for border in compact mode too
    let content_area = get_content_area(area, config);

    if y < content_area.y || y >= content_area.y + content_area.height {
        return None;
    }

    let relative_y = y - content_area.y;
    let spacing = config.line_spacing.max(1) as f64;

    // Braille has 4 dots per cell. Convert terminal row to dot position.
    // Entries are rendered from top (entry 0 at top) with spacing between them.
    // In render_compact: pixel_y = canvas_height - (idx * spacing)
    // So entry 0 is at the top (canvas_height), entry N is at canvas_height - N*spacing
    //
    // To reverse this: given a terminal row, find which entry is closest.
    // Terminal row 0 corresponds to Braille dots 0-3 (from bottom of canvas)
    // But render uses inverted Y where entry 0 is at canvas_height (visual top).
    //
    // Simplify: relative_y=0 is the first content row (top of canvas visually).
    // Entry 0 is drawn at canvas_height, entry 1 at canvas_height - spacing, etc.
    // In terminal rows (inverted): entry 0 is at row 0, entry 1 is at row spacing/4, etc.
    //
    // Entry index = floor(relative_y * 4 / spacing)
    let dot_y = (relative_y as f64) * 4.0;
    let entry_idx = (dot_y / spacing).floor() as usize;

    if entry_idx < entries.len() {
        Some(entry_idx)
    } else {
        None
    }
}

/// Find entry at position in expanded mode.

/// Find entry at position in expanded mode.
///
/// # Arguments
///
/// * `x` - Screen X coordinate.
/// * `y` - Screen Y coordinate.
/// * `content_area` - The content area inside the border.
/// * `entries` - The TOC entries.
/// * `toc_scroll_offset` - Current scroll offset within the TOC.
///
/// # Returns
///
/// The entry index at that position, or None if no entry is there.
pub fn find_entry_at_position_expanded(
    x: u16,
    y: u16,
    content_area: Rect,
    entries: &[TocEntry],
    toc_scroll_offset: usize,
) -> Option<usize> {
    // Check if position is within the content area horizontally
    if x < content_area.x || x >= content_area.x + content_area.width {
        return None;
    }

    // Check vertical bounds - must be at or below the content start
    if y < content_area.y {
        return None;
    }

    // Use TOC scroll offset for position calculation
    let relative_y = (y - content_area.y) as usize;
    let entry_idx = toc_scroll_offset + relative_y;

    // Return the entry at position if it exists
    if entry_idx < entries.len() {
        Some(entry_idx)
    } else {
        None
    }
}

/// Get content area inside border for position calculations.

/// Get the content area inside border for position calculations.
///
/// # Arguments
///
/// * `area` - The outer TOC area.
/// * `config` - The TOC configuration.
///
/// # Returns
///
/// The inner content area, accounting for border if enabled.
pub fn get_content_area(area: Rect, config: &TocConfig) -> Rect {
    if config.show_border && area.width >= 4 && area.height >= 3 {
        Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        }
    } else {
        area
    }
}

/// Get expanded content area sized to fit all entries.

/// Get the content area for expanded mode, sized to fit all entries.
///
/// This ensures click detection works even if the passed area height
/// doesn't match the actual number of entries.
///
/// # Arguments
///
/// * `area` - The outer TOC area.
/// * `config` - The TOC configuration.
/// * `entry_count` - Number of entries in the TOC.
///
/// # Returns
///
/// The inner content area with height based on entry count.
pub fn get_expanded_content_area(area: Rect, config: &TocConfig, entry_count: usize) -> Rect {
    let border_offset = if config.show_border { 1 } else { 0 };
    let border_size = if config.show_border { 2 } else { 0 };

    Rect {
        x: area.x + border_offset,
        y: area.y + border_offset,
        width: area.width.saturating_sub(border_size),
        // Use entry count as height to ensure all entries are clickable
        height: entry_count as u16,
    }
}

/// Border rendering for the TOC widget in expanded mode.

impl<'a> Toc<'a> {
    /// Render the border around the TOC.
    ///
    /// Draws a rounded border with a title header and decorative separator.
    ///
    /// # Arguments
    ///
    /// * `area` - The area to render the border in.
    /// * `buf` - The buffer to render to.
    ///
    /// # Returns
    ///
    /// The inner area available for content after the border.
    pub(crate) fn render_border(&self, area: Rect, buf: &mut Buffer) -> Rect {
        let bg = self.config.background_style.bg.unwrap_or(Color::Black);
        let border_style = self.config.border_style.bg(bg);
        let title_style = self.config.title_style.bg(bg);

        let pane = Pane::new("TOC")
            .border_style(border_style)
            .title_style(title_style);

        let (content_area, _) = pane.render_block_in_buffer(area, buf);
        content_area
    }
}

/// Compact mode rendering for TOC using Canvas with Braille markers for thin lines.

impl<'a> Toc<'a> {
    /// Render the TOC in compact mode using Canvas with Braille markers.
    ///
    /// Braille gives 2x4 dots per cell for thin lines with sub-pixel positioning.
    /// `line_spacing` controls the spacing between lines in dot units.
    /// Uses two-pass rendering to ensure active line color is correct.
    pub(crate) fn render_compact(&self, area: Rect, buf: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        // Fill entire area with background first (including under border)
        fill_background(buf, area, self.panel_background_style());

        // Draw border on top of background
        let content_area = if self.config.show_border {
            self.render_border(area, buf)
        } else {
            area
        };

        if content_area.width == 0 || content_area.height == 0 {
            return;
        }

        let entries = &self.toc_state.entries;
        if entries.is_empty() {
            return;
        }

        // Get active index from TocState (hovered_entry when hovered, otherwise None for now)
        // Note: Active index tracking may need to be added to TocState
        let active_index = self.toc_state.hovered_entry;

        render_compact_lines(
            entries,
            content_area,
            buf,
            self.config.line_spacing,
            self.config.line_style.fg.unwrap_or(Color::Gray),
            self.config.active_line_style.fg.unwrap_or(Color::Yellow),
            active_index,
        );
    }
}

/// Fill an area with background style.
fn fill_background(buf: &mut Buffer, area: Rect, style: ratatui::style::Style) {
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_char(' ').set_style(style);
            }
        }
    }
}

/// Render compact lines using Canvas with two-pass rendering.
fn render_compact_lines(
    entries: &[TocEntry],
    content_area: Rect,
    buf: &mut Buffer,
    line_spacing: u8,
    normal_color: Color,
    active_color: Color,
    active_index: Option<usize>,
) {
    let spacing = line_spacing.max(1) as f64;

    // Canvas coordinates: x = 0..width*2, y = 0..height*4 (Braille: 2x4 dots per cell)
    let canvas_width = (content_area.width as f64) * 2.0;
    let canvas_height = (content_area.height as f64) * 4.0;

    // Two-pass rendering: non-active lines first, then active line
    let canvas = Canvas::default()
        .marker(Marker::Braille)
        .x_bounds([0.0, canvas_width])
        .y_bounds([0.0, canvas_height])
        .paint(move |ctx| {
            // Pass 1: Draw all non-active lines
            for (idx, entry) in entries.iter().enumerate() {
                if Some(idx) == active_index {
                    continue;
                }

                let pixel_y = canvas_height - (idx as f64 * spacing);
                if pixel_y <= 0.0 {
                    break;
                }

                let line_width = calculate_line_width(canvas_width, entry.level);
                let x_start = canvas_width - line_width;

                ctx.draw(&Line {
                    x1: x_start,
                    y1: pixel_y,
                    x2: canvas_width,
                    y2: pixel_y,
                    color: normal_color,
                });
            }

            // Pass 2: Draw active line last so it wins shared cells
            if let Some(active_idx) = active_index {
                if let Some(entry) = entries.get(active_idx) {
                    let pixel_y = canvas_height - (active_idx as f64 * spacing);
                    if pixel_y > 0.0 {
                        let line_width = calculate_line_width(canvas_width, entry.level);
                        let x_start = canvas_width - line_width;

                        ctx.draw(&Line {
                            x1: x_start,
                            y1: pixel_y,
                            x2: canvas_width,
                            y2: pixel_y,
                            color: active_color,
                        });
                    }
                }
            }
        });

    canvas.render(content_area, buf);
}

/// Expanded mode rendering for TOC (full heading text).

const DEPTH_OFFSET_DEPTH_1_2: u16 = 2;
const DEPTH_OFFSET_DEPTH_3: u16 = 4;
const DEPTH_OFFSET_DEPTH_4_PLUS: u16 = 6;

const LINE_OFFSET_DEPTH_LT_3: u16 = 0;
const LINE_OFFSET_DEPTH_GTE_3: u16 = 2;

fn get_item_offset(depth: u8) -> u16 {
    match depth {
        1 | 2 => DEPTH_OFFSET_DEPTH_1_2,
        3 => DEPTH_OFFSET_DEPTH_3,
        _ => DEPTH_OFFSET_DEPTH_4_PLUS,
    }
}

fn get_line_offset(depth: u8) -> u16 {
    if depth >= 3 {
        LINE_OFFSET_DEPTH_GTE_3
    } else {
        LINE_OFFSET_DEPTH_LT_3
    }
}

impl<'a> Toc<'a> {
    /// Render the TOC in expanded mode (full heading text).
    ///
    /// Shows heading text with indentation based on level.
    /// Active heading is shown in blue, hovered has background highlight.
    pub(crate) fn render_expanded(&self, area: Rect, buf: &mut Buffer) {
        let entries = &self.toc_state.entries;
        if entries.is_empty() || area.height == 0 {
            return;
        }

        let padding_left: u16 = 2;
        let padding_right: u16 = 1;
        let available_width = area.width.saturating_sub(padding_left + padding_right) as usize;

        let visible_count = area.height as usize;
        let start_idx = self.toc_state.scroll_offset;

        let hovered_index = self.toc_state.hovered_entry;
        let active_index: Option<usize> = None;

        for (display_idx, entry_idx) in (start_idx..entries.len()).take(visible_count).enumerate() {
            let entry = &entries[entry_idx];
            let y = area.y + display_idx as u16;

            if y >= area.y + area.height {
                break;
            }

            let indent = get_item_offset(entry.level);
            let line_offset = get_line_offset(entry.level);

            let available_for_text = available_width.saturating_sub(indent as usize);
            let display_text = truncate_text(&entry.text, available_for_text);

            let (text_style, fill_bg) = if Some(entry_idx) == hovered_index {
                (self.config.hover_style, true)
            } else if Some(entry_idx) == active_index {
                (self.config.active_style, false)
            } else {
                (self.config.text_style, false)
            };

            let is_active_or_hovered =
                Some(entry_idx) == hovered_index || Some(entry_idx) == active_index;
            let accent_style = if is_active_or_hovered {
                self.config.active_accent_style
            } else if self.config.style == TocStyle::Clerk {
                self.config.accent_style
            } else {
                text_style
            };

            if self.config.style == TocStyle::Clerk {
                self.render_clerk_lines(
                    area,
                    buf,
                    y,
                    entry.level,
                    line_offset,
                    entry_idx,
                    entries.len(),
                    start_idx,
                    hovered_index,
                    accent_style,
                );
            }

            if fill_bg {
                for x in area.x + indent..area.x + area.width {
                    if let Some(cell) = buf.cell_mut((x, y)) {
                        cell.set_style(self.config.hover_style);
                    }
                }
            }

            let x = area.x + indent;
            let full_text = display_text.to_string();

            let mut current_x = x;
            for ch in full_text.chars() {
                if current_x >= area.x + area.width - padding_right {
                    break;
                }
                let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1) as u16;
                if let Some(cell) = buf.cell_mut((current_x, y)) {
                    cell.set_char(ch).set_style(text_style);
                }
                current_x += ch_width;
            }
        }
    }

    fn render_clerk_lines(
        &self,
        area: Rect,
        buf: &mut Buffer,
        y: u16,
        depth: u8,
        line_offset: u16,
        entry_idx: usize,
        total_entries: usize,
        start_idx: usize,
        _hovered_index: Option<usize>,
        accent_style: ratatui::style::Style,
    ) {
        let line_x = area.x + line_offset;
        let border_x = area.x + line_offset;
        let next_entry_depth = if entry_idx + 1 < total_entries {
            Some(self.toc_state.entries[entry_idx + 1].level)
        } else {
            None
        };
        let prev_entry_idx = if entry_idx > start_idx {
            Some(entry_idx - 1)
        } else {
            None
        };
        let prev_entry_depth = prev_entry_idx
            .map(|idx| self.toc_state.entries[idx].level)
            .unwrap_or(depth);

        let upper_offset = get_line_offset(prev_entry_depth);
        let lower_offset = next_entry_depth.map(get_line_offset).unwrap_or(line_offset);

        if line_offset != upper_offset {
            let zigzag_char = match (upper_offset, line_offset) {
                (0, 2) => '└',
                (2, 0) => '┌',
                _ => '│',
            };
            if let Some(cell) = buf.cell_mut((line_x, y)) {
                cell.set_char(zigzag_char).set_style(accent_style);
            }
        }

        let vertical_x = border_x;
        let show_top = line_offset != upper_offset;
        let show_bottom = lower_offset != line_offset;

        let start_y = if show_top { y + 1 } else { y };
        let end_y = if show_bottom { y.saturating_sub(1) } else { y };

        if start_y <= end_y {
            for vy in start_y..=end_y {
                if let Some(cell) = buf.cell_mut((vertical_x, vy)) {
                    cell.set_char('│').set_style(accent_style);
                }
            }
        }

        if let Some(prev_y) = y.checked_sub(1) {
            if line_offset != upper_offset {
                let corner_char = '├';
                if let Some(cell) = buf.cell_mut((line_x, prev_y)) {
                    cell.set_char(corner_char).set_style(accent_style);
                }
            }
        }
    }
}

/// Truncate text to fit within a given width, adding ellipsis if needed.
fn truncate_text(text: &str, max_width: usize) -> String {
    if text.width() <= max_width {
        return text.to_string();
    }

    if max_width <= 3 {
        return "...".chars().take(max_width).collect();
    }

    let mut result = String::new();
    let mut current_width = 0;
    let target_width = max_width - 1;

    for ch in text.chars() {
        let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(1);
        if current_width + ch_width > target_width {
            break;
        }
        result.push(ch);
        current_width += ch_width;
    }

    result.push('\u{2026}');
    result
}

/// Calculate required height for compact TOC mode.

impl<'a> Toc<'a> {
    /// Calculate the required height for compact mode.
    ///
    /// With Braille markers, we have 4 vertical dots per cell.
    /// Height = ceil(entries * spacing / 4) + border_height.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to scan.
    /// * `line_spacing` - Spacing between lines in dot units.
    /// * `show_border` - Whether the border is shown.
    ///
    /// # Returns
    ///
    /// The required height in rows.
    pub fn required_compact_height(content: &str, line_spacing: u8, show_border: bool) -> u16 {
        let entry_count = Self::count_headings(content);
        if entry_count == 0 {
            return if show_border { 3 } else { 1 };
        }

        let spacing = line_spacing.max(1) as f64;
        // Total dots needed = entries * spacing
        // Cells needed = ceil(dots / 4)
        let dots_needed = entry_count as f64 * spacing;
        let cells_needed = (dots_needed / 4.0).ceil() as u16;

        let border_height = if show_border { 2 } else { 0 };
        cells_needed + border_height
    }
}

/// Calculate required width for expanded TOC mode.

impl<'a> Toc<'a> {
    /// Calculate the required width to display all headings without truncation.
    ///
    /// Takes into account:
    /// - Indentation based on heading level (2 chars per level)
    /// - Actual text width using Unicode width
    /// - Padding (left and right)
    /// - Border if enabled
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to scan.
    /// * `show_border` - Whether the border is shown.
    ///
    /// # Returns
    ///
    /// The required width in columns.
    pub fn required_expanded_width(content: &str, show_border: bool) -> u16 {
        let padding_left: u16 = 2;
        let padding_right: u16 = 1;
        let border_width: u16 = if show_border { 2 } else { 0 };

        let mut max_width: u16 = 0;

        for line in content.lines() {
            let trimmed = line.trim_start();
            if !trimmed.starts_with('#') {
                continue;
            }

            // Count heading level
            let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
            if !(1..=6).contains(&hash_count) {
                continue;
            }

            // Extract heading text
            let after_hashes = &trimmed[hash_count..];
            if !after_hashes.starts_with(' ') && !after_hashes.is_empty() {
                continue;
            }

            let text = after_hashes.trim();
            if text.is_empty() {
                continue;
            }

            // Calculate width: indent + text width
            let indent = ((hash_count - 1) * 2) as u16;
            let text_width = text.width() as u16;
            let entry_width = indent + text_width;

            if entry_width > max_width {
                max_width = entry_width;
            }
        }

        // Minimum width if no headings found
        if max_width == 0 {
            return if show_border { 10 } else { 8 };
        }

        max_width + padding_left + padding_right + border_width
    }
}

/// Static method to calculate required TOC height.

impl<'a> Toc<'a> {
    /// Calculate the required height for expanded mode.
    ///
    /// Accounts for border (2 rows) and one row per entry.
    ///
    /// # Arguments
    ///
    /// * `content` - The markdown content to scan.
    /// * `show_border` - Whether the border is shown.
    ///
    /// # Returns
    ///
    /// The required height in rows.
    pub fn required_height(content: &str, show_border: bool) -> u16 {
        let heading_count = Self::count_headings(content) as u16;
        let border_height = if show_border { 2 } else { 0 };
        heading_count + border_height
    }
}

/// Default trait implementation for TocConfig.

impl Default for TocConfig {
    fn default() -> Self {
        Self {
            compact_width: 12,
            expanded_width: 32,
            height: 20,
            style: TocStyle::default(),
            text_style: Style::default().fg(Color::Rgb(160, 160, 160)),
            active_style: Style::default().fg(Color::Rgb(97, 175, 239)), // Blue
            hover_style: Style::default().fg(Color::White).bg(Color::Rgb(60, 60, 70)),
            accent_style: Style::default()
                .fg(Color::Rgb(160, 160, 160))
                .add_modifier(ratatui::style::Modifier::DIM),
            active_accent_style: Style::default().fg(Color::Rgb(97, 175, 239)),
            background_style: Style::default().bg(Color::Rgb(30, 32, 38)),
            line_style: Style::default().fg(Color::Rgb(120, 120, 130)),
            active_line_style: Style::default().fg(Color::Rgb(230, 180, 80)), // Gold/yellow for visibility
            show_border: true,
            border_style: Style::default().fg(Color::Rgb(138, 99, 210)), // Purple
            title_style: Style::default().fg(Color::Rgb(138, 99, 210)),  // Purple
            title: "TOC".to_string(),
            line_spacing: 2, // 2 dots per entry (tight spacing)
        }
    }
}

/// Widget trait implementation for Toc.

impl<'a> Widget for Toc<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        if self.expanded {
            // Expanded mode (hovered): show border + text entries
            for y in area.y..area.y + area.height {
                for x in area.x..area.x + area.width {
                    if let Some(cell) = buf.cell_mut((x, y)) {
                        cell.set_char(' ').set_style(self.panel_background_style());
                    }
                }
            }

            let content_area = if self.config.show_border {
                self.render_border(area, buf)
            } else {
                area
            };
            self.render_expanded(content_area, buf);
        } else {
            // Compact mode (not hovered): show horizontal lines
            self.render_compact(area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;

    fn row_text(buf: &Buffer, area: Rect, row: u16) -> String {
        let mut text = String::new();
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell((x, row)) {
                text.push_str(cell.symbol());
            }
        }
        text
    }

    #[test]
    fn compact_toc_renders_toc_title_in_border() {
        let toc_state = TocState::from_content("# Heading");
        let toc = Toc::new(&toc_state)
            .expanded(false)
            .config(TocConfig::default());

        let area = Rect::new(0, 0, 20, 6);
        let mut buf = Buffer::empty(area);

        toc.render(area, &mut buf);

        let top_row = row_text(&buf, area, area.y);
        assert!(
            top_row.contains("TOC"),
            "expected TOC title in top border, got: {top_row:?}"
        );
    }
}
