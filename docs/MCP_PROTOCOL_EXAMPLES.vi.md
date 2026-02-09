# Ví dụ về giao thức MCP (JSON-RPC thủ công)

Thực chất của MCP (Model Context Protocol) là việc trao đổi các thông báo **JSON-RPC 2.0** thông qua đầu vào/đầu ra tiêu chuẩn (stdio).
Dưới đây là ví dụ về các chuỗi ký tự thực tế được trao đổi giữa AI (Client) và `mcp-logger` (Server).

## 1. Bắt tay (Khởi tạo)

Khi kết nối bắt đầu, AI yêu cầu thông tin của máy chủ.

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
Máy chủ trả về tên, phiên bản và các khả năng (Capabilities) của nó.
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
Thông báo rằng việc bắt tay đã hoàn tất. Không có phản hồi cho thông báo này.
```json
{
  "jsonrpc": "2.0",
  "method": "notifications/initialized"
}
```

---

## 2. Lấy danh sách công cụ (Khám phá)

AI yêu cầu danh sách các công cụ để biết "nó có thể làm gì".

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 2
}
```

### Response (Server -> Client)
Máy chủ trả về các công cụ đã định nghĩa (`send-syslog`) và giản đồ đối số (Input Schema) của chúng.
AI nhìn vào giản đồ này để hiểu "cần truyền những đối số nào".

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

## 3. Thực thi công cụ (Gọi)

Nếu AI quyết định sử dụng một công cụ, nó sẽ gửi yêu cầu thực thi với các đối số cụ thể.

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
Máy chủ thực thi quy trình (gửi gói tin UDP) và trả về kết quả dưới dạng văn bản.
`isError` là `false` nếu thành công và `true` nếu thất bại.

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

## Tóm tắt

Bản chất của MCP chỉ có vậy. Không có giao thức nhị phân đặc biệt nào cả; nó chỉ đơn giản là **"ném các chuỗi JSON một dòng qua lại thông qua I/O tiêu chuẩn"**.
Do đó, bằng cách phân tách JSON này và xử lý phân nhánh bằng các câu lệnh `if` hoặc `match` trong `src/main.rs`, nó sẽ hoạt động hoàn hảo như một máy chủ MCP.
