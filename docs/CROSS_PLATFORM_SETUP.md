# Google Antigravity (MCP) Setup Guide

This document explains the setup procedures for using `mcp-logger` with Google Antigravity (or other MCP clients) by platform.

## Overview

`mcp-logger` is a standalone binary that communicates via JSON-RPC over standard input/output (stdio).
It integrates with clients like Antigravity by adding an entry to the configuration file (`mcp_config.json`) that tells the client "run this command".

---

## 1. macOS (Currently Configured)

Automatically configured. The following settings have been added to `~/.gemini/antigravity/mcp_config.json`.

```json
"mcp-logger": {
  "command": "cargo",
  "args": [
    "run",
    "--release",
    "--quiet",
    "--manifest-path",
    "[LOCAL-PATH]/mcp-logger/Cargo.toml"
  ]
}
```

*Note: The first execution will trigger a build; subsequent launches will be fast.*

---

## 2. Windows 11 (Remote-Server / Local-Server)

Instructions for using Antigravity (or Claude Desktop, etc.) in a Windows environment.

### Prerequisites
1. Install Rust (`rustup-init.exe`)
2. Place the source code in the `rust-logger` folder (if using Google Drive sync, you can use it as is).

### `mcp_config.json` Settings
Configure according to the Windows path format.

```json
{
  "mcpServers": {
    "mcp-logger": {
      "command": "cargo.exe",
      "args": [
        "run",
        "--release",
        "--quiet",
        "--manifest-path",
        "C:\\[LOCAL-PATH]\\[mcp-logger]\\Cargo.toml"
      ]
    }
  }
}
```
*Note: Please change the path (`C:\\Users\\...`) to match your actual deployment location.*

---

## 3. Ubuntu (Dev Server)

Instructions for running Antigravity on Ubuntu.

### Prerequisites
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential
```

### `mcp_config.json` Settings

```json
{
  "mcpServers": {
    "mcp-logger": {
      "command": "cargo",
      "args": [
        "run",
        "--release",
        "--quiet",
        "--manifest-path",
        "/home/[USER]/dev/rust-logger/mcp-logger/Cargo.toml"
      ]
    }
  }
}
```

---

## Troubleshooting

- **Build takes too long**: The first time involves compiling dependency libraries, which can take several dozen seconds.
- **JsonRpc Error**: If extra logs are mixed into the `cargo run` output, an error will occur. The `--quiet` option is mandatory. Furthermore, the tool is implemented to use `eprintln!` (standard error output) for debugging instead of `println!`.
