#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum Model {
    #[default]
    ClaudeSonnet4,
    ClaudeOpus4,
    ClaudeHaiku3,
}

#[allow(dead_code)]
impl Model {
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::ClaudeSonnet4 => "us.anthropic.claude-sonnet-4-20250514-v1:0",
            Model::ClaudeOpus4 => "anthropic.claude-opus-4-20250514-v1:0",
            Model::ClaudeHaiku3 => "anthropic.claude-haiku-3-20250307-v1:0",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Model::ClaudeSonnet4 => "Claude Sonnet 4",
            Model::ClaudeOpus4 => "Claude Opus 4",
            Model::ClaudeHaiku3 => "Claude Haiku 3",
        }
    }

    pub fn context_window(&self) -> usize {
        match self {
            Model::ClaudeSonnet4 => 200_000,
            Model::ClaudeOpus4 => 200_000,
            Model::ClaudeHaiku3 => 200_000,
        }
    }
}

