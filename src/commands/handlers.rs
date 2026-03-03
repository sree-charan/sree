// Slash command handlers

use crate::message::Message;
use crate::llm::token_counter::estimate_message_tokens;
use chrono;

pub enum CommandResult {
    Message(String),
    Quit,
    Clear,
    ModelSwitch(String),
    Export(String),  // filename
    SystemPrompt(String),
    ShowConfig,
    History,
    Compact,
    ThemeSwitch(String),
}

pub fn handle_command(input: &str, messages: &[Message], current_model: &str) -> Option<CommandResult> {
    let input = input.trim();
    if !input.starts_with('/') {
        return None;
    }

    let parts: Vec<&str> = input.splitn(2, ' ').collect();
    let command = parts[0];
    let args = parts.get(1).copied();

    match command {
        "/help" => Some(CommandResult::Message(help_text())),
        "/quit" | "/exit" => Some(CommandResult::Quit),
        "/clear" => Some(CommandResult::Clear),
        "/tokens" => Some(CommandResult::Message(tokens_info(messages))),
        "/history" => Some(CommandResult::History),
        "/model" => Some(handle_model_command(args, current_model)),
        "/version" => Some(CommandResult::Message(version_info())),
        "/export" => Some(handle_export_command(args, messages)),
        "/system" => Some(handle_system_command(args)),
        "/config" => Some(CommandResult::ShowConfig),
        "/compact" => Some(CommandResult::Compact),
        "/theme" => Some(handle_theme_command(args)),
        _ => Some(CommandResult::Message(format!("Unknown command: {}. Type /help for available commands.", command))),
    }
}

fn help_text() -> String {
    r#"Available Commands:
  /help              Show this help message
  /quit, /exit       Exit the application
  /clear             Clear conversation history
  /tokens            Show detailed token usage
  /history           Show conversation statistics
  /model [name]      Show current model or switch to a different model
  /version           Show version information
  /export [file]     Export conversation to markdown file
  /system <prompt>   Set custom system prompt
  /config            Show current configuration
  /compact           Compact conversation history to save tokens
  /theme [name]      Show current theme or switch theme (dark, light, monokai, dracula, nord, solarized)

Keybindings:
  Enter              Send message
  Shift+Enter        New line in input
  Ctrl+C             Cancel generation / quit
  Ctrl+L             Clear screen
  PageUp/PageDown    Scroll chat history
  Mouse Wheel        Scroll chat history

Available Models:
  claude-sonnet-4-20250514  (default, balanced)
  claude-opus-4-20250514    (most capable)
  claude-haiku-3-20250307   (fastest)"#.to_string()
}

fn tokens_info(messages: &[Message]) -> String {
    if messages.is_empty() {
        return "No messages yet. Token count: 0".to_string();
    }

    let mut total_tokens = 0;
    let mut user_tokens = 0;
    let mut assistant_tokens = 0;
    let mut system_tokens = 0;

    let mut breakdown = String::from("Token Usage Breakdown:\n\n");

    for (i, msg) in messages.iter().enumerate() {
        let msg_tokens = estimate_message_tokens(&msg.content);
        total_tokens += msg_tokens;

        match msg.role {
            crate::message::MessageRole::User => {
                user_tokens += msg_tokens;
                breakdown.push_str(&format!("  [{}] User: {} tokens\n", i + 1, msg_tokens));
            }
            crate::message::MessageRole::Assistant => {
                assistant_tokens += msg_tokens;
                breakdown.push_str(&format!("  [{}] Assistant: {} tokens\n", i + 1, msg_tokens));
            }
            crate::message::MessageRole::System => {
                system_tokens += msg_tokens;
                breakdown.push_str(&format!("  [{}] System: {} tokens\n", i + 1, msg_tokens));
            }
        }
    }

    let summary = format!(
        "\nSummary:\n  Total: {} tokens\n  User: {} tokens\n  Assistant: {} tokens\n  System: {} tokens\n  Messages: {}\n\nContext Limit: 200,000 tokens\nRemaining: {} tokens ({:.1}%)",
        total_tokens,
        user_tokens,
        assistant_tokens,
        system_tokens,
        messages.len(),
        200_000 - total_tokens,
        ((200_000 - total_tokens) as f64 / 200_000.0) * 100.0
    );

    format!("{}{}", breakdown, summary)
}

