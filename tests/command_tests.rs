use sree::commands::handlers::{handle_command, CommandResult};
use sree::message::Message;

#[test]
fn test_help_command() {
    let messages = vec![];
    let result = handle_command("/help", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::Message(msg) => {
            assert!(msg.contains("Available Commands"));
            assert!(msg.contains("/help"));
            assert!(msg.contains("/quit"));
            assert!(msg.contains("/tokens"));
            assert!(msg.contains("/export"));
            assert!(msg.contains("/system"));
            assert!(msg.contains("/config"));
        }
        _ => panic!("Expected Message result"),
    }
}

#[test]
fn test_quit_command() {
    let messages = vec![];
    let result = handle_command("/quit", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    assert!(matches!(result.unwrap(), CommandResult::Quit));
}

#[test]
fn test_exit_command() {
    let messages = vec![];
    let result = handle_command("/exit", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    assert!(matches!(result.unwrap(), CommandResult::Quit));
}

#[test]
fn test_clear_command() {
    let messages = vec![];
    let result = handle_command("/clear", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    assert!(matches!(result.unwrap(), CommandResult::Clear));
}

#[test]
fn test_tokens_command_empty() {
    let messages = vec![];
    let result = handle_command("/tokens", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::Message(msg) => {
            assert!(msg.contains("No messages yet"));
            assert!(msg.contains("Token count: 0"));
        }
        _ => panic!("Expected Message result"),
    }
}

#[test]
fn test_tokens_command_with_messages() {
    let messages = vec![
        Message::user("Hello".to_string()),
        Message::assistant("Hi there!".to_string()),
    ];
    let result = handle_command("/tokens", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::Message(msg) => {
            assert!(msg.contains("Token Usage Breakdown"));
            assert!(msg.contains("User:"));
            assert!(msg.contains("Assistant:"));
            assert!(msg.contains("Total:"));
            assert!(msg.contains("Remaining:"));
            assert!(msg.contains("200,000 tokens"));
        }
        _ => panic!("Expected Message result"),
    }
}

#[test]
fn test_model_command_show_current() {
    let messages = vec![];
    let result = handle_command("/model", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::Message(msg) => {
            assert!(msg.contains("Current model: claude-sonnet-4-20250514"));
            assert!(msg.contains("Available models"));
        }
        _ => panic!("Expected Message result"),
    }
}

#[test]
fn test_model_command_switch_valid() {
    let messages = vec![];
    let result = handle_command("/model claude-opus-4-20250514", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::ModelSwitch(model) => {
            assert_eq!(model, "claude-opus-4-20250514");
        }
        _ => panic!("Expected ModelSwitch result"),
    }
}

#[test]
fn test_model_command_switch_invalid() {
    let messages = vec![];
    let result = handle_command("/model invalid-model", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::Message(msg) => {
            assert!(msg.contains("Unknown model"));
            assert!(msg.contains("invalid-model"));
        }
        _ => panic!("Expected Message result"),
    }
}

#[test]
fn test_version_command() {
    let messages = vec![];
    let result = handle_command("/version", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::Message(msg) => {
            assert!(msg.contains("sree v"));
            assert!(msg.contains("Rust CLI AI Assistant"));
        }
        _ => panic!("Expected Message result"),
    }
}

#[test]
fn test_unknown_command() {
    let messages = vec![];
    let result = handle_command("/unknown", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::Message(msg) => {
            assert!(msg.contains("Unknown command"));
            assert!(msg.contains("/unknown"));
        }
        _ => panic!("Expected Message result"),
    }
}

#[test]
fn test_non_command_returns_none() {
    let messages = vec![];
    let result = handle_command("regular message", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_none());
}

#[test]
fn test_export_command_with_filename() {
    let messages = vec![
        Message::user("Hello".to_string()),
        Message::assistant("Hi there!".to_string()),
    ];
    let result = handle_command("/export test.md", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::Export(filename) => {
            assert_eq!(filename, "test.md");
        }
        _ => panic!("Expected Export result"),
    }
}

#[test]
fn test_export_command_default_filename() {
    let messages = vec![Message::user("Hello".to_string())];
    let result = handle_command("/export", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::Export(filename) => {
            assert_eq!(filename, "conversation.md");
        }
        _ => panic!("Expected Export result"),
    }
}

#[test]
fn test_system_command_with_prompt() {
    let messages = vec![];
    let result = handle_command("/system You are a helpful assistant", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::SystemPrompt(prompt) => {
            assert_eq!(prompt, "You are a helpful assistant");
        }
        _ => panic!("Expected SystemPrompt result"),
    }
}

#[test]
fn test_system_command_without_prompt() {
    let messages = vec![];
    let result = handle_command("/system", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    match result.unwrap() {
        CommandResult::Message(msg) => {
            assert!(msg.contains("Usage"));
            assert!(msg.contains("/system"));
        }
        _ => panic!("Expected Message result"),
    }
}

#[test]
fn test_config_command() {
    let messages = vec![];
    let result = handle_command("/config", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    assert!(matches!(result.unwrap(), CommandResult::ShowConfig));
}

#[test]
fn test_history_command() {
    let messages = vec![
        Message::system("Welcome".to_string()),
        Message::user("Hello".to_string()),
        Message::assistant("Hi there".to_string()),
    ];
    let result = handle_command("/history", &messages, "claude-sonnet-4-20250514");
    assert!(matches!(result, Some(CommandResult::History)));
}

#[test]
fn test_compact_command() {
    let messages = vec![];
    let result = handle_command("/compact", &messages, "claude-sonnet-4-20250514");
    
    assert!(result.is_some());
    assert!(matches!(result.unwrap(), CommandResult::Compact));
}

#[test]
fn test_theme_command_switch_valid() {
    let messages = vec![];
    let result = handle_command("/theme dark", &messages, "claude-sonnet-4-20250514");
    assert!(matches!(result, Some(CommandResult::ThemeSwitch(ref theme)) if theme == "dark"));
    
    let result = handle_command("/theme light", &messages, "claude-sonnet-4-20250514");
    assert!(matches!(result, Some(CommandResult::ThemeSwitch(ref theme)) if theme == "light"));
    
    let result = handle_command("/theme monokai", &messages, "claude-sonnet-4-20250514");
    assert!(matches!(result, Some(CommandResult::ThemeSwitch(ref theme)) if theme == "monokai"));
    
    let result = handle_command("/theme dracula", &messages, "claude-sonnet-4-20250514");
    assert!(matches!(result, Some(CommandResult::ThemeSwitch(ref theme)) if theme == "dracula"));
    
    let result = handle_command("/theme nord", &messages, "claude-sonnet-4-20250514");
    assert!(matches!(result, Some(CommandResult::ThemeSwitch(ref theme)) if theme == "nord"));
    
    let result = handle_command("/theme solarized", &messages, "claude-sonnet-4-20250514");
    assert!(matches!(result, Some(CommandResult::ThemeSwitch(ref theme)) if theme == "solarized"));
}

#[test]
fn test_theme_command_switch_invalid() {
    let messages = vec![];
    let result = handle_command("/theme invalid", &messages, "claude-sonnet-4-20250514");
    assert!(matches!(result, Some(CommandResult::Message(ref msg)) if msg.contains("Unknown theme")));
}

#[test]
fn test_theme_command_show_available() {
    let messages = vec![];
    let result = handle_command("/theme", &messages, "claude-sonnet-4-20250514");
    assert!(matches!(result, Some(CommandResult::Message(ref msg)) if msg.contains("Available themes")));
}
