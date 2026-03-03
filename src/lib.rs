//! # sree - A Production-Grade AI CLI Assistant
//!
//! `sree` is a modern, feature-rich terminal-based AI assistant built in Rust.
//! It provides a beautiful TUI interface powered by ratatui, streaming responses
//! from Claude AI, and a comprehensive tool system for file operations, shell
//! commands, and code search.
//!
//! ## Features
//!
//! - **Modern Terminal UI**: Built with ratatui for a responsive, beautiful interface
//! - **Streaming Responses**: Real-time token-by-token display from Claude AI
//! - **Agentic Tool System**: 6 built-in tools (file_read, file_write, bash, grep, glob, web_search)
//! - **Markdown Rendering**: Full markdown support with syntax-highlighted code blocks
//! - **Theme System**: 6 color themes (dark, light, monokai, dracula, nord, solarized)
//! - **Input History**: Persistent command history with Up/Down navigation
//! - **Conversation Export**: Save conversations to markdown files
//! - **Graceful Shutdown**: Proper signal handling and terminal restoration
//!
//! ## Architecture
//!
//! The crate is organized into several modules:
//!
//! - [`llm`]: Anthropic API client with SSE streaming support
//! - [`tools`]: Tool system with registry, executor, and built-in tools
//! - [`ui`]: Terminal UI components (layout, chat view, input, widgets)
//! - [`context`]: System context and conversation management
//! - [`config`]: Configuration and settings management
//! - [`commands`]: Slash command handlers
//! - [`agent`]: Agentic loop for tool execution
//! - [`message`]: Message types and conversation history
//! - [`logging`]: Structured logging to file

pub mod llm;
pub mod context;
pub mod message;
pub mod ui;
pub mod tools;
pub mod config;
pub mod commands;
pub mod agent;
pub mod logging;
