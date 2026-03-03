# Agentic Loop Integration Architecture

## Flow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                            User Input                            │
│                     "Read the file test.txt"                     │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                         App (app.rs)                             │
│  • Captures user input                                           │
│  • Creates Message::user()                                       │
│  • Calls send_to_llm()                                          │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    AgentLoop (agent/loop.rs)                     │
│  • Converts UI messages → API messages                           │
│  • Sends request to Anthropic API with tools                     │
│  • Streams response tokens                                       │
│  • Detects tool_use blocks                                       │
└────────────────────────────┬────────────────────────────────────┘
                             │
                ┌────────────┴────────────┐
                │                         │
                ▼                         ▼
    ┌───────────────────┐     ┌──────────────────┐
    │   Text Response   │     │   Tool Call      │
    │   "I'll read..."  │     │   file_read()    │
    └─────────┬─────────┘     └────────┬─────────┘
              │                        │
              │                        ▼
              │            ┌──────────────────────┐
              │            │  ToolRegistry        │
              │            │  • Finds tool        │
              │            │  • Executes async    │
              │            │  • Returns result    │
              │            └──────────┬───────────┘
              │                       │
              │                       ▼
              │            ┌──────────────────────┐
              │            │  Tool Result         │
              │            │  "File content..."   │
              │            └──────────┬───────────┘
              │                       │
              │                       ▼
              │            ┌──────────────────────┐
              │            │  Feed back to LLM    │
              │            │  (tool_result)       │
              │            └──────────┬───────────┘
              │                       │
              └───────────────────────┘
                          │
                          ▼
              ┌──────────────────────┐
              │  Final Response      │
              │  "The file contains" │
              └──────────┬───────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────────┐
│                    UI Rendering (app.rs)                         │
│                                                                  │
│  You: Read the file test.txt                                    │
│                                                                  │
│  sree: I'll read that file for you.                             │
│                                                                  │
│  🔧 Tool: file_read                                             │
│  ├─ Status: ✓ Success                                          │
│  └─ File content here...                                        │
│                                                                  │
│  The file contains the following content...                     │
└─────────────────────────────────────────────────────────────────┘
```

## Event Flow

### 1. User Message → AgentLoop
```rust
AppEvent::Token(text) → current_response.push_str()
```

### 2. Tool Call Detection
```rust
AgentEvent::ToolCallStart(tool_call) 
  → AppEvent::ToolCallStart { id, name }
  → Message.tool_calls.push(ToolCallInfo { status: Running })
```

### 3. Tool Execution
```rust
ToolRegistry.execute(name, input)
  → Tool.execute(input)
  → ToolResult { success, content }
```

### 4. Tool Result Display
```rust
AgentEvent::ToolCallComplete(id, result)
  → AppEvent::ToolCallComplete { id, result }
  → Update ToolCallInfo.status to Success/Error
  → UI re-renders with status icon
```

### 5. Loop Continuation
```rust
if tool_calls.is_empty():
  → AgentEvent::Complete
  → AppEvent::Complete
  → is_generating = false
else:
  → Feed tool_result back to API
  → Continue loop (max 25 rounds)
```

## Key Components

### Message (message.rs)
- Stores conversation history
- Tracks tool calls with status
- Supports User, Assistant, System roles

### AgentLoop (agent/loop.rs)
- Manages multi-turn conversation
- Detects and executes tool calls
- Streams events to UI via callback
- Handles up to 25 tool rounds

### ToolRegistry (tools/registry.rs)
- Registers all available tools
- Generates tool schemas for API
- Dispatches tool execution
- Cloneable for async tasks

### App (app.rs)
- Main UI event loop
- Renders messages and tool calls
- Handles user input
- Displays real-time status updates

## Status Indicators

| Status    | Icon | Color  | Meaning                    |
|-----------|------|--------|----------------------------|
| Pending   | ⏳   | Yellow | Tool call queued           |
| Running   | ⏳   | Yellow | Tool currently executing   |
| Success   | ✓    | Green  | Tool completed successfully|
| Error     | ✗    | Red    | Tool execution failed      |
