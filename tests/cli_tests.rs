// Tests for CLI flag handling and configuration overrides

use sree::config::settings::Settings;

#[test]
fn test_load_from_custom_path() {
    // Create a temporary config file
    let temp_dir = std::env::temp_dir();
    let config_path = temp_dir.join("test_config.toml");
    
    let config_content = r#"
[llm]
model = "claude-opus-4-20250514"
max_tokens = 4096

[ui]
theme = "light"
"#;
    
    std::fs::write(&config_path, config_content).unwrap();
    
    let settings = Settings::load_from_path(config_path.to_str().unwrap()).unwrap();
    
    assert_eq!(settings.llm.model, "claude-opus-4-20250514");
    assert_eq!(settings.llm.max_tokens, 4096);
    assert_eq!(settings.ui.theme, "light");
    
    // Cleanup
    std::fs::remove_file(&config_path).ok();
}

#[test]
fn test_load_from_nonexistent_path() {
    let result = Settings::load_from_path("/nonexistent/config.toml");
    assert!(result.is_err());
}

#[test]
fn test_model_override() {
    let mut settings = Settings::default();
    assert_eq!(settings.llm.model, "anthropic.claude-sonnet-4-20250514-v1:0");
    
    // Simulate CLI override
    settings.llm.model = "anthropic.claude-haiku-3-20250307-v1:0".to_string();
    assert_eq!(settings.llm.model, "anthropic.claude-haiku-3-20250307-v1:0");
}
