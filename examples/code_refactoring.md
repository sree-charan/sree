# Code Refactoring Example

This example shows a complete refactoring session where we improve error handling across a small Rust project.

## Initial State

Project structure:
```
myapp/
├── src/
│   ├── main.rs
│   ├── api.rs
│   └── db.rs
└── Cargo.toml
```

## Session Transcript

```
You: I want to refactor this project to use proper error handling with anyhow and thiserror

sree: I'll help you refactor the error handling. Let me first examine the current code.

🔧 Tool: glob("src/**/*.rs")
├─ Status: ✓ Success
└─ Found 3 files: main.rs, api.rs, db.rs

🔧 Tool: file_read("src/main.rs")
🔧 Tool: file_read("src/api.rs")
🔧 Tool: file_read("src/db.rs")

I can see the current code uses unwrap() and expect() in several places. Here's my refactoring plan:

1. Add anyhow and thiserror to dependencies
2. Create a custom error type with thiserror for domain errors
3. Replace unwrap()/expect() with proper error propagation
4. Update main() to handle errors gracefully

Shall I proceed?

You: Yes, go ahead

sree: Starting refactoring...

**Step 1: Update Cargo.toml**

🔧 Tool: file_read("Cargo.toml")
├─ Status: ✓ Success

🔧 Tool: file_write(
    command="str_replace",
    path="Cargo.toml",
    old_str="[dependencies]\ntokio = { version = \"1.0\", features = [\"full\"] }",
    new_str="[dependencies]\ntokio = { version = \"1.0\", features = [\"full\"] }\nanyhow = \"1.0\"\nthiserror = \"1.0\""
)
├─ Status: ✓ Success

**Step 2: Create error types**

🔧 Tool: file_write(
    command="create",
    path="src/error.rs",
    content="use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(\"Database error: {0}\")]
    Database(#[from] sqlx::Error),
    
    #[error(\"API error: {0}\")]
    Api(String),
    
    #[error(\"Configuration error: {0}\")]
    Config(String),
    
    #[error(\"IO error: {0}\")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
"
)
├─ Status: ✓ Success
└─ Created src/error.rs

**Step 3: Update main.rs**

🔧 Tool: file_write(
    command="str_replace",
    path="src/main.rs",
    old_str="fn main() {
    let config = load_config().expect(\"Failed to load config\");
    let db = connect_db(&config).expect(\"Failed to connect to DB\");
    start_server(config, db).expect(\"Server failed\");
}",
    new_str="mod error;
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config()?;
    let db = connect_db(&config).await?;
    start_server(config, db).await?;
    Ok(())
}"
)
├─ Status: ✓ Success

**Step 4: Update api.rs**

🔧 Tool: file_write(
    command="str_replace",
    path="src/api.rs",
    old_str="pub fn fetch_data(url: &str) -> String {
    reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap()
}",
    new_str="use crate::error::{AppError, Result};

pub async fn fetch_data(url: &str) -> Result<String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| AppError::Api(e.to_string()))?;
    
    let text = response
        .text()
        .await
        .map_err(|e| AppError::Api(e.to_string()))?;
    
    Ok(text)
}"
)
├─ Status: ✓ Success

**Step 5: Update db.rs**

🔧 Tool: file_write(
    command="str_replace",
    path="src/db.rs",
    old_str="pub fn connect_db(config: &Config) -> Database {
    Database::connect(&config.db_url).unwrap()
}

pub fn query_users(db: &Database) -> Vec<User> {
    db.query(\"SELECT * FROM users\").unwrap()
}",
    new_str="use crate::error::Result;

pub async fn connect_db(config: &Config) -> Result<Database> {
    let db = Database::connect(&config.db_url).await?;
    Ok(db)
}

pub async fn query_users(db: &Database) -> Result<Vec<User>> {
    let users = db.query(\"SELECT * FROM users\").await?;
    Ok(users)
}"
)
├─ Status: ✓ Success

**Step 6: Add error module to lib.rs**

🔧 Tool: file_write(
    command="insert",
    path="src/lib.rs",
    insert_line=0,
    new_str="pub mod error;"
)
├─ Status: ✓ Success

✅ Refactoring complete! Summary of changes:

1. ✓ Added anyhow and thiserror dependencies
2. ✓ Created src/error.rs with custom error types
3. ✓ Updated main.rs to return Result and use ? operator
4. ✓ Updated api.rs to propagate errors properly
5. ✓ Updated db.rs to use async and proper error handling
6. ✓ Removed all unwrap() and expect() calls

The code now has:
- Proper error propagation with ?
- Custom error types for domain-specific errors
- Automatic error conversion with #[from]
- Better error messages for debugging

You can now run `cargo build` to verify everything compiles.

You: Great! Can you also add error context to make debugging easier?

sree: Good idea! I'll add context to the error chains using anyhow's context feature.

🔧 Tool: file_write(
    command="str_replace",
    path="src/main.rs",
    old_str="#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config()?;
    let db = connect_db(&config).await?;
    start_server(config, db).await?;
    Ok(())
}",
    new_str="use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config()
        .context(\"Failed to load configuration\")?;
    
    let db = connect_db(&config).await
        .context(\"Failed to connect to database\")?;
    
    start_server(config, db).await
        .context(\"Server failed to start\")?;
    
    Ok(())
}"
)
├─ Status: ✓ Success

🔧 Tool: file_write(
    command="str_replace",
    path="src/api.rs",
    old_str="pub async fn fetch_data(url: &str) -> Result<String> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| AppError::Api(e.to_string()))?;
    
    let text = response
        .text()
        .await
        .map_err(|e| AppError::Api(e.to_string()))?;
    
    Ok(text)
}",
    new_str="use anyhow::Context;

pub async fn fetch_data(url: &str) -> anyhow::Result<String> {
    let response = reqwest::get(url)
        .await
        .with_context(|| format!(\"Failed to fetch URL: {}\", url))?;
    
    let text = response
        .text()
        .await
        .context(\"Failed to read response body\")?;
    
    Ok(text)
}"
)
├─ Status: ✓ Success

✅ Added error context! Now when errors occur, you'll see the full chain:

```
Error: Server failed to start

