use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tokio::sync::mpsc;
use tui_textarea::TextArea;
use crate::message::{Message, MessageRole, ToolCallInfo, ToolCallStatus};
use crate::llm::client::Client;
use crate::llm::messages::{ApiMessage, ContentBlock};
use crate::context::system::SystemContext;
use crate::tools::{ToolRegistry, create_default_registry};
use crate::agent::{AgentLoop, AgentEvent};
use crate::ui::widgets::spinner::Spinner;
use crate::commands::handlers::{handle_command, CommandResult};
use crate::config::settings::Settings;
use crate::ui::theme::Theme;

enum AppEvent {
    Token(String),
    ToolCallStart { id: String, name: String, input: serde_json::Value },
    ToolCallComplete { id: String, result: String },
    Complete,
    Error(String),
}

pub struct App<'a> {
    should_quit: bool,
    input: TextArea<'a>,
    messages: Vec<Message>,
    scroll_offset: u16,
    auto_scroll: bool,
    llm_client: Option<Client>,
    system_context: SystemContext,
    is_generating: bool,
    current_model: String,
    current_response: String,
    event_rx: mpsc::UnboundedReceiver<AppEvent>,
    event_tx: mpsc::UnboundedSender<AppEvent>,
    #[allow(dead_code)]
    tool_registry: ToolRegistry,
    spinner: Spinner,
    spinner_tick: u8,
    settings: Settings,
    should_compact: bool,
    theme: Theme,
    input_history: Vec<String>,
    history_index: Option<usize>,
    temp_input: String,
    tools_enabled: bool,
}

impl<'a> App<'a> {
    #[allow(dead_code)]
    pub async fn new() -> Self {
        Self::with_cli_overrides(None, None, None, None, false, None).await
    }

    pub async fn with_cli_overrides(
        config_path: Option<String>,
        model_override: Option<String>,
        region_override: Option<String>,
        profile_override: Option<String>,
        no_tools: bool,
        system_prompt: Option<String>,
    ) -> Self {
        let mut input = TextArea::default();
        input.set_placeholder_text("Type your message...");
        
        let settings = if let Some(path) = config_path {
            Settings::load_from_path(&path).unwrap_or_else(|e| {
                tracing::warn!("Failed to load config from {}: {}", path, e);
                Settings::default()
            })
        } else {
            Settings::load().unwrap_or_default()
        };
        
        let mut settings = settings;
        if let Some(model) = model_override {
            tracing::info!("Overriding model from CLI: {}", model);
            settings.llm.model = model;
        }
        if let Some(region) = region_override.clone() {
            tracing::info!("Overriding region from CLI: {}", region);
            settings.llm.aws_region = region;
        }
        
        tracing::info!("Loaded settings: model={}, region={}, max_tokens={}", 
            settings.llm.model, settings.llm.aws_region, settings.llm.max_tokens);
        
        let llm_client = Client::new(&settings.llm.aws_region, profile_override.as_deref()).await.ok();
        
        let welcome_msg = if llm_client.is_some() {
            if no_tools {
                "Welcome to sree! Tools are disabled. Type a message and press Enter."
            } else {
                "Welcome to sree! Type a message and press Enter."
            }
        } else {
            "Welcome to sree! Configure AWS credentials to enable AI features."
        };
        
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let current_model = settings.llm.model.clone();
        let theme = Theme::from_name(&settings.ui.theme);
        
        let mut system_context = SystemContext::new();
        if let Some(prompt) = &system_prompt {
            system_context.set_custom_prompt(prompt.clone());
        }
        
        Self { 
            should_quit: false,
            input,
            messages: vec![Message::system(welcome_msg.to_string())],
            scroll_offset: 0,
            auto_scroll: settings.ui.auto_scroll,
            llm_client,
            system_context,
            is_generating: false,
            current_model,
            current_response: String::new(),
            event_rx,
            event_tx,
            tool_registry: create_default_registry(),
            spinner: Spinner::new(),
            spinner_tick: 0,
            settings,
            should_compact: false,
            theme,
            input_history: Self::load_history(),
            history_index: None,
            temp_input: String::new(),
            tools_enabled: !no_tools,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("Starting app event loop");
        let mut terminal = setup_terminal()?;
        
        while !self.should_quit {
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
            
            // Handle compaction if requested
            if self.should_compact {
                self.compact_conversation();
                self.should_compact = false;
                if self.auto_scroll {
                    self.scroll_offset = u16::MAX;
                }
            }
        }
        
        tracing::info!("App event loop ended");
        self.save_history();
        restore_terminal()?;
        Ok(())
    }

    pub fn shutdown(&mut self) {
        tracing::info!("Shutting down gracefully");
        self.should_quit = true;
        self.save_history();
        let _ = restore_terminal();
    }

    fn render(&self, f: &mut ratatui::Frame) {
        use ratatui::{
            layout::{Constraint, Direction, Layout},
            widgets::{Block, Borders, Paragraph, Wrap},
            style::{Color, Modifier, Style},
            text::{Line, Span},
        };
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Chat area
                Constraint::Length(3),  // Input
                Constraint::Length(1),  // Status bar
            ])
            .split(f.area());

