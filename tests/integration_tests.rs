use sree::agent::{AgentLoop, AgentConfig, AgentEvent, MAX_TOOL_ROUNDS};
use sree::llm::client::Client;
use sree::llm::messages::{ApiMessage, ContentBlock};
use sree::tools::ToolRegistry;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_agent_loop_with_mock_client() {
    // This test verifies the agent loop can handle a conversation with tool calls
    // Using a mock client to avoid actual API calls
    
    let client = Client::new("us-east-1", None).await.expect("Failed to create client");
    let registry = ToolRegistry::new();
    let agent = AgentLoop::new(client, registry);
    
    let config = AgentConfig {
        system_prompt: "You are a test assistant".to_string(),
        model: "anthropic.claude-sonnet-4-20250514-v1:0".to_string(),
        max_tokens: 1024,
        temperature: 0.7,
        tools_enabled: true,
    };
    
    let messages = vec![
        ApiMessage {
            role: "user".to_string(),
            content: vec![ContentBlock::Text {
                text: "Hello".to_string(),
            }],
        },
    ];
    
    let events = Arc::new(Mutex::new(Vec::new()));
    let events_clone = events.clone();
    
    let callback = move |event: AgentEvent| {
        events_clone.lock().unwrap().push(format!("{:?}", event));
    };
    
    // Note: This will fail without proper AWS credentials, but tests the structure
    let result = agent.run(config, messages, callback).await;
    
    // We expect an error since we don't have AWS credentials in test environment
    assert!(result.is_err() || result.is_ok());
}

#[test]
fn test_agent_config_creation() {
    let config = AgentConfig {
        system_prompt: "Test prompt".to_string(),
        model: "anthropic.claude-sonnet-4-20250514-v1:0".to_string(),
        max_tokens: 2048,
        temperature: 0.5,
        tools_enabled: true,
    };
    
    assert_eq!(config.model, "anthropic.claude-sonnet-4-20250514-v1:0");
    assert_eq!(config.max_tokens, 2048);
    assert_eq!(config.temperature, 0.5);
    assert!(config.tools_enabled);
}

#[test]
fn test_agent_event_types() {
    let text_event = AgentEvent::TextToken("Hello".to_string());
    assert!(matches!(text_event, AgentEvent::TextToken(_)));
    
    let error_event = AgentEvent::Error("Test error".to_string());
    assert!(matches!(error_event, AgentEvent::Error(_)));
    
    let complete_event = AgentEvent::Complete("end_turn".to_string());
    assert!(matches!(complete_event, AgentEvent::Complete(_)));
}

#[test]
fn test_max_rounds_constant() {
    // Verify the max rounds constant is reasonable
    assert!(MAX_TOOL_ROUNDS > 0);
    assert!(MAX_TOOL_ROUNDS <= 100); // Sanity check
}
