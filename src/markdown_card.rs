//! Markdown and code file rendering components.
//!
//! This module provides native GPUI rendering for markdown content and code files,
//! including collapsed card views for the canvas and full markdown parsing with
//! rich styling support.
//!
//! ## Features
//!
//! - **Collapsed Cards**: Compact file cards for canvas items (markdown, code)
//! - **Rich Markdown**: Headings, lists, code blocks, tables, blockquotes
//! - **Inline Styles**: Bold, italic, strikethrough, inline code
//! - **Theme Support**: Light/dark mode aware color schemes

use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{Icon, IconName};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use std::path::PathBuf;

/// Render a collapsed code file card (similar to markdown)
/// Takes theme colors as parameters for proper theming support
pub fn render_collapsed_code(
    filename: &str,
    language: &str,
    zoom: f32,
    bg: Hsla,
    border: Hsla,
    hover_bg: Hsla,
    hover_border: Hsla,
    icon_color: Hsla,
    text_color: Hsla,
    badge_bg: Hsla,
    badge_text: Hsla,
) -> Div {
    div()
        .size_full()
        .bg(bg)
        .rounded(px(6.0 * zoom))
        .border(px(1.0 * zoom))
        .border_color(border)
        .cursor(CursorStyle::PointingHand)
        .hover(move |s| s.bg(hover_bg).border_color(hover_border))
        .flex()
        .items_center()
        .gap(px(8.0 * zoom))
        .px(px(12.0 * zoom))
        .child(
            Icon::new(IconName::SquareTerminal)
                .size(px(16.0 * zoom))
                .text_color(icon_color),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(12.0 * zoom))
                .font_weight(FontWeight::MEDIUM)
                .text_color(text_color)
                .overflow_hidden()
                .whitespace_nowrap()
                .child(filename.to_string()),
        )
        .child(
            // Language badge
            div()
                .px(px(6.0 * zoom))
                .py(px(2.0 * zoom))
                .bg(badge_bg)
                .rounded(px(3.0 * zoom))
                .text_size(px(9.0 * zoom))
                .font_weight(FontWeight::MEDIUM)
                .text_color(badge_text)
                .child(language.to_uppercase()),
        )
}

/// Native markdown card with rich preview (no WebView)
pub struct MarkdownCard {
    pub path: PathBuf,
    pub title: String,
    pub content: String,
    pub expanded: bool,
}

impl MarkdownCard {
    pub fn new(path: PathBuf, content: String) -> Self {
        let title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string();

        Self {
            path,
            title,
            content,
            expanded: false,
        }
    }

    /// Parse and extract first N lines of meaningful content for preview
    pub fn preview_text(&self, max_lines: usize) -> String {
        let parser = Parser::new(&self.content);
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for event in parser {
            match event {
                Event::Text(text) | Event::Code(text) => {
                    current_line.push_str(&text);
                }
                Event::SoftBreak | Event::HardBreak => {
                    if !current_line.trim().is_empty() {
                        lines.push(current_line.trim().to_string());
                        current_line.clear();
                        if lines.len() >= max_lines {
                            break;
                        }
                    }
                }
                Event::End(TagEnd::Paragraph)
                | Event::End(TagEnd::Heading(_))
                | Event::End(TagEnd::Item) => {
                    if !current_line.trim().is_empty() {
                        lines.push(current_line.trim().to_string());
                        current_line.clear();
                        if lines.len() >= max_lines {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        if !current_line.trim().is_empty() && lines.len() < max_lines {
            lines.push(current_line.trim().to_string());
        }

        if lines.is_empty() {
            "Empty note".to_string()
        } else {
            lines.join(" • ")
        }
    }

    /// Get first heading or filename as title
    pub fn extract_title(&self) -> String {
        let parser = Parser::new(&self.content);
        let mut in_heading = false;
        let mut heading_text = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. })
                    if level == pulldown_cmark::HeadingLevel::H1 =>
                {
                    in_heading = true;
                }
                Event::Text(text) if in_heading => {
                    heading_text.push_str(&text);
                }
                Event::End(TagEnd::Heading(_)) if in_heading => {
                    if !heading_text.is_empty() {
                        return heading_text;
                    }
                }
                _ => {}
            }
        }

        self.title.clone()
    }
}

/// Extract title from markdown content (first H1 heading or fallback)
pub fn extract_title_from_content(content: &str, fallback: &str) -> String {
    let parser = Parser::new(content);
    let mut in_heading = false;
    let mut heading_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. })
                if level == pulldown_cmark::HeadingLevel::H1 =>
            {
                in_heading = true;
            }
            Event::Text(text) if in_heading => {
                heading_text.push_str(&text);
            }
            Event::End(TagEnd::Heading(_)) if in_heading => {
                if !heading_text.is_empty() {
                    return heading_text;
                }
            }
            _ => {}
        }
    }

    fallback.to_string()
}

