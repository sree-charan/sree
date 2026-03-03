use sree::message::{Message, MessageRole, ToolCallInfo, ToolCallStatus};
use serde_json::json;

#[test]
fn test_message_with_tool_calls() {
    let mut msg = Message::assistant("Here's the file content:".to_string());
    
    msg.tool_calls.push(ToolCallInfo {
        id: "call_123".to_string(),
        name: "file_read".to_string(),
        input: json!({"path": "test.txt"}),
        result: Some("File content here".to_string()),
        status: ToolCallStatus::Success,
    });
    
    assert_eq!(msg.tool_calls.len(), 1);
    assert_eq!(msg.tool_calls[0].name, "file_read");
    assert!(matches!(msg.tool_calls[0].status, ToolCallStatus::Success));
}

#[test]
fn test_tool_call_status_transitions() {
    let mut tool_call = ToolCallInfo {
        id: "call_456".to_string(),
        name: "bash".to_string(),
        input: json!({"command": "ls"}),
        result: None,
        status: ToolCallStatus::Running,
    };
    
    assert!(matches!(tool_call.status, ToolCallStatus::Running));
    
    tool_call.status = ToolCallStatus::Success;
    tool_call.result = Some("file1.txt\nfile2.txt".to_string());
    
    assert!(matches!(tool_call.status, ToolCallStatus::Success));
    assert!(tool_call.result.is_some());
}

#[test]
fn test_message_roles() {
    let user_msg = Message::user("Hello".to_string());
    let assistant_msg = Message::assistant("Hi there".to_string());
    let system_msg = Message::system("System message".to_string());
    
    assert!(matches!(user_msg.role, MessageRole::User));
    assert!(matches!(assistant_msg.role, MessageRole::Assistant));
    assert!(matches!(system_msg.role, MessageRole::System));
}
