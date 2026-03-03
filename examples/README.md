# sree Examples

This directory contains example usage scenarios for sree. Each example demonstrates different features and capabilities.

## Available Examples

### 1. Basic Chat (`basic_chat.md`)
Simple conversation examples showing how to interact with sree for general questions and coding help.

### 2. File Operations (`file_operations.md`)
Examples of reading, writing, and modifying files using sree's tool capabilities.

### 3. Code Refactoring (`code_refactoring.md`)
Step-by-step example of using sree to refactor code across multiple files.

### 4. Project Analysis (`project_analysis.md`)
Using sree to analyze a codebase, find patterns, and suggest improvements.

### 5. Debugging Session (`debugging_session.md`)
Example of using sree to debug an issue by reading logs, examining code, and testing fixes.

### 6. Configuration Examples (`config_examples/`)
Sample configuration files for different use cases and preferences.

## Running Examples

These examples assume you have:
1. Built sree: `cargo build --release`
2. Set your API key: `export ANTHROPIC_API_KEY=your_key_here`
3. The binary is in your PATH or you're running `./target/release/sree`

## Tips

- Use `/help` to see all available commands
- Press `Ctrl+C` to cancel ongoing operations
- Use `/clear` to start a fresh conversation
- Try different themes with `/theme <name>`
- Export conversations with `/export <filename>`