/// Render a collapsed markdown card directly from content (avoids MarkdownCard allocation)
/// Takes theme colors as parameters for proper theming support
pub fn render_collapsed_markdown(
    title: &str,
    content: &str,
    zoom: f32,
    bg: Hsla,
    border: Hsla,
    hover_bg: Hsla,
    hover_border: Hsla,
    icon_color: Hsla,
    text_color: Hsla,
) -> Div {
    let display_title = extract_title_from_content(content, title);

    div()
        .size_full()
        .bg(bg)
        .rounded(px(6.0 * zoom))
        .border(px(1.0 * zoom))
        .border_color(border)
        .cursor(CursorStyle::PointingHand)
        .hover(move |s| s.bg(hover_bg).border_color(hover_border))
        .flex()
        .items_center()
        .gap(px(8.0 * zoom))
        .px(px(12.0 * zoom))
        .child(
            Icon::new(IconName::File)
                .size(px(16.0 * zoom))
                .text_color(icon_color),
        )
        .child(
            div()
                .flex_1()
                .text_size(px(12.0 * zoom))
                .font_weight(FontWeight::MEDIUM)
                .text_color(text_color)
                .overflow_hidden()
                .whitespace_nowrap()
                .child(display_title),
        )
}

/// Text segment with styling info
#[derive(Clone)]
struct TextSegment {
    text: String,
    bold: bool,
    italic: bool,
    code: bool,
    strikethrough: bool,
}

impl TextSegment {
    fn new(text: String, bold: bool, italic: bool, code: bool, strikethrough: bool) -> Self {
        Self {
            text,
            bold,
            italic,
            code,
            strikethrough,
        }
    }
}

/// Render a paragraph with mixed inline styles
fn render_styled_paragraph(
    segments: Vec<TextSegment>,
    zoom: f32,
    text_color: Hsla,
    text_bold: Hsla,
    text_italic: Hsla,
    text_muted: Hsla,
    code_bg: Hsla,
    code_text: Hsla,
) -> Div {
    // For simple paragraphs without mixed styles, just render as text
    if segments.len() == 1 && !segments[0].code && !segments[0].bold && !segments[0].italic {
        return div()
            .text_size(px(13.0 * zoom))
            .text_color(text_color)
            .line_height(relative(1.6))
            .child(segments[0].text.clone());
    }

    // For mixed styles, we need to combine into a single text with styling
    // Since GPUI doesn't support inline spans easily, we'll render segments inline
    let container = div()
        .w_full()
        .text_size(px(13.0 * zoom))
        .line_height(relative(1.6));

    // Build combined text for simple cases
    let all_plain = segments
        .iter()
        .all(|s| !s.code && !s.bold && !s.italic && !s.strikethrough);

    if all_plain {
        let combined: String = segments.iter().map(|s| s.text.as_str()).collect();
        return container.text_color(text_color).child(combined);
    }

    // For mixed formatting, use inline-flex with wrapping
    let mut inline_container = div()
        .flex()
        .flex_wrap()
        .items_baseline()
        .gap_y(px(4.0 * zoom));

    for segment in segments {
        let text = segment.text;
        if text.is_empty() {
            continue;
        }

        let span = if segment.code {
            // Inline code
            div()
                .flex_shrink_0()
                .px(px(4.0 * zoom))
                .py(px(1.0 * zoom))
                .bg(code_bg)
                .rounded(px(3.0 * zoom))
                .text_size(px(12.0 * zoom))
                .font_family("Iosevka Nerd Font")
                .text_color(code_text)
                .child(text)
        } else {
            let mut span = div().text_color(text_color);
            if segment.bold {
                span = span.font_weight(FontWeight::BOLD).text_color(text_bold);
            }
            if segment.italic {
                span = span.italic().text_color(text_italic);
            }
            if segment.strikethrough {
                span = span.text_color(text_muted);
            }
            span.child(text)
        };
        inline_container = inline_container.child(span);
    }

    container.child(inline_container)
}

