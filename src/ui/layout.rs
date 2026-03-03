use ratatui::layout::{Constraint, Direction, Layout, Rect};

#[allow(dead_code)]
pub struct AppLayout {
    pub header: Rect,
    pub chat: Rect,
    pub input: Rect,
    pub status: Rect,
}

#[allow(dead_code)]
impl AppLayout {
    pub fn new(area: Rect) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Header
                Constraint::Min(0),     // Chat area
                Constraint::Length(3),  // Input area
                Constraint::Length(1),  // Status bar
            ])
            .split(area);
        
        Self {
            header: chunks[0],
            chat: chunks[1],
            input: chunks[2],
            status: chunks[3],
        }
    }
}
