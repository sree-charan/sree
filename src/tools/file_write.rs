use super::{Tool, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;
use tokio::fs;

pub struct FileWriteTool;

#[derive(Debug, Deserialize, Serialize)]
struct FileWriteInput {
    path: String,
    command: String,
    #[serde(default)]
    file_text: Option<String>,
    #[serde(default)]
    old_str: Option<String>,
    #[serde(default)]
    new_str: Option<String>,
    #[serde(default)]
    insert_line: Option<usize>,
}

#[async_trait]
impl Tool for FileWriteTool {
    fn name(&self) -> &str {
        "file_write"
    }

    fn description(&self) -> &str {
        "Create or modify files. Commands: create, str_replace, insert, append"
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "File path"},
                "command": {"type": "string", "enum": ["create", "str_replace", "insert", "append"]},
                "file_text": {"type": "string", "description": "Content for create command"},
                "old_str": {"type": "string", "description": "String to replace (str_replace)"},
                "new_str": {"type": "string", "description": "Replacement string (str_replace/insert/append)"},
                "insert_line": {"type": "integer", "description": "Line number for insert"}
            },
            "required": ["path", "command"]
        })
    }

    async fn execute(&self, input: Value) -> anyhow::Result<ToolResult> {
        let input: FileWriteInput = serde_json::from_value(input)?;
        let path = Path::new(&input.path);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        match input.command.as_str() {
            "create" => {
                let content = input.file_text.ok_or_else(|| anyhow::anyhow!("file_text required"))?;
                fs::write(path, &content).await?;
                let lines = content.lines().count();
                Ok(ToolResult::success(format!("Created {} ({} lines)", input.path, lines)))
            }
            "str_replace" => {
                let old = input.old_str.ok_or_else(|| anyhow::anyhow!("old_str required"))?;
                let new = input.new_str.ok_or_else(|| anyhow::anyhow!("new_str required"))?;
                let content = fs::read_to_string(path).await?;
                let matches: Vec<_> = content.match_indices(&old).collect();
                if matches.is_empty() {
                    return Ok(ToolResult::error("old_str not found".to_string()));
                }
                if matches.len() > 1 {
                    return Ok(ToolResult::error(format!("old_str found {} times, must be unique", matches.len())));
                }
                let new_content = content.replace(&old, &new);
                fs::write(path, &new_content).await?;
                Ok(ToolResult::success(format!("Replaced in {}", input.path)))
            }
            "insert" => {
                let line_num = input.insert_line.ok_or_else(|| anyhow::anyhow!("insert_line required"))?;
                let new = input.new_str.ok_or_else(|| anyhow::anyhow!("new_str required"))?;
                let content = fs::read_to_string(path).await?;
                let mut lines: Vec<&str> = content.lines().collect();
                if line_num > lines.len() {
                    return Ok(ToolResult::error(format!("Line {} exceeds file length {}", line_num, lines.len())));
                }
                lines.insert(line_num, &new);
                fs::write(path, lines.join("\n") + "\n").await?;
                Ok(ToolResult::success(format!("Inserted at line {} in {}", line_num, input.path)))
            }
            "append" => {
                let new = input.new_str.ok_or_else(|| anyhow::anyhow!("new_str required"))?;
                let mut content = if path.exists() {
                    fs::read_to_string(path).await?
                } else {
                    String::new()
                };
                if !content.is_empty() && !content.ends_with('\n') {
                    content.push('\n');
                }
                content.push_str(&new);
                fs::write(path, &content).await?;
                Ok(ToolResult::success(format!("Appended to {}", input.path)))
            }
            _ => Ok(ToolResult::error(format!("Unknown command: {}", input.command)))
        }
    }
}
