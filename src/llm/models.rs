#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum Model {
    // Claude 4.6
    ClaudeOpus46,
    #[default]
    ClaudeSonnet46,
    // Claude 4.5
    ClaudeSonnet45,
    ClaudeHaiku45,
    // Claude 4
    ClaudeOpus4,
    ClaudeSonnet4,
}

#[allow(dead_code)]
impl Model {
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::ClaudeOpus46 => "anthropic.claude-opus-4-6-v1",
            Model::ClaudeSonnet46 => "anthropic.claude-sonnet-4-6",
            Model::ClaudeSonnet45 => "anthropic.claude-sonnet-4-5-20250929-v1:0",
            Model::ClaudeHaiku45 => "anthropic.claude-haiku-4-5-20251001-v1:0",
            Model::ClaudeOpus4 => "anthropic.claude-opus-4-20250514-v1:0",
            Model::ClaudeSonnet4 => "us.anthropic.claude-sonnet-4-20250514-v1:0",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Model::ClaudeOpus46 => "Claude Opus 4.6",
            Model::ClaudeSonnet46 => "Claude Sonnet 4.6",
            Model::ClaudeSonnet45 => "Claude Sonnet 4.5",
            Model::ClaudeHaiku45 => "Claude Haiku 4.5",
            Model::ClaudeOpus4 => "Claude Opus 4",
            Model::ClaudeSonnet4 => "Claude Sonnet 4",
        }
    }

    pub fn context_window(&self) -> usize {
        match self {
            Model::ClaudeOpus46 | Model::ClaudeSonnet46 => 200_000,
            Model::ClaudeSonnet45 | Model::ClaudeHaiku45 => 200_000,
            Model::ClaudeOpus4 | Model::ClaudeSonnet4 => 200_000,
        }
    }

    pub fn max_output(&self) -> usize {
        match self {
            Model::ClaudeOpus46 => 128_000,
            Model::ClaudeSonnet46 | Model::ClaudeSonnet45 | Model::ClaudeHaiku45 => 64_000,
            Model::ClaudeOpus4 | Model::ClaudeSonnet4 => 32_000,
        }
    }
}
