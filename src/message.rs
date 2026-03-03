use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ToolCallStatus {
    Pending,
    Running,
    Success,
    Error,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ToolCallInfo {
    pub id: String,
    pub name: String,
    pub input: Value,
    pub result: Option<String>,
    pub status: ToolCallStatus,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Message {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub tool_calls: Vec<ToolCallInfo>,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn user(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::User,
            content,
            tool_calls: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn assistant(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::Assistant,
            content,
            tool_calls: Vec::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn system(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::System,
            content,
            tool_calls: Vec::new(),
            timestamp: Utc::now(),
        }
    }
}
