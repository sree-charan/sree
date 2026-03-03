use anyhow::Result;
use serde_json::Value;
use crate::llm::client::Client;
use crate::llm::messages::{ApiMessage, ApiRequest, ContentBlock, ToolSpec};
use crate::llm::streaming::StreamEvent;
use crate::tools::ToolRegistry;

#[allow(dead_code)]
pub const MAX_TOOL_ROUNDS: usize = 25;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub input: Value,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AgentEvent {
    TextToken(String),
    ToolCallStart(ToolCall),
    ToolCallInput(String),
    ToolCallComplete(String, String), // id, result
    Complete(String), // stop_reason
    Error(String),
}

#[allow(dead_code)]
pub struct AgentConfig {
    pub system_prompt: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub tools_enabled: bool,
}

#[allow(dead_code)]
pub struct AgentLoop {
    client: Client,
    registry: ToolRegistry,
    max_rounds: usize,
}

#[allow(dead_code)]
impl AgentLoop {
    pub fn new(client: Client, registry: ToolRegistry) -> Self {
        Self {
            client,
            registry,
            max_rounds: MAX_TOOL_ROUNDS,
        }
    }
    
    /// Run the agentic loop: send messages, handle tool calls, repeat until completion.
    ///
    /// This is the core of the agent's behavior. It:
    /// 1. Sends messages to the LLM with tool definitions
    /// 2. Streams back the response token by token
    /// 3. If the LLM requests tool calls, executes them
    /// 4. Feeds tool results back to the LLM
    /// 5. Repeats until the LLM returns a final text response
    ///
    /// The loop is bounded by `max_rounds` to prevent infinite loops.
    pub async fn run(
        &self,
        config: AgentConfig,
        messages: Vec<ApiMessage>,
        callback: impl Fn(AgentEvent) + Send + Sync,
    ) -> Result<Vec<ApiMessage>> {
        let mut current_messages = messages;
        let mut rounds = 0;
        
        loop {
            // Safety check: prevent infinite tool call loops
            if rounds >= self.max_rounds {
                callback(AgentEvent::Error("Max tool rounds exceeded".to_string()));
                break;
            }
            rounds += 1;
            
            // Prepare tools array if enabled
            let tools = if config.tools_enabled {
                self.registry.to_api_tools().into_iter().map(|t| {
                    ToolSpec {
                        name: t["name"].as_str().unwrap_or("").to_string(),
                        description: t["description"].as_str().unwrap_or("").to_string(),
                        input_schema: t["input_schema"].clone(),
                    }
                }).collect()
            } else {
                vec![]
            };
            
            // Build API request with current conversation state
            let request = ApiRequest {
                model: config.model.clone(),
                max_tokens: config.max_tokens,
                system: Some(config.system_prompt.clone()),
                messages: current_messages.clone(),
                tools,
                temperature: Some(config.temperature),
            };
            
            // Stream the response from the LLM
            let mut rx = self.client.stream_messages(request).await?;
            
            // Accumulate response content as it streams in
            let mut text_content = String::new();
            let mut tool_calls: Vec<(String, String, String)> = Vec::new(); // id, name, input_json
            let mut current_tool: Option<(String, String, String)> = None; // Currently streaming tool call
            let mut stop_reason = None;
            
            // Process each event from the stream
            while let Some(result) = rx.recv().await {
                match result {
                    Ok(event) => match event {
                        StreamEvent::ContentBlockStart { block_type, tool_use_id, tool_name, .. } => {
                            if block_type == "tool_use" {
                                if let (Some(id), Some(name)) = (tool_use_id, tool_name) {
                                    current_tool = Some((id, name, String::new()));
                                }
                            }
                        }
                        StreamEvent::TextDelta { text, .. } => {
                            text_content.push_str(&text);
                            callback(AgentEvent::TextToken(text));
                        }
                        StreamEvent::ToolInputDelta { json, .. } => {
                            if let Some((_, _, ref mut input)) = current_tool {
                                input.push_str(&json);
                                callback(AgentEvent::ToolCallInput(json));
                            }
                        }
                        StreamEvent::ContentBlockStop => {
                            if let Some((id, name, input)) = current_tool.take() {
                                let input_value: Value = serde_json::from_str(&input).unwrap_or(Value::Null);
                                callback(AgentEvent::ToolCallStart(ToolCall {
                                    id: id.clone(),
                                    name: name.clone(),
                                    input: input_value,
                                }));
                                tool_calls.push((id, name, input));
                            }
                        }
                        StreamEvent::MessageStop { stop_reason: sr } => {
                            stop_reason = sr;
                            break;
                        }
                        _ => {}
                    },
                    Err(e) => {
                        callback(AgentEvent::Error(e.to_string()));
                        return Err(e);
                    }
                }
            }
            
            // Build assistant message with text and tool calls
            let mut content_blocks = Vec::new();
            if !text_content.is_empty() {
                content_blocks.push(ContentBlock::Text { text: text_content });
            }
            for (id, name, input_json) in &tool_calls {
                let input: Value = serde_json::from_str(input_json)?;
                content_blocks.push(ContentBlock::ToolUse {
                    id: id.clone(),
                    name: name.clone(),
                    input,
                });
            }
            
            current_messages.push(ApiMessage {
                role: "assistant".to_string(),
                content: content_blocks,
            });
            
            // If no tool calls, we're done
            if tool_calls.is_empty() {
                callback(AgentEvent::Complete(stop_reason.unwrap_or_default()));
                break;
            }
            
            // Execute tool calls
            let mut tool_results = Vec::new();
            for (id, name, input_json) in tool_calls {
                let input: Value = serde_json::from_str(&input_json)?;
                match self.registry.execute(&name, input).await {
                    Ok(result) => {
                        let content = if result.is_error() {
                            format!("Error: {}", result.content)
                        } else {
                            result.content.clone()
                        };
                        callback(AgentEvent::ToolCallComplete(id.clone(), content.clone()));
                        tool_results.push(ContentBlock::ToolResult {
                            tool_use_id: id,
                            content,
                        });
                    }
                    Err(e) => {
                        let error_msg = format!("Tool execution failed: {}", e);
                        callback(AgentEvent::ToolCallComplete(id.clone(), error_msg.clone()));
                        tool_results.push(ContentBlock::ToolResult {
                            tool_use_id: id,
                            content: error_msg,
                        });
                    }
                }
            }
            
            current_messages.push(ApiMessage {
                role: "user".to_string(),
                content: tool_results,
            });
        }
        
        Ok(current_messages)
    }
}
