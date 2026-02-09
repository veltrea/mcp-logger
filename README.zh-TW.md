# MCP Logger ([English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md))

## 概述
本專案提供了一個 MCP (Model Context Protocol) 伺服器，讓 AI 助手（如 Claude、Gemini 等）能夠利用 Syslog 基礎設施進行日誌記錄和監控。
它本身並不是一個「Syslog 伺服器」，而是為 AI 提供的一個「用戶端功能」，以便向現端有的 Syslog 伺服器（例如 `rsyslog`、`syslog-ng` 或 `vlt-syslogd`）發送日誌。

它與面向人類的 CLI 日誌工具 `rust-logger` 共享核心邏輯，同時提供了針對 AI 互動優化的介面。

## 主要特性
- **發送 Syslog 訊息**：AI 可以指定伺服器、連接埠、設施 (facility)、嚴重性 (severity)、標籤、編碼和訊息內容來發送 Syslog。
- **支援雙位元組字元 (UTF-8 with BOM)**：專門設計用於正確處理非 ASCII 字元（中文、日文、韓文等），確保 AI 分析時不會出現亂碼。
- **CLI-MCP 雙重設計**：既可以作為人類使用的獨立 CLI 工具，也可以作為 AI 使用的 MCP 伺服器。

## 安裝
需要 Rust 環境。

```bash
cargo build --release
```

## 使用方法 (CLI / MCP 雙模式)

該工具結合了「面向人類的 CLI」和「面向 AI 的 MCP 伺服器」。

### 1. 人類 CLI 模式
您可以直接從終端機發送日誌。

```bash
# 基本發送
mcp-logger send 192.168.1.100 "來自 CLI 的問候"

# 帶選項發送
mcp-logger send 192.168.1.100 "發生錯誤" --severity error --tag my-app
```

在終端機中執行不帶參數的 `mcp-logger` 將顯示 **MCP 設置指引**。

### 2. AI 助手模式 (MCP)
在您的 `claude_desktop_config.json` 或等效配置文件中添加以下內容。

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

## MCP 工具 (Tools)
提供給 AI 的功能。

### `send-syslog`
將 Syslog 訊息發送到指定的伺服器。

**參數 (Arguments):**
- `server` (string, 必填): Syslog 伺服器的主機名或 IP。
- `port` (number): 連接埠號 (預設: `514`)。
- `facility` (string): 設施 (例如 `user`, `local0`)。預設為 `user`。
- `severity` (string): 嚴重性 (例如 `info`, `error`)。預設為 `info`。
- `tag` (string): 程式名稱 (標籤)。預設為 `mcp-syslog`。
- `encoding` (string): 編碼 (例如 `utf-8`, `shift_jis`)。預設為 `utf-8`。
- `message` (string, 必填): 日志訊息正文。

## 重要：多位元組字元處理
對於使用雙位元組字元的語言（中文、日文、韓文等），AI 被指示引使用 `utf-8`。伺服器會自動添加 **UTF-8 BOM**，以確保 Syslog 伺服器和後續分析工具能夠正確識別編碼。

## 文件
- [詳細手冊](docs/MANUAL.zh-TW.md)
- [多平台設定指南](docs/CROSS_PLATFORM_SETUP.zh-TW.md)
- [LM Studio 設定指南](docs/LM_STUDIO_SETUP.zh-TW.md)
- [MCP 協定範例](docs/MCP_PROTOCOL_EXAMPLES.zh-TW.md)
- [規格說明書 (SPEC)](docs/SPEC.md)
