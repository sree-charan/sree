//! Server-Sent Events (SSE) stream types for Anthropic API.
//!
//! The Anthropic API uses SSE to stream responses token by token. This module
//! defines the event types that can be received in the stream.
//!
//! ## Event Flow
//!
//! A typical streaming response follows this pattern:
//!
//! 1. `MessageStart` - Initial message metadata and usage
//! 2. `ContentBlockStart` - Start of a text or tool_use block
//! 3. `ContentBlockDelta` (multiple) - Streaming content within the block
//! 4. `ContentBlockStop` - End of the content block
//! 5. (Repeat 2-4 for each content block)
//! 6. `MessageDelta` - Final metadata with stop_reason
//! 7. `MessageStop` - End of message
//!
//! ## Example
//!
//! ```text
//! event: message_start
//! data: {"type":"message_start","message":{"id":"msg_123",...}}
//!
//! event: content_block_start
//! data: {"type":"content_block_start","index":0,"content_block":{"type":"text","text":""}}
//!
//! event: content_block_delta
//! data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}
//!
//! event: content_block_stop
//! data: {"type":"content_block_stop","index":0}
//!
//! event: message_delta
//! data: {"type":"message_delta","delta":{"stop_reason":"end_turn"}}
//!
//! event: message_stop
//! data: {"type":"message_stop"}
//! ```

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StreamEvent {
    MessageStart {
        role: String,
    },
    ContentBlockStart {
        index: i32,
        block_type: String,
        tool_use_id: Option<String>,
        tool_name: Option<String>,
    },
    TextDelta {
        index: i32,
        text: String,
    },
    ToolInputDelta {
        index: i32,
        json: String,
    },
    ContentBlockStop,
    MessageStop {
        stop_reason: Option<String>,
    },
}
