# Google Antigravity (MCP) 設定指南

本文件按平台介紹了在 Google Antigravity（或其他 MCP 用戶端）中使用 `mcp-logger` 的設定步驟。

## 概覽

`mcp-logger` 是一個透過標準輸入/輸出 (stdio) 進行 JSON-RPC 通訊的獨立二進位檔案。
透過在 Antigravity 等用戶端設定檔 (`mcp_config.json`) 中告訴系統「執行此命令」來實現關聯。

---

## 1. macOS (已設定)

已自動完成設定。`~/.gemini/antigravity/mcp_config.json` 中已添加以下設定。

```json
"mcp-logger": {
  "command": "cargo",
  "args": [
    "run",
    "--release",
    "--quiet",
    "--manifest-path",
    "/path/to/your/rust-logger/mcp-logger/Cargo.toml"
  ]
}
```

※ 首次執行時會進行編譯，之後啟動速度會很快。

---

## 2. Windows 11 (LLM-SVR1 / WORK1)

在 Windows 環境中使用 Antigravity（Claude Desktop 等）時的步驟。

### 前期準備
1. 安裝 Rust (`rustup-init.exe`)
2. 將原始碼放入 `rust-logger` 資料夾（如果已同步 Google Drive，可直接使用）

### `mcp_config.json` 設定
根據 Windows 路徑格式進行設定。

```json
{
  "mcpServers": {
    "mcp-logger": {
      "command": "cargo.exe",
      "args": [
        "run",
        "--release",
        "--quiet",
        "--manifest-path",
        "C:\\path\\to\\your\\rust-logger\\mcp-logger\\Cargo.toml"
      ]
    }
  }
}
```
※ 請根據實際部署路徑修改路徑 (`C:\\Users\\...`)。

---

## 3. Ubuntu (開發伺服器)

在 Ubuntu 上執行 Antigravity 的步驟。

### 前期準備
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential
```

### `mcp_config.json` 設定

```json
{
  "mcpServers": {
    "mcp-logger": {
      "command": "cargo",
      "args": [
        "run",
        "--release",
        "--quiet",
        "--manifest-path",
        "/home/veltrea/DEV/rust-logger/mcp-logger/Cargo.toml"
      ]
    }
  }
}
```

---

## 故障排除

- **編譯未完成**: 首次執行時需要編譯相依函式庫，可能需要等待數十秒。
- **JsonRpc 錯誤**: 如果 `cargo run` 的輸出中混入多餘日誌，會導致錯誤。`--quiet` 參數是必需的。此外，程式已實作使用 `eprintln!`（標準錯誤輸出）而非 `println!` 輸出偵錯資訊。
