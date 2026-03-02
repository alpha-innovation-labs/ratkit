use crate::primitives::pane::Pane;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::scrollbar::CustomScrollbar;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::selection::should_render_line;
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::toc::Toc;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::{
    render_with_options, RenderOptions,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::helpers::hash_content;
use crate::widgets::markdown_preview::widgets::markdown_widget::state::{
    ParsedCache, RenderCache, TocState,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::features::filter::element_to_plain_text_for_filter;
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::features::selection::apply_selection_highlighting;
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::{
    MarkdownWidget, CURRENT_LINE_BG, CURRENT_LINE_DRAG_BG,
};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Widget};
use unicode_width::UnicodeWidthChar;

impl<'a> Widget for &mut MarkdownWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
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

            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_type(pane.border_type)
                .border_style(pane.border_style)
                .title(pane.title);

            if let Some(icon) = &pane.icon {
                let title = format!(" {} ", icon);
                block = block.title(Line::from(vec![Span::styled(title, pane.title_style)]));
            }

            if let Some(ref footer) = pane.text_footer {
                block = block.title_bottom(footer.clone().style(pane.footer_style));
            }

            let inner = block.inner(area);
            block.render(area, buf);

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

            let padded = Rect {
                x: inner.x + pane.padding.3,
                y: inner.y + pane.padding.0,
                width: inner.width.saturating_sub(pane.padding.1 + pane.padding.3),
                height: inner.height.saturating_sub(pane.padding.0 + pane.padding.2),
            };

            if let Some(footer_area) = pane_footer {
                if let Some(ref footer) = pane.text_footer {
                    footer.render(footer_area, buf);
                }
            }

            (padded, None::<Rect>)
        } else {
            (area, None::<Rect>)
        };

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

        let content_area = main_area;
        let overlay_area = self.calculate_toc_area(main_area);

        self.scroll.update_viewport(content_area);

        let line_num_width = if self.display.show_document_line_numbers {
            6
        } else {
            0
        };

        let width = (content_area.width as usize).saturating_sub(line_num_width);
        let content_hash = hash_content(&self.content);
        let show_line_numbers = self.display.show_line_numbers;
        let theme = self.display.code_block_theme;

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

        let (all_lines, line_boundaries): (Vec<Line<'static>>, Vec<(usize, usize)>) =
            if render_cache_valid {
                let cache = self.cache.render.as_ref().expect("render cache present");
                (cache.lines.clone(), cache.line_boundaries.clone())
            } else {
                let parsed_cache_valid = self
                    .cache
                    .parsed
                    .as_ref()
                    .map(|c| c.content_hash == content_hash)
                    .unwrap_or(false);

                let elements = if parsed_cache_valid {
                    self.cache
                        .parsed
                        .as_ref()
                        .expect("parsed cache present")
                        .elements
                        .clone()
                } else {
                    let parsed = self.parse_elements();
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

                let filter_lower = self
                    .filter_mode
                    .then(|| self.filter.as_deref().unwrap_or("").to_lowercase());

                let mut lines: Vec<Line<'static>> = Vec::new();
                let mut boundaries: Vec<(usize, usize)> = Vec::new();

                for (idx, element) in elements.iter().enumerate() {
                    if !should_render_line(element, idx, &self.collapse) {
                        continue;
                    }

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

        self.scroll.update_total_lines(all_lines.len());
        self.rendered_lines = all_lines.clone();

        let start = self.scroll.scroll_offset.min(all_lines.len());
        let end = (self.scroll.scroll_offset + content_area.height as usize).min(all_lines.len());
        let visible_lines: Vec<Line<'static>> = all_lines[start..end].to_vec();

        let visible_lines = if self.selection_active {
            apply_selection_highlighting(visible_lines, &self.selection, self.scroll.scroll_offset)
        } else {
            visible_lines
        };

        let current_visual_line = self.scroll.current_line.saturating_sub(1);

        let final_lines: Vec<Line<'_>> = if self.display.show_document_line_numbers {
            let theme_colors = self.display.code_block_theme.colors();
            let line_num_style = Style::default()
                .fg(theme_colors.line_number)
                .bg(theme_colors.background);
            let border_style = Style::default()
                .fg(theme_colors.border)
                .bg(theme_colors.background);

            let mut visual_to_logical: Vec<(usize, bool)> = Vec::with_capacity(all_lines.len());
            for (logical_idx, (_start_idx, count)) in line_boundaries.iter().enumerate() {
                for offset in 0..*count {
                    visual_to_logical.push((logical_idx + 1, offset == 0));
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

                    let (num_str, border_str) = if is_first {
                        (format!("{:>3} ", logical_num), "│ ".to_string())
                    } else {
                        ("    ".to_string(), "│ ".to_string())
                    };

                    let num_span = Span::styled(num_str, line_num_style);
                    let border_span = Span::styled(border_str, border_style);
                    let mut new_spans = vec![num_span, border_span];

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
                                new_spans.push(span);
                            } else {
                                new_spans
                                    .push(Span::styled(span.content, span.style.bg(highlight_bg)));
                            }
                        }
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

        for (i, line) in final_lines.iter().enumerate() {
            if i < content_area.height as usize {
                let y = content_area.y + i as u16;
                let mut x = content_area.x;
                for span in &line.spans {
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

        if let Some(ov_area) = overlay_area {
            let mut auto_state = TocState::from_content(&self.content);
            auto_state.hovered = self.toc_hovered;
            auto_state.hovered_entry = self.toc_hovered_entry;
            auto_state.scroll_offset = self.toc_scroll_offset;

            let final_state = self.resolved_toc_state(&auto_state);
            let toc = Toc::new(final_state)
                .expanded(self.toc_hovered)
                .config(self.toc_config.clone());

            toc.render(ov_area, buf);
        }

        if let Some(sl_area) = statusline_area {
            self.render_statusline(sl_area, buf);
        }

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

        self.inner_area = Some(content_area);
    }
}

impl<'a> Widget for MarkdownWidget<'a> {
    fn render(mut self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        <&mut MarkdownWidget<'a> as Widget>::render(&mut self, area, buf);
    }
}
