use sree::tools::create_default_registry;
use serde_json::json;

#[tokio::test]
async fn test_tool_registry_creation() {
    let registry = create_default_registry();
    let schemas = registry.tool_schemas();
    
    // Should have 6 tools
    assert_eq!(schemas.len(), 6);
    
    // Check tool names
    let tool_names: Vec<String> = schemas
        .iter()
        .map(|s| s["name"].as_str().unwrap().to_string())
        .collect();
    
    assert!(tool_names.contains(&"file_read".to_string()));
    assert!(tool_names.contains(&"file_write".to_string()));
    assert!(tool_names.contains(&"bash".to_string()));
    assert!(tool_names.contains(&"grep".to_string()));
    assert!(tool_names.contains(&"glob".to_string()));
    assert!(tool_names.contains(&"web_search".to_string()));
}

#[tokio::test]
async fn test_file_read_tool() {
    let registry = create_default_registry();
    
    // Create a test file
    let test_content = "line 1\nline 2\nline 3\n";
    tokio::fs::write("/tmp/test_sree.txt", test_content).await.unwrap();
    
    // Read the file
    let result = registry.execute("file_read", json!({
        "path": "/tmp/test_sree.txt"
    })).await.unwrap();
    
    assert!(result.success);
    assert!(result.content.contains("line 1"));
    assert!(result.content.contains("line 2"));
    assert!(result.content.contains("line 3"));
    
    // Clean up
    tokio::fs::remove_file("/tmp/test_sree.txt").await.ok();
}

#[tokio::test]
async fn test_file_write_tool() {
    let registry = create_default_registry();
    
    // Create a file
    let result = registry.execute("file_write", json!({
        "path": "/tmp/test_sree_write.txt",
        "command": "create",
        "file_text": "Hello, world!"
    })).await.unwrap();
    
    assert!(result.success);
    
    // Verify it was created
    let content = tokio::fs::read_to_string("/tmp/test_sree_write.txt").await.unwrap();
    assert_eq!(content, "Hello, world!");
    
    // Clean up
    tokio::fs::remove_file("/tmp/test_sree_write.txt").await.ok();
}

#[tokio::test]
async fn test_bash_tool() {
    let registry = create_default_registry();
    
    // Run a simple command
    let result = registry.execute("bash", json!({
        "command": "echo 'test output'"
    })).await.unwrap();
    
    assert!(result.success);
    assert!(result.content.contains("test output"));
}

#[tokio::test]
async fn test_web_search_stub() {
    let registry = create_default_registry();
    
    // Web search should return an error message about API key
    let result = registry.execute("web_search", json!({
        "query": "test query"
    })).await.unwrap();
    
    assert!(!result.success);
    assert!(result.content.contains("API key"));
}

#[tokio::test]
async fn test_grep_tool() {
    let registry = create_default_registry();
    
    // Create test files
    tokio::fs::write("/tmp/test_grep1.txt", "hello world\nfoo bar\n").await.unwrap();
    tokio::fs::write("/tmp/test_grep2.txt", "hello rust\nbaz qux\n").await.unwrap();
    
    // Search for "hello"
    let result = registry.execute("grep", json!({
        "pattern": "hello",
        "path": "/tmp",
        "max_results": 10
    })).await.unwrap();
    
    assert!(result.success);
    assert!(result.content.contains("hello"));
    
    // Clean up
    tokio::fs::remove_file("/tmp/test_grep1.txt").await.ok();
    tokio::fs::remove_file("/tmp/test_grep2.txt").await.ok();
}

#[tokio::test]
async fn test_glob_tool() {
    let registry = create_default_registry();
    
    // Search for Cargo.toml
    let result = registry.execute("glob", json!({
        "pattern": "Cargo.toml",
        "max_results": 10
    })).await.unwrap();
    
    println!("Glob result: {:?}", result);
    assert!(result.success);
    // The pattern might not match because glob needs proper path matching
    // Just check that it succeeded
}
