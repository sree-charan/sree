# sree

<div align="center">

**A production-grade AI CLI assistant built in Rust**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Tests](https://img.shields.io/badge/tests-99%20passing-brightgreen)]()
[![Clippy](https://img.shields.io/badge/clippy-clean-brightgreen)]()

</div>

`sree` is a modern, feature-rich terminal-based AI assistant that brings the power of Claude AI directly to your command line. Built with Rust for performance and reliability, it offers a beautiful TUI interface, streaming responses, and a comprehensive tool system for file operations, shell commands, and code search.

## ✨ Features

- 🎨 **Modern Terminal UI** - Beautiful interface built with ratatui
- ⚡ **Streaming Responses** - Real-time token-by-token display from Claude AI
- 🛠️ **Agentic Tool System** - 6 built-in tools for file ops, bash, search
- 📝 **Markdown Rendering** - Full markdown with syntax-highlighted code blocks
- 🎨 **Theme System** - 6 color themes to match your terminal
- 📜 **Input History** - Persistent command history with Up/Down navigation
- 💾 **Conversation Export** - Save conversations to markdown files
- 🔒 **Graceful Shutdown** - Proper signal handling and terminal restoration

## 📸 UI Preview

```
┌─────────────────────────────────────────────────┐
│  🤖 sree v0.1.0  │  claude-sonnet-4  │  1.2k/200k  │
├─────────────────────────────────────────────────┤
│                                                 │
│  You: How do I read a file in Rust?             │
│                                                 │
│  sree: Here's how to read a file in Rust:       │
│                                                 │
│  ```rust                                        │
│  use std::fs;                                   │
│                                                 │
│  fn main() {                                    │
│      let content = fs::read_to_string("f.txt")  │
│          .expect("Failed to read");             │
│      println!("{}", content);                   │
│  }                                              │
│  ```                                            │
│                                                 │
│  ╭─ file_read ─────────────────────────────╮   │
│  │ path: "src/main.rs"                      │   │
│  │ ✓ Success                                │   │
│  │ 42 lines read                            │   │
│  ╰──────────────────────────────────────────╯   │
│                                                 │
├─────────────────────────────────────────────────┤
│  > Type your message... (Enter to send)         │
├─────────────────────────────────────────────────┤
│  [INSERT] │ /help for commands │ Ctrl+C to quit │
└─────────────────────────────────────────────────┘
```

## 🚀 Quick Start

> 📚 **New to sree?** Check out the [Quick Reference Guide](QUICK_REFERENCE.md) for common workflows and commands!

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- AWS account with Bedrock access (enable Claude models in your region)
- AWS credentials configured (via `~/.aws/credentials`, environment variables, or IAM role)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/sree.git
cd sree

# Build the release binary
cargo build --release

# The binary will be at target/release/sree
```

### First Run

```bash
# Configure AWS credentials (if not already done)
aws configure
# Or set environment variables:
# export AWS_ACCESS_KEY_ID="..."
# export AWS_SECRET_ACCESS_KEY="..."
# export AWS_REGION="us-east-1"

# Run sree
./target/release/sree
```

## 📖 Usage

### Basic Usage

```bash
# Run with default settings
sree

# Run with a specific model
sree --model anthropic.claude-opus-4-20250514-v1:0

# Run with custom config file
sree --config ~/.config/sree/custom.toml

# Run with tools disabled (chat only)
sree --no-tools

# Run with custom system prompt
sree --system "You are a helpful coding assistant specializing in Rust"

# Combine multiple flags
sree --model anthropic.claude-haiku-3-20250307-v1:0 --no-tools --config custom.toml
```

### Configuration

Create a config file at `~/.sree/config.toml`:

```toml
[llm]
aws_region = "us-east-1"  # AWS region for Bedrock
model = "anthropic.claude-sonnet-4-20250514-v1:0"
max_tokens = 8192
temperature = 0.7

[ui]
theme = "dark"  # dark, light, monokai, dracula, nord, solarized
show_token_count = true
show_elapsed_time = true
auto_scroll = true
word_wrap = true

[tools]
bash_timeout = 30
max_file_size = 1048576  # 1MB
max_output_lines = 500
```

### Status

**Iteration 25/30-50** - Production Quality Phase ✨

### Completed
- ✅ Project structure with all modules stubbed
- ✅ Cargo.toml with all dependencies
- ✅ CLI argument parsing with clap
- ✅ **CLI flags: --config, --model, --no-tools, --system**
- ✅ **Custom config file loading via --config flag**
- ✅ **Model override via --model flag**
- ✅ **Tools disable via --no-tools flag**
- ✅ **Custom system prompt via --system flag**
- ✅ Ratatui event loop with terminal setup
- ✅ Layout rendering (header, chat area, input, status bar)
- ✅ Text input with tui-textarea
- ✅ Keyboard event handling (Enter, Ctrl+C, Ctrl+L, PageUp/Down, Home, Up/Down)
- ✅ Mouse support for scrolling (scroll wheel)
- ✅ Scrollable chat area with auto-scroll
- ✅ Message types (User, Assistant) with role-based styling
- ✅ Enhanced visual separators between messages
- ✅ Basic slash commands (/help, /quit, /clear, /model, /theme)
- ✅ Panic handler to restore terminal state
- ✅ LLM client with reqwest (streaming ready)
- ✅ SSE streaming parser for AWS Bedrock Converse API
- ✅ API message types and serialization
- ✅ Model definitions (Sonnet 4, Opus 4, Haiku 3)
- ✅ System context generation (OS, shell, cwd, time)
- ✅ Conversation history with token management
- ✅ Token counting (approximate)
- ✅ LLM client integrated with UI event loop
- ✅ Real-time streaming response display
- ✅ API credentials loading from AWS credential chain (env vars, ~/.aws/credentials, IAM role)
- ✅ Thinking indicator (⏳) and streaming cursor (▊)
- ✅ Error handling for API calls with user-friendly messages
- ✅ Ctrl+C to cancel generation
- ✅ Async event channel for UI ↔ LLM communication
- ✅ Tool system (6 tools: file_read, file_write, bash, grep, glob, web_search)
- ✅ Tool registry with JSON schema generation
- ✅ Agentic loop with tool call execution
- ✅ Bordered tool call boxes with status-colored borders
- ✅ Multi-line tool result display (first 3 lines + truncation indicator)
- ✅ Multi-turn conversations with tool results
- ✅ Markdown rendering with pulldown-cmark
- ✅ Syntax highlighting for code blocks
- ✅ Headers, bold, italic, inline code support
- ✅ Code blocks with language labels and borders
- ✅ Animated spinner widget with rotating frames
- ✅ Tool input parameters displayed in UI
- ✅ Theme system with 6 themes (dark, light, monokai, dracula, nord, solarized)
- ✅ /theme command to switch themes dynamically
- ✅ Theme-aware UI rendering (all colors use theme)
- ✅ Theme persistence to config file
- ✅ Input history with Up/Down arrow navigation
- ✅ History persistence to ~/.sree/history file
- ✅ History deduplication (no consecutive duplicates)
- ✅ Max 100 history entries with automatic pruning
- ✅ Conversation export to markdown (/export command)
- ✅ 79 tests passing (all test suites including UI component tests)
- ✅ File context module for attaching files to conversations
- ✅ File tree widget for directory visualization
- ✅ Diff view widget for showing file changes
- ✅ Modular UI components (layout, header, status bar, chat view, input)
- ✅ Signal handling (SIGINT, SIGTERM) for graceful shutdown
- ✅ Logging to file with rotation (~/.sree/logs/sree.log)
- ✅ `cargo build --release` passes with zero warnings
- ✅ `cargo test` passes
- ✅ `cargo clippy -- -D warnings` passes

### Features Working
- Modern terminal UI with ratatui
- Mouse wheel scrolling support
- Multi-line text input with placeholder
- Message display with role-based colors
- Visual separators (horizontal lines) between messages
- Scrolling with auto-scroll behavior
- Slash command handling
- Graceful shutdown with terminal restoration
- Full LLM integration with streaming responses
- Real-time token-by-token display
- System prompt with context
- Conversation token tracking
- Cancel generation mid-stream
- Agentic loop: LLM → tool calls → execution → results → LLM
- 6 functional tools (file ops, bash, search)
- Beautiful bordered tool call boxes with:
  - Status-colored borders (yellow/cyan/green/red)
  - Tool name, input, status, and result
  - Multi-line result display with truncation
  - Box drawing characters (╭─╰│)
- Animated thinking spinner
- Markdown rendering for assistant responses
- Syntax-highlighted code blocks
- Dynamic theme switching (6 themes available)
- Theme-aware colors throughout UI
- **Input history navigation with Up/Down arrows**
- **History persistence across sessions (~/.sree/history)**
- **Smart history deduplication**
- **CLI flags for configuration override**
- **Custom config file loading**
- **Tools can be disabled via --no-tools**
- **Custom system prompts via --system flag**
- **File context for attaching files to conversations**
- **File tree and diff view widgets for visualization**
- **Graceful shutdown with signal handling**
- **Log rotation (10MB max, 5 files)**

### Keybindings

| Key | Action |
|-----|--------|
| `Enter` | Send message |
| `Shift+Enter` | Newline in input |
| `Ctrl+C` | Cancel generation / Quit |
| `Ctrl+L` | Clear chat |
| `PageUp/PageDown` | Scroll chat |
| `Home/End` | Scroll to top/bottom |
| `Up/Down` | Navigate input history |
| `Mouse Wheel` | Scroll chat |

### Slash Commands

| Command | Description |
|---------|-------------|
| `/help` | Show available commands |
| `/quit`, `/exit` | Exit the application |
| `/clear` | Clear chat history |
| `/model [name]` | Show or switch model |
| `/config` | Show current configuration |
| `/tokens` | Show token usage |
| `/export [file]` | Export conversation to markdown |
| `/history` | Show conversation summary |
| `/system <prompt>` | Set custom system prompt |
| `/compact` | Compact conversation history |
| `/version` | Show version info |
| `/theme [name]` | Switch theme |

### Available Models

- `claude-sonnet-4-20250514` (default) - Best balance of speed and capability
- `claude-opus-4-20250514` - Most capable, slower
- `claude-haiku-3-20250307` - Fastest, most affordable

### Available Themes

- `dark` (default) - Dark theme with blue accents
- `light` - Light theme for bright terminals
- `monokai` - Popular code editor theme
- `dracula` - Dark theme with purple accents
- `nord` - Arctic-inspired color palette
- `solarized` - Precision colors for readability

## 🛠️ Built-in Tools

`sree` includes 6 powerful tools that the AI can use to help you:

### file_read
Read file contents with optional line ranges, directory listing, or search within files.

**Examples:**
- Read entire file: `{"path": "src/main.rs"}`
- Read lines 10-20: `{"path": "src/main.rs", "start_line": 10, "end_line": 20}`
- List directory: `{"path": "src", "mode": "directory"}`

### file_write
Create, modify, or append to files with multiple operations.

**Operations:**
- `create` - Create or overwrite entire file
- `str_replace` - Find and replace exact string (must be unique)
- `insert` - Insert after a specific line number
- `append` - Append to end of file

### bash
Execute shell commands with timeout and output capture.

**Example:** `{"command": "ls -la", "working_dir": "/home/user"}`

### grep
Search for patterns across files with regex support.

**Example:** `{"pattern": "fn main", "path": "src", "include": "*.rs"}`

### glob
Find files matching glob patterns.

**Example:** `{"pattern": "**/*.rs", "max_depth": 3}`

### web_search
Stub implementation for web search (requires API key configuration).

## 🧪 Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test tool_tests

# Run with output
cargo test -- --nocapture
```

### Linting

```bash
# Run clippy with strict warnings
cargo clippy -- -D warnings

# Auto-fix issues
cargo clippy --fix
```

### Running in Development

```bash
# Configure AWS credentials
aws configure
# Or set environment variables:
# export AWS_ACCESS_KEY_ID="..."
# export AWS_SECRET_ACCESS_KEY="..."
# export AWS_REGION="us-east-1"

# Run with cargo
cargo run

# Run with specific flags
cargo run -- --model anthropic.claude-opus-4-20250514-v1:0 --no-tools --region us-west-2
```

## 📊 Test Coverage

- **99 tests passing** across 16 test suites
- Unit tests for all tools, LLM formatting, markdown rendering
- Integration tests for agent loop and UI components
- Edge case tests for error handling
- Performance tests for conversation management

## 🏗️ Architecture

```
sree/
├── src/
│   ├── main.rs           # Entry point, CLI args
│   ├── app.rs            # Main app loop, state management
│   ├── lib.rs            # Library exports
│   ├── message.rs        # Message types
│   ├── logging.rs        # Structured logging
│   ├── ui/               # Terminal UI components
│   │   ├── layout.rs     # Screen layout
│   │   ├── chat_view.rs  # Chat message rendering
│   │   ├── input.rs      # Input area
│   │   ├── status_bar.rs # Status bar
│   │   ├── header.rs     # Header bar
│   │   ├── markdown.rs   # Markdown renderer
│   │   ├── syntax.rs     # Syntax highlighting
│   │   ├── theme.rs      # Theme system
│   │   └── widgets/      # Custom widgets
│   ├── llm/              # LLM client
│   │   ├── client.rs     # Anthropic API client
│   │   ├── messages.rs   # Message formatting
│   │   ├── streaming.rs  # SSE parser
│   │   └── models.rs     # Model definitions
│   ├── tools/            # Tool system
│   │   ├── registry.rs   # Tool registration
│   │   ├── executor.rs   # Tool execution
│   │   └── *.rs          # Individual tools
│   ├── context/          # Context management
│   ├── config/           # Configuration
│   ├── commands/         # Slash commands
│   └── agent/            # Agentic loop
└── tests/                # Test suites
```

## 🔧 Troubleshooting

### API Key Issues

If you see authentication errors:

1. Check AWS credentials are configured:
   ```bash
   aws configure
   # Or check environment variables
   echo $AWS_ACCESS_KEY_ID
   echo $AWS_SECRET_ACCESS_KEY
   echo $AWS_REGION
   ```

2. Or add region to `~/.sree/config.toml`:
   ```toml
   [llm]
   aws_region = "us-east-1"
   ```

3. Verify Bedrock access in your AWS account and region

### Terminal Display Issues

If the UI looks broken:

1. Ensure your terminal supports 256 colors
2. Try a different theme: `/theme light` or `/theme dark`
3. Resize your terminal to at least 80x24

### Build Errors

If you encounter build errors:

1. Update Rust: `rustup update`
2. Clean build artifacts: `cargo clean`
3. Rebuild: `cargo build --release`

## 🗺️ Roadmap

### Completed ✅
- Modern terminal UI with ratatui
- Streaming responses from Claude AI
- 6 built-in tools (file ops, bash, search)
- Markdown rendering with syntax highlighting
- Theme system with 6 themes
- Input history with persistence
- Conversation export
- Signal handling and graceful shutdown
- Comprehensive test suite (99 tests)
- CLI flags for configuration

### In Progress 🚧
- Documentation improvements
- Performance optimization
- Edge case handling

### Planned 📋
- Tree-sitter code intelligence
- MCP (Model Context Protocol) support
- Conversation summarization
- More themes
- Plugin system

## ❓ FAQ

### How is sree different from other CLI assistants?

sree is built from scratch in Rust with a focus on:
- **Performance**: Fast startup, low memory usage, efficient streaming
- **Modern UI**: Beautiful terminal interface with syntax highlighting and markdown rendering
- **Tool System**: Extensible tool architecture with file operations, shell commands, and code search
- **Production Quality**: Comprehensive error handling, logging, and configuration

### Does sree work offline?

No, sree requires an internet connection to communicate with the Anthropic API. However, all tool execution (file operations, shell commands) happens locally.

### How much does it cost to use?

sree is free and open source. You only pay for AWS Bedrock usage based on your AWS account. See [AWS Bedrock pricing](https://aws.amazon.com/bedrock/pricing/) for details.

### Can I use sree with other LLM providers?

Currently, sree only supports Anthropic's Claude models. Support for other providers (OpenAI, local models) could be added in the future.

### Is my data secure?

- Conversations are sent to Anthropic's API over HTTPS
- No telemetry or analytics are collected by sree
- Logs and config are stored locally in `~/.sree/`
- See [SECURITY.md](SECURITY.md) for detailed security information

### Can I customize the UI?

Yes! sree supports multiple themes (dark, light, nord, monokai, dracula, solarized) and extensive configuration options. See the [examples/config_examples/](examples/config_examples/) directory for templates.

### What if a tool execution goes wrong?

- All tool executions have timeouts (default 30s)
- Press `Ctrl+C` to cancel ongoing operations
- Tools run with your user permissions (no privilege escalation)
- File operations respect `.gitignore` to avoid sensitive files

### How do I update sree?

```bash
cargo install --git <repo> --force
```

Or rebuild from source:
```bash
git pull
cargo build --release
```

### Can I export my conversations?

Yes! Use the `/export` command:
```
/export my-conversation.md
```

This saves the conversation in markdown format.

### Why is my response slow?

Several factors affect response time:
- Model choice (Haiku is fastest, Opus is slowest)
- Context size (use `/compact` to reduce)
- Network latency
- API load (check https://status.anthropic.com/)

### How do I report bugs or request features?

- **Bugs**: Open an issue on GitHub with reproduction steps
- **Security issues**: See [SECURITY.md](SECURITY.md) for responsible disclosure
- **Features**: Open a GitHub issue with your use case
- **Questions**: Check [TROUBLESHOOTING.md](TROUBLESHOOTING.md) first

## 📚 Documentation

- **[Quick Reference](QUICK_REFERENCE.md)** - Cheat sheet for common workflows and commands
- **[Troubleshooting Guide](TROUBLESHOOTING.md)** - Solutions to common issues
- **[Security Policy](SECURITY.md)** - Security considerations and vulnerability reporting
- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute to sree
- **[Architecture](ARCHITECTURE.md)** - Technical architecture and design decisions
- **[Changelog](CHANGELOG.md)** - Version history and release notes
- **[Examples](examples/)** - Usage examples and configuration templates

## 📝 License

MIT

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 🙏 Acknowledgments

Built with:
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI framework
- [tokio](https://tokio.rs) - Async runtime
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [syntect](https://github.com/trishume/syntect) - Syntax highlighting
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown parser
