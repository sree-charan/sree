# API Documentation

## Overview

This document describes the internal APIs for extending sree with new tools, themes, and commands.

## Adding a New Tool

### 1. Implement the Tool Trait

```rust
use async_trait::async_trait;
use serde_json::{json, Value};
use crate::tools::{Tool, ToolResult};

pub struct MyTool;

#[async_trait]
impl Tool for MyTool {
    fn name(&self) -> &str {
        "my_tool"
    }

    fn description(&self) -> &str {
        "Description of what my tool does"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "param1": {
                    "type": "string",
                    "description": "First parameter"
                },
                "param2": {
                    "type": "integer",
                    "description": "Second parameter"
                }
            },
            "required": ["param1"]
        })
    }

    async fn execute(&self, input: Value) -> anyhow::Result<ToolResult> {
        let param1 = input["param1"].as_str()
            .ok_or_else(|| anyhow::anyhow!("param1 is required"))?;
        let param2 = input["param2"].as_i64().unwrap_or(0);

        // Your tool logic here
        let result = format!("Processed {} with {}", param1, param2);

        Ok(ToolResult {
            content: result,
            is_error: false,
        })
    }
}
```

### 2. Register the Tool

In `src/tools/registry.rs`:

```rust
pub fn create_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(FileReadTool));
    registry.register(Box::new(FileWriteTool));
    registry.register(Box::new(BashTool));
    registry.register(Box::new(GrepTool));
    registry.register(Box::new(GlobTool));
    registry.register(Box::new(WebSearchTool));
    registry.register(Box::new(MyTool));  // Add your tool
    registry
}
```

### 3. Add Tests

In `tests/tool_tests.rs`:

```rust
#[tokio::test]
async fn test_my_tool() {
    let tool = MyTool;
    let input = json!({
        "param1": "test",
        "param2": 42
    });
    let result = tool.execute(input).await.unwrap();
    assert!(!result.is_error);
    assert!(result.content.contains("test"));
}
```

## Adding a New Theme

### 1. Define the Theme

In `src/ui/theme.rs`:

```rust
pub fn my_theme() -> Theme {
    Theme {
        name: "my_theme".to_string(),
        user_message: Style::default().fg(Color::Cyan),
        assistant_message: Style::default().fg(Color::Green),
        system_message: Style::default().fg(Color::Yellow),
        tool_call: Style::default().fg(Color::Magenta),
        tool_result: Style::default().fg(Color::Blue),
        error: Style::default().fg(Color::Red),
        header: Style::default().fg(Color::White).bg(Color::DarkGray),
        status_bar: Style::default().fg(Color::Black).bg(Color::White),
        input_border: Style::default().fg(Color::Cyan),
        code_block: Style::default().fg(Color::Green),
        border: Style::default().fg(Color::Gray),
    }
}
```

### 2. Register the Theme

In `src/ui/theme.rs`:

```rust
pub fn available_themes() -> Vec<String> {
    vec![
        "dark".to_string(),
        "light".to_string(),
        "monokai".to_string(),
        "dracula".to_string(),
        "nord".to_string(),
        "solarized".to_string(),
        "my_theme".to_string(),  // Add your theme
    ]
}

pub fn theme_from_name(name: &str) -> Theme {
    match name {
        "dark" => dark_theme(),
        "light" => light_theme(),
        "monokai" => monokai_theme(),
        "dracula" => dracula_theme(),
        "nord" => nord_theme(),
        "solarized" => solarized_theme(),
        "my_theme" => my_theme(),  // Add your theme
        _ => dark_theme(),
    }
}
```

## Adding a New Slash Command

### 1. Define the Command

In `src/commands/handlers.rs`:

