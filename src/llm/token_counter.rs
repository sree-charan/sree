// Approximate token counting for context management

#[allow(dead_code)]
pub fn estimate_tokens(text: &str) -> usize {
    // Rough approximation: ~4 characters per token
    // This is a simplification; real tokenization is more complex
    (text.len() as f64 / 4.0).ceil() as usize
}

#[allow(dead_code)]
pub fn estimate_message_tokens(content: &str) -> usize {
    // Add overhead for message structure
    estimate_tokens(content) + 4
}
