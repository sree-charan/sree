# Quick Reference Guide

A cheat sheet for common sree workflows and commands.

## Getting Started

```bash
# Set API key
export ANTHROPIC_API_KEY="your-key-here"

# Start sree
sree

# Start with specific model
sree --model claude-opus-4-20250514

# Start with custom config
sree --config ~/my-config.toml
```

## Slash Commands

| Command | Description | Example |
|---------|-------------|---------|
| `/help` | Show all commands | `/help` |
| `/quit` or `/exit` | Exit sree | `/quit` |
| `/clear` | Clear conversation history | `/clear` |
| `/model [name]` | Switch or show model | `/model claude-haiku-3-20250307` |
| `/theme [name]` | Switch or show theme | `/theme nord` |
| `/config` | Show current configuration | `/config` |
| `/export [file]` | Export conversation to markdown | `/export chat.md` |
| `/history` | Show conversation summary | `/history` |
| `/tokens` | Show detailed token usage | `/tokens` |
| `/system [prompt]` | Set custom system prompt | `/system You are a Python expert` |
| `/compact` | Summarize and compact history | `/compact` |
| `/version` | Show version info | `/version` |

## Keybindings

| Key | Action |
|-----|--------|
| `Enter` | Send message |
| `Shift+Enter` | New line in input |
| `Ctrl+C` | Cancel generation / quit |
| `Ctrl+L` | Clear screen |
| `Up/Down` | Navigate input history |
| `PageUp/PageDown` | Scroll chat history |
| `Home/End` | Jump to top/bottom |
| `Esc` | Cancel current input |

## Common Workflows

### File Operations

```
# Read a file
Can you read src/main.rs?

# Read specific lines
Show me lines 10-20 of config.toml

# List directory
What files are in the src/ directory?

# Create a file
Create a new file called hello.py with a hello world program

# Edit a file
In src/main.rs, replace "old text" with "new text"

# Append to file
Add a new function to the end of utils.rs
```

### Code Analysis

```
# Search for patterns
Find all TODO comments in the codebase

# Find files
Show me all Rust files in this project

# Analyze code
What does the main function in src/main.rs do?

# Review changes
Show me the diff between the old and new version
```

### Shell Commands

```
# Run commands
Run cargo test

# Check status
What's the git status?

# Build project
Build the project in release mode

# Install dependencies
Install the missing dependencies
```

### Project Understanding

```
# Overview
Give me an overview of this project

# Architecture
Explain the architecture of this codebase

# Dependencies
What dependencies does this project use?

# Documentation
Generate documentation for this module
```

### Debugging

```
# Find errors
Why is this test failing?

# Trace execution
Walk me through what happens when I call this function

# Fix bugs
This code has a bug, can you fix it?

# Performance
Why is this function slow?
```

## Configuration Quick Start

Create `~/.sree/config.toml`:

```toml
[llm]
api_key = "sk-ant-..."  # Or use ANTHROPIC_API_KEY env var
model = "claude-sonnet-4-20250514"
max_tokens = 8192
temperature = 0.7

[ui]
theme = "dark"
show_token_count = true
auto_scroll = true
word_wrap = true

[tools]
bash_timeout = 30
max_file_size = 1048576
max_output_lines = 500
```

## Available Models

| Model | Speed | Intelligence | Use Case |
|-------|-------|--------------|----------|
| `claude-haiku-3-20250307` | ⚡⚡⚡ | ⭐⭐ | Quick questions, simple tasks |
| `claude-sonnet-4-20250514` | ⚡⚡ | ⭐⭐⭐ | Balanced (default) |
| `claude-opus-4-20250514` | ⚡ | ⭐⭐⭐⭐ | Complex reasoning, large codebases |

## Available Themes

- `dark` - Default dark theme
- `light` - Light theme for bright terminals
- `nord` - Nord color scheme
- `monokai` - Monokai color scheme
- `dracula` - Dracula color scheme
- `solarized` - Solarized color scheme

## Tool Capabilities

### file_read
- Read entire files or specific line ranges
- List directory contents with depth control
- Search within files using regex patterns
- Respects `.gitignore`

### file_write
- Create new files
- Find and replace text (str_replace)
- Insert lines at specific positions
- Append to end of file

### bash
- Execute shell commands
- Capture stdout and stderr
- Configurable timeout
- Working directory support

### grep
- Regex search across files
- Context lines around matches
- File type filtering
- Respects `.gitignore`

### glob
- Find files by pattern
- Recursive search with depth control
- Sorted by modification time
- Respects `.gitignore`

### web_search
- Stub implementation (requires API key configuration)
- Structure ready for real implementation

## Tips & Tricks

### Save Tokens
```
# Compact conversation when it gets long
/compact

# Clear history and start fresh
/clear

# Use Haiku for simple questions
/model claude-haiku-3-20250307
```

### Improve Responses
```
# Set domain-specific system prompt
/system You are an expert in Rust systems programming

# Provide context
"I'm working on a web server in Rust. Here's my code..."

# Be specific
Instead of: "Fix this"
Try: "This function panics when input is empty. Add error handling."
```

### Work Faster
```
# Use input history
Press Up/Down to recall previous messages

# Export important conversations
/export important-discussion.md

# Use keyboard shortcuts
Ctrl+L to clear screen, Ctrl+C to cancel
```

### Debug Issues
```
# Check token usage
/tokens

# View configuration
/config

# Check logs
cat ~/.sree/logs/sree.log

# Enable debug mode
RUST_LOG=debug sree
```

## Environment Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `ANTHROPIC_API_KEY` | API authentication | `export ANTHROPIC_API_KEY="sk-ant-..."` |
| `RUST_LOG` | Logging level | `export RUST_LOG=debug` |
| `RUST_BACKTRACE` | Show backtraces on panic | `export RUST_BACKTRACE=1` |
| `TERM` | Terminal type | `export TERM=xterm-256color` |

## File Locations

| Path | Contents |
|------|----------|
| `~/.sree/config.toml` | Configuration file |
| `~/.sree/history` | Input history |
| `~/.sree/logs/` | Application logs |

## Getting Help

1. **In-app help**: `/help`
2. **Documentation**: Check README.md
3. **Troubleshooting**: See TROUBLESHOOTING.md
4. **Examples**: Browse examples/ directory
5. **Issues**: Open GitHub issue

## Example Session

```
$ sree

🤖 sree v0.1.0  │  claude-sonnet-4-20250514  │  tokens: 0/200k

> What files are in this directory?

sree: Let me check the current directory for you.

🔧 Tool: file_read(".")
├─ Status: ✓ Success
└─ Found 15 entries

Here are the files in the current directory:
- src/ (directory)
- Cargo.toml
- Cargo.lock
- README.md
- ...

> Read src/main.rs

sree: I'll read that file for you.

🔧 Tool: file_read("src/main.rs")
├─ Status: ✓ Success
└─ 42 lines read

Here's the content of src/main.rs:
```rust
fn main() {
    println!("Hello, world!");
}
```

This is a simple Rust program that prints "Hello, world!" to the console.

> /quit

Goodbye! 👋
```

## Pro Tips

1. **Use specific line numbers** when editing files to avoid ambiguity
2. **Review tool calls** before they execute (future feature)
3. **Export conversations** for documentation or sharing
4. **Compact regularly** on long sessions to save tokens
5. **Use themes** that match your terminal background
6. **Set custom system prompts** for specialized tasks
7. **Check logs** when something goes wrong
8. **Use Haiku** for quick questions to save costs
9. **Provide context** about your project for better responses
10. **Be specific** in your requests for better results
