use sree::llm::messages::{ApiMessage, ApiRequest, ContentBlock};
use sree::llm::models::Model;

#[test]
fn test_api_message_structure() {
    let msg = ApiMessage {
        role: "user".to_string(),
        content: vec![ContentBlock::Text {
            text: "Hello".to_string(),
        }],
    };

    let json = serde_json::to_string(&msg).unwrap();
    assert!(json.contains("\"role\":\"user\""));
    assert!(json.contains("Hello"));
}

#[test]
fn test_api_request_structure() {
    let request = ApiRequest {
        model: Model::ClaudeSonnet4.as_str().to_string(),
        max_tokens: 1024,
        system: Some("Test system prompt".to_string()),
        messages: vec![ApiMessage {
            role: "user".to_string(),
            content: vec![ContentBlock::Text {
                text: "Test".to_string(),
            }],
        }],
        tools: vec![],
        temperature: Some(0.7),
    };

    assert_eq!(request.model, "anthropic.claude-sonnet-4-20250514-v1:0");
    assert_eq!(request.max_tokens, 1024);
}

#[test]
fn test_tool_use_content_block() {
    let block = ContentBlock::ToolUse {
        id: "tool_123".to_string(),
        name: "file_read".to_string(),
        input: serde_json::json!({"path": "/test.txt"}),
    };

    let json = serde_json::to_string(&block).unwrap();
    assert!(json.contains("file_read"));
}

#[test]
fn test_model_properties() {
    assert_eq!(Model::ClaudeSonnet4.as_str(), "anthropic.claude-sonnet-4-20250514-v1:0");
    assert_eq!(Model::ClaudeOpus4.as_str(), "anthropic.claude-opus-4-20250514-v1:0");
    assert_eq!(Model::ClaudeHaiku3.as_str(), "anthropic.claude-haiku-3-20250307-v1:0");
    assert_eq!(Model::ClaudeSonnet4.context_window(), 200_000);
}
