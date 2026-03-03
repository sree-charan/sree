use sree::config::Settings;

#[test]
fn test_default_settings() {
    let settings = Settings::default();
    assert_eq!(settings.llm.model, "anthropic.claude-sonnet-4-20250514-v1:0");
    assert_eq!(settings.llm.aws_region, "us-east-1");
    assert_eq!(settings.llm.max_tokens, 8192);
    assert_eq!(settings.ui.theme, "dark");
    assert!(settings.ui.show_token_count);
    assert_eq!(settings.tools.bash_timeout, 30);
}

#[test]
fn test_settings_serialization() {
    let settings = Settings::default();
    let toml_str = toml::to_string(&settings).unwrap();
    assert!(toml_str.contains("model"));
    assert!(toml_str.contains("aws_region"));
}

#[test]
fn test_settings_deserialization() {
    let toml_str = r#"
        [llm]
        aws_region = "us-west-2"
        model = "anthropic.claude-opus-4-20250514-v1:0"
        max_tokens = 4096
        temperature = 0.5

        [ui]
        theme = "light"
        show_token_count = false
        show_elapsed_time = true
        auto_scroll = true
        word_wrap = true
        code_theme = "base16-ocean.dark"

        [tools]
        bash_timeout = 60
        max_file_size = 2097152
        max_output_lines = 1000
    "#;
    
    let settings: Settings = toml::from_str(toml_str).unwrap();
    assert_eq!(settings.llm.aws_region, "us-west-2");
    assert_eq!(settings.llm.model, "anthropic.claude-opus-4-20250514-v1:0");
    assert_eq!(settings.llm.max_tokens, 4096);
    assert_eq!(settings.ui.theme, "light");
    assert!(!settings.ui.show_token_count);
    assert_eq!(settings.tools.bash_timeout, 60);
}

#[test]
fn test_aws_region_from_env() {
    std::env::set_var("AWS_REGION", "eu-west-1");
    let region = std::env::var("AWS_REGION").unwrap();
    assert_eq!(region, "eu-west-1");
    std::env::remove_var("AWS_REGION");
}
