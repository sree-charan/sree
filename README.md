# sree

Terminal-based AI assistant built in Rust. Uses Claude on AWS Bedrock.

## Features

- TUI built with [ratatui](https://github.com/ratatui-org/ratatui)
- Streaming responses from Claude (Sonnet 4.6, Opus 4.6, Haiku 4.5)
- 6 built-in tools: file read/write, bash, grep, glob, web search
- Markdown rendering with syntax highlighting
- 6 color themes (dark, light, monokai, dracula, nord, solarized)
- Input history, conversation export

## Install

### Homebrew (macOS / Linux)

```bash
brew tap sree-charan/tools
brew install sree
```

### Download binary

Grab the latest from [Releases](https://github.com/sree-charan/sree/releases/latest):

| Platform | Download |
|----------|----------|
| macOS (Apple Silicon) | [sree-aarch64-apple-darwin.tar.gz](https://github.com/sree-charan/sree/releases/latest/download/sree-aarch64-apple-darwin.tar.gz) |
| Linux (x86_64) | [sree-x86_64-unknown-linux-gnu.tar.gz](https://github.com/sree-charan/sree/releases/latest/download/sree-x86_64-unknown-linux-gnu.tar.gz) |
| Windows (x86_64) | [sree-x86_64-pc-windows-msvc.zip](https://github.com/sree-charan/sree/releases/latest/download/sree-x86_64-pc-windows-msvc.zip) |

Then move the binary to your PATH:

```bash
# macOS / Linux
tar xzf sree-*.tar.gz
sudo mv sree /usr/local/bin/

# Windows — extract the zip and add sree.exe to your PATH
```

### From source

```bash
git clone https://github.com/sree-charan/sree.git
cd sree
cargo build --release
cp target/release/sree ~/.local/bin/
```

## Prerequisites

- AWS account with Bedrock access (Claude models enabled in your region)
- AWS credentials configured:
  ```bash
  aws configure
  # or set AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_REGION
  ```

## Usage

```bash
sree                                    # start with defaults
sree --model anthropic.claude-opus-4-6  # specific model
sree --no-tools                         # chat only
sree --system "You are a Rust expert"   # custom system prompt
```

### Keybindings

| Key | Action |
|-----|--------|
| Enter | Send message |
| Ctrl+C | Cancel / Quit |
| Up/Down | Input history |
| PageUp/Down | Scroll |
| Mouse wheel | Scroll |

### Commands

| Command | Description |
|---------|-------------|
| `/help` | Show commands |
| `/quit` | Exit |
| `/clear` | Clear chat |
| `/model [name]` | Show/switch model |
| `/theme [name]` | Switch theme |
| `/export [file]` | Export to markdown |
| `/tokens` | Show token usage |
| `/compact` | Compact history |

## Config

`~/.sree/config.toml`:

```toml
[llm]
aws_region = "us-east-1"
model = "anthropic.claude-sonnet-4-6"
max_tokens = 8192

[ui]
theme = "dark"
```

## Tools

The AI can use these tools during conversation:

| Tool | Description |
|------|-------------|
| `file_read` | Read files, list directories, search in files |
| `file_write` | Create, replace, insert, append to files |
| `bash` | Execute shell commands |
| `grep` | Regex search across files |
| `glob` | Find files by pattern |
| `web_search` | Web search (requires API config) |

## Development

```bash
cargo build          # debug build
cargo build --release # release build
cargo test           # run tests
cargo clippy         # lint
```

## License

MIT
