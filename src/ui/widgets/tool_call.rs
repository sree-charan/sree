use ratatui::{
    text::{Line, Span},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph},
    layout::Rect,
    Frame,
};
use crate::message::ToolCallStatus;

#[allow(dead_code)]
pub struct ToolCallWidget {
    pub name: String,
    pub input: String,
    pub status: ToolCallStatus,
    pub result: Option<String>,
    pub expanded: bool,
}

#[allow(dead_code)]
impl ToolCallWidget {
    pub fn new(name: String, input: String, status: ToolCallStatus) -> Self {
        Self {
            name,
            input,
            status,
            result: None,
            expanded: false,
        }
    }

    pub fn set_result(&mut self, result: String) {
        self.result = Some(result);
    }

    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let (status_icon, status_color) = match self.status {
            ToolCallStatus::Pending => ("⏳", Color::Yellow),
            ToolCallStatus::Running => ("⚙", Color::Cyan),
            ToolCallStatus::Success => ("✓", Color::Green),
            ToolCallStatus::Error => ("✗", Color::Red),
        };

        let mut lines = vec![
            Line::from(vec![
                Span::styled(status_icon, Style::default().fg(status_color)),
                Span::raw(" Tool: "),
                Span::styled(&self.name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
        ];

        if !self.input.is_empty() {
            let input_display = if self.input.len() > 80 && !self.expanded {
                format!("{}...", &self.input[..77])
            } else {
                self.input.clone()
            };
            lines.push(Line::from(vec![
                Span::raw("├─ Input: "),
                Span::styled(input_display, Style::default().fg(Color::Gray)),
            ]));
        }

        if let Some(result) = &self.result {
            let result_display = if result.len() > 200 && !self.expanded {
                format!("{}...", &result[..197])
            } else {
                result.clone()
            };
            lines.push(Line::from(vec![
                Span::raw("└─ Result: "),
                Span::styled(result_display, Style::default().fg(Color::White)),
            ]));
            
            if result.len() > 200 {
                lines.push(Line::from(Span::styled(
                    if self.expanded { "  [Press 't' to collapse]" } else { "  [Press 't' to expand]" },
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(status_color));

        let paragraph = Paragraph::new(lines).block(block);
        frame.render_widget(paragraph, area);
    }
}