```rust
pub fn handle_my_command(args: &str, app: &mut App) -> String {
    // Parse arguments
    let parts: Vec<&str> = args.split_whitespace().collect();
    
    // Validate input
    if parts.is_empty() {
        return "Usage: /mycommand <arg>".to_string();
    }
    
    // Execute command logic
    let result = format!("Executed with: {}", parts[0]);
    
    // Update app state if needed
    // app.some_state = new_value;
    
    result
}
```

### 2. Register the Command

In `src/commands/mod.rs`:

```rust
pub fn handle_command(input: &str, app: &mut App) -> Option<String> {
    if !input.starts_with('/') {
        return None;
    }

    let parts: Vec<&str> = input[1..].splitn(2, ' ').collect();
    let command = parts[0];
    let args = parts.get(1).unwrap_or(&"");

    match command {
        "help" => Some(handle_help()),
        "quit" | "exit" => Some(handle_quit()),
        "clear" => Some(handle_clear(app)),
        "model" => Some(handle_model(args, app)),
        "config" => Some(handle_config(app)),
        "theme" => Some(handle_theme(args, app)),
        "export" => Some(handle_export(args, app)),
        "history" => Some(handle_history(app)),
        "tokens" => Some(handle_tokens(app)),
        "system" => Some(handle_system(args, app)),
        "compact" => Some(handle_compact(app)),
        "version" => Some(handle_version()),
        "mycommand" => Some(handle_my_command(args, app)),  // Add your command
        _ => Some(format!("Unknown command: /{}. Type /help for available commands.", command)),
    }
}
```

### 3. Update Help Text

In `src/commands/handlers.rs`:

```rust
pub fn handle_help() -> String {
    r#"Available commands:
/help              - Show this help message
/quit, /exit       - Exit the application
/clear             - Clear conversation history
/model [name]      - Show or switch model
/config            - Show current configuration
/theme [name]      - Show or switch theme
/export [file]     - Export conversation to markdown
/history           - Show conversation summary
/tokens            - Show token usage details
/system <prompt>   - Set custom system prompt
/compact           - Summarize and compact conversation
/version           - Show version information
/mycommand <arg>   - Description of your command

Keybindings:
Enter              - Send message
Shift+Enter        - New line in input
Ctrl+C             - Cancel generation / quit
Ctrl+L             - Clear screen
Up/Down            - Navigate input history
PageUp/PageDown    - Scroll chat history
"#.to_string()
}
```

## Core Data Structures

### Message

```rust
pub struct Message {
    pub role: Role,
    pub content: Vec<ContentBlock>,
    pub timestamp: DateTime<Utc>,
}

pub enum Role {
    User,
    Assistant,
    System,
}

pub enum ContentBlock {
    Text { text: String },
    ToolUse { id: String, name: String, input: Value },
    ToolResult { tool_use_id: String, content: String, is_error: bool },
}
```

### Conversation

```rust
pub struct Conversation {
    messages: Vec<Message>,
    token_count: usize,
    max_tokens: usize,
}

impl Conversation {
    pub fn add_message(&mut self, message: Message);
    pub fn truncate_if_needed(&mut self);
    pub fn to_api_format(&self) -> Vec<ApiMessage>;
    pub fn token_count(&self) -> usize;
}
```

### ToolRegistry

```rust
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn register(&mut self, tool: Box<dyn Tool>);
    pub fn get(&self, name: &str) -> Option<&dyn Tool>;
    pub fn to_api_format(&self) -> Vec<Value>;
    pub fn execute(&self, name: &str, input: Value) -> Result<ToolResult>;
}
```

### App State

```rust
pub struct App {
    pub conversation: Conversation,
    pub config: Settings,
    pub theme: Theme,
    pub input_history: Vec<String>,
    pub current_model: String,
    pub is_generating: bool,
    pub tool_registry: ToolRegistry,
}
```

## Event System

### Event Types

```rust
pub enum AppEvent {
    Input(KeyEvent),
    Resize(u16, u16),
    StreamToken(String),
    ToolCall(ToolUse),
    ToolResult(String, ToolResult),
    Error(String),
    Quit,
}
```

