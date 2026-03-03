use chrono::Local;
use std::env;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SystemContext {
    pub os: String,
    pub arch: String,
    pub shell: String,
    pub cwd: String,
    pub time: String,
    pub custom_prompt: Option<String>,
}

#[allow(dead_code)]
impl SystemContext {
    pub fn new() -> Self {
        let os = env::consts::OS.to_string();
        let arch = env::consts::ARCH.to_string();
        let shell = env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
        let cwd = env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        let time = Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();

        Self {
            os,
            arch,
            shell,
            cwd,
            time,
            custom_prompt: None,
        }
    }
    
    pub fn set_custom_prompt(&mut self, prompt: String) {
        self.custom_prompt = Some(prompt);
    }

    pub fn generate_system_prompt(&self) -> String {
        if let Some(custom) = &self.custom_prompt {
            return format!(
                r#"{}

# System Information
- Operating System: {} ({})
- Shell: {}
- Current Directory: {}
- Current Time: {}"#,
                custom, self.os, self.arch, self.shell, self.cwd, self.time
            );
        }
        
        format!(
            r#"You are sree, an AI assistant running in the user's terminal.

# System Information
- Operating System: {} ({})
- Shell: {}
- Current Directory: {}
- Current Time: {}

# Your Capabilities
You have access to tools that allow you to:
- Read and write files
- Execute shell commands
- Search for files and content
- Navigate the filesystem

# Guidelines
- Be concise and direct in your responses
- When asked to perform tasks, use the available tools
- Always confirm before destructive operations
- Provide clear explanations of what you're doing
- Format code blocks with appropriate syntax highlighting
- If you're unsure, ask for clarification

# Safety Rules
- Never delete files without explicit confirmation
- Be cautious with shell commands that modify the system
- Prefer read operations over write operations when exploring
- Always validate file paths before operations"#,
            self.os, self.arch, self.shell, self.cwd, self.time
        )
    }
}

impl Default for SystemContext {
    fn default() -> Self {
        Self::new()
    }
}
