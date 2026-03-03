# Configuration Examples

This directory contains sample configuration files for different use cases.

## Files

- `minimal.toml` - Minimal configuration with just API key
- `developer.toml` - Configuration for development work
- `power_user.toml` - Advanced configuration with all options
- `light_theme.toml` - Configuration for light terminal backgrounds
- `performance.toml` - Optimized for performance and token efficiency

## Usage

Copy any example to `~/.sree/config.toml`:

```bash
cp examples/config_examples/developer.toml ~/.sree/config.toml
```

Then edit to set your API key and preferences.

## Environment Variables

You can also set configuration via environment variables:

```bash
export ANTHROPIC_API_KEY=your_key_here
export SREE_MODEL=claude-sonnet-4-20250514
export SREE_THEME=dark
```

Environment variables take precedence over config file settings.
