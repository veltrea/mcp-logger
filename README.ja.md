# MCP Logger ([English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md))

## 概要
本プロジェクトは、AI アシスタント（Claude, Gemini 等）が Syslog を活用できるようにするための、MCP (Model Context Protocol) サーバー機能を提供します。
「Syslog サーバー」として動作するのではなく、既存の Syslog サーバー（例: `rsyslog`, `syslog-ng`, `vlt-syslogd`）に対してログを送信する「クライアント機能」を AI に付与するものです。

人間用の CLI ロガーである `rust-logger` とコアロジックを共有しつつ、AI との対話に最適化されたインターフェースを提供します。

## 主な機能
- **Syslog 送信**: AI がサーバー、ポート、ファシリティ、重要度、タグ、エンコーディング、メッセージを指定して Syslog を送信可能。
- **2バイト文字対応 (UTF-8 with BOM)**: 日本語、韓国語、中国語などの非 ASCII 文字を AI が分析する際に、文字化けを防ぐための特別設計。
- **CLI-MCP 兼用設計**: 人間が使うコマンドラインツールとしても、AI が使う MCP サーバーとしても動作。

## インストール
Rust 環境が必要です。

```bash
cargo build --release
```

## 使い方 (CLI / MCP 両対応)

本ツールは「人間用の CLI」と「AI 用の MCP サーバー」を兼ねています。

### 1. 人間用 CLI モード
ターミナルから直接ログを送信できます。

```bash
# 基本送信
mcp-logger send 192.168.1.100 "Hello from CLI"

# オプション指定
mcp-logger send 192.168.1.100 "エラー発生" --severity error --tag my-app
```

引数なしで実行すると **MCP 設定ガイド** が表示されます。

### 2. AI アシスタントモード (MCP)
`claude_desktop_config.json` 等に以下の設定を追加してください。

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

## 提供ツール (MCP Tools)
AI が利用可能な機能です。

### `send-syslog`
指定したサーバーに Syslog メッセージを送信します。

**引数:**
- `server` (string, 必須): Syslog サーバーのホスト名または IP。
- `port` (number): ポート番号 (デフォルト: `514`)。
- `facility` (string): ファシリティ (例: `user`, `local0`)。デフォルトは `user`。
- `severity` (string): 重要度 (例: `info`, `error`)。デフォルトは `info`。
- `tag` (string): プログラム名 (タグ)。デフォルトは `mcp-syslog`。
- `encoding` (string): エンコーディング (例: `utf-8`, `shift_jis`)。デフォルトは `utf-8`。
- `message` (string, 必須): ログメッセージ本文。

## 重要：マルチバイト文字の取り扱い
日本語・中国語・韓国語などの2バイト文字を扱う際、AI は `utf-8` を使用するよう指示されます。本サーバーは自動的に **UTF-8 BOM** を付与して送信するため、受信側の Syslog サーバーやその後の分析ツールで正しく文字コードが識別されます。

## ドキュメント
- [詳細マニュアル](docs/MANUAL.ja.md)
- [プラットフォーム別セットアップガイド (Google Antigravity)](docs/CROSS_PLATFORM_SETUP.ja.md)
- [LM Studio 連携ガイド](docs/LM_STUDIO_SETUP.ja.md)
- [MCP プロトコル通信例 (JSON-RPC)](docs/MCP_PROTOCOL_EXAMPLES.ja.md)
- [仕様書 (SPEC)](docs/SPEC.md)
