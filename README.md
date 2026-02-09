# MCP Logger ([English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md))

## Overview
This project provides an MCP (Model Context Protocol) server that enables AI assistants (such as Claude, Gemini, etc.) to utilize Syslog infrastructure for logging and monitoring.
It is not a "Syslog Server" itself, but a "Client Capability" for AI to send logs to an existing Syslog server (e.g., `rsyslog`, `syslog-ng`, or `vlt-syslogd`).

It shares core logic with `rust-logger` (a CLI logger for humans) while providing an interface optimized for AI interaction.

## Key Features
- **Send Syslog Messages**: AI can specify server, port, facility, severity, tag, encoding, and message to send Syslog.
- **2-Byte Character Support (UTF-8 with BOM)**: Specifically designed to handle non-ASCII characters (Japanese, Korean, Chinese, etc.) correctly for AI analysis.
- **CLI-MCP Dual Design**: Works as both a standalone CLI tool for humans and an MCP server for AIs.

## Installation
Requires Rust environment.

```bash
cargo build --release
```

## Usage (CLI / MCP Dual Mode)

This tool combines "CLI for Humans" and "MCP Server for AI".

### 1. Human CLI Mode
You can send logs directly from your terminal.

```bash
# Basic send
mcp-logger send 192.168.1.100 "Hello from CLI"

# With options
mcp-logger send 192.168.1.100 "Error occurred" --severity error --tag my-app
```

Running `mcp-logger` without arguments in a terminal will display the **MCP Setup Guide**.

### 2. AI Assistant Mode (MCP)
Add the following to your `claude_desktop_config.json` or equivalent.

```json
{
  "mcpServers": {
    "mcp-logger": {
      "command": "/path/to/mcp-logger",
      "args": ["run-mcp"]
    }
  }
}
```

## MCP Tools
Features provided to the AI.

### `send-syslog`
Sends a Syslog message to a specified server.

**Arguments:**
- `server` (string, required): Hostname or IP of the Syslog server.
- `port` (number): Port number (default: `514`).
- `facility` (string): Facility (e.g., `user`, `local0`). Default is `user`.
- `severity` (string): Severity (e.g., `info`, `error`). Default is `info`.
- `tag` (string): Program name (tag). Default is `mcp-syslog`.
- `encoding` (string): Encoding (e.g., `utf-8`, `shift_jis`). Default is `utf-8`.
- `message` (string, required): The log message body.

## Important: Multi-Byte Character Handling
For languages using 2-byte characters (Japanese, Chinese, Korean, etc.), the AI is instructed to use `utf-8`. The server automatically attaches a **UTF-8 BOM** to ensure the Syslog server and subsequent analysis tools correctly identify the encoding.

## Documentation
- [Detailed Manual](docs/MANUAL.md)
- [Cross-Platform Setup Guide](docs/CROSS_PLATFORM_SETUP.md)
- [LM Studio Setup Guide](docs/LM_STUDIO_SETUP.md)
- [MCP Protocol Examples](docs/MCP_PROTOCOL_EXAMPLES.md)
- [Specification](docs/SPEC.md)
