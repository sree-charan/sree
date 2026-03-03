use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub llm: LlmSettings,
    #[serde(default)]
    pub ui: UiSettings,
    #[serde(default)]
    pub tools: ToolSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSettings {
    #[serde(default = "default_region")]
    pub aws_region: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_true")]
    pub show_token_count: bool,
    #[serde(default = "default_true")]
    pub show_elapsed_time: bool,
    #[serde(default = "default_true")]
    pub auto_scroll: bool,
    #[serde(default = "default_true")]
    pub word_wrap: bool,
    #[serde(default = "default_code_theme")]
    pub code_theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSettings {
    #[serde(default = "default_bash_timeout")]
    pub bash_timeout: u64,
    #[serde(default = "default_max_file_size")]
    pub max_file_size: usize,
    #[serde(default = "default_max_output_lines")]
    pub max_output_lines: usize,
}

fn default_model() -> String {
    "us.anthropic.claude-sonnet-4-20250514-v1:0".to_string()
}

fn default_region() -> String {
    "us-east-1".to_string()
}

fn default_max_tokens() -> u32 {
    8192
}

fn default_temperature() -> f32 {
    0.7
}

fn default_theme() -> String {
    "dark".to_string()
}

fn default_code_theme() -> String {
    "base16-ocean.dark".to_string()
}

fn default_true() -> bool {
    true
}

fn default_bash_timeout() -> u64 {
    30
}

fn default_max_file_size() -> usize {
    1_048_576
}

fn default_max_output_lines() -> usize {
    500
}

#[allow(clippy::derivable_impls)]
impl Default for Settings {
    fn default() -> Self {
        Self {
            llm: LlmSettings::default(),
            ui: UiSettings::default(),
            tools: ToolSettings::default(),
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for LlmSettings {
    fn default() -> Self {
        Self {
            aws_region: default_region(),
            model: default_model(),
            max_tokens: default_max_tokens(),
            temperature: default_temperature(),
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            show_token_count: default_true(),
            show_elapsed_time: default_true(),
            auto_scroll: default_true(),
            word_wrap: default_true(),
            code_theme: default_code_theme(),
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for ToolSettings {
    fn default() -> Self {
        Self {
            bash_timeout: default_bash_timeout(),
            max_file_size: default_max_file_size(),
            max_output_lines: default_max_output_lines(),
        }
    }
}

impl Settings {
    #[allow(dead_code)]
    pub fn config_path() -> anyhow::Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        Ok(home.join(".sree").join("config.toml"))
    }

    #[allow(dead_code)]
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        
        let content = std::fs::read_to_string(&path)?;
        let settings: Settings = toml::from_str(&content)?;
        Ok(settings)
    }

    #[allow(dead_code)]
    pub fn load_from_path(path: &str) -> anyhow::Result<Self> {
        let path = PathBuf::from(path);
        if !path.exists() {
            return Err(anyhow::anyhow!("Config file not found: {}", path.display()));
        }
        
        let content = std::fs::read_to_string(&path)?;
        let settings: Settings = toml::from_str(&content)?;
        Ok(settings)
    }

    #[allow(dead_code)]
    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }
}

