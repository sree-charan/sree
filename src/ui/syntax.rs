use ratatui::style::{Color, Style};
use ratatui::text::Span;
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Theme};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[allow(dead_code)]
pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

#[allow(dead_code)]
impl SyntaxHighlighter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();
        
        Self { syntax_set, theme }
    }

    pub fn highlight(&self, code: &str, lang: &str) -> Vec<Span<'static>> {
        let syntax = self.syntax_set
            .find_syntax_by_token(lang)
            .or_else(|| self.syntax_set.find_syntax_by_extension(lang))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let mut highlighter = HighlightLines::new(syntax, &self.theme);
        let mut spans = Vec::new();

        for line in LinesWithEndings::from(code) {
            if let Ok(ranges) = highlighter.highlight_line(line, &self.syntax_set) {
                for (style, text) in ranges {
                    let fg = Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                    spans.push(Span::styled(text.to_string(), Style::default().fg(fg)));
                }
            }
        }

        spans
    }
}

// Production-quality syntax highlighting using syntect
pub fn highlight_code(code: &str, lang: &str) -> Vec<Span<'static>> {
    use std::sync::OnceLock;
    
    static HIGHLIGHTER: OnceLock<SyntaxHighlighter> = OnceLock::new();
    let highlighter = HIGHLIGHTER.get_or_init(SyntaxHighlighter::new);
    
    let syntax = highlighter.syntax_set
        .find_syntax_by_token(lang)
        .or_else(|| highlighter.syntax_set.find_syntax_by_extension(lang))
        .unwrap_or_else(|| highlighter.syntax_set.find_syntax_plain_text());

    let mut hl = HighlightLines::new(syntax, &highlighter.theme);
    let mut spans = Vec::new();

    for line in LinesWithEndings::from(code) {
        if let Ok(ranges) = hl.highlight_line(line, &highlighter.syntax_set) {
            for (style, text) in ranges {
                let fg = Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                spans.push(Span::styled(text.to_string(), Style::default().fg(fg)));
            }
        } else {
            // Fallback to plain text if highlighting fails
            spans.push(Span::styled(line.to_string(), Style::default().fg(Color::White)));
        }
    }

    if spans.is_empty() {
        spans.push(Span::styled(code.to_string(), Style::default().fg(Color::White)));
    }

    spans
}
