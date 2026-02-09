# MCP 協定範例 (手動 JSON-RPC)

MCP (Model Context Protocol) 的核心其實是透過標準輸入/輸出 (stdio) 進行的 **JSON-RPC 2.0** 訊息交換。
下面展示了 AI (Client) 與 `mcp-logger` (Server) 之間交換的具體字串。

## 1. 握手 (初始化)

連線開始時，AI 會請求伺服器資訊。

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
伺服器回傳其名稱、版本和功能 (Capabilities)。
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
通知握手已完成。該訊息沒有回應。
```json
{
  "jsonrpc": "2.0",
  "method": "notifications/initialized"
}
```

---

## 2. 取得工具列表 (發現)

AI 請求工具列表以了解「它可以做什麼」。

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 2
}
```

### Response (Server -> Client)
伺服器回傳定義的工具 (`send-syslog`) 及其參數模式 (Input Schema)。
AI 根據該模式理解「應該傳遞哪些參數」。

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

## 3. 執行工具 (呼叫)

如果 AI 決定使用某個工具，它會發送帶有具體參數的執行請求。

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
伺服器執行處理（發送 UDP 封包）並以文字形式回傳結果。
`isError` 為 `false` 表示成功，`true` 表示失敗。

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

## 總結

MCP 的本質就這麼多。沒有特殊的二進位協定，它只是**「透過標準 I/O 來回發送單行 JSON 字串」**。
因此，只需在 `src/main.rs` 中解析該 JSON，並透過 `if` 或 `match` 語句進行分支處理，它就能完美地作為 MCP 伺服器運作。
