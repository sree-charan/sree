# Contributing to sree

Thank you for your interest in contributing to sree! This guide will help you get started.

## Development Setup

### Prerequisites

- Rust 1.70 or later (install via [rustup](https://rustup.rs/))
- Git
- A terminal emulator that supports ANSI colors and Unicode

### Getting Started

1. Fork and clone the repository:
```bash
git clone https://github.com/yourusername/sree.git
cd sree
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Run clippy:
```bash
cargo clippy -- -D warnings
```

## Project Structure

```
sree/
├── src/
│   ├── main.rs              # Entry point and CLI
│   ├── app.rs               # Main application state and event loop
│   ├── ui/                  # Terminal UI components
│   ├── llm/                 # LLM client and message handling
│   ├── tools/               # Tool implementations
│   ├── context/             # Context management
│   ├── config/              # Configuration system
│   ├── agent/               # Agentic loop
│   └── commands/            # Slash command handlers
└── tests/                   # Integration and unit tests
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture documentation.

## Development Workflow

### Making Changes

1. Create a new branch for your feature or fix:
```bash
git checkout -b feature/your-feature-name
```

2. Make your changes, following the code style guidelines below

3. Add tests for new functionality

4. Ensure all tests pass:
```bash
cargo test
```

5. Ensure clippy is happy:
```bash
cargo clippy -- -D warnings
```

6. Format your code:
```bash
cargo fmt
```

7. Commit your changes with a descriptive message:
```bash
git commit -m "Add feature: description"
```

### Code Style Guidelines

- Follow Rust standard naming conventions (snake_case for functions/variables, PascalCase for types)
- Use `Result` and `?` for error handling - avoid `unwrap()` in production code
- Add documentation comments (`///`) for public APIs
- Keep functions focused and under 50 lines when possible
- Use meaningful variable names
- Add module-level documentation (`//!`) explaining the module's purpose

### Error Handling

- Use `anyhow::Result` for application errors
- Use `thiserror` for custom error types
- Provide context with `.context()` or `.with_context()`
- Make error messages user-friendly and actionable

Example:
```rust
use anyhow::{Context, Result};

fn read_config() -> Result<Config> {
    let content = fs::read_to_string(path)
        .context("Failed to read config file. Run 'sree --init' to create one.")?;
    // ...
}
```

### Testing

- Write unit tests for individual functions
- Write integration tests for end-to-end workflows
- Test error cases, not just happy paths
- Use descriptive test names: `test_file_read_with_line_range`
- Mock external dependencies (API calls, file system when appropriate)

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_execution_success() {
        // Arrange
        let tool = FileTool::new();
        let input = json!({"path": "test.txt"});
        
        // Act
        let result = tool.execute(input).await;
        
        // Assert
        assert!(result.is_ok());
    }
}
```

### Adding a New Tool

1. Create a new file in `src/tools/` (e.g., `my_tool.rs`)

2. Implement the `Tool` trait:
```rust
use async_trait::async_trait;
use serde_json::json;

pub struct MyTool;

#[async_trait]
impl Tool for MyTool {
    fn name(&self) -> &str {
        "my_tool"
    }

    fn description(&self) -> &str {
        "Description of what the tool does"
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "param": {
                    "type": "string",
                    "description": "Parameter description"
                }
            },
            "required": ["param"]
        })
    }

    async fn execute(&self, input: serde_json::Value) -> Result<ToolResult> {
        // Implementation
        Ok(ToolResult::success("Result"))
    }
}
```

3. Register the tool in `src/tools/registry.rs`:
```rust
registry.register(Box::new(MyTool));
```

4. Add tests in `tests/tool_tests.rs`

### Adding a New Slash Command

1. Add the command handler in `src/commands/handlers.rs`

2. Update the command parser in `src/commands/mod.rs`

3. Add help text for the command

4. Add tests in `tests/command_tests.rs`

### UI Components

When working on UI components:

- Use `ratatui` widgets and styling
- Test rendering with different terminal sizes
- Ensure proper text wrapping and scrolling
- Support both light and dark themes
- Handle edge cases (empty content, very long lines)

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test tool_tests

# Run specific test
cargo test test_file_read_tool

# Run with output
cargo test -- --nocapture
```

### Test Coverage

We aim for >80% test coverage on core modules. To check coverage:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html
```

## Documentation

### Code Documentation

- Add `///` doc comments for all public functions, structs, and traits
- Include examples in doc comments when helpful
- Add `//!` module-level documentation explaining the module's purpose
- Keep documentation concise but complete

### User Documentation

- Update README.md for user-facing changes
- Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format
- Add examples for new features

## Performance

- Profile before optimizing
- Avoid unnecessary allocations
- Use streaming for large data
- Benchmark performance-critical code

## Submitting Changes

### Pull Request Process

1. Update documentation (README, CHANGELOG, code comments)
2. Ensure all tests pass
3. Ensure clippy passes with `-D warnings`
4. Format code with `cargo fmt`
5. Push to your fork
6. Open a pull request with:
   - Clear description of changes
   - Link to related issues
   - Screenshots for UI changes
   - Test results

### PR Checklist

- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] `cargo test` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt` applied
- [ ] No `unwrap()` in production code
- [ ] Error messages are user-friendly

## Common Issues

### Build Failures

If you encounter build failures:

1. Update Rust: `rustup update`
2. Clean build: `cargo clean && cargo build`
3. Check dependency versions in `Cargo.toml`

### Test Failures

If tests fail:

1. Run with output: `cargo test -- --nocapture`
2. Run specific test: `cargo test test_name`
3. Check for timing issues in async tests

### Clippy Warnings

Fix clippy warnings before submitting:

```bash
cargo clippy --fix -- -D warnings
```

## Getting Help

- Open an issue for bugs or feature requests
- Check existing issues before creating new ones
- Be specific and provide examples
- Include error messages and logs

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others learn and grow
- Assume good intentions

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to sree! 🚀
