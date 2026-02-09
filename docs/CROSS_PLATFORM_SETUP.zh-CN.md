# Google Antigravity (MCP) 设置指南

本文件按平台介绍了在 Google Antigravity（或其他 MCP 客户端）中使用 `mcp-logger` 的设置步骤。

## 概览

`mcp-logger` 是一个通过标准输入/输出 (stdio) 进行 JSON-RPC 通信的独立二进制文件。
通过在 Antigravity 等客户端配置文件 (`mcp_config.json`) 中告诉系统“执行此命令”来实现关联。

---

## 1. macOS (已设置)

已自动完成设置。`~/.gemini/antigravity/mcp_config.json` 中已添加以下配置。

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

※ 首次运行时会执行构建，之后启动速度会很快。

---

## 2. Windows 11 (LLM-SVR1 / WORK1)

在 Windows 环境中使用 Antigravity（Claude Desktop 等）时的步骤。

### 前期准备
1. 安装 Rust (`rustup-init.exe`)
2. 将源代码放入 `rust-logger` 文件夹（如果已同步 Google Drive，可直接使用）

### `mcp_config.json` 设置
根据 Windows 路径格式进行设置。

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
※ 请根据实际部署路径修改路径 (`C:\\Users\\...`)。

---

## 3. Ubuntu (开发服务器)

在 Ubuntu 上运行 Antigravity 的步骤。

### 前期准备
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential
```

### `mcp_config.json` 设置

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

- **构建未完成**: 首次运行时需要编译依赖库，可能需要等待数十秒。
- **JsonRpc 错误**: 如果 `cargo run` 的输出中混入多余日志，会导致错误。`--quiet` 参数是必需的。此外，程序已实现使用 `eprintln!`（标准错误输出）而非 `println!` 输出调试信息。
