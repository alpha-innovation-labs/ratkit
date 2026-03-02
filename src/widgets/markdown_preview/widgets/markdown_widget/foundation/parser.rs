//! Markdown parser module.
//!
//! Provides parsing of markdown content into structured elements
//! using pulldown-cmark for parsing.

/// Flush accumulated text segments as a paragraph or blockquote.
use crate::widgets::markdown_preview::widgets::markdown_widget::foundation::elements::{
    CheckboxState, CodeBlockBorderKind, ColumnAlignment, ElementKind, MarkdownElement,
    TableBorderKind, TextSegment,
};

/// Flush accumulated segments as a paragraph or blockquote.
///
/// # Arguments
///
/// * `lines` - The vector of styled lines to append to
/// * `segments` - The accumulated text segments to flush
/// * `blockquote_depth` - The current blockquote nesting depth
/// * `section_id` - The current section ID for collapse/expand tracking
/// * `source_line` - The source line number for this content
pub fn flush_paragraph(
    lines: &mut Vec<MarkdownElement>,
    segments: &mut Vec<TextSegment>,
    blockquote_depth: usize,
    section_id: Option<usize>,
    source_line: usize,
) {
    if segments.is_empty() {
        return;
    }

    let content = std::mem::take(segments);

    if blockquote_depth > 0 {
        lines.push(MarkdownElement {
            kind: ElementKind::Blockquote {
                content,
                depth: blockquote_depth,
            },
            section_id,
            source_line,
        });
    } else {
        lines.push(MarkdownElement {
            kind: ElementKind::Paragraph(content),
            section_id,
            source_line,
        });
    }
}

/// Parse YAML frontmatter from markdown content.

/// Parse YAML frontmatter from the beginning of content.
///
/// # Arguments
///
/// * `content` - The markdown content that may contain frontmatter
///
/// # Returns
///
/// A tuple containing:
/// - `Option<Vec<(String, String)>>` - The parsed frontmatter fields as key-value pairs
/// - `&str` - The remaining content after frontmatter
/// - `usize` - The line count of the frontmatter (includes opening and closing `---` lines)
pub fn parse_frontmatter(content: &str) -> (Option<Vec<(String, String)>>, &str, usize) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, content, 0);
    }

    // Find the closing ---
    let after_opening = &trimmed[3..];
    if let Some(end_pos) = after_opening.find("\n---") {
        let frontmatter_text = &after_opening[..end_pos];
        let remaining = &after_opening[end_pos + 4..]; // Skip past "\n---"

        // Count lines: 1 for opening ---, lines in frontmatter_text, 1 for closing ---
        let frontmatter_lines = frontmatter_text.lines().count();
        let total_lines = 1 + frontmatter_lines + 1; // opening + content + closing

        // Parse the frontmatter fields
        let mut fields = Vec::new();
        for line in frontmatter_text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_string();
                let value = line[colon_pos + 1..].trim().to_string();
                // Remove surrounding quotes from value if present
                let value = if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    value[1..value.len() - 1].to_string()
                } else {
                    value
                };
                fields.push((key, value));
            }
        }

        if !fields.is_empty() {
            return (Some(fields), remaining, total_lines);
        }
    }

    (None, content, 0)
}

/// Render markdown content to markdown elements.
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};

use crate::widgets::markdown_preview::widgets::markdown_widget::extensions::theme::SyntaxHighlighter;

/// Calculate the display width of a string for terminal rendering.
/// This uses unicode_width but treats emoji as width 1 since many terminals
/// render emoji at width 1 instead of the Unicode-standard width 2.
fn terminal_display_width(s: &str) -> usize {
    s.chars()
        .map(|c| {
            // Check if character is likely an emoji (simplified check)
            // Emoji are typically in these ranges or have variation selectors
            let cp = c as u32;
            if (0x1F300..=0x1F9FF).contains(&cp)  // Miscellaneous Symbols and Pictographs, Emoticons, etc.
                || (0x2600..=0x26FF).contains(&cp)  // Miscellaneous Symbols
                || (0x2700..=0x27BF).contains(&cp)  // Dingbats
                || (0x1F600..=0x1F64F).contains(&cp) // Emoticons
                || (0x1F680..=0x1F6FF).contains(&cp) // Transport and Map Symbols
                || (0x2300..=0x23FF).contains(&cp)
            // Miscellaneous Technical
            {
                // Treat emoji as width 1 for terminal compatibility
                1
            } else {
                // Use standard unicode width for other characters
                unicode_width::UnicodeWidthChar::width(c).unwrap_or(0)
            }
        })
        .sum()
}

