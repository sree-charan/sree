use ratatui::{
    text::{Line, Span},
    style::{Color, Style, Modifier},
};

#[allow(dead_code)]
pub fn render_header(version: &str, model: &str, token_count: usize, max_tokens: usize) -> Line<'static> {
    Line::from(vec![
        Span::styled(" 🤖 sree ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(format!("v{}", version), Style::default().fg(Color::DarkGray)),
        Span::raw(" │ "),
        Span::styled(model.to_string(), Style::default().fg(Color::Yellow)),
        Span::raw(" │ "),
        Span::styled(
            format!("tokens: {}/{}", token_count, max_tokens),
            Style::default().fg(Color::Green),
        ),
    ])
}
