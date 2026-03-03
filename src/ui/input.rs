use ratatui::{
    widgets::{Block, Borders},
    layout::Rect,
    Frame,
    style::Style,
};
use tui_textarea::TextArea;
use crate::ui::theme::Theme;

#[allow(dead_code)]
pub fn render_input<'a>(
    frame: &mut Frame,
    area: Rect,
    textarea: &mut TextArea<'a>,
    theme: &Theme,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.input_border))
        .title(" Input ");
    
    textarea.set_block(block);
    frame.render_widget(&*textarea, area);
}
