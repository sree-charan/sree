//! Terminal user interface components built with ratatui.
//!
//! This module provides all the UI components for the sree terminal interface,
//! including layout management, message rendering, input handling, and custom
//! widgets for displaying tool calls, file trees, and diffs.
//!
//! # Architecture
//!
//! - [`layout`]: Main screen layout (header, chat area, input, status bar)
//! - [`chat_view`]: Scrollable chat message rendering
//! - [`input`]: Multi-line input area with tui-textarea
//! - [`status_bar`]: Bottom status bar showing model, tokens, mode
//! - [`header`]: Top header bar with app info
//! - [`markdown`]: Markdown to ratatui styled text renderer
//! - [`syntax`]: Syntax highlighting for code blocks
//! - [`theme`]: Color theme system with 6 themes
//! - [`widgets`]: Custom widgets (spinner, tool calls, file tree, diff view)
//!
//! # Themes
//!
//! The UI supports 6 color themes:
//! - `dark` - Dark theme with blue accents (default)
//! - `light` - Light theme for bright terminals
//! - `monokai` - Popular code editor theme
//! - `dracula` - Dark theme with purple accents
//! - `nord` - Arctic-inspired color palette
//! - `solarized` - Precision colors for readability

pub mod layout;
pub mod chat_view;
pub mod input;
pub mod status_bar;
pub mod header;
pub mod markdown;
pub mod syntax;
pub mod theme;
pub mod widgets;
