# MCP 协议示例 (手动 JSON-RPC)

MCP (Model Context Protocol) 的核心其实是通过标准输入/输出 (stdio) 进行的 **JSON-RPC 2.0** 消息交换。
下面展示了 AI (Client) 与 `mcp-logger` (Server) 之间交换的具体字符串。

## 1. 握手 (初始化)

连接开始时，AI 会请求服务器信息。

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
服务器返回其名称、版本和功能 (Capabilities)。
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
通知握手已完成。该消息没有响应。
```json
{
  "jsonrpc": "2.0",
  "method": "notifications/initialized"
}
```

---

## 2. 获取工具列表 (发现)

AI 请求工具列表以了解“它可以做什么”。

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 2
}
```

### Response (Server -> Client)
服务器返回定义的工具 (`send-syslog`) 及其参数模式 (Input Schema)。
AI 根据该模式理解“应该传递哪些参数”。

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

## 3. 执行工具 (调用)

如果 AI 决定使用某个工具，它会发送带有具体参数的执行请求。

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "send-syslog",
    "arguments": {
      "server": "192.168.1.100",
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
服务器执行处理（发送 UDP 数据包）并以文本形式返回结果。
`isError` 为 `false` 表示成功，`true` 表示失败。

```json
{
  "jsonrpc": "2.0",
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Success: Sent to 192.168.1.100:514 (Encoding: UTF-8)"
      }
    ],
    "isError": false
  },
  "id": 3
}
```

---

## 总结

MCP 的本质就这么多。没有特殊的二进制协议，它只是**“通过标准 I/O 来回发送单行 JSON 字符串”**。
因此，只需在 `src/main.rs` 中解析该 JSON，并通过 `if` 或 `match` 语句进行分支处理，它就能完美地作为 MCP 服务器运行。
