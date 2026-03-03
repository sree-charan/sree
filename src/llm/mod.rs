//! LLM client for interacting with the Anthropic Claude API.
//!
//! This module provides a complete implementation of the Anthropic Messages API,
//! including support for streaming responses via Server-Sent Events (SSE) and
//! tool use for agentic workflows.
//!
//! # Architecture
//!
//! - [`client`]: HTTP client for making API requests
//! - [`messages`]: Message types and formatting for the API
//! - [`streaming`]: SSE stream parser for real-time responses
//! - [`models`]: Model definitions and capabilities
//! - [`token_counter`]: Approximate token counting for context management

pub mod client;
pub mod messages;
pub mod streaming;
pub mod models;
pub mod token_counter;
