# Примеры протокола MCP (JSON-RPC)

Суть MCP (Model Context Protocol) заключается в обмене сообщениями **JSON-RPC 2.0** через стандартный ввод/вывод (stdio).
Вот примеры сообщений, которыми на самом деле обмениваются ИИ (клиент) и `mcp-logger` (сервер).

## 1. Инициализация (Handshake)

При установлении соединения ИИ запрашивает информацию о сервере.

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
Сервер отвечает своим именем, версией и возможностями (Capabilities).
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

---

## 2. Вызов инструмента (Call send-syslog)

Когда ИИ решает отправить лог.

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "send-syslog",
    "arguments": {
      "server": "192.168.1.100",
      "message": "Тест отправки сообщения на русском языке",
      "severity": "info",
      "encoding": "utf-8"
    }
  },
  "id": 2
}
```

### Response (Server -> Client)
Сервер попытается отправить UDP-пакет и сообщит о результате.
```json
{
  "jsonrpc": "2.0",
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Successfully sent Syslog to 192.168.1.100:514"
      }
    ]
  },
  "id": 2
}
```

---
Назад к [ README.ru.md](../../README.ru.md) | [MANUAL.ru.md](MANUAL.ru.md)
