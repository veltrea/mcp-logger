# MCP Protocol Examples (Manual JSON-RPC)

The Model Context Protocol (MCP) essentially exchanges **JSON-RPC 2.0** messages via standard input/output (stdio).
Below are examples of the actual strings exchanged between the AI (Client) and `mcp-logger` (Server).

## 1. Handshake (Initialization)

When the connection starts, the AI requests server information.

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "initialize",
  "params": {
    "protocolVersion": "2024-11-05",
    "capabilities": {},
    "clientInfo": {
      "name": "Claude",
      "version": "3.5-sonnet"
    }
  },
  "id": 1
}
```

### Response (Server -> Client)
The server returns its name, version, and capabilities.
```json
{
  "jsonrpc": "2.0",
  "result": {
    "protocolVersion": "2024-11-05",
    "capabilities": {
      "tools": {}
    },
    "serverInfo": {
      "name": "mcp-logger",
      "version": "0.1.0"
    }
  },
  "id": 1
}
```

### Notification (Client -> Server)
Notifies that the handshake is complete. There is no response to this.
```json
{
  "jsonrpc": "2.0",
  "method": "notifications/initialized"
}
```

---

## 2. Discovery (List Tools)

The AI requests a list of tools to know "what it can do".

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 2
}
```

### Response (Server -> Client)
The server returns defined tools (`send-syslog`) and their argument schema (Input Schema).
The AI looks at this schema to understand "what arguments to pass".

```json
{
  "jsonrpc": "2.0",
  "result": {
    "tools": [
      {
        "name": "send-syslog",
        "description": "Send a standard Syslog message...",
        "inputSchema": {
          "type": "object",
          "properties": {
            "server": { "type": "string", "description": "IP address..." },
            "port": { "type": "integer", "default": 514 },
            "message": { "type": "string" },
            "severity": { "type": "string", "default": "info" },
            "facility": { "type": "string", "default": "user" }
          },
          "required": ["server", "message"]
        }
      }
    ]
  },
  "id": 2
}
```

---

## 3. Execution (Call Tool)

If the AI decides to use a tool, it sends an execution request with specific arguments.

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "send-syslog",
    "arguments": {
      "server": "192.168.1.40",
      "port": 514,
      "facility": "local0",
      "severity": "error",
      "message": "Deployment failed due to timeout"
    }
  },
  "id": 3
}
```

### Response (Server -> Client)
The server executes the process (sends UDP packet) and returns the result as text.
`isError` is `false` for success and `true` for failure.

```json
{
  "jsonrpc": "2.0",
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Success: Sent to 192.168.1.40:514 (Encoding: UTF-8)"
      }
    ],
    "isError": false
  },
  "id": 3
}
```

---

## Summary

This is all there is to MCP. There is no special binary protocol; it's simply **"throwing one-line JSON strings back and forth through standard I/O"**.
Therefore, by parsing this JSON and branching the process with `if` or `match` statements in `src/main.rs`, it functions perfectly as an MCP server.
