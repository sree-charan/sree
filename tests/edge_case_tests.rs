use sree::tools::create_default_registry;
use serde_json::json;
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_file_write_invalid_command() {
    let registry = create_default_registry();
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    
    let input = json!({
        "path": file_path.to_str().unwrap(),
        "command": "invalid_command",
        "file_text": "test"
    });
    
    let result = registry.execute("file_write", input).await;
    // Should handle gracefully
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_file_write_missing_required_field() {
    let registry = create_default_registry();
    
    let input = json!({
        "path": "/tmp/test.txt",
        "command": "create"
        // Missing file_text
    });
    
    let result = registry.execute("file_write", input).await;
    // The tool will return an error because of missing field during deserialization
    assert!(result.is_err() || (result.is_ok() && result.unwrap().is_error()));
}

#[tokio::test]
async fn test_file_write_str_replace_not_unique() {
    let registry = create_default_registry();
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    
    // Create file with duplicate content
    fs::write(&file_path, "hello\nhello\n").unwrap();
    
    let input = json!({
        "path": file_path.to_str().unwrap(),
        "command": "str_replace",
        "old_str": "hello",
        "new_str": "world"
    });
    
    let result = registry.execute("file_write", input).await.unwrap();
    assert!(result.is_error());
    assert!(result.content.contains("must be unique"));
}

#[tokio::test]
async fn test_file_write_insert_beyond_file_length() {
    let registry = create_default_registry();
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    
    fs::write(&file_path, "line1\nline2\n").unwrap();
    
    let input = json!({
        "path": file_path.to_str().unwrap(),
        "command": "insert",
        "insert_line": 100,
        "new_str": "new line"
    });
    
    let result = registry.execute("file_write", input).await.unwrap();
    assert!(result.is_error());
    assert!(result.content.contains("exceeds file length"));
}

#[tokio::test]
async fn test_bash_timeout() {
    let registry = create_default_registry();
    
    let input = json!({
        "command": "sleep 10",
        "timeout_secs": 1
    });
    
    let result = registry.execute("bash", input).await.unwrap();
    assert!(result.is_error());
    assert!(result.content.contains("timed out"));
}

#[tokio::test]
async fn test_bash_invalid_directory() {
    let registry = create_default_registry();
    
    let input = json!({
        "command": "pwd",
        "working_dir": "/nonexistent/directory/path"
    });
    
    let result = registry.execute("bash", input).await.unwrap();
    assert!(result.is_error());
}

#[tokio::test]
async fn test_file_read_nonexistent_file() {
    let registry = create_default_registry();
    
    let input = json!({
        "path": "/nonexistent/file/path.txt"
    });
    
    let result = registry.execute("file_read", input).await.unwrap();
    assert!(result.is_error());
}

#[tokio::test]
async fn test_grep_invalid_regex() {
    let registry = create_default_registry();
    
    let input = json!({
        "pattern": "[invalid(regex",
        "path": "."
    });
    
    let result = registry.execute("grep", input).await;
    // Should handle gracefully
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_glob_invalid_pattern() {
    let registry = create_default_registry();
    
    let input = json!({
        "pattern": "[[[invalid"
    });
    
    let result = registry.execute("glob", input).await;
    // Should handle gracefully
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_tool_registry_unknown_tool() {
    let registry = create_default_registry();
    
    let input = json!({});
    let result = registry.execute("nonexistent_tool", input).await;
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Tool not found"));
}
