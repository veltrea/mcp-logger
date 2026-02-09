# MCP प्रोटोकॉल उदाहरण (JSON-RPC)

MCP (मॉडल संदर्भ प्रोटोकॉल) का मुख्य हिस्सा मानक इनपुट/आउटपुट (stdio) के माध्यम से **JSON-RPC 2.0** संदेशों का आदान-प्रदान है।
यहाँ उन संदेशों के उदाहरण दिए गए हैं जिन्हें AI (क्लाइंट) और `mcp-logger` (सर्वर) वास्तव में साझा करते हैं।

## 1. आरंभीकरण (Handshake)

कनेक्शन शुरू करते समय, AI सर्वर की जानकारी का अनुरोध करता है।

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
सर्वर अपने नाम, संस्करण और क्षमताओं (Capabilities) के साथ प्रतिक्रिया देता है।
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

## 2. टूल कॉल (Call send-syslog)

जब AI लॉग भेजने का निर्णय लेता है।

### Request (Client -> Server)
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "send-syslog",
    "arguments": {
      "server": "192.168.1.100",
      "message": "हिंदी संदेश भेजने का परीक्षण",
      "severity": "info",
      "encoding": "utf-8"
    }
  },
  "id": 2
}
```

### Response (Server -> Client)
सर्वर UDP पैकेट भेजने का प्रयास करेगा और परिणाम की रिपोर्ट करेगा।
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
वापस [ README.hi.md](../../README.hi.md) | [MANUAL.hi.md](MANUAL.hi.md)
