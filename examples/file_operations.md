# File Operations Examples

## Example 1: Reading Files

```
You: Read the main.rs file

sree: 🔧 Tool: file_read("src/main.rs")
├─ Status: ✓ Success
└─ 45 lines read

[File contents displayed with line numbers]

You: Show me just lines 10-20

sree: 🔧 Tool: file_read("src/main.rs", start_line=10, end_line=20)
├─ Status: ✓ Success
└─ 11 lines read

[Lines 10-20 displayed]
```

## Example 2: Creating a New File

```
You: Create a new file called config.toml with default settings

sree: I'll create a config file with sensible defaults.

🔧 Tool: file_write(command="create", path="config.toml", content="...")
├─ Status: ✓ Success
└─ Created with 15 lines

Created config.toml:
```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
url = "postgres://localhost/mydb"
max_connections = 10

[logging]
level = "info"
format = "json"
```
```

## Example 3: Modifying Existing Files

```
You: In src/lib.rs, replace the old error handling with anyhow

sree: I'll update the error handling to use anyhow.

🔧 Tool: file_read("src/lib.rs")
├─ Status: ✓ Success
└─ 120 lines read

🔧 Tool: file_write(
    command="str_replace",
    path="src/lib.rs",
    old_str="use std::error::Error;\n\npub type Result<T> = std::result::Result<T, Box<dyn Error>>;",
    new_str="use anyhow::Result;"
)
├─ Status: ✓ Success
└─ Replaced 1 occurrence

Updated! Don't forget to add anyhow to Cargo.toml:
```toml
[dependencies]
anyhow = "1.0"
```
```

## Example 4: Inserting Code

```
You: Add a new function after line 50 in utils.rs that formats timestamps

sree: I'll add a timestamp formatting function.

🔧 Tool: file_write(
    command="insert",
    path="src/utils.rs",
    insert_line=50,
    new_str="
pub fn format_timestamp(ts: i64) -> String {
    use chrono::{DateTime, Utc};
    let dt = DateTime::<Utc>::from_timestamp(ts, 0)
        .expect(\"Invalid timestamp\");
    dt.format(\"%Y-%m-%d %H:%M:%S UTC\").to_string()
}
"
)
├─ Status: ✓ Success
└─ Inserted 6 lines after line 50

Function added! Requires chrono dependency.
```

## Example 5: Appending to Files

```
You: Add a new test to the end of tests/api_tests.rs

sree: I'll append a new test case.

🔧 Tool: file_write(
    command="append",
    path="tests/api_tests.rs",
    new_str="
#[tokio::test]
async fn test_rate_limiting() {
    let client = TestClient::new();
    
    // Make requests until rate limited
    for _ in 0..100 {
        let resp = client.get(\"/api/data\").await;
        if resp.status() == 429 {
            return; // Rate limit works
        }
    }
    
    panic!(\"Rate limiting not enforced\");
}
"
)
├─ Status: ✓ Success
└─ Appended 15 lines

Test added to the end of the file.
```

## Example 6: Directory Listing

```
You: What files are in the src directory?

sree: 🔧 Tool: file_read(path="src", mode="directory")
├─ Status: ✓ Success
└─ 12 entries

src/
├── main.rs
├── lib.rs
├── config.rs
├── api/
│   ├── mod.rs
│   ├── handlers.rs
│   └── routes.rs
├── db/
│   ├── mod.rs
│   └── models.rs
└── utils.rs

You: Show me the api directory structure with depth 2

sree: 🔧 Tool: file_read(path="src/api", mode="directory", depth=2)
├─ Status: ✓ Success
└─ 8 entries

src/api/
├── mod.rs
├── handlers.rs
├── routes.rs
├── middleware/
│   ├── mod.rs
│   ├── auth.rs
│   └── logging.rs
└── types.rs
```

## Example 7: Batch File Operations

```
You: Refactor all files in src/ to use the new error type

sree: I'll update all source files to use the new error handling.

🔧 Tool: glob("src/**/*.rs")
├─ Status: ✓ Success
└─ Found 15 files

🔧 Tool: file_read("src/main.rs")
🔧 Tool: file_read("src/lib.rs")
🔧 Tool: file_read("src/config.rs")
[... reading all files ...]

Now updating each file:

🔧 Tool: file_write(command="str_replace", path="src/main.rs", ...)
├─ Status: ✓ Success

🔧 Tool: file_write(command="str_replace", path="src/lib.rs", ...)
├─ Status: ✓ Success

[... updating all files ...]

✓ Updated 15 files to use the new error type
```

## Example 8: Safe File Modifications

```
You: Update the database connection string in config.toml

sree: I'll update the database URL. Let me read it first to ensure accuracy.

🔧 Tool: file_read("config.toml")
├─ Status: ✓ Success
└─ 20 lines read

Current config:
```toml
[database]
url = "postgres://localhost/mydb"
```

What should the new connection string be?

You: postgres://localhost/production_db

sree: 🔧 Tool: file_write(
    command="str_replace",
    path="config.toml",
    old_str="url = \"postgres://localhost/mydb\"",
    new_str="url = \"postgres://localhost/production_db\""
)
├─ Status: ✓ Success
└─ Updated 1 line

Database URL updated to production_db.
```
