use ratatui::{
    text::{Line, Span},
    style::{Color, Style},
};

#[allow(dead_code)]
pub fn render_status_bar(mode: &str, help_text: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!(" [{}] ", mode), Style::default().fg(Color::Cyan)),
        Span::raw("│ "),
        Span::styled(help_text.to_string(), Style::default().fg(Color::DarkGray)),
    ])
}
