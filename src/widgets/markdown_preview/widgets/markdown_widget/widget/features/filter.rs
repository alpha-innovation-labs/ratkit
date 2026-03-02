use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::selection::should_render_line;
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::{
    render, ElementKind, TextSegment,
};
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::parser::render_markdown_to_elements;
use crate::widgets::markdown_preview::widgets::markdown_widget::state::CollapseState;
use crate::widgets::markdown_preview::widgets::markdown_widget::widget::FRONTMATTER_SECTION_ID;

#[allow(dead_code)]
pub(crate) fn get_filtered_visual_lines(
    content: &str,
    filter_text: &str,
    collapse: &CollapseState,
    width: usize,
) -> Vec<usize> {
    let filter_lower = filter_text.to_lowercase();
    let elements = render_markdown_to_elements(
        content,
        collapse.is_section_collapsed(FRONTMATTER_SECTION_ID),
    );
    let mut filtered_visual_lines: Vec<usize> = Vec::new();
    let mut visual_line_idx = 0;

    for (idx, element) in elements.iter().enumerate() {
        if !should_render_line(element, idx, collapse) {
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
pub(crate) fn find_next_filtered_line(
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
pub(crate) fn find_prev_filtered_line(
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

    for &line in filtered.iter().rev() {
        if line < current_visual_line {
            return Some(line);
        }
    }

    filtered.last().copied()
}

fn text_segment_to_string(segment: &TextSegment) -> String {
    match segment {
        TextSegment::Plain(s)
        | TextSegment::Bold(s)
        | TextSegment::Italic(s)
        | TextSegment::BoldItalic(s)
        | TextSegment::InlineCode(s)
        | TextSegment::Strikethrough(s)
        | TextSegment::Html(s) => s.clone(),
        TextSegment::Link { text, .. } => text.clone(),
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
