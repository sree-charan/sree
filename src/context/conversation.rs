use crate::llm::messages::{ApiMessage, ContentBlock};
use crate::llm::token_counter;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Conversation {
    messages: Vec<ApiMessage>,
    max_tokens: usize,
}

#[allow(dead_code)]
impl Conversation {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_tokens,
        }
    }

    pub fn add_message(&mut self, message: ApiMessage) {
        self.messages.push(message);
        self.truncate_if_needed();
    }

    pub fn messages(&self) -> &[ApiMessage] {
        &self.messages
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn estimate_tokens(&self) -> usize {
        self.messages
            .iter()
            .map(|msg| {
                msg.content.iter().map(|block| self.estimate_block_tokens(block)).sum::<usize>()
            })
            .sum::<usize>()
    }

    fn estimate_block_tokens(&self, block: &ContentBlock) -> usize {
        match block {
            ContentBlock::Text { text } => token_counter::estimate_tokens(text),
            ContentBlock::ToolUse { name, input, .. } => {
                token_counter::estimate_tokens(name)
                    + token_counter::estimate_tokens(&input.to_string())
            }
            ContentBlock::ToolResult { content, .. } => token_counter::estimate_tokens(content),
        }
    }

    fn truncate_if_needed(&mut self) {
        while self.estimate_tokens() > self.max_tokens && self.messages.len() > 1 {
            self.messages.remove(0);
        }
    }
}

impl Default for Conversation {
    fn default() -> Self {
        Self::new(150_000) // Leave room for system prompt and response
    }
}