/// Render markdown content to markdown elements.
///
/// # Arguments
///
/// * `content` - The markdown content to render
/// * `frontmatter_collapsed` - Whether frontmatter should be collapsed by default
///
/// # Returns
///
/// A vector of MarkdownElement, ready for rendering.
pub fn render_markdown_to_elements(
    content: &str,
    frontmatter_collapsed: bool,
) -> Vec<MarkdownElement> {
    let mut lines = Vec::new();

    // Parse frontmatter first
    let (frontmatter, remaining_content, _frontmatter_line_count) = parse_frontmatter(content);

    // Track current source line (1-indexed)
    let mut current_source_line: usize = 1;

    // Add frontmatter if present - each field gets its own line
    if let Some(fields) = frontmatter {
        // Get context_id for collapsed display
        let context_id = fields
            .iter()
            .find(|(k, _)| k == "context_id")
            .map(|(_, v)| v.clone());

        // Line 1: Opening --- with collapse icon
        lines.push(MarkdownElement {
            kind: ElementKind::FrontmatterStart {
                collapsed: frontmatter_collapsed,
                context_id,
            },
            section_id: None, // Start is always visible (it's the toggle)
            source_line: current_source_line,
        });
        current_source_line += 1;

        // Each field gets its own line (section_id: Some(0) for frontmatter section)
        for (key, value) in fields {
            lines.push(MarkdownElement {
                kind: ElementKind::FrontmatterField { key, value },
                section_id: Some(0), // Frontmatter section
                source_line: current_source_line,
            });
            current_source_line += 1;
        }

        // Closing ---
        lines.push(MarkdownElement {
            kind: ElementKind::FrontmatterEnd,
            section_id: Some(0), // Part of frontmatter section
            source_line: current_source_line,
        });
        current_source_line += 1;

        // Empty line after frontmatter
        lines.push(MarkdownElement {
            kind: ElementKind::Empty,
            section_id: None,
            source_line: current_source_line,
        });
    }

    let mut current_segments: Vec<TextSegment> = Vec::new();
    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_started = false;
    let mut list_stack: Vec<(bool, usize)> = Vec::new(); // (ordered, current_number)
    let mut blockquote_depth: usize = 0;

    // Section tracking for collapse/expand
    let mut current_section_id: Option<usize> = None;
    let mut next_section_id: usize = 1; // Start from 1 (0 is reserved for frontmatter)

    // Table state
    let mut in_table = false;
    let mut table_header_done = false;
    let mut current_row_cells: Vec<String> = Vec::new();
    let mut table_col_widths: Vec<usize> = Vec::new();
    let mut table_alignments: Vec<ColumnAlignment> = Vec::new();
    let mut pending_table_rows: Vec<(Vec<String>, bool)> = Vec::new(); // (cells, is_header)

    // Text formatting state
    let mut in_bold = false;
    let mut in_italic = false;
    let mut in_link = false;
    let mut link_url = String::new();
    let mut link_icon_shown = false; // Track if we've shown the icon for the current link
    let mut in_strikethrough = false;

    // Build byte offset to line number mapping for remaining content
    let mut byte_to_line: Vec<usize> = Vec::with_capacity(remaining_content.len());
    let mut line_num = current_source_line;
    for ch in remaining_content.chars() {
        for _ in 0..ch.len_utf8() {
            byte_to_line.push(line_num);
        }
        if ch == '\n' {
            line_num += 1;
        }
    }
    // Helper to get line number from byte offset
    let get_line = |offset: usize| -> usize {
        byte_to_line
            .get(offset)
            .copied()
            .unwrap_or(current_source_line)
    };

    let options = Options::all();
    let parser = Parser::new_ext(remaining_content, options).into_offset_iter();

    // Track the last event's source line for use in flush_paragraph
    let mut last_event_source_line = current_source_line;

    for (event, range) in parser {
        let event_source_line = get_line(range.start);
        last_event_source_line = event_source_line;

        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { .. } => {
                    flush_paragraph(
                        &mut lines,
                        &mut current_segments,
                        blockquote_depth,
                        current_section_id,
                        event_source_line,
                    );
                }
                Tag::Paragraph => {
                    // Nothing special needed at start
                }
                Tag::CodeBlock(kind) => {
                    flush_paragraph(
                        &mut lines,
                        &mut current_segments,
                        blockquote_depth,
                        current_section_id,
                        event_source_line,
                    );
                    in_code_block = true;
                    code_block_started = false;
                    code_block_lang = match kind {
                        CodeBlockKind::Fenced(lang) => lang.to_string(),
                        CodeBlockKind::Indented => String::new(),
                    };
                }
                Tag::List(start) => {
                    // If we're inside a list item and have content, create the list item first
                    if !list_stack.is_empty() && !current_segments.is_empty() {
                        let depth = list_stack.len().saturating_sub(1);
                        let (ordered, number) = list_stack.last().copied().unwrap_or((false, 1));
                        let content = std::mem::take(&mut current_segments);

                        lines.push(MarkdownElement {
                            kind: ElementKind::ListItem {
                                depth,
                                ordered,
                                number: if ordered { Some(number) } else { None },
                                content,
                            },
                            section_id: current_section_id,
                            source_line: event_source_line,
                        });

                        // Increment number for ordered lists
                        if let Some((is_ordered, num)) = list_stack.last_mut() {
                            if *is_ordered {
                                *num += 1;
                            }
                        }
                    } else {
                        flush_paragraph(
                            &mut lines,
                            &mut current_segments,
                            blockquote_depth,
                            current_section_id,
                            event_source_line,
                        );
                    }
                    let ordered = start.is_some();
                    let number = start.unwrap_or(1) as usize;
                    list_stack.push((ordered, number));
                }
                Tag::Item => {
                    // Will be handled with text content
                }
                Tag::BlockQuote(_) => {
                    flush_paragraph(
                        &mut lines,
                        &mut current_segments,
                        blockquote_depth,
                        current_section_id,
                        event_source_line,
                    );
                    blockquote_depth += 1;
                }
                Tag::Emphasis => {
                    in_italic = true;
                }
                Tag::Strong => {
                    in_bold = true;
                }
                Tag::Link { dest_url, .. } => {
                    in_link = true;
                    link_url = dest_url.to_string();
                    link_icon_shown = false; // Reset for new link
                }
                Tag::Strikethrough => {
                    in_strikethrough = true;
                }
                Tag::Table(alignments) => {
                    flush_paragraph(
                        &mut lines,
                        &mut current_segments,
                        blockquote_depth,
                        current_section_id,
                        event_source_line,
                    );
                    in_table = true;
                    table_header_done = false;
                    table_col_widths.clear();
                    table_alignments = alignments
                        .iter()
                        .map(|a| match a {
                            pulldown_cmark::Alignment::None => ColumnAlignment::None,
                            pulldown_cmark::Alignment::Left => ColumnAlignment::Left,
                            pulldown_cmark::Alignment::Center => ColumnAlignment::Center,
                            pulldown_cmark::Alignment::Right => ColumnAlignment::Right,
                        })
                        .collect();
                    pending_table_rows.clear();
                }
                Tag::TableHead => {
                    current_row_cells.clear();
                }
                Tag::TableRow => {
                    current_row_cells.clear();
                }
                Tag::TableCell => {
                    // Start a new cell - push empty string to accumulate text into
                    current_row_cells.push(String::new());
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Heading(level) => {
                    let level_num = match level {
                        pulldown_cmark::HeadingLevel::H1 => 1,
                        pulldown_cmark::HeadingLevel::H2 => 2,
                        pulldown_cmark::HeadingLevel::H3 => 3,
                        pulldown_cmark::HeadingLevel::H4 => 4,
                        pulldown_cmark::HeadingLevel::H5 => 5,
                        pulldown_cmark::HeadingLevel::H6 => 6,
                    };

                    let text = std::mem::take(&mut current_segments);

                    // Create new section for this heading
                    let section_id = next_section_id;
                    next_section_id += 1;
                    current_section_id = Some(section_id);

                    lines.push(MarkdownElement {
                        kind: ElementKind::Heading {
                            level: level_num,
                            text,
                            section_id,
                            collapsed: false, // Default to expanded
                        },
                        section_id: None, // Headings themselves are not in a section (always visible)
                        source_line: event_source_line,
                    });

                    // Add empty line after heading to match source layout
                    lines.push(MarkdownElement {
                        kind: ElementKind::Empty,
                        section_id: current_section_id,
                        source_line: event_source_line,
                    });
                }
                TagEnd::Paragraph => {
                    flush_paragraph(
                        &mut lines,
                        &mut current_segments,
                        blockquote_depth,
                        current_section_id,
                        event_source_line,
                    );
                    // Add spacing line after paragraph.
                    // Use a plain empty line so a trailing blank line after a blockquote
                    // does not render an extra quote marker.
                    lines.push(MarkdownElement {
                        kind: ElementKind::Empty,
                        section_id: current_section_id,
                        source_line: event_source_line,
                    });
                }
                TagEnd::CodeBlock => {
                    // End code block
                    lines.push(MarkdownElement {
                        kind: ElementKind::CodeBlockBorder {
                            kind: CodeBlockBorderKind::Bottom,
                            blockquote_depth,
                        },
                        section_id: current_section_id,
                        source_line: event_source_line,
                    });
                    lines.push(MarkdownElement {
                        kind: ElementKind::Empty,
                        section_id: current_section_id,
                        source_line: event_source_line,
                    });
                    in_code_block = false;
                    code_block_lang.clear();
                }
                TagEnd::List(_) => {
                    list_stack.pop();
                    if list_stack.is_empty() {
                        lines.push(MarkdownElement {
                            kind: ElementKind::Empty,
                            section_id: current_section_id,
                            source_line: event_source_line,
                        });
                    }
                }
                TagEnd::Item => {
                    if !current_segments.is_empty() {
                        let depth = list_stack.len().saturating_sub(1);
                        let (ordered, number) = list_stack.last().copied().unwrap_or((false, 1));
                        let content = std::mem::take(&mut current_segments);

                        lines.push(MarkdownElement {
                            kind: ElementKind::ListItem {
                                depth,
                                ordered,
                                number: if ordered { Some(number) } else { None },
                                content,
                            },
                            section_id: current_section_id,
                            source_line: event_source_line,
                        });

                        // Increment number for ordered lists
                        if let Some((is_ordered, num)) = list_stack.last_mut() {
                            if *is_ordered {
                                *num += 1;
                            }
                        }
                    }
                }
                TagEnd::BlockQuote(_) => {
                    flush_paragraph(
                        &mut lines,
                        &mut current_segments,
                        blockquote_depth,
                        current_section_id,
                        event_source_line,
                    );
                    blockquote_depth = blockquote_depth.saturating_sub(1);
                }
                TagEnd::Emphasis => {
                    in_italic = false;
                }
                TagEnd::Strong => {
                    in_bold = false;
                }
                TagEnd::Link => {
                    in_link = false;
                    link_url.clear();
                }
                TagEnd::Strikethrough => {
                    in_strikethrough = false;
                }
                TagEnd::Table => {
                    // Render the complete table with proper borders
                    if !pending_table_rows.is_empty() {
                        // Top border
                        lines.push(MarkdownElement {
                            kind: ElementKind::TableBorder(TableBorderKind::Top(
                                table_col_widths.clone(),
                            )),
                            section_id: current_section_id,
                            source_line: event_source_line,
                        });

                        for (cells, is_header) in pending_table_rows.drain(..) {
                            // Pad cells to column widths with proper alignment
                            let padded_cells: Vec<String> = cells
                                .iter()
                                .enumerate()
                                .map(|(j, cell)| {
                                    let width = table_col_widths
                                        .get(j)
                                        .copied()
                                        .unwrap_or_else(|| terminal_display_width(cell));
                                    let alignment = table_alignments
                                        .get(j)
                                        .copied()
                                        .unwrap_or(ColumnAlignment::None);
                                    // Pad based on terminal display width (emoji-aware)
                                    let cell_width = terminal_display_width(cell);
                                    let padding = width.saturating_sub(cell_width);
                                    match alignment {
                                        ColumnAlignment::Right => {
                                            format!("{}{}", " ".repeat(padding), cell)
                                        }
                                        ColumnAlignment::Center => {
                                            let left_pad = padding / 2;
                                            let right_pad = padding - left_pad;
                                            format!(
                                                "{}{}{}",
                                                " ".repeat(left_pad),
                                                cell,
                                                " ".repeat(right_pad)
                                            )
                                        }
                                        ColumnAlignment::Left | ColumnAlignment::None => {
                                            format!("{}{}", cell, " ".repeat(padding))
                                        }
                                    }
                                })
                                .collect();

                            lines.push(MarkdownElement {
                                kind: ElementKind::TableRow {
                                    cells: padded_cells,
                                    is_header,
                                    alignments: table_alignments.clone(),
                                },
                                section_id: current_section_id,
                                source_line: event_source_line,
                            });

                            // Header separator after first row
                            if is_header {
                                lines.push(MarkdownElement {
                                    kind: ElementKind::TableBorder(
                                        TableBorderKind::HeaderSeparator(table_col_widths.clone()),
                                    ),
                                    section_id: current_section_id,
                                    source_line: event_source_line,
                                });
                            }
                        }

                        // Bottom border
                        lines.push(MarkdownElement {
                            kind: ElementKind::TableBorder(TableBorderKind::Bottom(
                                table_col_widths.clone(),
                            )),
                            section_id: current_section_id,
                            source_line: event_source_line,
                        });
                    }

                    in_table = false;
                    lines.push(MarkdownElement {
                        kind: ElementKind::Empty,
                        section_id: current_section_id,
                        source_line: event_source_line,
                    });
                }
                TagEnd::TableHead => {
                    // Finalize header row
                    for (i, cell) in current_row_cells.iter().enumerate() {
                        let cell_width = terminal_display_width(cell);
                        if i >= table_col_widths.len() {
                            table_col_widths.push(cell_width);
                        } else {
                            table_col_widths[i] = table_col_widths[i].max(cell_width);
                        }
                    }
                    pending_table_rows.push((current_row_cells.clone(), true)); // Header
                    current_row_cells.clear();
                    table_header_done = true;
                }
                TagEnd::TableRow => {
                    // Finalize body row
                    if table_header_done {
                        for (i, cell) in current_row_cells.iter().enumerate() {
                            let cell_width = terminal_display_width(cell);
                            if i >= table_col_widths.len() {
                                table_col_widths.push(cell_width);
                            } else {
                                table_col_widths[i] = table_col_widths[i].max(cell_width);
                            }
                        }

                        pending_table_rows.push((current_row_cells.clone(), false));
                    }
                    current_row_cells.clear();
                }
                TagEnd::TableCell => {
                    // Cell content already added via Text events
                }
                _ => {}
            },
            Event::Text(text) => {
                if in_code_block {
                    // Start code block with header if not done yet
                    if !code_block_started {
                        // Header includes the top border
                        lines.push(MarkdownElement {
                            kind: ElementKind::CodeBlockHeader {
                                language: code_block_lang.clone(),
                                blockquote_depth,
                            },
                            section_id: current_section_id,
                            source_line: event_source_line,
                        });
                        code_block_started = true;
                    }

                    // Add each line of code with syntax highlighting
                    let highlighter = SyntaxHighlighter::new();
                    for (i, line) in text.lines().enumerate() {
                        let highlighted = highlighter.highlight(line, &code_block_lang);
                        lines.push(MarkdownElement {
                            kind: ElementKind::CodeBlockContent {
                                content: line.to_string(),
                                highlighted,
                                line_number: i + 1,
                                blockquote_depth,
                            },
                            section_id: current_section_id,
                            source_line: event_source_line + i,
                        });
                    }
                } else if in_table {
                    // Accumulate text for current cell
                    if let Some(last_cell) = current_row_cells.last_mut() {
                        last_cell.push_str(&text);
                    } else {
                        current_row_cells.push(text.to_string());
                    }
                } else {
                    let segment = if in_link {
                        // Detect autolink: text matches URL (with or without protocol)
                        let text_str = text.to_string();
                        let is_autolink = text_str == link_url
                            || link_url.ends_with(&text_str)
                            || text_str.starts_with("http://")
                            || text_str.starts_with("https://");
                        let show_icon = !link_icon_shown;
                        link_icon_shown = true; // Mark that we've shown the icon
                        TextSegment::Link {
                            text: text_str,
                            url: link_url.clone(),
                            is_autolink,
                            bold: in_bold,
                            italic: in_italic,
                            show_icon,
                        }
                    } else if in_strikethrough {
                        TextSegment::Strikethrough(text.to_string())
                    } else if in_bold && in_italic {
                        TextSegment::BoldItalic(text.to_string())
                    } else if in_bold {
                        TextSegment::Bold(text.to_string())
                    } else if in_italic {
                        TextSegment::Italic(text.to_string())
                    } else {
                        TextSegment::Plain(text.to_string())
                    };
                    current_segments.push(segment);
                }
            }
            Event::Code(code) => {
                if in_table {
                    // Add inline code to cell - just use the raw content
                    // The code content is displayed as-is without wrapping backticks
                    if let Some(last_cell) = current_row_cells.last_mut() {
                        last_cell.push_str(&code);
                    } else {
                        current_row_cells.push(code.to_string());
                    }
                } else {
                    current_segments.push(TextSegment::InlineCode(code.to_string()));
                }
            }
            Event::TaskListMarker(checked) => {
                // Add checkbox segment at the start of the list item
                let state = if checked {
                    CheckboxState::Checked
                } else {
                    CheckboxState::Unchecked
                };
                current_segments.insert(0, TextSegment::Checkbox(state));
            }
            Event::SoftBreak => {
                // Treat soft breaks as actual line breaks to match source file layout
                if !in_code_block && !in_table {
                    flush_paragraph(
                        &mut lines,
                        &mut current_segments,
                        blockquote_depth,
                        current_section_id,
                        event_source_line,
                    );
                }
            }
            Event::HardBreak => {
                if !in_code_block {
                    flush_paragraph(
                        &mut lines,
                        &mut current_segments,
                        blockquote_depth,
                        current_section_id,
                        event_source_line,
                    );
                }
            }
            Event::Rule => {
                flush_paragraph(
                    &mut lines,
                    &mut current_segments,
                    blockquote_depth,
                    current_section_id,
                    event_source_line,
                );
                lines.push(MarkdownElement {
                    kind: ElementKind::HorizontalRule,
                    section_id: current_section_id,
                    source_line: event_source_line,
                });
                lines.push(MarkdownElement {
                    kind: ElementKind::Empty,
                    section_id: current_section_id,
                    source_line: event_source_line,
                });
            }
            _ => {}
        }
    }

    // Flush any remaining content
    flush_paragraph(
        &mut lines,
        &mut current_segments,
        blockquote_depth,
        current_section_id,
        last_event_source_line,
    );

    // Remove trailing empty lines
    while matches!(lines.last(), Some(l) if matches!(l.kind, ElementKind::Empty)) {
        lines.pop();
    }

    if lines.is_empty() {
        lines.push(MarkdownElement {
            kind: ElementKind::Empty,
            section_id: None,
            source_line: 1,
        });
    }

    lines
}
