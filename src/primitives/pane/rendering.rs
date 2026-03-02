use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui::Frame;

use crate::primitives::pane::Pane;

impl<'a> Pane<'a> {
    pub fn render<W>(&self, frame: &mut Frame, area: Rect, content: W)
    where
        W: Widget,
    {
        let padded_area = self.get_padded_area(area);
        let block = self.build_block();
        let inner = block.inner(padded_area);

        frame.render_widget(block, padded_area);
        frame.render_widget(content, inner);
    }

    pub fn render_with_footer<C, F>(&self, frame: &mut Frame, area: Rect, content: C, footer: F)
    where
        C: Widget,
        F: Widget,
    {
        let padded_area = self.get_padded_area(area);
        let block = self.build_block();
        let inner = block.inner(padded_area);

        frame.render_widget(block, padded_area);

        if self.footer_height == 0 {
            frame.render_widget(content, inner);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(self.footer_height)])
            .split(inner);

        frame.render_widget(content, chunks[0]);
        frame.render_widget(footer, chunks[1]);
    }

    pub fn render_paragraph(&self, frame: &mut Frame, area: Rect, content: Vec<Line<'a>>) {
        let paragraph = Paragraph::new(content);
        self.render(frame, area, paragraph);
    }

    pub fn render_paragraph_with_footer<F>(
        &self,
        frame: &mut Frame,
        area: Rect,
        content: Vec<Line<'a>>,
        footer: F,
    ) where
        F: Widget,
    {
        let paragraph = Paragraph::new(content);
        self.render_with_footer(frame, area, paragraph, footer);
    }

    pub fn render_block(&self, frame: &mut Frame, area: Rect) -> (Rect, Option<Rect>) {
        let padded_area = self.get_padded_area(area);
        let block = self.build_block();
        let inner = block.inner(padded_area);

        frame.render_widget(block, padded_area);

        if self.footer_height == 0 {
            return (inner, None);
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(self.footer_height)])
            .split(inner);

        (chunks[0], Some(chunks[1]))
    }

    pub fn render_block_in_buffer(&self, area: Rect, buf: &mut Buffer) -> (Rect, Option<Rect>) {
        let padded_area = self.get_padded_area(area);
        let block = self.build_block();
        let inner = block.inner(padded_area);

        block.render(padded_area, buf);

        if self.footer_height == 0 {
            return (inner, None);
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(self.footer_height)])
            .split(inner);

        (chunks[0], Some(chunks[1]))
    }

    fn get_padded_area(&self, area: Rect) -> Rect {
        Rect {
            x: area.x + self.padding.3,
            y: area.y + self.padding.0,
            width: area.width.saturating_sub(self.padding.1 + self.padding.3),
            height: area.height.saturating_sub(self.padding.0 + self.padding.2),
        }
    }

    fn build_title_line(&self) -> Line<'a> {
        use ratatui::text::Span;

        let mut spans = vec![Span::raw(" ")];

        if let Some(ref icon) = self.icon {
            spans.push(Span::styled(icon.clone(), self.title_style));
            spans.push(Span::raw(" "));
        }

        spans.push(Span::styled(self.title.clone(), self.title_style));
        spans.push(Span::raw(" "));

        Line::from(spans)
    }

    fn build_block(&self) -> Block<'a> {
        let mut block = Block::default()
            .borders(Borders::ALL)
            .border_type(self.border_type)
            .border_style(self.border_style)
            .title(self.build_title_line());

        if let Some(ref footer) = self.text_footer {
            block = block.title_bottom(footer.clone().style(self.footer_style));
        }

        block
    }
}
