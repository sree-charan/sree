# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of sree seriously. If you discover a security vulnerability, please follow these steps:

### 1. Do Not Disclose Publicly

Please do not open a public GitHub issue for security vulnerabilities. This helps protect users while we work on a fix.

### 2. Report Privately

Send an email to the maintainers with:
- A description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Any suggested fixes (optional)

### 3. Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 1-7 days
  - High: 7-14 days
  - Medium: 14-30 days
  - Low: 30-90 days

## Security Considerations

### API Keys

- Never commit API keys to version control
- Store keys in environment variables or config files outside the repository
- The config file at `~/.sree/config.toml` should have restricted permissions (600)
- sree will warn if config file permissions are too open

### Command Execution

- The `bash` tool executes shell commands with your user privileges
- Review tool calls before confirming execution in interactive mode
- Use `--no-tools` flag to disable tool execution if you only want chat
- Commands have a 30-second timeout by default (configurable)

### File Operations

- File read/write operations respect your filesystem permissions
- sree will not follow symlinks outside the current directory tree by default
- The `.gitignore` file is respected to avoid exposing sensitive files
- Large files (>1MB by default) are truncated to prevent memory issues

### Network Requests

- All API requests go to `api.anthropic.com` over HTTPS
- No telemetry or analytics are collected
- No data is sent to third parties except Anthropic's API
- Web search tool is a stub and requires explicit configuration

### Logging

- Logs are stored in `~/.sree/logs/` with restricted permissions
- Logs may contain conversation history and tool outputs
- API keys are redacted from logs
- Consider log rotation and cleanup for sensitive projects

## Best Practices

1. **Keep sree Updated**: Run `cargo install --git <repo>` regularly for security patches
2. **Review Tool Calls**: Always review what tools the LLM wants to execute
3. **Limit Scope**: Run sree in project directories, not system directories
4. **Use .gitignore**: Ensure sensitive files are in `.gitignore`
5. **Secure Config**: Set `chmod 600 ~/.sree/config.toml`
6. **Environment Variables**: Prefer `ANTHROPIC_API_KEY` env var over config file
7. **Audit Logs**: Periodically review `~/.sree/logs/` for unexpected activity

## Known Security Limitations

- **No Sandboxing**: Tool execution runs with your user privileges
- **No Input Validation**: LLM-generated commands are executed as-is
- **No Rate Limiting**: No built-in protection against API abuse
- **No Encryption**: Config and logs are stored in plaintext
- **No Audit Trail**: Limited tracking of what commands were executed

## Threat Model

### In Scope

- Vulnerabilities in sree's code
- Insecure defaults or configurations
- Information disclosure through logs or errors
- Command injection through tool parameters
- Path traversal in file operations

### Out of Scope

- Vulnerabilities in dependencies (report to upstream)
- Social engineering attacks
- Physical access to the machine
- Anthropic API security (report to Anthropic)
- User misconfiguration (covered in documentation)

## Security Roadmap

Future versions may include:

- [ ] Config file encryption
- [ ] Command approval mode (confirm before execution)
- [ ] Sandboxed tool execution
- [ ] Audit log with cryptographic signatures
- [ ] Rate limiting and quota management
- [ ] Integration with system keychains for API keys
- [ ] File operation allowlist/denylist
- [ ] Network request filtering

## Acknowledgments

We appreciate security researchers who responsibly disclose vulnerabilities. Contributors will be acknowledged in release notes (unless they prefer to remain anonymous).
