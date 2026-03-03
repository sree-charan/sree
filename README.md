# sree

Terminal-based AI assistant built in Rust. Uses Claude on AWS Bedrock.

## Features

- TUI built with ratatui
- Streaming responses from Claude (Sonnet 4.6, Opus 4.6, Haiku 4.5)
- 6 built-in tools: file read/write, bash, grep, glob, web search
- Markdown rendering with syntax highlighting
- 6 color themes
- Input history, conversation export

## Install

### Homebrew (macOS)

```bash
brew tap sree-charan/tap
brew install sree
```

### From source

```bash
git clone https://github.com/sree-charan/sree.git
cd sree
cargo build --release
cp target/release/sree ~/.local/bin/
```

### From releases

Download the binary for your platform from [Releases](https://github.com/sree-charan/sree/releases).

## Prerequisites

- AWS account with Bedrock access (Claude models enabled)
- AWS credentials configured (`aws configure` or env vars)

## Usage

```bash
sree                          # start with defaults
sree --model claude-opus-4-6  # use a specific model
sree --no-tools               # chat only, no tools
sree --system "You are a Rust expert"
```

### Keybindings

| Key | Action |
|-----|--------|
| Enter | Send message |
| Ctrl+C | Cancel / Quit |
| Up/Down | Input history |
| PageUp/Down | Scroll chat |
| Mouse wheel | Scroll chat |

### Commands

`/help` `/quit` `/clear` `/model` `/theme` `/export` `/tokens` `/compact`

## Config

`~/.sree/config.toml`:

```toml
[llm]
aws_region = "us-east-1"
model = "anthropic.claude-sonnet-4-6"
max_tokens = 8192

[ui]
theme = "dark"  # dark, light, monokai, dracula, nord, solarized
```

## License

MIT
