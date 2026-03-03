use ratatui::{
    text::{Line, Span},
    style::{Color, Style},
};

#[derive(Default)]
pub struct Spinner {
    frame: usize,
}

impl Spinner {
    const FRAMES: &'static [&'static str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        self.frame = (self.frame + 1) % Self::FRAMES.len();
    }

    pub fn render(&self) -> Line<'static> {
        Line::from(vec![
            Span::styled(
                Self::FRAMES[self.frame],
                Style::default().fg(Color::Cyan),
            ),
            Span::raw(" Thinking..."),
        ])
    }
}
