use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::selection::should_render_line;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::{
    render, ElementKind, TextSegment,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::MarkdownWidget;

impl<'a> MarkdownWidget<'a> {
    pub fn get_line_info_at_position(
        &self,
        y: usize,
        width: usize,
    ) -> Option<(usize, String, String)> {
        let document_y = y + self.scroll.scroll_offset;
        self.resolve_line_info_at_document_y(document_y, width)
    }

    pub fn get_current_line_info(&self, width: usize) -> Option<(usize, String, String)> {
        let document_y = self.scroll.current_line.saturating_sub(1);
        self.resolve_line_info_at_document_y(document_y, width)
    }

    pub fn set_rendered_lines(&mut self, lines: Vec<ratatui::text::Line<'static>>) {
        self.rendered_lines = lines;
    }

    pub fn is_selection_active(&self) -> bool {
        self.selection.is_active()
    }

    pub fn selection(
        &self,
    ) -> &crate::widgets::markdown_preview::widgets::markdown_widget::state::SelectionState {
        &self.selection
    }

    pub(crate) fn get_element_text(&self, kind: &ElementKind) -> String {
        fn segment_to_text(seg: &TextSegment) -> &str {
            match seg {
                TextSegment::Plain(s)
                | TextSegment::Bold(s)
                | TextSegment::Italic(s)
                | TextSegment::BoldItalic(s)
                | TextSegment::InlineCode(s)
                | TextSegment::Strikethrough(s)
                | TextSegment::Html(s) => s,
                TextSegment::Link { text, .. } => text,
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

    fn resolve_line_info_at_document_y(
        &self,
        document_y: usize,
        width: usize,
    ) -> Option<(usize, String, String)> {
        let elements = self.parse_elements();
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
