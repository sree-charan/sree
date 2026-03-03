use super::{Tool, ToolResult};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct ToolRegistry {
    tools: Arc<HashMap<String, Arc<dyn Tool>>>,
}

#[allow(dead_code)]
impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(HashMap::new()),
        }
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        Arc::make_mut(&mut self.tools).insert(tool.name().to_string(), tool);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    pub fn tool_schemas(&self) -> Vec<Value> {
        self.tools
            .values()
            .map(|tool| {
                json!({
                    "name": tool.name(),
                    "description": tool.description(),
                    "input_schema": tool.input_schema()
                })
            })
            .collect()
    }
    
    pub fn to_api_tools(&self) -> Vec<Value> {
        self.tool_schemas()
    }

    pub async fn execute(&self, name: &str, input: Value) -> anyhow::Result<ToolResult> {
        tracing::info!("Executing tool: {}", name);
        tracing::debug!("Tool input: {}", serde_json::to_string(&input).unwrap_or_default());
        
        let tool = self
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", name))?;
        
        let result = tool.execute(input).await;
        
        match &result {
            Ok(r) => {
                tracing::info!("Tool {} completed successfully, output length: {}", name, r.content.len());
            }
            Err(e) => {
                tracing::error!("Tool {} failed: {}", name, e);
            }
        }
        
        result
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
