use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::scrollbar::{
    click_to_offset, is_in_scrollbar_area,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::selection::should_render_line;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::{
    render, ElementKind,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::events::MarkdownEvent;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::helpers::is_in_area;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::types::SelectionPos;
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::{
    MarkdownWidget, MarkdownWidgetMode, FRONTMATTER_SECTION_ID,
};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

impl<'a> MarkdownWidget<'a> {
    pub(crate) fn handle_mouse_internal(
        &mut self,
        event: &MouseEvent,
        area: Rect,
    ) -> MarkdownEvent {
        if !is_in_area(event.column, event.row, area) {
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

        let document_y = (relative_y + self.scroll.scroll_offset) as i32;
        let document_x = relative_x as i32;

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

        if let Some(scrollbar_area) = self.calculate_scrollbar_area(area) {
            if is_in_scrollbar_area(event.column, event.row, scrollbar_area) {
                match event.kind {
                    MouseEventKind::Down(MouseButton::Left)
                    | MouseEventKind::Drag(MouseButton::Left) => {
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
                            direction: self.scroll.scroll_offset.saturating_sub(old_offset) as i32,
                        };
                    }
                    _ => {}
                }
            }
        }

        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if self.selection.is_active() {
                    self.selection.exit();
                    self.selection_active = false;
                }

                if self.handle_click_collapse(relative_x, relative_y, width) {
                    self.double_click.clear_pending();
                    let clicked_line = self.scroll.scroll_offset + relative_y + 1;
                    if clicked_line <= self.scroll.total_lines {
                        self.scroll.set_current_line(clicked_line);
                    }
                    return MarkdownEvent::FocusedLine { line: clicked_line };
                }

                let (is_double, _should_process_pending) = self.double_click.process_click(
                    event.column,
                    event.row,
                    self.scroll.scroll_offset,
                );

                if is_double {
                    if let Some(evt) = self.get_line_info_at_position(relative_y, width) {
                        self.last_double_click = Some((evt.0, evt.1, evt.2));
                    }
                    return MarkdownEvent::None;
                }

                let clicked_line = self.scroll.scroll_offset + relative_y + 1;
                if clicked_line <= self.scroll.total_lines {
                    self.scroll.set_current_line(clicked_line);
                }

                MarkdownEvent::FocusedLine { line: clicked_line }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                let event_result = if !self.selection.is_active() {
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

                self.selection.update_cursor(document_x, document_y);

                event_result
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if self.selection.is_active() {
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
                        if let Some(event) = self.copy_text_to_clipboard(text, true) {
                            return event;
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
                    direction: self.scroll.scroll_offset.saturating_sub(old_offset) as i32,
                }
            }
            _ => MarkdownEvent::None,
        };

        self.check_pending_click_internal(area)
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

    fn handle_click_collapse(&mut self, _x: usize, y: usize, width: usize) -> bool {
        let elements = self.parse_elements();
        let document_y = y + self.scroll.scroll_offset;
        let mut line_idx = 0;

        for (idx, element) in elements.iter().enumerate() {
            if !should_render_line(element, idx, &self.collapse) {
                continue;
            }

            let rendered = render(element, width);
            let line_count = rendered.len();

            if document_y >= line_idx && document_y < line_idx + line_count {
                match &element.kind {
                    ElementKind::Heading { section_id, .. } => {
                        if self.display.show_heading_collapse {
                            self.collapse.toggle_section(*section_id);
                            self.cache.invalidate();
                            return true;
                        }
                    }
                    ElementKind::Frontmatter { .. } | ElementKind::FrontmatterStart { .. } => {
                        self.collapse.toggle_section(FRONTMATTER_SECTION_ID);
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
}
