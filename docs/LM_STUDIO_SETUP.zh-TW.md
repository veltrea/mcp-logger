# mcp-logger 的 LM Studio 設定指南

這是在 LM Studio 中使用 `mcp-logger` 的設定指南。
本專案經過修改，已完全符合 LM Studio 特有的「四個本地規則」。

## 特有規則對應情況

1.  **安裝路徑**: 雖然在其他用戶端（如 Google Antigravity）中可以使用任意絕對路徑，但在 LM Studio 中，已確認將其放置在 `~/.lmstudio/extensions/plugins/mcp/mcp-logger/`（Windows 為 `C:\Users\<USERNAME>\.lmstudio\extensions\plugins\mcp\mcp-logger\`）才能正常識別。
2.  **協定版本**: 已實現根據用戶端（LM Studio）提示的版本原樣回傳（例如：`2024-11-05`）。
3.  **JSON-RPC 格式**: 已實現排除非必填欄位（如 `error` 為 null 時）進行回應。
4.  **工具回應**: 以包含 `content` 陣列和 `isError` 標誌的格式進行回應。

## 設定步驟

請在 LM Studio 的 MCP 設定檔（通常為 `~/.lmstudio/config.json` 或 `mcp.json`，取決於版本，通常可透過 GUI 設定）中添加以下內容。

### macOS / Linux (Ubuntu)

```json
"mcp-logger": {
  "command": "/Users/<USERNAME>/.local/bin/mcp-logger",
  "args": []
}
```
※ 請根據實際環境指定編譯的執行檔路徑和使用者名稱。

### Windows (Remote-Server 等)

```json
"mcp-logger": {
  "command": "C:\\Users\\<USERNAME>\\AppData\\Local\\Programs\\mcp-logger\\mcp-logger.exe",
  "args": []
}
```

## 運作確認

1.  啟動 LM Studio，確認伺服器連線（Initialize）是否成功（日誌中會顯示 `Connected to mcp-logger` 等）。
2.  在對話框中輸入「發送 Syslog」，確認工具是否被識別。