/// Render parsed markdown with rich styling
pub fn render_markdown_content<V: 'static>(content: &str, zoom: f32, cx: &mut Context<V>) -> Div {
    use gpui_component::ActiveTheme as _;

    // Get theme colors
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let bg = cx.theme().background;
    let muted_bg = cx.theme().muted;
    let border = cx.theme().border;
    let _primary = cx.theme().primary;
    let success = cx.theme().success;
    let danger = cx.theme().danger;

    // Text colors - use theme foreground with varying opacity for heading hierarchy
    let heading_1 = fg;
    let heading_2 = fg.opacity(0.95);
    let heading_3 = fg.opacity(0.9);
    let heading_4 = fg.opacity(0.85);
    let heading_5 = fg.opacity(0.8);
    let text_color = fg.opacity(0.9);
    let text_bold = fg;
    let text_italic = fg.opacity(0.85);
    let text_muted = muted_fg;
    let text_quote = muted_fg;

    // Background and border colors from theme
    let code_bg = muted_bg;
    let code_border = border;
    let code_block_bg = muted_bg.opacity(0.5);
    let code_text = danger;           // Inline code uses danger color (red-ish)
    let code_block_color = success;   // Code block uses success color (green-ish)
    let border_color = border;
    let quote_border = border;
    let table_bg_header = muted_bg;
    let table_bg_alt = bg;
    let table_text = fg.opacity(0.85);
    let table_text_header = fg;
    let hr_color = border;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);
    let mut container = div()
        .flex()
        .flex_col()
        .gap(px(8.0 * zoom))
        .p(px(16.0 * zoom));

    // Style tracking
    let mut in_bold = false;
    let mut in_italic = false;
    let mut in_strikethrough = false;
    let mut heading_level: u8 = 0;
    let mut in_code_block = false;
    let mut code_block_text = String::new();
    let mut in_blockquote = false;
    let mut blockquote_segments: Vec<TextSegment> = Vec::new();

    // List tracking
    let mut list_stack: Vec<Option<u64>> = Vec::new(); // None = unordered, Some(n) = ordered starting at n
    let mut current_list_num: Vec<u64> = Vec::new();

    // Table tracking
    let mut in_table = false;
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();
    let mut current_cell = String::new();
    let mut is_table_header = false;

    // Current paragraph segments
    let mut paragraph_segments: Vec<TextSegment> = Vec::new();
    let mut current_text = String::new();

    // Helper to flush current text into segments
    let flush_text = |text: &mut String,
                      segments: &mut Vec<TextSegment>,
                      bold: bool,
                      italic: bool,
                      strikethrough: bool| {
        if !text.is_empty() {
            segments.push(TextSegment::new(
                text.clone(),
                bold,
                italic,
                false,
                strikethrough,
            ));
            text.clear();
        }
    };

    for event in parser {
        match event {
            // Headings
            Event::Start(Tag::Heading { level, .. }) => {
                heading_level = level as u8;
            }
            Event::End(TagEnd::Heading(_)) => {
                let text = std::mem::take(&mut current_text);
                let heading = match heading_level {
                    1 => div()
                        .text_size(px(26.0 * zoom))
                        .font_weight(FontWeight::BOLD)
                        .text_color(heading_1)
                        .pb(px(8.0 * zoom))
                        .mb(px(4.0 * zoom))
                        .border_b(px(1.0 * zoom))
                        .border_color(border_color)
                        .child(text),
                    2 => div()
                        .text_size(px(22.0 * zoom))
                        .font_weight(FontWeight::BOLD)
                        .text_color(heading_2)
                        .pt(px(8.0 * zoom))
                        .pb(px(4.0 * zoom))
                        .child(text),
                    3 => div()
                        .text_size(px(18.0 * zoom))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(heading_3)
                        .pt(px(6.0 * zoom))
                        .child(text),
                    4 => div()
                        .text_size(px(16.0 * zoom))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(heading_4)
                        .child(text),
                    _ => div()
                        .text_size(px(14.0 * zoom))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(heading_5)
                        .child(text),
                };
                container = container.child(heading);
                heading_level = 0;
            }

            // Paragraphs
            Event::Start(Tag::Paragraph) => {
                paragraph_segments.clear();
            }
            Event::End(TagEnd::Paragraph) => {
                flush_text(
                    &mut current_text,
                    &mut paragraph_segments,
                    in_bold,
                    in_italic,
                    in_strikethrough,
                );
                if !paragraph_segments.is_empty() {
                    if in_blockquote {
                        blockquote_segments.append(&mut paragraph_segments);
                    } else {
                        let segments = std::mem::take(&mut paragraph_segments);
                        container = container.child(render_styled_paragraph(
                            segments,
                            zoom,
                            text_color,
                            text_bold,
                            text_italic,
                            text_muted,
                            code_bg,
                            code_text,
                        ));
                    }
                }
            }

            // Bold/Strong
            Event::Start(Tag::Strong) => {
                flush_text(
                    &mut current_text,
                    &mut paragraph_segments,
                    in_bold,
                    in_italic,
                    in_strikethrough,
                );
                in_bold = true;
            }
            Event::End(TagEnd::Strong) => {
                flush_text(
                    &mut current_text,
                    &mut paragraph_segments,
                    in_bold,
                    in_italic,
                    in_strikethrough,
                );
                in_bold = false;
            }

            // Italic/Emphasis
            Event::Start(Tag::Emphasis) => {
                flush_text(
                    &mut current_text,
                    &mut paragraph_segments,
                    in_bold,
                    in_italic,
                    in_strikethrough,
                );
                in_italic = true;
            }
            Event::End(TagEnd::Emphasis) => {
                flush_text(
                    &mut current_text,
                    &mut paragraph_segments,
                    in_bold,
                    in_italic,
                    in_strikethrough,
                );
                in_italic = false;
            }

            // Strikethrough
            Event::Start(Tag::Strikethrough) => {
                flush_text(
                    &mut current_text,
                    &mut paragraph_segments,
                    in_bold,
                    in_italic,
                    in_strikethrough,
                );
                in_strikethrough = true;
            }
            Event::End(TagEnd::Strikethrough) => {
                flush_text(
                    &mut current_text,
                    &mut paragraph_segments,
                    in_bold,
                    in_italic,
                    in_strikethrough,
                );
                in_strikethrough = false;
            }

            // Inline code
            Event::Code(code) => {
                flush_text(
                    &mut current_text,
                    &mut paragraph_segments,
                    in_bold,
                    in_italic,
                    in_strikethrough,
                );
                paragraph_segments.push(TextSegment::new(
                    code.to_string(),
                    false,
                    false,
                    true,
                    false,
                ));
            }

            // Code blocks
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
                code_block_text.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                let text = std::mem::take(&mut code_block_text);
                container = container.child(
                    div()
                        .bg(code_block_bg)
                        .rounded(px(6.0 * zoom))
                        .p(px(12.0 * zoom))
                        .border(px(1.0 * zoom))
                        .border_color(code_border)
                        .overflow_x_hidden()
                        .child(
                            div()
                                .text_size(px(12.0 * zoom))
                                .text_color(code_block_color)
                                .font_family("Iosevka Nerd Font")
                                .line_height(relative(1.5))
                                .whitespace_nowrap()
                                .child(text),
                        ),
                );
                in_code_block = false;
            }

            // Blockquotes
            Event::Start(Tag::BlockQuote) => {
                in_blockquote = true;
                blockquote_segments.clear();
            }
            Event::End(TagEnd::BlockQuote) => {
                let segments = std::mem::take(&mut blockquote_segments);
                container = container.child(
                    div()
                        .pl(px(12.0 * zoom))
                        .border_l(px(3.0 * zoom))
                        .border_color(quote_border)
                        .child(
                            render_styled_paragraph(
                                segments,
                                zoom,
                                text_quote,
                                text_bold,
                                text_italic,
                                text_muted,
                                code_bg,
                                code_text,
                            )
                            .italic(),
                        ),
                );
                in_blockquote = false;
            }

            // Lists
            Event::Start(Tag::List(first_num)) => {
                list_stack.push(first_num);
                current_list_num.push(first_num.unwrap_or(1));
            }
            Event::End(TagEnd::List(_)) => {
                list_stack.pop();
                current_list_num.pop();
            }
            Event::Start(Tag::Item) => {
                paragraph_segments.clear();
            }
            Event::End(TagEnd::Item) => {
                flush_text(
                    &mut current_text,
                    &mut paragraph_segments,
                    in_bold,
                    in_italic,
                    in_strikethrough,
                );
                let segments = std::mem::take(&mut paragraph_segments);

                let indent = (list_stack.len().saturating_sub(1)) as f32 * 16.0;
                let is_ordered = list_stack.last().map(|o| o.is_some()).unwrap_or(false);

                let bullet = if is_ordered {
                    if let Some(num) = current_list_num.last_mut() {
                        let s = format!("{}.", num);
                        *num += 1;
                        s
                    } else {
                        "1.".to_string()
                    }
                } else {
                    "•".to_string()
                };

                container = container.child(
                    div()
                        .flex()
                        .gap(px(8.0 * zoom))
                        .pl(px(indent * zoom))
                        .child(
                            div()
                                .w(px(20.0 * zoom))
                                .text_size(px(13.0 * zoom))
                                .text_color(text_muted)
                                .child(bullet),
                        )
                        .child(div().flex_1().child(render_styled_paragraph(
                            segments,
                            zoom,
                            text_color,
                            text_bold,
                            text_italic,
                            text_muted,
                            code_bg,
                            code_text,
                        ))),
                );
            }

            // Task list items
            Event::TaskListMarker(checked) => {
                let marker = if checked { "☑" } else { "☐" };
                current_text.push_str(marker);
                current_text.push(' ');
            }

            // Tables
            Event::Start(Tag::Table(_)) => {
                in_table = true;
                table_rows.clear();
            }
            Event::End(TagEnd::Table) => {
                // Render the table
                let rows = std::mem::take(&mut table_rows);
                if !rows.is_empty() {
                    let mut table = div()
                        .flex()
                        .flex_col()
                        .border(px(1.0 * zoom))
                        .border_color(border_color)
                        .rounded(px(4.0 * zoom))
                        .overflow_hidden();

                    for (row_idx, row) in rows.iter().enumerate() {
                        let is_header = row_idx == 0;
                        let mut row_div = div()
                            .flex()
                            .when(is_header, |d| {
                                d.bg(table_bg_header).font_weight(FontWeight::SEMIBOLD)
                            })
                            .when(!is_header && row_idx % 2 == 0, |d| d.bg(table_bg_alt));

                        for cell in row {
                            row_div = row_div.child(
                                div()
                                    .flex_1()
                                    .px(px(12.0 * zoom))
                                    .py(px(8.0 * zoom))
                                    .border_r(px(1.0 * zoom))
                                    .border_color(border_color)
                                    .text_size(px(12.0 * zoom))
                                    .text_color(if is_header {
                                        table_text_header
                                    } else {
                                        table_text
                                    })
                                    .child(cell.clone()),
                            );
                        }
                        table = table.child(row_div);
                    }
                    container = container.child(table);
                }
                in_table = false;
            }
            Event::Start(Tag::TableHead) => {
                is_table_header = true;
                current_row.clear();
            }
            Event::End(TagEnd::TableHead) => {
                table_rows.push(std::mem::take(&mut current_row));
                is_table_header = false;
            }
            Event::Start(Tag::TableRow) => {
                current_row.clear();
            }
            Event::End(TagEnd::TableRow) => {
                if !is_table_header {
                    table_rows.push(std::mem::take(&mut current_row));
                }
            }
            Event::Start(Tag::TableCell) => {
                current_cell.clear();
            }
            Event::End(TagEnd::TableCell) => {
                current_row.push(std::mem::take(&mut current_cell));
            }

            // Horizontal rule
            Event::Rule => {
                container = container.child(
                    div()
                        .h(px(1.0 * zoom))
                        .w_full()
                        .my(px(16.0 * zoom))
                        .bg(hr_color),
                );
            }

            // Links
            Event::Start(Tag::Link { .. }) => {
                flush_text(
                    &mut current_text,
                    &mut paragraph_segments,
                    in_bold,
                    in_italic,
                    in_strikethrough,
                );
            }
            Event::End(TagEnd::Link) => {
                // Link text is in current_text, style it as a link
                let text = std::mem::take(&mut current_text);
                if !text.is_empty() {
                    paragraph_segments.push(TextSegment {
                        text,
                        bold: false,
                        italic: false,
                        code: false,
                        strikethrough: false,
                    });
                }
            }

            // Text content
            Event::Text(text) => {
                if in_code_block {
                    code_block_text.push_str(&text);
                } else if in_table {
                    current_cell.push_str(&text);
                } else {
                    current_text.push_str(&text);
                }
            }

            Event::SoftBreak => {
                if in_code_block {
                    code_block_text.push('\n');
                } else {
                    current_text.push(' ');
                }
            }
            Event::HardBreak => {
                if in_code_block {
                    code_block_text.push('\n');
                } else {
                    flush_text(
                        &mut current_text,
                        &mut paragraph_segments,
                        in_bold,
                        in_italic,
                        in_strikethrough,
                    );
                    // Add a line break element
                }
            }

            _ => {}
        }
    }

    // Handle any remaining text
    flush_text(
        &mut current_text,
        &mut paragraph_segments,
        in_bold,
        in_italic,
        in_strikethrough,
    );
    if !paragraph_segments.is_empty() {
        container = container.child(render_styled_paragraph(
            paragraph_segments,
            zoom,
            text_color,
            text_bold,
            text_italic,
            text_muted,
            code_bg,
            code_text,
        ));
    }

    container
}
