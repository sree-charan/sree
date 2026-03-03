use ratatui::{
    text::{Line, Span},
    style::{Color, Style},
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FileTreeNode {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<FileTreeNode>,
    pub depth: usize,
}

#[allow(dead_code)]
impl FileTreeNode {
    pub fn new(name: String, is_dir: bool, depth: usize) -> Self {
        Self {
            name,
            is_dir,
            children: Vec::new(),
            depth,
        }
    }
    
    pub fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        self.render_recursive(&mut lines);
        lines
    }
    
    fn render_recursive(&self, lines: &mut Vec<Line<'static>>) {
        let indent = "  ".repeat(self.depth);
        let icon = if self.is_dir { "📁" } else { "📄" };
        let color = if self.is_dir { Color::Cyan } else { Color::White };
        
        lines.push(Line::from(vec![
            Span::raw(indent),
            Span::styled(icon, Style::default().fg(color)),
            Span::raw(" "),
            Span::styled(self.name.clone(), Style::default().fg(color)),
        ]));
        
        for child in &self.children {
            child.render_recursive(lines);
        }
    }
}
