# Changelog

All notable changes to sree will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-03-02

### Added

#### Core Features
- Modern terminal UI built with ratatui and crossterm
- Streaming responses from Claude AI (Anthropic API)
- Agentic loop with tool execution and multi-turn conversations
- 6 built-in tools: file_read, file_write, bash, grep, glob, web_search
- Markdown rendering with syntax-highlighted code blocks
- 6 color themes: dark, light, monokai, dracula, nord, solarized
- Input history with Up/Down navigation (persistent across sessions)
- Conversation export to markdown files

#### UI Components
- Scrollable chat area with auto-scroll behavior
- Multi-line text input with tui-textarea
- Status bar showing model, token count, and mode
- Header bar with app info and model selection
- Animated thinking spinner during LLM generation
- Bordered tool call boxes with status indicators (⏳/✓/✗)
- Mouse wheel scrolling support
- Visual separators between messages
- Theme-aware colors throughout the interface

#### LLM Integration
- Direct Anthropic API client built with reqwest
- Server-Sent Events (SSE) streaming parser
- Support for claude-sonnet-4, claude-opus-4, claude-haiku-3
- Token counting and context window management
- Automatic conversation truncation when approaching limits
- Real-time token-by-token display
- Cancel generation with Ctrl+C

#### Tool System
- Tool registry with JSON schema generation
- Async tool execution with timeout handling
- file_read: Read files, list directories, search within files
- file_write: Create, str_replace, insert, append operations
- bash: Execute shell commands with output capture
- grep: Regex search across files (gitignore-aware)
- glob: Find files matching patterns
- web_search: Stub implementation for future API integration

#### Configuration
- TOML config file at ~/.sree/config.toml
- Environment variable support (ANTHROPIC_API_KEY)
- CLI flags: --config, --model, --no-tools, --system
- Theme persistence
- Configurable timeouts and limits

#### Slash Commands
- /help - Show available commands
- /quit, /exit - Exit application
- /clear - Clear chat history
- /model [name] - Show or switch model
- /config - Show current configuration
- /tokens - Show token usage
- /export [file] - Export conversation to markdown
- /history - Show conversation summary
- /system <prompt> - Set custom system prompt
- /compact - Compact conversation history
- /version - Show version info
- /theme [name] - Switch color theme

#### Developer Experience
- Comprehensive test suite (99 tests passing)
- Unit tests for all tools and core modules
- Integration tests for agent loop
- Edge case tests for error handling
- Performance tests for conversation management
- Clippy clean with -D warnings
- Structured logging to ~/.sree/logs/sree.log
- Log rotation (10MB max, 5 files)
- Graceful shutdown with signal handling (SIGINT, SIGTERM)
- Proper terminal restoration on panic

#### Documentation
- Comprehensive README with usage examples
- Module-level documentation for all public APIs
- Inline documentation for key functions
- Architecture overview
- Troubleshooting guide
- Development guide

### Technical Details

#### Dependencies
- tokio: Async runtime
- ratatui + crossterm: Terminal UI
- tui-textarea: Multi-line input widget
- reqwest: HTTP client for Anthropic API
- serde + serde_json: JSON serialization
- clap: CLI argument parsing
- syntect: Syntax highlighting
- pulldown-cmark: Markdown parsing
- ignore: Gitignore-aware file walking
- regex: Pattern matching
- dirs: Config directory resolution
- tracing: Structured logging
- anyhow + thiserror: Error handling
- uuid: Unique IDs
- chrono: Timestamps

#### Architecture
- Modular design with clear separation of concerns
- Event-driven UI with async message passing
- Tool registry pattern for extensibility
- Theme system for customization
- Conversation management with token tracking
- Graceful error handling throughout

### Known Limitations
- Tree-sitter code intelligence not yet implemented
- MCP (Model Context Protocol) support not yet implemented
- Web search tool requires API key configuration
- Conversation summarization not yet implemented

## [Unreleased]

### Planned
- Tree-sitter integration for code intelligence
- MCP client support
- Conversation summarization for context compaction
- More color themes
- Plugin system for custom tools
- Improved markdown table rendering
- Performance optimizations for large conversations