### Event Handling

```rust
impl App {
    pub async fn handle_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Input(key) => self.handle_input(key),
            AppEvent::StreamToken(token) => self.append_token(token),
            AppEvent::ToolCall(tool_use) => self.execute_tool(tool_use).await,
            AppEvent::Error(msg) => self.show_error(msg),
            AppEvent::Quit => self.quit(),
            _ => {}
        }
    }
}
```

## LLM Client API

### Making API Calls

```rust
use crate::llm::client::AnthropicClient;

let client = AnthropicClient::new(api_key);
let messages = conversation.to_api_format();
let tools = tool_registry.to_api_format();

let mut stream = client.stream_messages(
    model,
    messages,
    tools,
    max_tokens,
).await?;

while let Some(event) = stream.next().await {
    match event {
        StreamEvent::TextDelta(text) => {
            // Handle streaming text
        }
        StreamEvent::ToolUse(tool_use) => {
            // Handle tool call
        }
        StreamEvent::Done => break,
        StreamEvent::Error(e) => {
            // Handle error
        }
    }
}
```

## Testing Utilities

### Mock LLM Client

```rust
pub struct MockClient {
    responses: Vec<String>,
    current: usize,
}

impl MockClient {
    pub fn new(responses: Vec<String>) -> Self {
        Self { responses, current: 0 }
    }
}

#[async_trait]
impl LlmClient for MockClient {
    async fn stream_messages(&mut self, ...) -> Result<Stream> {
        // Return mock stream
    }
}
```

### Test Helpers

```rust
pub fn create_test_app() -> App {
    App::new(test_config(), test_theme())
}

pub fn create_test_message(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentBlock::Text { text: text.to_string() }],
        timestamp: Utc::now(),
    }
}
```

## Configuration Schema

### Settings Structure

```rust
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub llm: LlmSettings,
    pub ui: UiSettings,
    pub tools: ToolSettings,
    pub keybindings: Keybindings,
}

#[derive(Serialize, Deserialize)]
pub struct LlmSettings {
    pub api_key: Option<String>,
    pub model: String,
    pub max_tokens: usize,
    pub temperature: f32,
}
```

### Loading Configuration

```rust
use crate::config::Settings;

let settings = Settings::load()?;
// or
let settings = Settings::from_file(path)?;
// or
let settings = Settings::default();
```

## Error Handling

### Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SreeError {
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Tool execution failed: {0}")]
    Tool(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

### Error Propagation

```rust
use anyhow::{Context, Result};

pub fn my_function() -> Result<()> {
    let data = read_file(path)
        .context("Failed to read configuration file")?;
    
    process_data(data)
        .context("Failed to process data")?;
    
    Ok(())
}
```

## Logging

### Log Levels

```rust
use tracing::{debug, info, warn, error};

debug!("Detailed debug information");
info!("General information");
warn!("Warning message");
error!("Error occurred: {}", error);
```

### Structured Logging

```rust
use tracing::instrument;

#[instrument(skip(self))]
async fn execute_tool(&self, name: &str, input: Value) -> Result<ToolResult> {
    info!("Executing tool: {}", name);
    // ...
}
```

## Best Practices

1. **Always use `anyhow::Result` for error handling**
2. **Implement `Drop` for cleanup (terminal restoration)**
3. **Use `async_trait` for async trait methods**
4. **Add tests for all new functionality**
5. **Document public APIs with doc comments**
6. **Use structured logging with `tracing`**
7. **Validate all user input**
8. **Handle cancellation gracefully**
9. **Keep UI responsive (don't block on I/O)**
10. **Follow Rust naming conventions**

## Examples

See the `examples/` directory for complete examples:

- `examples/custom_tool.rs` - Adding a custom tool
- `examples/custom_theme.rs` - Creating a custom theme
- `examples/custom_command.rs` - Adding a slash command
