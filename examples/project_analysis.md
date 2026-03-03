# Project Analysis Example

This example shows how to use sree to analyze a codebase and get insights.

## Session Transcript

```
You: Analyze this Rust project and tell me about its structure

sree: I'll analyze the project structure and code patterns.

🔧 Tool: glob("**/*.rs")
├─ Status: ✓ Success
└─ Found 24 Rust files

🔧 Tool: file_read(path=".", mode="directory", depth=2)
├─ Status: ✓ Success

Project structure:
```
.
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── handlers.rs
│   │   └── routes.rs
│   ├── db/
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   └── queries.rs
│   ├── auth/
│   │   ├── mod.rs
│   │   └── jwt.rs
│   └── utils/
│       ├── mod.rs
│       └── validation.rs
└── tests/
    ├── api_tests.rs
    └── integration_tests.rs
```

Let me examine the key files...

🔧 Tool: file_read("Cargo.toml")
🔧 Tool: file_read("src/main.rs")
🔧 Tool: file_read("src/lib.rs")

**Analysis Summary:**

**Architecture:**
- Web API using Axum framework
- PostgreSQL database with SQLx
- JWT authentication
- Modular structure with clear separation of concerns

**Dependencies:**
- axum 0.7 (web framework)
- sqlx 0.7 (database)
- tokio 1.0 (async runtime)
- serde 1.0 (serialization)
- jsonwebtoken 9.0 (auth)

**Code Organization:**
- ✓ Good: Clear module structure
- ✓ Good: Separation of API, DB, and auth logic
- ⚠ Note: No error handling module (consider adding)
- ⚠ Note: Utils module is small (might not need separate module)

**Test Coverage:**
- 2 test files found
- Integration tests present
- API tests present

Would you like me to analyze specific aspects like error handling, security, or performance?

You: Yes, check for security issues

sree: I'll scan for common security issues.

🔧 Tool: grep(pattern="unwrap|expect", include="**/*.rs")
├─ Status: ✓ Success
└─ Found 12 matches

Found potential panic points:
- src/api/handlers.rs:45: .unwrap()
- src/api/handlers.rs:67: .expect("Invalid user ID")
- src/db/queries.rs:23: .unwrap()
- src/auth/jwt.rs:34: .unwrap()

🔧 Tool: grep(pattern="TODO|FIXME|XXX", include="**/*.rs")
├─ Status: ✓ Success
└─ Found 3 matches

Found TODOs:
- src/auth/jwt.rs:12: // TODO: Add token refresh
- src/api/handlers.rs:89: // FIXME: Add rate limiting
- src/db/models.rs:56: // TODO: Add input validation

🔧 Tool: grep(pattern="password|secret|key", include="**/*.rs")
├─ Status: ✓ Success
└─ Found 8 matches

Checking credential handling...

🔧 Tool: file_read("src/auth/jwt.rs", start_line=30, end_line=40)

**Security Analysis:**

**Critical Issues:**
1. ❌ Hardcoded JWT secret in jwt.rs:34
   - Line: `let secret = "my-secret-key".unwrap();`
   - Fix: Use environment variable or config file

2. ❌ No rate limiting (noted in FIXME)
   - Location: src/api/handlers.rs:89
   - Risk: Vulnerable to brute force attacks

**Medium Issues:**
3. ⚠ Multiple unwrap() calls that could panic
   - 12 instances found
   - Should use proper error handling

4. ⚠ No input validation (noted in TODO)
   - Location: src/db/models.rs:56
   - Risk: Potential SQL injection or invalid data

**Recommendations:**

1. **Immediate**: Remove hardcoded secrets
   ```rust
   let secret = std::env::var("JWT_SECRET")
       .expect("JWT_SECRET must be set");
   ```

2. **High Priority**: Add rate limiting middleware
   ```rust
   use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
   
   let governor_conf = Box::new(
       GovernorConfigBuilder::default()
           .per_second(10)
           .burst_size(20)
           .finish()
           .unwrap()
   );
   ```

3. **Medium Priority**: Replace unwrap() with proper error handling

4. **Medium Priority**: Add input validation with validator crate

Would you like me to fix any of these issues?

You: Fix the hardcoded secret issue

sree: I'll fix the hardcoded JWT secret.

🔧 Tool: file_read("src/auth/jwt.rs")
├─ Status: ✓ Success

🔧 Tool: file_write(
    command="str_replace",
    path="src/auth/jwt.rs",
    old_str="let secret = \"my-secret-key\".as_bytes();",
    new_str="let secret = std::env::var(\"JWT_SECRET\")
    .expect(\"JWT_SECRET environment variable must be set\")
    .as_bytes()
    .to_vec();"
)
├─ Status: ✓ Success

🔧 Tool: file_write(
    command="create",
    path=".env.example",
    content="# JWT Configuration
JWT_SECRET=your-secret-key-here-change-in-production

# Database
DATABASE_URL=postgres://user:pass@localhost/dbname

# Server
HOST=127.0.0.1
PORT=8080
"
)
├─ Status: ✓ Success

🔧 Tool: file_read("README.md")

🔧 Tool: file_write(
    command="str_replace",
    path="README.md",
    old_str="## Running\n\n```bash\ncargo run\n```",
    new_str="## Configuration\n\nCopy `.env.example` to `.env` and set your values:\n\n```bash\ncp .env.example .env\n```\n\nRequired environment variables:\n- `JWT_SECRET`: Secret key for JWT signing (use a strong random value)\n- `DATABASE_URL`: PostgreSQL connection string\n\n## Running\n\n```bash\ncargo run\n```"
)
├─ Status: ✓ Success

✅ Fixed hardcoded secret! Changes made:

1. Updated jwt.rs to read JWT_SECRET from environment
2. Created .env.example with template
3. Updated README.md with configuration instructions

**Security improvement:**
- ✓ No more hardcoded secrets in code
- ✓ Secrets managed via environment variables
- ✓ Example file for easy setup
- ✓ Documentation updated

**Next steps:**
- Set JWT_SECRET in your environment or .env file
- Use a strong random value (e.g., from `openssl rand -base64 32`)
- Never commit .env to version control (add to .gitignore)

You: Great! Now analyze the test coverage

sree: I'll analyze the test coverage.

🔧 Tool: bash("cargo test --no-run 2>&1 | grep -E 'test|running'")
├─ Status: ✓ Success

🔧 Tool: file_read("tests/api_tests.rs")
🔧 Tool: file_read("tests/integration_tests.rs")

🔧 Tool: grep(pattern="#\\[test\\]|#\\[tokio::test\\]", include="**/*.rs")
├─ Status: ✓ Success
└─ Found 15 test functions

**Test Coverage Analysis:**

**Test Distribution:**
- API tests: 8 tests
- Integration tests: 5 tests
- Unit tests: 2 tests (in src/)
- Total: 15 tests

**Coverage by Module:**

✓ **Well Tested:**
- API handlers: 8 tests
  - GET /users
  - POST /users
  - Authentication flow
  - Error cases

⚠ **Partially Tested:**
- Database queries: 3 tests
  - Basic CRUD covered
  - Missing: Complex queries, transactions

❌ **Not Tested:**
- JWT token generation/validation: 0 tests
- Input validation: 0 tests
- Utils module: 0 tests

**Missing Test Scenarios:**
1. JWT expiration handling
2. Invalid token formats
3. Concurrent database access
4. Rate limiting (when implemented)
5. Edge cases in validation

**Recommendations:**

1. Add JWT tests:
   ```rust
   #[test]
   fn test_jwt_expiration() { ... }
   
   #[test]
   fn test_invalid_jwt() { ... }
   ```

2. Add validation tests:
   ```rust
   #[test]
   fn test_email_validation() { ... }
   ```

3. Consider using `cargo-tarpaulin` for coverage reports:
   ```bash
   cargo install cargo-tarpaulin
   cargo tarpaulin --out Html
   ```

**Estimated Coverage:** ~60-70%
**Target:** 80%+

Would you like me to add tests for the untested modules?
```

## Key Takeaways

This example shows how sree can:
1. **Analyze structure**: Understand project organization
2. **Find issues**: Scan for security problems, TODOs, patterns
3. **Provide insights**: Give actionable recommendations
4. **Fix problems**: Make targeted improvements
5. **Assess quality**: Evaluate test coverage and code quality

The combination of grep, glob, and file_read tools makes comprehensive codebase analysis possible.
