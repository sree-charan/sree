use super::{Tool, ToolResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::process::Command;
use tokio::time::{timeout, Duration};

pub struct BashTool;

#[derive(Debug, Deserialize, Serialize)]
struct BashInput {
    command: String,
    #[serde(default)]
    working_dir: Option<String>,
    #[serde(default = "default_timeout")]
    timeout_secs: u64,
}

fn default_timeout() -> u64 {
    30
}

#[async_trait]
impl Tool for BashTool {
    fn name(&self) -> &str {
        "bash"
    }

    fn description(&self) -> &str {
        "Execute shell commands. Returns stdout, stderr, and exit code."
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {"type": "string", "description": "Shell command to execute"},
                "working_dir": {"type": "string", "description": "Working directory (optional)"},
                "timeout_secs": {"type": "integer", "description": "Timeout in seconds (default 30)"}
            },
            "required": ["command"]
        })
    }

    async fn execute(&self, input: Value) -> anyhow::Result<ToolResult> {
        let input: BashInput = serde_json::from_value(input)?;
        
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(&input.command);
        
        if let Some(dir) = &input.working_dir {
            cmd.current_dir(dir);
        }

        let result = timeout(
            Duration::from_secs(input.timeout_secs),
            cmd.output()
        ).await;

        match result {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let exit_code = output.status.code().unwrap_or(-1);
                
                let mut result_text = format!("Exit code: {}\n", exit_code);
                if !stdout.is_empty() {
                    result_text.push_str(&format!("\nStdout:\n{}", stdout));
                }
                if !stderr.is_empty() {
                    result_text.push_str(&format!("\nStderr:\n{}", stderr));
                }
                
                Ok(ToolResult {
                    success: exit_code == 0,
                    content: result_text,
                })
            }
            Ok(Err(e)) => Ok(ToolResult::error(format!("Failed to execute: {}", e))),
            Err(_) => Ok(ToolResult::error(format!("Command timed out after {} seconds", input.timeout_secs))),
        }
    }
}
