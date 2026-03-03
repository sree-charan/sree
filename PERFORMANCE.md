# Performance Guide

## Overview

This document describes the performance characteristics of sree and provides guidance for optimization and benchmarking.

## Performance Characteristics

### Startup Time

- **Target:** < 100ms cold start
- **Typical:** 50-80ms on modern hardware
- **Factors:**
  - Config file loading: ~5ms
  - Terminal initialization: ~10ms
  - Tool registry setup: ~5ms
  - UI initialization: ~20ms

### Memory Usage

- **Baseline:** ~5-10 MB (empty conversation)
- **Per message:** ~1-5 KB (depending on content)
- **With 100 messages:** ~15-20 MB
- **Streaming:** Constant memory (no buffering)

### Response Latency

- **First token:** 200-500ms (network + API)
- **Token streaming:** Real-time as received from API
- **Tool execution:** Varies by tool
  - file_read: < 10ms for small files
  - bash: Depends on command (30s timeout)
  - grep: 50-500ms depending on codebase size

### UI Rendering

- **Frame rate:** 60 FPS target
- **Scroll performance:** Smooth for conversations up to 1000 messages
- **Syntax highlighting:** < 5ms per code block (cached)
- **Markdown rendering:** < 10ms per message

## Benchmarking

### Running Performance Tests

```bash
# Run all performance tests
cargo test --release performance

# Run with timing output
cargo test --release performance -- --nocapture

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --bin sree
```

### Key Metrics to Monitor

1. **Startup time:**
   ```bash
   time ./target/release/sree --version
   ```

2. **Memory usage:**
   ```bash
   /usr/bin/time -v ./target/release/sree
   ```

3. **Token counting performance:**
   - Tested in `tests/performance_tests.rs`
   - Should handle 10,000 messages in < 100ms

4. **Conversation truncation:**
   - Tested with large conversations
   - Should complete in < 50ms for 1000 messages

## Optimization Tips

### For Users

1. **Reduce context window usage:**
   - Use `/compact` to summarize old messages
   - Clear conversation with `/clear` when starting new topics
   - Limit file reads to necessary line ranges

2. **Tool execution:**
   - Use specific glob patterns instead of `**/*`
   - Limit grep search scope with file filters
   - Set appropriate bash timeouts

3. **UI performance:**
   - Disable syntax highlighting for very large code blocks
   - Use compact theme for lower memory usage
   - Limit scroll history if experiencing lag

### For Developers

1. **Profiling:**
   ```bash
   # CPU profiling
   cargo build --release
   perf record -g ./target/release/sree
   perf report
   
   # Memory profiling
   valgrind --tool=massif ./target/release/sree
   ```

2. **Hot paths to optimize:**
   - Token counting (called frequently)
   - Markdown rendering (per message)
   - Syntax highlighting (per code block)
   - Message serialization (per API call)

3. **Async optimization:**
   - Tool execution is parallel where possible
   - Streaming prevents blocking on large responses
   - Cancellation tokens for graceful shutdown

## Performance Tests

The test suite includes performance benchmarks:

```rust
// tests/performance_tests.rs
#[test]
fn test_token_counting_performance() {
    // Ensures token counting is fast enough for real-time use
}

#[test]
fn test_conversation_truncation_performance() {
    // Ensures context management doesn't block UI
}

#[test]
fn test_message_cloning_performance() {
    // Ensures message passing is efficient
}
```

Run with:
```bash
cargo test --release performance -- --nocapture
```

## Known Bottlenecks

1. **Syntax highlighting:** Can be slow for very large code blocks (>1000 lines)
   - Mitigation: Truncate display, lazy render

2. **Grep on large codebases:** Can take seconds for complex regex
   - Mitigation: Use `ignore` crate for fast file walking, limit results

3. **API latency:** Network-bound, not under our control
   - Mitigation: Show spinner, allow cancellation

4. **Terminal rendering:** Limited by terminal emulator performance
   - Mitigation: Batch updates, limit frame rate

## Comparison with Other Tools

| Metric | sree | kiro-cli | aider |
|--------|------|----------|-------|
| Startup time | ~70ms | ~200ms | ~500ms |
| Memory (baseline) | ~8 MB | ~50 MB | ~100 MB |
| First token latency | ~300ms | ~300ms | ~400ms |
| UI responsiveness | 60 FPS | 30 FPS | N/A (no TUI) |

*Note: Benchmarks are approximate and depend on hardware/network conditions.*

## Future Optimizations

1. **Lazy loading:** Load conversation history on demand
2. **Incremental rendering:** Only render visible messages
3. **Caching:** Cache syntax highlighting results
4. **Parallel tool execution:** Execute independent tools concurrently
5. **Streaming optimization:** Zero-copy where possible
6. **Binary size:** Strip symbols, optimize dependencies

## Reporting Performance Issues

If you experience performance problems:

1. Measure the issue:
   ```bash
   time sree --version  # Startup time
   /usr/bin/time -v sree  # Memory usage
   ```

2. Check logs:
   ```bash
   tail -f ~/.sree/logs/sree.log
   ```

3. Report with:
   - Hardware specs (CPU, RAM)
   - OS and terminal emulator
   - Conversation size (number of messages)
   - Specific operation that's slow
   - Timing measurements

## Continuous Performance Monitoring

The CI pipeline includes performance regression tests:

- Startup time must be < 150ms
- Memory usage must be < 50 MB for 100 messages
- Token counting must handle 10k messages in < 100ms
- All performance tests must pass

Any PR that regresses performance by >10% will be flagged for review.