        // Header
        let total_tokens: usize = self.messages.iter()
            .map(|m| crate::llm::token_counter::estimate_message_tokens(&m.content))
            .sum();
        let header_text = format!("🤖 sree v0.1.0  │  {}  │  tokens: {}/200k", self.current_model, total_tokens);
        let header = Paragraph::new(header_text)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(self.theme.header_bg)))
            .style(Style::default().fg(self.theme.input_border).add_modifier(Modifier::BOLD));
        f.render_widget(header, chunks[0]);

        // Chat area - render messages with styling
        let mut lines = Vec::new();
        for (i, msg) in self.messages.iter().enumerate() {
            if i > 0 {
                // Visual separator between messages
                lines.push(Line::from(Span::styled(
                    "─".repeat(80),
                    Style::default().fg(self.theme.separator),
                )));
                lines.push(Line::from(""));
            }
            
            let (prefix, style) = match msg.role {
                MessageRole::User => ("You: ", self.theme.user_message),
                MessageRole::Assistant => ("sree: ", self.theme.assistant_message),
                MessageRole::System => ("ℹ ", self.theme.system_message),
            };
            
            // Render message content with markdown for assistant messages
            if msg.role == MessageRole::Assistant && !msg.content.is_empty() {
                lines.push(Line::from(Span::styled(prefix, style)));
                let markdown_lines = crate::ui::markdown::render_markdown(&msg.content);
                lines.extend(markdown_lines);
            } else {
                lines.push(Line::from(vec![
                    Span::styled(prefix, style),
                    Span::raw(&msg.content),
                ]));
            }
            
            // Render tool calls in bordered boxes
            for tool_call in &msg.tool_calls {
                lines.push(Line::from(""));
                let (status_icon, status_color) = match tool_call.status {
                    ToolCallStatus::Pending => ("⏳", self.theme.tool_pending),
                    ToolCallStatus::Running => ("⚙", self.theme.tool_running),
                    ToolCallStatus::Success => ("✓", self.theme.tool_success),
                    ToolCallStatus::Error => ("✗", self.theme.tool_error),
                };
                
                // Top border
                lines.push(Line::from(Span::styled(
                    format!("  ╭─ {} Tool Call ─────────────────────", status_icon),
                    Style::default().fg(status_color),
                )));
                
                lines.push(Line::from(vec![
                    Span::styled("  │ ", Style::default().fg(status_color)),
                    Span::styled("Name: ", Style::default().fg(self.theme.assistant_message.fg.unwrap_or(Color::White))),
                    Span::styled(&tool_call.name, Style::default().fg(self.theme.input_border).add_modifier(Modifier::BOLD)),
                ]));
                
                // Show input parameters
                if !tool_call.input.is_null() {
                    let input_str = tool_call.input.to_string();
                    let input_display = if input_str.len() > 60 {
                        format!("{}...", &input_str[..57])
                    } else {
                        input_str
                    };
                    lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(status_color)),
                        Span::styled("Input: ", Style::default().fg(self.theme.assistant_message.fg.unwrap_or(Color::White))),
                        Span::styled(input_display, Style::default().fg(self.theme.separator)),
                    ]));
                }
                
                lines.push(Line::from(vec![
                    Span::styled("  │ ", Style::default().fg(status_color)),
                    Span::styled("Status: ", Style::default().fg(self.theme.assistant_message.fg.unwrap_or(Color::White))),
                    Span::styled(
                        match tool_call.status {
                            ToolCallStatus::Pending => "Pending",
                            ToolCallStatus::Running => "Running",
                            ToolCallStatus::Success => "Success",
                            ToolCallStatus::Error => "Error",
                        },
                        Style::default().fg(status_color),
                    ),
                ]));
                
                if let Some(result) = &tool_call.result {
                    lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(status_color)),
                        Span::styled("Result: ", Style::default().fg(self.theme.assistant_message.fg.unwrap_or(Color::White))),
                    ]));
                    
                    let result_lines: Vec<&str> = result.lines().collect();
                    let num_lines = result_lines.len();
                    let display_count = num_lines.min(3);
                    
                    for line in result_lines.iter().take(display_count) {
                        lines.push(Line::from(vec![
                            Span::styled("  │   ", Style::default().fg(status_color)),
                            Span::styled(line.to_string(), Style::default().fg(self.theme.separator)),
                        ]));
                    }
                    
                    if num_lines > 3 {
                        lines.push(Line::from(vec![
                            Span::styled("  │   ", Style::default().fg(status_color)),
                            Span::styled(
                                format!("... ({} more lines)", num_lines - 3),
                                Style::default().fg(self.theme.separator).add_modifier(Modifier::ITALIC),
                            ),
                        ]));
                    }
                }
                
                // Bottom border
                lines.push(Line::from(Span::styled(
                    "  ╰────────────────────────────────────",
                    Style::default().fg(status_color),
                )));
            }
        }
        
        // Add current streaming response or thinking indicator
        if self.is_generating {
            lines.push(Line::from(""));
            if self.current_response.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("sree: ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                ]));
                lines.push(self.spinner.render());
            } else {
                lines.push(Line::from(vec![
                    Span::styled("sree: ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                ]));
                let markdown_lines = crate::ui::markdown::render_markdown(&self.current_response);
                lines.extend(markdown_lines);
                lines.push(Line::from(Span::styled("▊", Style::default().fg(self.theme.input_border))));  // Cursor
            }
        }
        
        let total_lines = lines.len() as u16;
        let chat_height = chunks[1].height.saturating_sub(2); // minus borders
        let max_scroll = total_lines.saturating_sub(chat_height);
        let effective_scroll = if self.auto_scroll {
            max_scroll
        } else {
            self.scroll_offset.min(max_scroll)
        };
        let chat = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Chat").border_style(Style::default().fg(self.theme.input_border)))
            .wrap(Wrap { trim: false })
            .scroll((effective_scroll, 0));
        f.render_widget(chat, chunks[1]);

        // Input area
        f.render_widget(&self.input, chunks[2]);

        // Status bar
        let scroll_indicator = if self.auto_scroll { "🔽 auto" } else { "📜 manual" };
        let status = Paragraph::new(format!(
            "[INSERT] │ /help for commands │ Ctrl+C to quit │ Mouse/PageUp/Down to scroll │ {}",
            scroll_indicator
        ))
        .style(Style::default().fg(self.theme.separator));
        f.render_widget(status, chunks[3]);
    }

    fn handle_events(&mut self) -> Result<()> {
        // Tick spinner for animation
        if self.is_generating {
            self.spinner_tick = self.spinner_tick.wrapping_add(1);
            if self.spinner_tick.is_multiple_of(5) {  // Update every 5 frames
                self.spinner.tick();
            }
        }
        
        // Check for app events (streaming tokens, tool calls)
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                AppEvent::Token(token) => {
                    self.current_response.push_str(&token);
                }
                AppEvent::ToolCallStart { id, name, input } => {
                    // Add tool call to the last assistant message or create new one
                    if let Some(last_msg) = self.messages.last_mut() {
                        if matches!(last_msg.role, MessageRole::Assistant) {
                            last_msg.tool_calls.push(ToolCallInfo {
                                id,
                                name,
                                input,
                                result: None,
                                status: ToolCallStatus::Running,
                            });
                        }
                    }
                }
                AppEvent::ToolCallComplete { id, result } => {
                    // Update tool call status
                    if let Some(last_msg) = self.messages.last_mut() {
                        if let Some(tool_call) = last_msg.tool_calls.iter_mut().find(|tc| tc.id == id) {
                            tool_call.result = Some(result.clone());
                            tool_call.status = if result.starts_with("Error:") {
                                ToolCallStatus::Error
                            } else {
                                ToolCallStatus::Success
                            };
                        }
                    }
                }
                AppEvent::Complete => {
                    if !self.current_response.is_empty() {
                        self.messages.push(Message::assistant(self.current_response.clone()));
                        self.current_response.clear();
                    }
                    self.is_generating = false;
                    if self.auto_scroll {
                        self.scroll_offset = u16::MAX;
                    }
                }
                AppEvent::Error(err) => {
                    self.messages.push(Message::system(format!("Error: {}", err)));
                    self.current_response.clear();
                    self.is_generating = false;
                    if self.auto_scroll {
                        self.scroll_offset = u16::MAX;
                    }
                }
            }
        }
        
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    // Ctrl+C to quit or cancel generation
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                        if self.is_generating {
                            self.is_generating = false;
                            self.current_response.clear();
                            self.messages.push(Message::system("Generation cancelled.".to_string()));
                            if self.auto_scroll {
                                self.scroll_offset = u16::MAX;
                            }
                        } else {
                            self.should_quit = true;
                        }
                        return Ok(());
                    }
                    
                    // Ctrl+L to clear
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('l') {
                        self.messages.clear();
                        self.messages.push(Message::system("Chat cleared.".to_string()));
                        self.scroll_offset = 0;
                        self.auto_scroll = true;
                        return Ok(());
                    }
                    
                    // PageUp/PageDown for scrolling
                    match key.code {
                        KeyCode::PageUp => {
                            self.scroll_offset = self.scroll_offset.saturating_sub(5);
                            self.auto_scroll = false;
                            return Ok(());
                        }
                        KeyCode::PageDown => {
                            self.scroll_offset = self.scroll_offset.saturating_add(5);
                            self.auto_scroll = false;
                            return Ok(());
                        }
                        KeyCode::Home => {
                            self.scroll_offset = 0;
                            self.auto_scroll = false;
                            return Ok(());
                        }
                        KeyCode::End => {
                            self.scroll_offset = u16::MAX;
                            self.auto_scroll = true;
                            return Ok(());
                        }
                        _ => {}
                    }
                    
                    // Up/Down arrow for history navigation (only when input is single-line or cursor on first/last line)
                    if key.code == KeyCode::Up && !self.is_generating {
                        self.navigate_history_up();
                        return Ok(());
                    }
                    
                    if key.code == KeyCode::Down && !self.is_generating {
                        self.navigate_history_down();
                        return Ok(());
                    }
                    
                    // Enter to send message
                    if key.code == KeyCode::Enter && !key.modifiers.contains(KeyModifiers::SHIFT) {
                        let text = self.input.lines().join("\n");
                        if !text.trim().is_empty() && !self.is_generating {
                            self.add_to_history(text.clone());
                            
                            if text.trim().starts_with('/') {
                                tracing::debug!("Handling command: {}", text.trim());
                                self.handle_command(text.trim());
                            } else {
                                tracing::info!("User message: {} chars", text.len());
                                self.messages.push(Message::user(text.clone()));
                                if self.auto_scroll {
                                    self.scroll_offset = u16::MAX;
                                }
                                
                                if let Some(client) = &self.llm_client {
                                    self.is_generating = true;
                                    self.send_to_llm(client.clone(), text);
                                } else {
                                    self.messages.push(Message::system("No API key configured. Set ANTHROPIC_API_KEY to use AI features.".to_string()));
                                }
                            }
                            self.input = TextArea::default();
                            self.input.set_placeholder_text("Type your message...");
                        }
                        return Ok(());
                    }
                    
                    // Pass other keys to textarea
                    self.input.input(key);
                }
                Event::Mouse(mouse) => {
                    use crossterm::event::MouseEventKind;
                    match mouse.kind {
                        MouseEventKind::ScrollUp => {
                            self.scroll_offset = self.scroll_offset.saturating_sub(3);
                            self.auto_scroll = false;
                        }
                        MouseEventKind::ScrollDown => {
                            self.scroll_offset = self.scroll_offset.saturating_add(3);
                            // Don't disable auto-scroll when scrolling down near the bottom
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    fn send_to_llm(&self, client: Client, _user_message: String) {
        tracing::debug!("Starting LLM request with model: {}", self.current_model);
        let tx = self.event_tx.clone();
        let system_prompt = self.system_context.generate_system_prompt();
        let model = self.current_model.clone();
        let registry = self.tool_registry.clone();
        
        // Convert UI messages to API messages
        let mut api_messages = Vec::new();
        for msg in &self.messages {
            match msg.role {
                MessageRole::User => {
                    api_messages.push(ApiMessage {
                        role: "user".to_string(),
                        content: vec![ContentBlock::Text { text: msg.content.clone() }],
                    });
                }
                MessageRole::Assistant => {
                    let mut content = Vec::new();
                    if !msg.content.is_empty() {
                        content.push(ContentBlock::Text { text: msg.content.clone() });
                    }
                    api_messages.push(ApiMessage {
                        role: "assistant".to_string(),
                        content,
                    });
                }
                MessageRole::System => {
                    // Skip system messages in API
                }
            }
        }
        
        tracing::debug!("Converted {} UI messages to {} API messages", self.messages.len(), api_messages.len());
        
        let max_tokens = self.settings.llm.max_tokens;
        let temperature = self.settings.llm.temperature;
        let tools_enabled = self.tools_enabled;
        
        tokio::spawn(async move {
            let agent_loop = AgentLoop::new(client, registry);
            
            let config = crate::agent::AgentConfig {
                system_prompt,
                model,
                max_tokens,
                temperature,
                tools_enabled,
            };
            
            let callback = |event: AgentEvent| {
                match event {
                    AgentEvent::TextToken(token) => {
                        let _ = tx.send(AppEvent::Token(token));
                    }
                    AgentEvent::ToolCallStart(tool_call) => {
                        tracing::info!("Tool call started: {}", tool_call.name);
                        let _ = tx.send(AppEvent::ToolCallStart {
                            id: tool_call.id,
                            name: tool_call.name,
                            input: tool_call.input,
                        });
                    }
                    AgentEvent::ToolCallComplete(id, result) => {
                        tracing::info!("Tool call completed: {}", id);
                        let _ = tx.send(AppEvent::ToolCallComplete { id, result });
                    }
                    AgentEvent::Complete(_) => {
                        tracing::debug!("Agent loop completed");
                        let _ = tx.send(AppEvent::Complete);
                    }
                    AgentEvent::Error(err) => {
                        tracing::error!("Agent error: {}", err);
                        let _ = tx.send(AppEvent::Error(err));
                    }
                    _ => {}
                }
            };
            
            match agent_loop.run(config, api_messages, callback).await {
                Ok(_) => {
                    tracing::debug!("Agent loop finished successfully");
                }
                Err(e) => {
                    tracing::error!("Agent loop error: {}", e);
                    let _ = tx.send(AppEvent::Error(e.to_string()));
                }
            }
        });
    }
    
    fn handle_command(&mut self, cmd: &str) {
        tracing::debug!("Processing command: {}", cmd);
        if let Some(result) = handle_command(cmd, &self.messages, &self.current_model) {
            match result {
                CommandResult::Message(msg) => {
                    self.messages.push(Message::system(msg));
                    if self.auto_scroll {
                        self.scroll_offset = u16::MAX;
                    }
                }
                CommandResult::Quit => {
                    self.should_quit = true;
                }
                CommandResult::Clear => {
                    self.messages.clear();
                    self.messages.push(Message::system("Chat cleared.".to_string()));
                    self.scroll_offset = 0;
                    self.auto_scroll = true;
                }
                CommandResult::ModelSwitch(model) => {
                    self.current_model = model.clone();
                    self.messages.push(Message::system(format!("Switched to model: {}", model)));
                    if self.auto_scroll {
                        self.scroll_offset = u16::MAX;
                    }
                }
                CommandResult::Export(filename) => {
                    match self.export_conversation(&filename) {
                        Ok(_) => {
                            self.messages.push(Message::system(format!("Conversation exported to: {}", filename)));
                            tracing::info!("Exported conversation to {}", filename);
                        }
                        Err(e) => {
                            self.messages.push(Message::system(format!("Failed to export: {}", e)));
                            tracing::error!("Export failed: {}", e);
                        }
                    }
                    if self.auto_scroll {
                        self.scroll_offset = u16::MAX;
                    }
                }
                CommandResult::SystemPrompt(prompt) => {
                    self.system_context.set_custom_prompt(prompt.clone());
                    self.messages.push(Message::system(format!("Custom system prompt set: {}", prompt)));
                    tracing::info!("System prompt updated");
                    if self.auto_scroll {
                        self.scroll_offset = u16::MAX;
                    }
                }
                CommandResult::ShowConfig => {
                    let config_text = self.format_config();
                    self.messages.push(Message::system(config_text));
                    if self.auto_scroll {
                        self.scroll_offset = u16::MAX;
                    }
                }
                CommandResult::History => {
                    let history_text = crate::commands::handlers::history_info(&self.messages, &self.current_model);
                    self.messages.push(Message::system(history_text));
                    if self.auto_scroll {
                        self.scroll_offset = u16::MAX;
                    }
                }
                CommandResult::Compact => {
                    self.messages.push(Message::system("Compacting conversation... This will summarize older messages to save tokens.".to_string()));
                    if self.auto_scroll {
                        self.scroll_offset = u16::MAX;
                    }
                    // Trigger compaction in the next event loop
                    self.should_compact = true;
                }
                CommandResult::ThemeSwitch(theme_name) => {
                    self.theme = Theme::from_name(&theme_name);
                    self.settings.ui.theme = theme_name.clone();
                    
                    // Save theme to config file
                    if let Err(e) = self.settings.save() {
                        tracing::error!("Failed to save theme to config: {}", e);
                        self.messages.push(Message::system(format!("Switched to {} theme (not persisted: {})", theme_name, e)));
                    } else {
                        self.messages.push(Message::system(format!("Switched to {} theme (saved to config)", theme_name)));
                        tracing::info!("Theme switched to: {} and saved to config", theme_name);
                    }
                    
                    if self.auto_scroll {
                        self.scroll_offset = u16::MAX;
                    }
                }
            }
        }
    }
    
    fn export_conversation(&self, filename: &str) -> Result<()> {
        let mut markdown = String::from("# Conversation Export\n\n");
        markdown.push_str(&format!("Exported: {}\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
        markdown.push_str("---\n\n");

        for msg in &self.messages {
            let role = match msg.role {
                MessageRole::User => "**You**",
                MessageRole::Assistant => "**sree**",
                MessageRole::System => "*System*",
            };

            markdown.push_str(&format!("{}\n\n{}\n\n---\n\n", role, msg.content));
        }

        std::fs::write(filename, markdown)?;
        Ok(())
    }
    
    fn format_config(&self) -> String {
        let client_status = if self.llm_client.is_some() {
            "✓ Connected"
        } else {
            "✗ Not connected (check AWS credentials)"
        };
        
        format!(
            r#"Current Configuration:

[LLM]
  AWS Client: {}
  Region: {}
  Model: {}
  Max Tokens: {}
  Temperature: {}

[UI]
  Theme: {}
  Show Token Count: {}
  Show Elapsed Time: {}
  Auto Scroll: {}
  Word Wrap: {}
  Code Theme: {}

[Tools]
  Bash Timeout: {}s
  Max File Size: {} bytes
  Max Output Lines: {}

Config file: ~/.sree/config.toml"#,
            client_status,
            self.settings.llm.aws_region,
            self.settings.llm.model,
            self.settings.llm.max_tokens,
            self.settings.llm.temperature,
            self.settings.ui.theme,
            self.settings.ui.show_token_count,
            self.settings.ui.show_elapsed_time,
            self.settings.ui.auto_scroll,
            self.settings.ui.word_wrap,
            self.settings.ui.code_theme,
            self.settings.tools.bash_timeout,
            self.settings.tools.max_file_size,
            self.settings.tools.max_output_lines,
        )
    }
    
    fn compact_conversation(&mut self) {
        // Keep system messages and the last 5 messages, summarize the rest
        if self.messages.len() <= 6 {
            self.messages.push(Message::system("Conversation is already compact (6 or fewer messages).".to_string()));
            return;
        }
        
        let mut system_messages = Vec::new();
        let mut recent_messages = Vec::new();
        let mut to_summarize = Vec::new();
        
        // Separate system messages, recent messages, and messages to summarize
        for (i, msg) in self.messages.iter().enumerate() {
            if msg.role == MessageRole::System {
                system_messages.push(msg.clone());
            } else if i >= self.messages.len() - 5 {
                recent_messages.push(msg.clone());
            } else {
                to_summarize.push(msg.clone());
            }
        }
        
        if to_summarize.is_empty() {
            self.messages.push(Message::system("No messages to compact.".to_string()));
            return;
        }
        
        // Create a summary
        let tokens_saved: usize = to_summarize.iter()
            .map(|m| crate::llm::token_counter::estimate_message_tokens(&m.content))
            .sum();
        let summary = format!(
            "[Conversation Summary: {} messages compacted. Topics discussed: various user queries and assistant responses. This summary saves approximately {} tokens.]",
            to_summarize.len(),
            tokens_saved
        );
        
        // Rebuild messages: system messages + summary + recent messages
        let mut new_messages = system_messages;
        new_messages.push(Message::system(summary));
        new_messages.extend(recent_messages);
        
        let old_count = self.messages.len();
        self.messages = new_messages;
        let new_count = self.messages.len();
        
        self.messages.push(Message::system(format!(
            "Conversation compacted: {} messages → {} messages",
            old_count, new_count
        )));
        
        tracing::info!("Compacted conversation: {} -> {} messages", old_count, new_count);
    }

    fn load_history() -> Vec<String> {
        let history_path = dirs::home_dir()
            .map(|h| h.join(".sree").join("history"))
            .unwrap_or_default();
        
        if let Ok(content) = std::fs::read_to_string(&history_path) {
            content.lines()
                .map(|s| s.to_string())
                .collect()
        } else {
            Vec::new()
        }
    }

    fn save_history(&self) {
        let history_path = dirs::home_dir()
            .map(|h| h.join(".sree").join("history"))
            .unwrap_or_default();
        
        if let Some(parent) = history_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        
        let content = self.input_history.join("\n");
        let _ = std::fs::write(&history_path, content);
    }

    fn add_to_history(&mut self, input: String) {
        if !input.trim().is_empty() && self.input_history.last() != Some(&input) {
            self.input_history.push(input);
            if self.input_history.len() > 100 {
                self.input_history.remove(0);
            }
        }
        self.history_index = None;
        self.temp_input.clear();
    }

    fn navigate_history_up(&mut self) {
        if self.input_history.is_empty() {
            return;
        }

        let current_input = self.input.lines().join("\n");
        
        match self.history_index {
            None => {
                self.temp_input = current_input;
                self.history_index = Some(self.input_history.len() - 1);
            }
            Some(idx) if idx > 0 => {
                self.history_index = Some(idx - 1);
            }
            _ => return,
        }

        if let Some(idx) = self.history_index {
            let history_text = self.input_history[idx].clone();
            self.input = TextArea::from(history_text.lines().map(|s| s.to_string()).collect::<Vec<_>>());
        }
    }

    fn navigate_history_down(&mut self) {
        if self.history_index.is_none() {
            return;
        }

        match self.history_index {
            Some(idx) if idx < self.input_history.len() - 1 => {
                self.history_index = Some(idx + 1);
                let history_text = self.input_history[self.history_index.unwrap()].clone();
                self.input = TextArea::from(history_text.lines().map(|s| s.to_string()).collect::<Vec<_>>());
            }
            _ => {
                self.history_index = None;
                let temp = self.temp_input.clone();
                self.input = TextArea::from(temp.lines().map(|s| s.to_string()).collect::<Vec<_>>());
            }
        }
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    Ok(())
}
