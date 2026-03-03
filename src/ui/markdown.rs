//! Markdown to ratatui styled text renderer.
//!
//! This module converts markdown text into ratatui `Line` and `Span` structures
//! with appropriate styling. It uses `pulldown-cmark` for parsing and handles:
//!
//! - Headers (styled in cyan + bold)
//! - Bold and italic text
//! - Code blocks with syntax highlighting (via syntect)
//! - Inline code (styled in yellow)
//! - Lists (ordered and unordered, with nesting)
//! - Blockquotes (styled in gray + italic)
//! - Tables (formatted with borders)
//! - Links (styled in blue + underlined)
//!
//! ## Implementation Notes
//!
//! The renderer maintains a style stack to handle nested formatting (e.g., bold
//! text inside a blockquote). It accumulates spans into lines and flushes lines
//! when encountering block-level elements.
//!
//! Code blocks are syntax-highlighted using syntect with the configured theme.
//! If syntax highlighting fails, the code is displayed with basic styling.

use pulldown_cmark::{Event, Parser, Tag, TagEnd, CodeBlockKind};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub fn render_markdown(text: &str) -> Vec<Line<'static>> {
    let parser = Parser::new(text);
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_buffer = String::new();
    let mut style_stack = vec![Style::default()];
    let mut list_depth = 0;
    let mut list_item_number = vec![0];
    let mut in_blockquote = false;
    let mut in_table = false;
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut current_table_row: Vec<String> = Vec::new();
    let mut current_cell = String::new();

    for event in parser {
        match event {
            Event::Start(tag) => {
                let new_style = match &tag {
                    Tag::Heading { .. } => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    Tag::Strong => style_stack.last().unwrap().add_modifier(Modifier::BOLD),
                    Tag::Emphasis => style_stack.last().unwrap().add_modifier(Modifier::ITALIC),
                    Tag::CodeBlock(kind) => {
                        in_code_block = true;
                        if let CodeBlockKind::Fenced(lang) = kind {
                            code_lang = lang.to_string();
                        }
                        Style::default()
                    }
                    Tag::List(start) => {
                        list_depth += 1;
                        if let Some(num) = start {
                            if list_item_number.len() <= list_depth {
                                list_item_number.push(*num as i32);
                            } else {
                                list_item_number[list_depth] = *num as i32;
                            }
                        } else if list_item_number.len() <= list_depth {
                            list_item_number.push(0);
                        }
                        *style_stack.last().unwrap()
                    }
                    Tag::Item => {
                        let indent = "  ".repeat(list_depth.saturating_sub(1));
                        let marker = if list_item_number[list_depth] > 0 {
                            let num = list_item_number[list_depth];
                            list_item_number[list_depth] += 1;
                            format!("{}{}. ", indent, num)
                        } else {
                            format!("{}• ", indent)
                        };
                        current_line.push(Span::styled(marker, Style::default().fg(Color::Green)));
                        *style_stack.last().unwrap()
                    }
                    Tag::BlockQuote(_) => {
                        in_blockquote = true;
                        Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC)
                    }
                    Tag::Table(_) => {
                        in_table = true;
                        table_rows.clear();
                        *style_stack.last().unwrap()
                    }
                    Tag::TableHead | Tag::TableRow => {
                        current_table_row.clear();
                        *style_stack.last().unwrap()
                    }
                    Tag::TableCell => {
                        current_cell.clear();
                        *style_stack.last().unwrap()
                    }
                    _ => *style_stack.last().unwrap(),
                };
                style_stack.push(new_style);
            }
            Event::End(tag) => {
                style_stack.pop();
                match tag {
                    TagEnd::Heading(_) | TagEnd::Paragraph => {
                        if !current_line.is_empty() {
                            if in_blockquote {
                                let mut quote_line = vec![Span::styled("│ ", Style::default().fg(Color::DarkGray))];
                                quote_line.extend(current_line.clone());
                                lines.push(Line::from(quote_line));
                            } else {
                                lines.push(Line::from(current_line.clone()));
                            }
                            current_line.clear();
                        }
                        if !in_blockquote {
                            lines.push(Line::from(""));
                        }
                    }
                    TagEnd::CodeBlock => {
                        in_code_block = false;
                        if !code_buffer.is_empty() {
                            lines.extend(render_code_block(&code_buffer, &code_lang));
                            code_buffer.clear();
                            code_lang.clear();
                        }
                    }
                    TagEnd::List(_) => {
                        list_depth = list_depth.saturating_sub(1);
                        if list_depth == 0 {
                            lines.push(Line::from(""));
                        }
                    }
                    TagEnd::Item => {
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                        }
                    }
                    TagEnd::BlockQuote(_) => {
                        in_blockquote = false;
                        lines.push(Line::from(""));
                    }
                    TagEnd::Table => {
                        in_table = false;
                        if !table_rows.is_empty() {
                            lines.extend(render_table(&table_rows));
                            table_rows.clear();
                        }
                    }
                    TagEnd::TableHead | TagEnd::TableRow => {
                        if !current_table_row.is_empty() {
                            table_rows.push(current_table_row.clone());
                            current_table_row.clear();
                        }
                    }
                    TagEnd::TableCell => {
                        current_table_row.push(current_cell.clone());
                        current_cell.clear();
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                if in_code_block {
                    code_buffer.push_str(&text);
                } else if in_table {
                    current_cell.push_str(&text);
                } else {
                    let style = *style_stack.last().unwrap();
                    current_line.push(Span::styled(text.to_string(), style));
                }
            }
            Event::Code(code) => {
                let style = Style::default().fg(Color::Yellow);
                current_line.push(Span::styled(code.to_string(), style));
            }
            Event::SoftBreak | Event::HardBreak => {
                if !current_line.is_empty() {
                    if in_blockquote {
                        let mut quote_line = vec![Span::styled("│ ", Style::default().fg(Color::DarkGray))];
                        quote_line.extend(current_line.clone());
                        lines.push(Line::from(quote_line));
                    } else {
                        lines.push(Line::from(current_line.clone()));
                    }
                    current_line.clear();
                }
            }
            _ => {}
        }
    }

    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }

    lines
}

