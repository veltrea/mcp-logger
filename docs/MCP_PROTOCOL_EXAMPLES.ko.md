# MCP 프로토콜 예제 (수동 JSON-RPC)

MCP (Model Context Protocol)의 실체는 표준 입출력 (stdio)을 통한 **JSON-RPC 2.0** 메시지 교환입니다.
AI (Client)와 `mcp-logger` (Server)가 구체적으로 어떤 문자열을 주고받는지 아래에 보여줍니다.

## 1. 핸드셰이크 (초기화)

연결 시작 시 AI는 서버 정보를 요청합니다.

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
서버는 자신의 이름, 버전 및 기능(Capabilities)을 반환합니다.
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
핸드셰이크 완료를 알립니다. 이에 대한 응답은 없습니다.
```json
{
  "jsonrpc": "2.0",
  "method": "notifications/initialized"
}
```

---

## 2. 도구 목록 가져오기 (Discovery)

AI는 "무엇을 할 수 있는지" 알기 위해 도구 목록을 요청합니다.

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 2
}
```

### Response (Server -> Client)
서버는 정의된 도구 (`send-syslog`)와 인자 스키마 (Input Schema)를 반환합니다.
AI는 이 스키마를 보고 "어떤 인자를 전달해야 하는지" 이해합니다.

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

## 3. 도구 실행 (Execution)

AI가 도구를 사용하기로 결정하면 구체적인 인자를 지정하여 실행 요청을 보냅니다.

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
서버는 프로세스를 실행 (UDP 패킷 전송)하고 결과를 텍스트로 반환합니다.
`isError`가 `false`이면 성공, `true`이면 실패입니다.

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

## 요약

MCP의 실체는 이것이 전부입니다. 특별한 바이너리 프로토콜이 있는 것이 아니라, **"표준 입출력을 통해 한 줄의 JSON 문자열을 주고받는 것뿐"**입니다.
따라서 `src/main.rs`에서 이 JSON을 파싱하고 `if`나 `match` 문으로 분기하여 처리하기만 하면 훌륭한 MCP 서버로 작동합니다.
