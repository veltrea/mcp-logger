# أمثلة بروتوكول MCP (JSON-RPC)

جوهر MCP (بروتوكول سياق النموذج) هو تبادل رسائل **JSON-RPC 2.0** عبر الإدخال/الإخراج القياسي (stdio).
إليك أمثلة على الرسائل التي يتبادلها الذكاء الاصطناعي (العميل) و `mcp-logger` (الخادم) فعلياً.

## 1. التهيئة (Handshake)

عند بدء الاتصال، يطلب الذكاء الاصطناعي معلومات الخادم.

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
يستجيب الخادم باسمه وإصداره وقدراته (Capabilities).
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

## 2. استدعاء الأدوات (Call send-syslog)

عندما يقرر الذكاء الاصطناعي إرسال سجل.

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "send-syslog",
    "arguments": {
      "server": "192.168.1.100",
      "message": "اختبار إرسال رسالة بالعربية",
      "severity": "info",
      "encoding": "utf-8"
    }
  },
  "id": 2
}
```

### Response (Server -> Client)
سيعيد الخادم محاولة إرسال حزمة UDP ويبلغ عن النتيجة.
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
العودة إلى [ README.ar.md](../../README.ar.md) | [MANUAL.ar.md](MANUAL.ar.md)