fn handle_model_command(args: Option<&str>, current_model: &str) -> CommandResult {
    match args {
        None => CommandResult::Message(format!(
            "Current model: {}\n\nAvailable models:\n  - claude-sonnet-4-20250514 (default, balanced)\n  - claude-opus-4-20250514 (most capable)\n  - claude-haiku-3-20250307 (fastest)",
            current_model
        )),
        Some(model) => {
            let model = model.trim();
            match model {
                "claude-sonnet-4-20250514" | "claude-opus-4-20250514" | "claude-haiku-3-20250307" => {
                    CommandResult::ModelSwitch(model.to_string())
                }
                _ => CommandResult::Message(format!(
                    "Unknown model: {}. Available models:\n  - claude-sonnet-4-20250514\n  - claude-opus-4-20250514\n  - claude-haiku-3-20250307",
                    model
                )),
            }
        }
    }
}

fn version_info() -> String {
    format!(
        "sree v{}\nRust CLI AI Assistant\n\nBuilt with:\n  - ratatui (TUI)\n  - tokio (async runtime)\n  - Anthropic Claude API",
        env!("CARGO_PKG_VERSION")
    )
}

fn handle_export_command(args: Option<&str>, messages: &[Message]) -> CommandResult {
    let filename = match args {
        Some(f) => f.trim(),
        None => "conversation.md",
    };

    let mut markdown = String::from("# Conversation Export\n\n");
    markdown.push_str(&format!("Exported: {}\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    markdown.push_str("---\n\n");

    for msg in messages {
        let role = match msg.role {
            crate::message::MessageRole::User => "**You**",
            crate::message::MessageRole::Assistant => "**sree**",
            crate::message::MessageRole::System => "*System*",
        };

        markdown.push_str(&format!("{}\n\n{}\n\n---\n\n", role, msg.content));
    }

    CommandResult::Export(filename.to_string())
}

fn handle_system_command(args: Option<&str>) -> CommandResult {
    match args {
        Some(prompt) => CommandResult::SystemPrompt(prompt.trim().to_string()),
        None => CommandResult::Message("Usage: /system <prompt>\nExample: /system You are a helpful coding assistant.".to_string()),
    }
}

pub fn history_info(messages: &[Message], current_model: &str) -> String {
    if messages.is_empty() {
        return "No conversation history yet.".to_string();
    }

    let mut user_count = 0;
    let mut assistant_count = 0;
    let mut system_count = 0;
    let mut tool_call_count = 0;

    for msg in messages {
        match msg.role {
            crate::message::MessageRole::User => user_count += 1,
            crate::message::MessageRole::Assistant => assistant_count += 1,
            crate::message::MessageRole::System => system_count += 1,
        }
        tool_call_count += msg.tool_calls.len();
    }

    let total_tokens: usize = messages.iter()
        .map(|m| estimate_message_tokens(&m.content))
        .sum();

    format!(
        "Conversation Statistics:\n\n\
        Model: {}\n\
        Total Messages: {}\n\
        ├─ User: {}\n\
        ├─ Assistant: {}\n\
        └─ System: {}\n\n\
        Tool Calls: {}\n\
        Total Tokens: {} / 200,000 ({:.1}%)\n\
        Remaining: {} tokens",
        current_model,
        messages.len(),
        user_count,
        assistant_count,
        system_count,
        tool_call_count,
        total_tokens,
        (total_tokens as f64 / 200_000.0) * 100.0,
        200_000 - total_tokens
    )
}

fn handle_theme_command(args: Option<&str>) -> CommandResult {
    use crate::ui::theme::Theme;
    
    match args {
        Some(theme_name) => {
            let theme = theme_name.trim().to_lowercase();
            let available = Theme::available_themes();
            if available.contains(&theme.as_str()) {
                CommandResult::ThemeSwitch(theme)
            } else {
                CommandResult::Message(format!(
                    "Unknown theme: {}\nAvailable themes: {}",
                    theme_name,
                    available.join(", ")
                ))
            }
        }
        None => {
            let available = Theme::available_themes();
            CommandResult::Message(format!(
                "Available themes: {}\nUsage: /theme <name>",
                available.join(", ")
            ))
        }
    }
}
