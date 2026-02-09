# mcp-logger 的 LM Studio 设置指南

这是在 LM Studio 中使用 `mcp-logger` 的设置指南。
该项目经过修改，已完全符合 LM Studio 特有的“四个本地规则”。

## 特有规则对应情况

1.  **安装路径**: 虽然在其他客户端（如 Google Antigravity）中可以使用任意绝对路径，但在 LM Studio 中，已确认将其放置在 `~/.lmstudio/extensions/plugins/mcp/mcp-logger/`（Windows 为 `C:\Users\<USERNAME>\.lmstudio\extensions\plugins\mcp\mcp-logger\`）才能正常识别。
2.  **协议版本**: 已实现根据客户端（LM Studio）提示的版本原样返回（例如：`2024-11-05`）。
3.  **JSON-RPC 格式**: 已实现排除非必填字段（如 `error` 为 null 时）进行响应。
4.  **工具响应**: 以包含 `content` 数组和 `isError` 标志的格式进行响应。

## 设置步骤

请在 LM Studio 的 MCP 配置文件（通常为 `~/.lmstudio/config.json` 或 `mcp.json`，取决于版本，通常可通过 GUI 设置）中添加以下内容。

### macOS / Linux (Ubuntu)

```json
"mcp-logger": {
  "command": "/Users/<USERNAME>/.local/bin/mcp-logger",
  "args": []
}
```
※ 请根据实际环境指定构建的二进制文件路径和用户名。

### Windows (Remote-Server 等)

```json
"mcp-logger": {
  "command": "C:\\Users\\<USERNAME>\\AppData\\Local\\Programs\\mcp-logger\\mcp-logger.exe",
  "args": []
}
```

## 运行确认

1.  启动 LM Studio，确认服务器连接（Initialize）是否成功（日志中会显示 `Connected to mcp-logger` 等）。
2.  在对话框中输入“发送 Syslog”，确认工具是否被识别。
