use ratatui::{
    text::Line,
    widgets::{Block, Borders, Paragraph},
    layout::Rect,
    Frame,
    style::Style,
};
use crate::ui::theme::Theme;

#[allow(dead_code)]
pub fn render_chat_view<'a>(
    frame: &mut Frame,
    area: Rect,
    lines: Vec<Line<'a>>,
    scroll_offset: u16,
    _theme: &Theme,
) {
    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());
    
    let paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((scroll_offset, 0));
    
    frame.render_widget(paragraph, area);
}
