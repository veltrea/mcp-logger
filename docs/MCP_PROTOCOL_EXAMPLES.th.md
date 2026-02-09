# ตัวอย่างโปรโตคอล MCP (JSON-RPC)

หัวใจสำคัญของ MCP (Model Context Protocol) คือการแลกเปลี่ยนข้อความ **JSON-RPC 2.0** ผ่านทางอินพุต/เอาต์พุตมาตรฐาน (stdio)
นี่คือตัวอย่างของข้อความที่ AI (ไคลเอนต์) และ `mcp-logger` (เซิร์ฟเวอร์) แลกเปลี่ยนกันจริง

## 1. การเริ่มต้น (Handshake)

เมื่อเริ่มการเชื่อมต่อ AI จะขอข้อมูลของเซิร์ฟเวอร์

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
เซิร์ฟเวอร์จะตอบกลับด้วยชื่อ, เวอร์ชัน และความสามารถ (Capabilities) ของตน
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

## 2. การเรียกใช้เครื่องมือ (Call send-syslog)

เมื่อ AI ตัดสินใจส่งบันทึก

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "send-syslog",
    "arguments": {
      "server": "192.168.1.40",
      "message": "ทดสอบการส่งข้อความภาษาไทย",
      "severity": "info",
      "encoding": "utf-8"
    }
  },
  "id": 2
}
```

### Response (Server -> Client)
เซิร์ฟเวอร์จะพยายามส่งแพ็กเกจ UDP และรายงานผลลัพธ์
```json
{
  "jsonrpc": "2.0",
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Successfully sent Syslog to 192.168.1.40:514"
      }
    ]
  },
  "id": 2
}
```

---
กลับไปที่ [ README.th.md](../../README.th.md) | [MANUAL.th.md](MANUAL.th.md)