Caused by:
    0: Failed to connect to database
    1: Connection refused (os error 111)
```

This makes debugging much easier in production.

You: Perfect! Let's test it

sree: Let's verify the build and run tests:

🔧 Tool: bash("cargo build")
├─ Status: ✓ Success (exit code: 0)
└─ Output: Compiling myapp v0.1.0
           Finished dev [unoptimized + debuginfo] target(s) in 3.2s

🔧 Tool: bash("cargo test")
├─ Status: ✓ Success (exit code: 0)
└─ Output: running 5 tests
           test tests::test_config ... ok
           test tests::test_db_connection ... ok
           test tests::test_api_fetch ... ok
           test tests::test_error_handling ... ok
           test tests::test_server_start ... ok
           
           test result: ok. 5 passed; 0 failed

🔧 Tool: bash("cargo clippy -- -D warnings")
├─ Status: ✓ Success (exit code: 0)
└─ Output: Checking myapp v0.1.0
           Finished dev [unoptimized + debuginfo] target(s) in 1.8s

✅ All checks passed! The refactoring is complete and working correctly.

Summary:
- Replaced all unwrap()/expect() with proper error handling
- Added custom error types with thiserror
- Added error context with anyhow
- All tests pass
- No clippy warnings
- Better error messages for debugging
```

## Key Takeaways

This example demonstrates:
1. **Planning**: sree analyzed the code first before making changes
2. **Systematic approach**: Step-by-step refactoring with verification
3. **Tool usage**: Multiple tools working together (file_read, file_write, bash)
4. **Best practices**: Using thiserror for library errors, anyhow for application errors
5. **Verification**: Testing after changes to ensure nothing broke
6. **Iterative improvement**: Adding context after the initial refactoring
