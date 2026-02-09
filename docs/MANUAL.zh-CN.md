# MCP Logger - 详细手册

## 1. 简介
`mcp-logger` 是一个 MCP (Model Context Protocol) 服务端，旨在连接 AI 助手和 Syslog 基础设施。它允许 AI 利用 Syslog 进行日志记录和状态追踪。

## 2. 配置方法

### Claude Desktop
将其添加到 `%APPDATA%\Claude\claude_desktop_config.json` (Windows) 或 `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) 中。

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

## 3. 工具参考 (Tool Reference)

### `send-syslog`
向 Syslog 服务器发送消息。

| 参数 | 类型 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- |
| `server` | string | (必填) | 目标服务器 IP 或主机名。 |
| `port` | number | 514 | UDP 端口号。 |
| `facility` | string | user | Syslog 设施 (kern, user, mail 等) |
| `severity` | string | info | Syslog 严重性 (emerg, alert, crit, err, warning, notice, info, debug) |
| `tag` | string | mcp-syslog | 程序名称/标签。 |
| `encoding` | string | utf-8 | 字符编码。 |
| `message` | string | (必填) | 日志内容。 |

## 4. 多字节字符指南 (针对使用双字节字符的用户)
如果您在中文、日文、韩文、越南文等使用双字节字符的国家/地区，请遵循以下准则：

### 建议：使用 UTF-8
`mcp-logger` 针对 UTF-8 进行了优化。它会自动检测消息中是否包含非 ASCII 字符，并在需要时自动添加 **UTF-8 BOM (字节顺序标记)**。

### 与 Syslog 服务器的配合
许多基于 Windows 的 Syslog 服务器（如 `vlt-syslogd`）或现代 Linux `rsyslog` 设置，只要能正确识别编码，就能处理 UTF-8。BOM 确保这些工具不会将日志误认为是本地传统编码（如 CP932 或 EUC-KR）。

## 5. 常见问题排除
- **AI 无法识别工具**：请确保配置文件中的路径是绝对路径。
- **日志未送达**：检查目标服务器的防火墙是否允许 UDP 端口 514。
- **字符乱码**：确保 `encoding` 参数设置为 `utf-8`。AI 模型通常优先使用 UTF-8。

## 6. 补充指南
- [多平台设置指南](CROSS_PLATFORM_SETUP.zh-CN.md)
- [LM Studio 设置指南](LM_STUDIO_SETUP.zh-CN.md)
- [MCP 协议示例](MCP_PROTOCOL_EXAMPLES.zh-CN.md)
- [规格说明书 (SPEC)](SPEC.md)
