# MCP Logger - 詳細手冊

## 1. 簡介
`mcp-logger` 是一個 MCP (Model Context Protocol) 伺服器，旨在連接 AI 助手和 Syslog 基礎設施。它允許 AI 利用 Syslog 進行日誌記錄和狀態追蹤。

## 2. 配置方法

### Claude Desktop
將其添加到 `%APPDATA%\Claude\claude_desktop_config.json` (Windows) 或 `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) 中。

```json
{
  "mcpServers": {
    "mcp-logger": {
      "command": "/path/to/mcp-logger",
      "args": ["run-mcp"]
    }
  }
}
```

## 3. 工具參考 (Tool Reference)

### `send-syslog`
向 Syslog 伺服器發送訊息。

| 參數 | 類型 | 預設值 | 說明 |
| :--- | :--- | :--- | :--- |
| `server` | string | (必填) | 目標伺服器 IP 或主機名。 |
| `port` | number | 514 | UDP 連接埠號。 |
| `facility` | string | user | Syslog 設施 (kern, user, mail 等) |
| `severity` | string | info | Syslog 嚴重性 (emerg, alert, crit, err, warning, notice, info, debug) |
| `tag` | string | mcp-syslog | 程式名稱/標籤。 |
| `encoding` | string | utf-8 | 字元編碼。 |
| `message` | string | (必填) | 日誌內容。 |

## 4. 多位元組字元指引 (針對使用雙位元組字元的用戶)
如果您在中文、日文、韓文、越南文等使用雙位元組字元的國家/地區，請遵循以下準則：

### 建議：使用 UTF-8
`mcp-logger` 針對 UTF-8 進行了優化。它會自動檢測訊息中是否包含非 ASCII 字元，並在需要時自動添加 **UTF-8 BOM (位元組順序標記)**。

### 與 Syslog 伺服器的配合
許多基於 Windows 的 Syslog 伺服器（如 `vlt-syslogd`）或現代 Linux `rsyslog` 設置，只要能正確識別編碼，就能處理 UTF-8。BOM 確保這些工具不會將日誌誤認為是本地傳統編碼（如 CP932 或 EUC-KR）。

## 5. 常見問題排除
- **AI 無法識別工具**：請確保配置文件中的路徑是絕對路徑。
- **日誌未送達**：檢查目標伺服器的防火牆是否允許 UDP 連接埠 514。
- **字元亂碼**：確保 `encoding` 參數設置為 `utf-8`。AI 模型通常優先使用 UTF-8。

## 6. 補充指南
- [多平台設定指南](CROSS_PLATFORM_SETUP.zh-TW.md)
- [LM Studio 設定指南](LM_STUDIO_SETUP.zh-TW.md)
- [MCP 協定範例](MCP_PROTOCOL_EXAMPLES.zh-TW.md)
- [規格說明書 (SPEC)](SPEC.md)
