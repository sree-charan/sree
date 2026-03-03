use super::{Tool, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use regex::Regex;
use ignore::WalkBuilder;

pub struct GrepTool;

#[derive(Debug, Deserialize, Serialize)]
struct GrepInput {
    pattern: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    case_sensitive: Option<bool>,
    #[serde(default)]
    max_results: Option<usize>,
}

#[async_trait]
impl Tool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }

    fn description(&self) -> &str {
        "Search for regex patterns in files. Respects .gitignore. Returns file:line:content format."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {"type": "string", "description": "Regex pattern to search"},
                "path": {"type": "string", "description": "Directory or file to search (default: current dir)"},
                "case_sensitive": {"type": "boolean", "description": "Case sensitive search (default: false)"},
                "max_results": {"type": "integer", "description": "Maximum results to return (default: 100)"}
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, input: Value) -> anyhow::Result<ToolResult> {
        let input: GrepInput = serde_json::from_value(input)?;
        let path = input.path.as_deref().unwrap_or(".");
        let case_sensitive = input.case_sensitive.unwrap_or(false);
        let max_results = input.max_results.unwrap_or(100);
        
        let regex = if case_sensitive {
            Regex::new(&input.pattern)?
        } else {
            Regex::new(&format!("(?i){}", input.pattern))?
        };
        
        let mut results = Vec::new();
        let walker = WalkBuilder::new(path)
            .hidden(false)
            .git_ignore(true)
            .build();
        
        for entry in walker {
            if results.len() >= max_results {
                break;
            }
            
            let entry = entry?;
            if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                continue;
            }
            
            let path = entry.path();
            if let Ok(content) = std::fs::read_to_string(path) {
                for (line_num, line) in content.lines().enumerate() {
                    if regex.is_match(line) {
                        results.push(format!("{}:{}:{}", path.display(), line_num + 1, line));
                        if results.len() >= max_results {
                            break;
                        }
                    }
                }
            }
        }
        
        if results.is_empty() {
            Ok(ToolResult::success("No matches found.".to_string()))
        } else {
            let output = format!("Found {} matches:\n{}", results.len(), results.join("\n"));
            Ok(ToolResult::success(output))
        }
    }
}
