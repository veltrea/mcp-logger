# MCP Logger - 詳細マニュアル

## 1. はじめに
`mcp-logger` は、AI アシスタントと Syslog インフラを橋渡しするために設計された MCP サーバーです。AI が Syslog を活用して、ログの記録や状態の追跡を行えるようにします。

## 2. 設定方法

### Claude Desktop
`%APPDATA%\Claude\claude_desktop_config.json` (Windows) または `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) に以下の設定を追加します。

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

## 3. ツールリファレンス

### `send-syslog`
Syslog サーバーにメッセージを送信します。

| パラメーター | 型 | デフォルト | 説明 |
| :--- | :--- | :--- | :--- |
| `server` | string | (必須) | 送信先サーバーの IP またはホスト名。 |
| `port` | number | 514 | UDP ポート番号。 |
| `facility` | string | user | Syslog ファシリティ (kern, user, mail 等) |
| `severity` | string | info | Syslog 重要度 (emerg, alert, crit, err, warning, notice, info, debug) |
| `tag` | string | mcp-syslog | プログラム名/タグ。 |
| `encoding` | string | utf-8 | 文字エンコーディング。 |
| `message` | string | (必須) | ログの内容。 |

## 4. マルチバイト文字ガイド (2バイト文字圏の方向け)
日本語、韓国語、中国語、ベトナム語などの2バイト文字を使用する地域では、以下のガイドラインに従ってください。

### 推奨: UTF-8 の使用
`mcp-logger` は UTF-8 に最適化されています。メッセージに非 ASCII 文字が含まれていることを自動的に検出し、必要に応じて **UTF-8 BOM (Byte Order Mark)** を付与します。

### Syslog サーバーとの連携
多くの Windows ベースの Syslog サーバー（`vlt-syslogd` など）や、最新の Linux `rsyslog` 設定は、正しく識別されれば UTF-8 を処理できます。BOM が付与されることで、これらのツールがログをローカルのレガシーエンコーディング（CP932 や EUC-KR など）として誤認するのを防ぎます。

## 5. トラブルシューティング
- **AI がツールを認識しない**: 設定ファイルのパスが絶対パスであることを確認してください。
- **ログが届かない**: 送信先サーバーのファイアウォールで UDP ポート 514 が許可されているか確認してください。
- **文字化けが発生する**: `encoding` パラメーターが `utf-8` に設定されていることを確認してください。AI モデルは通常 UTF-8 を優先します。

## 6. 補足ガイド
- [プラットフォーム別セットアップガイド (Google Antigravity)](CROSS_PLATFORM_SETUP.ja.md)
- [LM Studio 連携ガイド](LM_STUDIO_SETUP.ja.md)
- [MCP プロトコル通信例 (JSON-RPC)](MCP_PROTOCOL_EXAMPLES.ja.md)
- [仕様書 (SPEC)](SPEC.md)
