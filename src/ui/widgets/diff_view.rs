use ratatui::{
    text::{Line, Span},
    style::{Color, Style},
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DiffLine {
    Added(String),
    Removed(String),
    Context(String),
}

#[allow(dead_code)]
pub struct DiffView {
    pub lines: Vec<DiffLine>,
}

#[allow(dead_code)]
impl DiffView {
    pub fn new(lines: Vec<DiffLine>) -> Self {
        Self { lines }
    }
    
    pub fn from_strings(old: &str, new: &str) -> Self {
        let mut lines = Vec::new();
        let old_lines: Vec<&str> = old.lines().collect();
        let new_lines: Vec<&str> = new.lines().collect();
        
        // Simple line-by-line diff
        let max_len = old_lines.len().max(new_lines.len());
        for i in 0..max_len {
            match (old_lines.get(i), new_lines.get(i)) {
                (Some(old_line), Some(new_line)) => {
                    if old_line != new_line {
                        lines.push(DiffLine::Removed(old_line.to_string()));
                        lines.push(DiffLine::Added(new_line.to_string()));
                    } else {
                        lines.push(DiffLine::Context(old_line.to_string()));
                    }
                }
                (Some(old_line), None) => {
                    lines.push(DiffLine::Removed(old_line.to_string()));
                }
                (None, Some(new_line)) => {
                    lines.push(DiffLine::Added(new_line.to_string()));
                }
                (None, None) => break,
            }
        }
        
        Self { lines }
    }
    
    pub fn render(&self) -> Vec<Line<'static>> {
        self.lines.iter().map(|line| {
            match line {
                DiffLine::Added(text) => Line::from(vec![
                    Span::styled("+ ", Style::default().fg(Color::Green)),
                    Span::styled(text.clone(), Style::default().fg(Color::Green)),
                ]),
                DiffLine::Removed(text) => Line::from(vec![
                    Span::styled("- ", Style::default().fg(Color::Red)),
                    Span::styled(text.clone(), Style::default().fg(Color::Red)),
                ]),
                DiffLine::Context(text) => Line::from(vec![
                    Span::raw("  "),
                    Span::raw(text.clone()),
                ]),
            }
        }).collect()
    }
}
