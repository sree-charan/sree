# Troubleshooting Guide

This guide covers common issues and their solutions when using sree.

## Table of Contents

- [Installation Issues](#installation-issues)
- [Runtime Errors](#runtime-errors)
- [API Issues](#api-issues)
- [UI Problems](#ui-problems)
- [Performance Issues](#performance-issues)
- [Tool Execution Issues](#tool-execution-issues)
- [Configuration Issues](#configuration-issues)

---

## Installation Issues

### Cargo build fails with linking errors

**Symptom**: Compilation fails with linker errors or missing system libraries.

**Solution**:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential pkg-config libssl-dev

# Fedora/RHEL
sudo dnf install gcc openssl-devel

# macOS
xcode-select --install
```

### Slow compilation times

**Symptom**: `cargo build` takes a very long time.

**Solution**:
- Use `cargo build --release` only for final builds
- Enable parallel compilation: `export CARGO_BUILD_JOBS=4`
- Use `sccache` or `mold` linker for faster builds
- Consider using pre-built binaries when available

---

## Runtime Errors

### "Invalid API key" error

**Symptom**: Error message about invalid or missing API key.

**Solution**:
1. Set environment variable:
   ```bash
   export ANTHROPIC_API_KEY="your-key-here"
   ```

2. Or add to config file `~/.sree/config.toml`:
   ```toml
   [llm]
   api_key = "your-key-here"
   ```

3. Verify the key is correct at https://console.anthropic.com/

### Terminal display is corrupted

**Symptom**: UI elements are misaligned, colors are wrong, or characters are garbled.

**Solution**:
1. Check terminal compatibility:
   ```bash
   echo $TERM  # Should be xterm-256color or similar
   ```

2. Set correct TERM:
   ```bash
   export TERM=xterm-256color
   ```

3. Try a different terminal emulator (iTerm2, Alacritty, WezTerm)

4. Clear screen and restart:
   ```bash
   reset
   sree
   ```

### Application crashes on startup

**Symptom**: sree exits immediately with a panic or error.

**Solution**:
1. Check logs:
   ```bash
   cat ~/.sree/logs/sree.log
   ```

2. Remove corrupted config:
   ```bash
   mv ~/.sree/config.toml ~/.sree/config.toml.backup
   ```

3. Clear history:
   ```bash
   rm ~/.sree/history
   ```

4. Run with verbose logging:
   ```bash
   RUST_LOG=debug sree
   ```

---

## API Issues

### "Rate limit exceeded" error

**Symptom**: API returns 429 status code.

**Solution**:
- Wait for the retry (sree auto-retries with backoff)
- Reduce request frequency
- Check your API usage at https://console.anthropic.com/
- Consider upgrading your API plan

### Slow response times

**Symptom**: LLM takes a long time to respond.

**Solution**:
1. Check network connection:
   ```bash
   ping api.anthropic.com
   ```

2. Try a different model:
   ```
   /model claude-haiku-3-20250307
   ```

3. Reduce context size:
   ```
   /compact
   ```

4. Check Anthropic status: https://status.anthropic.com/

### Connection timeout errors

**Symptom**: "Connection timed out" or "Network error".

**Solution**:
1. Check internet connection
2. Verify firewall/proxy settings
3. Try with explicit proxy:
   ```bash
   export HTTPS_PROXY=http://proxy:port
   ```
4. Check if Anthropic API is accessible:
   ```bash
   curl -I https://api.anthropic.com/v1/messages
   ```

---

## UI Problems

### Text is cut off or wrapped incorrectly

**Symptom**: Messages don't fit the screen properly.

**Solution**:
1. Resize terminal window
2. Adjust word wrap setting in config:
   ```toml
   [ui]
   word_wrap = true
   ```
3. Use a larger terminal window (minimum 80x24 recommended)

### Colors look wrong

**Symptom**: Colors are too bright, too dark, or unreadable.

**Solution**:
1. Try a different theme:
   ```
   /theme light
   /theme dark
   /theme nord
   ```

2. Check terminal color support:
   ```bash
   tput colors  # Should output 256
   ```

3. Adjust terminal background color

### Scrolling doesn't work

**Symptom**: Can't scroll through chat history.

**Solution**:
- Use `PageUp`/`PageDown` keys
- Use mouse wheel (if terminal supports it)
- Use `Home`/`End` to jump to top/bottom
- Check if terminal has mouse support enabled

### Input field not responding

**Symptom**: Can't type in the input area.

**Solution**:
1. Press `Esc` to reset input mode
2. Press `Ctrl+C` to cancel current operation
3. Restart sree
4. Check if another process is capturing input

---

## Performance Issues

### High memory usage

**Symptom**: sree uses a lot of RAM.

**Solution**:
1. Clear conversation history:
   ```
   /clear
   ```

2. Compact conversation:
   ```
   /compact
   ```

3. Reduce max tokens in config:
   ```toml
   [llm]
   max_tokens = 4096
   ```

4. Limit tool output:
   ```toml
   [tools]
   max_output_lines = 100
   ```

### High CPU usage

**Symptom**: sree uses a lot of CPU even when idle.

**Solution**:
1. Check for runaway tool execution:
   ```
   Ctrl+C to cancel
   ```

2. Disable animations:
   ```toml
   [ui]
   show_spinner = false
   ```

3. Check logs for errors:
   ```bash
   tail -f ~/.sree/logs/sree.log
   ```

### Slow startup time

**Symptom**: sree takes a long time to start.

**Solution**:
1. Build in release mode:
   ```bash
   cargo build --release
   ```

2. Clear old logs:
   ```bash
   rm -rf ~/.sree/logs/*
   ```

3. Reduce history size:
   ```bash
   head -n 100 ~/.sree/history > ~/.sree/history.tmp
   mv ~/.sree/history.tmp ~/.sree/history
   ```

---

## Tool Execution Issues

### "Permission denied" errors

**Symptom**: Tools fail with permission errors.

**Solution**:
1. Check file permissions:
   ```bash
   ls -la <file>
   ```

2. Run from correct directory:
   ```bash
   cd /path/to/project
   sree
   ```

3. Verify user has necessary permissions

### Bash commands timeout

**Symptom**: Commands are killed after 30 seconds.

**Solution**:
1. Increase timeout in config:
   ```toml
   [tools]
   bash_timeout = 60
   ```

2. Use background jobs for long-running commands
3. Split into smaller commands

### File operations fail

**Symptom**: Can't read or write files.

**Solution**:
1. Check file exists:
   ```bash
   ls -la <file>
   ```

2. Verify path is correct (absolute or relative to cwd)
3. Check disk space:
   ```bash
   df -h
   ```

4. Verify file isn't locked by another process

### Grep/glob returns no results

**Symptom**: Search tools don't find expected files.

**Solution**:
1. Check `.gitignore` isn't excluding files
2. Verify pattern syntax:
   ```
   glob: **/*.rs (recursive)
   grep: use regex syntax
   ```
3. Check current directory:
   ```bash
   pwd
   ```

---

## Configuration Issues

### Config file not loaded

**Symptom**: Settings in config file are ignored.

**Solution**:
1. Verify file location:
   ```bash
   ls -la ~/.sree/config.toml
   ```

2. Check TOML syntax:
   ```bash
   cat ~/.sree/config.toml
   ```

3. Use explicit config path:
   ```bash
   sree --config /path/to/config.toml
   ```

### Invalid configuration error

**Symptom**: "Failed to parse config" error.

**Solution**:
1. Validate TOML syntax online: https://www.toml-lint.com/
2. Check for typos in keys
3. Verify value types match expected types
4. Start with minimal config and add settings incrementally

### Settings not persisting

**Symptom**: Changes made in sree don't persist.

**Solution**:
- Settings changed via slash commands are session-only
- Edit `~/.sree/config.toml` directly for persistent changes
- Restart sree after editing config file

---

## Getting More Help

If you're still experiencing issues:

1. **Check logs**: `~/.sree/logs/sree.log`
2. **Enable debug logging**: `RUST_LOG=debug sree`
3. **Search issues**: Check GitHub issues for similar problems
4. **Ask for help**: Open a GitHub issue with:
   - sree version (`sree --version`)
   - Operating system and terminal
   - Steps to reproduce
   - Relevant log excerpts
   - Config file (redact API key)

## Common Error Messages

| Error | Meaning | Solution |
|-------|---------|----------|
| "Connection refused" | Can't reach API | Check network/firewall |
| "Invalid JSON" | Malformed API response | Check API status |
| "Context length exceeded" | Too many tokens | Use `/compact` or `/clear` |
| "Tool not found" | Unknown tool name | Check tool registry |
| "Timeout" | Operation took too long | Increase timeout in config |
| "Permission denied" | Insufficient permissions | Check file/directory permissions |
| "No such file" | File doesn't exist | Verify path is correct |
| "Invalid regex" | Bad regex pattern | Check regex syntax |

## Performance Tuning

For optimal performance:

```toml
[llm]
max_tokens = 4096  # Reduce for faster responses

[tools]
bash_timeout = 30
max_file_size = 524288  # 512KB
max_output_lines = 200

[ui]
show_token_count = false  # Slight performance gain
auto_scroll = true
```

## Debug Mode

Run with full debug output:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 sree 2>&1 | tee debug.log
```

This captures all logs and backtraces for troubleshooting.
