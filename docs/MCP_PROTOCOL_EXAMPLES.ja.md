# MCP Protocol Examples (Manual JSON-RPC)

MCP (Model Context Protocol) の実体は、標準入出力 (stdio) を介した **JSON-RPC 2.0** メッセージのやり取りです。
AI (Client) と `mcp-logger` (Server) が具体的にどのような文字列を交換しているかを以下に示します。

## 1. 初期化 (Handshake)

接続開始時、AI はサーバーの情報を要求します。

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
サーバーは自身の名前、バージョン、および機能（Capabilities）を返します。
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
ハンドシェイクの完了を通知します。これに対する応答はありません。
```json
{
  "jsonrpc": "2.0",
  "method": "notifications/initialized"
}
```

---

## 2. ツール一覧の取得 (Discovery)

AI は「何ができるか」を知るためにツール一覧を要求します。

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 2
}
```

### Response (Server -> Client)
サーバーは定義済みのツール（`send-syslog`）とその引数スキーマ（Input Schema）を返します。
AI はこのスキーマを見て、「どのような引数を渡せばよいか」を理解します。

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

## 3. ツールの実行 (Execution)

AI がツールを使うと判断した場合、具体的な引数を指定して実行リクエストを送ります。

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
サーバーは処理を実行（UDPパケット送信）し、結果をテキストとして返します。
`isError` が `false` なら成功、`true` なら失敗です。

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

## まとめ

MCP の実体はこれだけです。特別なバイナリプロトコルがあるわけではなく、**「標準入出力を通じて、1行ごとの JSON 文字列を投げ合っているだけ」** です。
したがって、`src/main.rs` でこの JSON をパースし、if 文や match 文で分岐して処理するだけで、立派な MCP サーバーとして機能します。
