# MCP Logger ([English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md))

## 概述
该项目提供了一个 MCP (Model Context Protocol) 服务端，使 AI 助手（如 Claude、Gemini 等）能够利用 Syslog 基础设施进行日志记录和监控。
它本身不是一个“Syslog 服务器”，而是为 AI 提供的一个“客户端功能”，以便向现有的 Syslog 服务器（例如 `rsyslog`、`syslog-ng` 或 `vlt-syslogd`）发送日志。

它与面向人类的 CLI 日志工具 `rust-logger` 共享核心逻辑，同时提供了针对 AI 交互优化的接口。

## 主要特性
- **发送 Syslog 消息**：AI 可以指定服务器、端口、设施 (facility)、严重性 (severity)、标签、编码和消息内容来发送 Syslog。
- **支持双字节字符 (UTF-8 with BOM)**：专门设计用于正确处理非 ASCII 字符（中文、日文、韩文等），确保 AI 分析时不会出现乱码。
- **CLI-MCP 双重设计**：既可以作为人类使用的独立 CLI 工具，也可以作为 AI 使用的 MCP 服务端。

## 安装
需要 Rust 环境。

```bash
cargo build --release
```

## 使用方法 (CLI / MCP 双模式)

该工具结合了“面向人类的 CLI”和“面向 AI 的 MCP 服务端”。

### 1. 人类 CLI 模式
你可以直接从终端发送日志。

```bash
# 基本发送
mcp-logger send 192.168.1.40 "来自 CLI 的问候"

# 带选项发送
mcp-logger send 192.168.1.40 "发生错误" --severity error --tag my-app
```

在终端中运行不带参数的 `mcp-logger` 将显示 **MCP 设置指南**。

### 2. AI 助手模式 (MCP)
在您的 `claude_desktop_config.json` 或等效配置文件中添加以下内容。

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
提供给 AI 的功能。

### `send-syslog`
将 Syslog 消息发送到指定的服务器。

**参数 (Arguments):**
- `server` (string, 必填): Syslog 服务器的主机名或 IP。
- `port` (number): 端口号 (默认: `514`)。
- `facility` (string): 设施 (例如 `user`, `local0`)。默认为 `user`。
- `severity` (string): 严重性 (例如 `info`, `error`)。默认为 `info`。
- `tag` (string): 程序名称 (标签)。默认为 `mcp-syslog`。
- `encoding` (string): 编码 (例如 `utf-8`, `shift_jis`)。默认为 `utf-8`。
- `message` (string, 必填): 日志消息正文。

## 重要：多字节字符处理
对于使用双字节字符的语言（中文、日文、韩文等），AI 被指示使用 `utf-8`。服务端会自动添加 **UTF-8 BOM**，以确保 Syslog 服务器和后续分析工具能够正确识别编码。

## 文档
- [详细手册](docs/MANUAL.zh-CN.md)
- [多平台设置指南](docs/CROSS_PLATFORM_SETUP.zh-CN.md)
- [LM Studio 设置指南](docs/LM_STUDIO_SETUP.zh-CN.md)
- [MCP 协议示例](docs/MCP_PROTOCOL_EXAMPLES.zh-CN.md)
- [规格说明书 (SPEC)](docs/SPEC.md)
