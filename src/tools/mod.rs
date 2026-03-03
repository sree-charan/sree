//! Tool system for executing operations on behalf of the AI assistant.
//!
//! This module provides a flexible tool system that allows the AI to perform
//! various operations like reading files, executing shell commands, and searching
//! code. Tools are registered in a [`ToolRegistry`] and executed through the
//! [`executor`] module.
//!
//! # Architecture
//!
//! - [`Tool`] trait: Interface that all tools must implement
//! - [`ToolRegistry`]: Central registry for all available tools
//! - [`executor`]: Executes tool calls with timeout and error handling
//!
//! # Built-in Tools
//!
//! - [`FileReadTool`]: Read files, list directories, search within files
//! - [`FileWriteTool`]: Create, modify, or append to files
//! - [`BashTool`]: Execute shell commands with timeout
//! - [`GrepTool`]: Search for patterns across files
//! - [`GlobTool`]: Find files matching glob patterns
//! - [`WebSearchTool`]: Web search stub (requires API key)

use async_trait::async_trait;
use serde_json::Value;
use std::fmt;
use std::sync::Arc;

pub mod registry;
pub mod executor;
pub mod file_read;
pub mod file_write;
pub mod bash;
pub mod grep;
pub mod glob;
pub mod web_search;

pub use file_read::FileReadTool;
pub use file_write::FileWriteTool;
pub use bash::BashTool;
pub use grep::GrepTool;
pub use glob::GlobTool;
pub use web_search::WebSearchTool;
pub use registry::ToolRegistry;

/// Result of a tool execution.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ToolResult {
    /// Whether the tool execution succeeded.
    pub success: bool,
    /// The output content from the tool.
    pub content: String,
}

#[allow(dead_code)]
impl ToolResult {
    pub fn success(content: String) -> Self {
        Self { success: true, content }
    }
    
    pub fn error(message: String) -> Self {
        Self { success: false, content: message }
    }
    
    pub fn is_error(&self) -> bool {
        !self.success
    }
}

impl fmt::Display for ToolResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> Value;
    async fn execute(&self, input: Value) -> anyhow::Result<ToolResult>;
}

pub fn create_default_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry.register(Arc::new(FileReadTool));
    registry.register(Arc::new(FileWriteTool));
    registry.register(Arc::new(BashTool));
    registry.register(Arc::new(GrepTool));
    registry.register(Arc::new(GlobTool));
    registry.register(Arc::new(WebSearchTool));
    registry
}
