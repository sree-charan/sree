//! Configuration management for sree.
//!
//! This module handles loading and saving configuration from TOML files,
//! environment variables, and CLI arguments. Configuration is stored at
//! `~/.sree/config.toml` by default.
//!
//! # Configuration Structure
//!
//! ```toml
//! [llm]
//! api_key = "sk-ant-..."
//! model = "claude-sonnet-4-20250514"
//! max_tokens = 8192
//! temperature = 0.7
//!
//! [ui]
//! theme = "dark"
//! show_token_count = true
//! show_elapsed_time = true
//! auto_scroll = true
//! word_wrap = true
//!
//! [tools]
//! bash_timeout = 30
//! max_file_size = 1048576
//! max_output_lines = 500
//! ```
//!
//! # Example
//!
//! ```no_run
//! use sree::config::Settings;
//!
//! # fn example() -> anyhow::Result<()> {
//! // Load from default location (~/.sree/config.toml)
//! let settings = Settings::load()?;
//! # Ok(())
//! # }
//! ```

pub mod settings;
pub mod theme;

#[allow(unused_imports)]
pub use settings::Settings;
