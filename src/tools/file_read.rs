use super::{Tool, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;
use tokio::fs;

pub struct FileReadTool;

#[derive(Debug, Deserialize, Serialize)]
struct FileReadInput {
    path: String,
    #[serde(default)]
    start_line: Option<usize>,
    #[serde(default)]
    end_line: Option<isize>,
}

#[async_trait]
impl Tool for FileReadTool {
    fn name(&self) -> &str {
        "file_read"
    }

    fn description(&self) -> &str {
        "Read file contents with optional line range. For directories, lists contents."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to file or directory"
                },
                "start_line": {
                    "type": "integer",
                    "description": "Starting line number (1-indexed, optional)"
                },
                "end_line": {
                    "type": "integer",
                    "description": "Ending line number (optional, negative counts from end)"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: Value) -> anyhow::Result<ToolResult> {
        let input: FileReadInput = serde_json::from_value(input)?;
        let path = Path::new(&input.path);

        if !path.exists() {
            return Ok(ToolResult::error(format!("Path does not exist: {}", input.path)));
        }

        let metadata = fs::metadata(path).await?;

        if metadata.is_dir() {
            let mut entries = fs::read_dir(path).await?;
            let mut items = Vec::new();
            while let Some(entry) = entries.next_entry().await? {
                let name = entry.file_name().to_string_lossy().to_string();
                let meta = entry.metadata().await?;
                let kind = if meta.is_dir() { "dir" } else { "file" };
                items.push(format!("{} ({})", name, kind));
            }
            items.sort();
            return Ok(ToolResult::success(format!(
                "Directory: {}\n\n{}",
                input.path,
                items.join("\n")
            )));
        }

        let content = fs::read_to_string(path).await?;
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let start = input.start_line.unwrap_or(1).saturating_sub(1);
        let end = match input.end_line {
            Some(e) if e < 0 => (total_lines as isize + e + 1).max(0) as usize,
            Some(e) => (e as usize).min(total_lines),
            None => total_lines,
        };

        if start >= total_lines {
            return Ok(ToolResult::error(format!(
                "Start line {} exceeds file length {}",
                start + 1,
                total_lines
            )));
        }

        let selected_lines: Vec<String> = lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{:4} | {}", start + i + 1, line))
            .collect();

        Ok(ToolResult::success(format!(
            "File: {} (lines {}-{} of {})\n\n{}",
            input.path,
            start + 1,
            end,
            total_lines,
            selected_lines.join("\n")
        )))
    }
}
