use super::{Tool, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub struct WebSearchTool;

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
struct WebSearchInput {
    query: String,
}

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str {
        "web_search"
    }

    fn description(&self) -> &str {
        "Search the web for information (requires API key configuration)"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {"type": "string", "description": "Search query"}
            },
            "required": ["query"]
        })
    }

    async fn execute(&self, _input: Value) -> anyhow::Result<ToolResult> {
        Ok(ToolResult::error(
            "Web search requires API key configuration. Please configure a search API key in ~/.sree/config.toml".to_string()
        ))
    }
}
