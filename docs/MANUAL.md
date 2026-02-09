# MCP Logger - Detailed Manual

## 1. Introduction
`mcp-logger` is an MCP server designed to bridge AI assistants and Syslog infrastructure. It allows AI models to utilize Syslog for logging and state tracking.

## 2. Configuration

### Claude Desktop
Add to `%APPDATA%\Claude\claude_desktop_config.json` (Windows) or `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS).

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

## 3. Tool Reference

### `send-syslog`
Sends a message to a Syslog server.

| Parameter | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `server` | string | (Required) | Destination server IP or hostname. |
| `port` | number | 514 | UDP port number. |
| `facility` | string | user | Syslog facility (kern, user, mail, etc.) |
| `severity` | string | info | Syslog severity (emerg, alert, crit, err, warning, notice, info, debug) |
| `tag` | string | mcp-syslog | Program name/Tag. |
| `encoding` | string | utf-8 | Character encoding. |
| `message` | string | (Required) | The log content. |

## 4. Multi-Byte Character Guide (2-Byte Countries)
If you are in a country using 2-byte characters (Japan, Korea, China, Vietnam, etc.), please follow these guidelines:

### Recommendation: Use UTF-8
`mcp-logger` is optimized for UTF-8. It automatically detects if the message contains non-ASCII characters and provides a **UTF-8 BOM (Byte Order Mark)** if needed.

### Interaction with Syslog Servers
Many Windows-based Syslog servers (like `vlt-syslogd`) or modern Linux `rsyslog` setups can handle UTF-8 if properly identified. The BOM ensures that these tools do not treat the log as local legacy encoding (like CP932 or EUC-KR).

## 5. Troubleshooting
- **AI not recognizing the tool**: Ensure the path in the config file is absolute.
- **Logs not arriving**: Check if UDP port 514 is open on the target server's firewall.
- **Character corruption**: Ensure the `encoding` parameter is set to `utf-8`. AI models generally prefer UTF-8.

## 6. Supplementary Guides
- [Cross-Platform Setup Guide](CROSS_PLATFORM_SETUP.md)
- [LM Studio Setup Guide](LM_STUDIO_SETUP.md)
- [MCP Protocol Examples](MCP_PROTOCOL_EXAMPLES.md)
- [Specification](SPEC.md)
