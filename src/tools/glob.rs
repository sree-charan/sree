use super::{Tool, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ignore::WalkBuilder;
use glob::Pattern;

pub struct GlobTool;

#[derive(Debug, Deserialize, Serialize)]
struct GlobInput {
    pattern: String,
    #[serde(default)]
    max_results: Option<usize>,
}

#[async_trait]
impl Tool for GlobTool {
    fn name(&self) -> &str {
        "glob"
    }

    fn description(&self) -> &str {
        "Find files matching glob patterns. Respects .gitignore. Returns file paths sorted by modification time."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": {"type": "string", "description": "Glob pattern (e.g., '**/*.rs', 'src/**/*.toml')"},
                "max_results": {"type": "integer", "description": "Maximum results to return (default: 1000)"}
            },
            "required": ["pattern"]
        })
    }

    async fn execute(&self, input: Value) -> anyhow::Result<ToolResult> {
        let input: GlobInput = serde_json::from_value(input)?;
        let max_results = input.max_results.unwrap_or(1000);
        
        let pattern = Pattern::new(&input.pattern)?;
        
        let mut matches = Vec::new();
        let walker = WalkBuilder::new(".")
            .hidden(false)
            .git_ignore(true)
            .build();
        
        for entry in walker {
            if matches.len() >= max_results {
                break;
            }
            
            let entry = entry?;
            let path = entry.path();
            
            if pattern.matches_path(path) {
                let metadata = std::fs::metadata(path).ok();
                let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                let modified = metadata.and_then(|m| m.modified().ok());
                
                matches.push((path.display().to_string(), modified, size));
            }
        }
        
        // Sort by modification time (most recent first)
        matches.sort_by(|a, b| b.1.cmp(&a.1));
        
        if matches.is_empty() {
            Ok(ToolResult::success("No files matched the pattern.".to_string()))
        } else {
            let output = matches.iter()
                .map(|(path, _, size)| format!("{} ({} bytes)", path, size))
                .collect::<Vec<_>>()
                .join("\n");
            Ok(ToolResult::success(format!("Found {} files:\n{}", matches.len(), output)))
        }
    }
}