fn render_code_block(code: &str, lang: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    
    // Header with language
    if !lang.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("┌─ ".to_string(), Style::default().fg(Color::DarkGray)),
            Span::styled(lang.to_string(), Style::default().fg(Color::Green)),
        ]));
    } else {
        lines.push(Line::from(Span::styled("┌─────".to_string(), Style::default().fg(Color::DarkGray))));
    }

    // Code lines with syntax highlighting
    for line in code.lines() {
        let styled_line = if !lang.is_empty() {
            crate::ui::syntax::highlight_code(line, lang)
        } else {
            vec![Span::styled(line.to_string(), Style::default().fg(Color::White))]
        };
        
        let mut full_line = vec![Span::styled("│ ".to_string(), Style::default().fg(Color::DarkGray))];
        full_line.extend(styled_line);
        lines.push(Line::from(full_line));
    }

    // Footer
    lines.push(Line::from(Span::styled("└─────".to_string(), Style::default().fg(Color::DarkGray))));
    lines.push(Line::from("".to_string()));

    lines
}

fn render_table(rows: &[Vec<String>]) -> Vec<Line<'static>> {
    if rows.is_empty() {
        return Vec::new();
    }

    let mut lines = Vec::new();
    let num_cols = rows[0].len();
    
    // Calculate column widths
    let mut col_widths = vec![0; num_cols];
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            col_widths[i] = col_widths[i].max(cell.len());
        }
    }
    
    // Top border
    let mut top_border = String::from("┌");
    for (i, &width) in col_widths.iter().enumerate() {
        top_border.push_str(&"─".repeat(width + 2));
        if i < num_cols - 1 {
            top_border.push('┬');
        }
    }
    top_border.push('┐');
    lines.push(Line::from(Span::styled(top_border, Style::default().fg(Color::DarkGray))));
    
    // Header row (first row)
    if !rows.is_empty() {
        let mut header_spans = vec![Span::styled("│ ".to_string(), Style::default().fg(Color::DarkGray))];
        for (i, cell) in rows[0].iter().enumerate() {
            let padded = format!("{:width$}", cell, width = col_widths[i]);
            header_spans.push(Span::styled(padded, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)));
            header_spans.push(Span::styled(" │ ".to_string(), Style::default().fg(Color::DarkGray)));
        }
        lines.push(Line::from(header_spans));
        
        // Separator after header
        let mut sep = String::from("├");
        for (i, &width) in col_widths.iter().enumerate() {
            sep.push_str(&"─".repeat(width + 2));
            if i < num_cols - 1 {
                sep.push('┼');
            }
        }
        sep.push('┤');
        lines.push(Line::from(Span::styled(sep, Style::default().fg(Color::DarkGray))));
    }
    
    // Data rows
    for row in rows.iter().skip(1) {
        let mut row_spans = vec![Span::styled("│ ".to_string(), Style::default().fg(Color::DarkGray))];
        for (i, cell) in row.iter().enumerate() {
            let padded = format!("{:width$}", cell, width = col_widths[i]);
            row_spans.push(Span::styled(padded, Style::default()));
            row_spans.push(Span::styled(" │ ".to_string(), Style::default().fg(Color::DarkGray)));
        }
        lines.push(Line::from(row_spans));
    }
    
    // Bottom border
    let mut bottom_border = String::from("└");
    for (i, &width) in col_widths.iter().enumerate() {
        bottom_border.push_str(&"─".repeat(width + 2));
        if i < num_cols - 1 {
            bottom_border.push('┴');
        }
    }
    bottom_border.push('┘');
    lines.push(Line::from(Span::styled(bottom_border, Style::default().fg(Color::DarkGray))));
    lines.push(Line::from(""));
    
    lines
}
