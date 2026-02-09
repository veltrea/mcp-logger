# LM Studio Setup Guide for mcp-logger

This is a setup guide for using `mcp-logger` with LM Studio.
This project has been modified to fully comply with the "4 local rules" specific to LM Studio.

## Compliance with LM Studio Rules

1.  **Installation Path**: While other clients (e.g., Google Antigravity) work with any absolute path, LM Studio is confirmed to recognize the extension correctly when placed in `~/.lmstudio/extensions/plugins/mcp/mcp-logger/` (or `C:\Users\<USERNAME>\.lmstudio\extensions\plugins\mcp\mcp-logger\` for Windows).
2.  **Protocol Version**: It is implemented to return the version specified by the client (LM Studio) exactly as requested (e.g., `2024-11-05`).
3.  **JSON-RPC Format**: It is implemented to exclude optional fields (like `error` when it is null) from the response.
4.  **Tool Response**: Responds in a format that includes the `content` array and the `isError` flag.

## Setup Steps

Add the following to your LM Studio MCP configuration file (usually `~/.lmstudio/config.json` or `mcp.json`, though it can often be set via the GUI).

### macOS / Linux (Ubuntu)

```json
"mcp-logger": {
  "command": "/Users/<USERNAME>/.local/bin/mcp-logger",
  "args": []
}
```
*Note: Please specify the actual location of your built binary for the path and username.*

### Windows (Remote-Server, etc.)

```json
"mcp-logger": {
  "command": "C:\\Users\\<USERNAME>\\AppData\\Local\\Programs\\mcp-logger\\mcp-logger.exe",
  "args": []
}
```

## Verification

1.  Launch LM Studio and check if the server connection (Initialize) is successful (you should see `Connected to mcp-logger` or similar in the logs).
2.  Instruct the AI to "Send a Syslog" in the chat and check if the tool is recognized.
