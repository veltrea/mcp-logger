# LM Studio Setup Guide for mcp-logger

LM Studio で `mcp-logger` を使用するための設定ガイドです。
本プロジェクトは、LM Studio 特有の「4つのルール」に完全対応するように修正されています。

## 特有のルールへの対応状況

1.  **インストールパス**: 他のクライアント（Google Antigravity 等）では任意の絶対パスで動作しますが、LM Studio では `~/.lmstudio/extensions/plugins/mcp/mcp-logger/` （Windows の場合は `C:\Users\<USERNAME>\.lmstudio\extensions\plugins\mcp\mcp-logger\`）に配置することで正常に認識されることを確認しています。
2.  **プロトコルバージョン**: クライアント（LM Studio）が提示したバージョンをそのまま返すように実装済みです（例: `2024-11-05`）。
3.  **JSON-RPC 形式**: 必須外のフィールド (`error` が null の場合など) を除外してレスポンスするように実装されています。
4.  **Tool 応答**: `content` 配列と `isError` フラグを含む形式で応答します。

## 設定手順

LM Studio の MCP 設定ファイル（通常 `~/.lmstudio/config.json` や `mcp.json` など、バージョンにより異なりますが、GUI から設定可能な場合が多いです）に以下を追加してください。

### macOS / Linux (Ubuntu)

```json
"mcp-logger": {
  "command": "/Users/<USERNAME>/.local/bin/mcp-logger",
  "args": []
}
```
※ パスやユーザー名は環境に合わせて、ビルドしたバイナリの場所を指定してください。

### Windows (LLM-SVR1 等)

```json
"mcp-logger": {
  "command": "C:\\Users\\<USERNAME>\\AppData\\Local\\Programs\\mcp-logger\\mcp-logger.exe",
  "args": []
}
```

## 動作確認

1.  LM Studio を起動し、サーバー接続（Initialize）が成功するか確認してください（ログに `Connected to mcp-logger` 等が表示されます）。
2.  チャット欄で「Syslogを送って」と指示し、ツールが認識されているか確認してください。
