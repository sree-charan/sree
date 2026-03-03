# Basic Chat Examples

## Example 1: Simple Question

```
You: What's the difference between Vec and slice in Rust?

sree: A Vec is an owned, growable array on the heap, while a slice is a 
view into a contiguous sequence. Key differences:

- Vec<T>: Owns data, can grow/shrink, allocated on heap
- &[T]: Borrowed view, fixed size, points to existing data
- &mut [T]: Mutable borrowed view

Example:
```rust
let vec = vec![1, 2, 3];  // Vec<i32>
let slice = &vec[..];      // &[i32]
```
```

## Example 2: Code Explanation

```
You: Explain this code:
fn main() {
    let x = 5;
    let y = &x;
    println!("{}", *y);
}

sree: This demonstrates borrowing and dereferencing:

1. `let x = 5` - Creates an i32 variable
2. `let y = &x` - Creates an immutable reference to x
3. `*y` - Dereferences y to get the value (5)

The & creates a reference, * dereferences it. This is safe because
the reference doesn't outlive x.
```

## Example 3: Quick Code Generation

```
You: Write a function to read a JSON file and parse it into a HashMap

sree: Here's a function using serde_json:

```rust
use std::fs;
use std::collections::HashMap;
use serde_json::Value;
use anyhow::Result;

fn read_json_to_map(path: &str) -> Result<HashMap<String, Value>> {
    let content = fs::read_to_string(path)?;
    let map: HashMap<String, Value> = serde_json::from_str(&content)?;
    Ok(map)
}
```

Add these dependencies to Cargo.toml:
```toml
[dependencies]
serde_json = "1.0"
anyhow = "1.0"
```
```

## Example 4: Using Slash Commands

```
You: /model

sree: Current model: claude-sonnet-4-20250514

Available models:
- claude-sonnet-4-20250514 (current)
- claude-opus-4-20250514
- claude-haiku-3-20250307

Use /model <name> to switch.

You: /theme monokai

sree: Theme switched to: monokai

You: /tokens

sree: Token usage for this conversation:
- Input tokens: 1,234
- Output tokens: 567
- Total: 1,801 / 200,000 (0.9%)
```

## Example 5: Multi-turn Conversation

```
You: I need to implement a rate limiter in Rust

sree: I'll help you build a token bucket rate limiter. What's your use case?
- API rate limiting?
- Request throttling?
- Background job control?

You: API rate limiting for a web service

sree: Here's a simple token bucket implementation:

```rust
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub struct RateLimiter {
    tokens: Mutex<f64>,
    capacity: f64,
    refill_rate: f64,
    last_refill: Mutex<Instant>,
}

impl RateLimiter {
    pub fn new(capacity: f64, refill_per_sec: f64) -> Self {
        Self {
            tokens: Mutex::new(capacity),
            capacity,
            refill_rate: refill_per_sec,
            last_refill: Mutex::new(Instant::now()),
        }
    }

    pub async fn acquire(&self) -> bool {
        let mut tokens = self.tokens.lock().await;
        let mut last = self.last_refill.lock().await;
        
        let now = Instant::now();
        let elapsed = now.duration_since(*last).as_secs_f64();
        *tokens = (*tokens + elapsed * self.refill_rate).min(self.capacity);
        *last = now;

        if *tokens >= 1.0 {
            *tokens -= 1.0;
            true
        } else {
            false
        }
    }
}
```

Usage in an Axum handler:
```rust
async fn handler(
    State(limiter): State<Arc<RateLimiter>>,
) -> Result<Json<Response>, StatusCode> {
    if !limiter.acquire().await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    // Handle request
}
```

You: How do I test this?

sree: Here's a test suite:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_rate_limiter_allows_within_limit() {
        let limiter = RateLimiter::new(2.0, 1.0);
        assert!(limiter.acquire().await);
        assert!(limiter.acquire().await);
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_over_limit() {
        let limiter = RateLimiter::new(1.0, 1.0);
        assert!(limiter.acquire().await);
        assert!(!limiter.acquire().await);
    }

    #[tokio::test]
    async fn test_rate_limiter_refills() {
        let limiter = RateLimiter::new(1.0, 2.0);
        assert!(limiter.acquire().await);
        sleep(Duration::from_millis(600)).await;
        assert!(limiter.acquire().await);
    }
}
```
```
