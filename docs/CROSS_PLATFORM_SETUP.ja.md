# Google Antigravity (MCP) Setup Guide

このドキュメントでは、`mcp-logger` を Google Antigravity (またはその他の MCP クライアント) で利用するための設定手順をプラットフォーム別に解説します。

## 概要

`mcp-logger` は標準入出力 (stdio) で JSON-RPC をお話しする単独のバイナリです。
Antigravity などのクライアント設定ファイル (`mcp_config.json`) に、「このコマンドを実行してね」と教えることで連携します。

---

## 1. macOS (現在設定済み)

自動設定済みです。`~/.gemini/antigravity/mcp_config.json` に以下の設定が追加されています。

```json
"mcp-logger": {
  "command": "cargo",
  "args": [
    "run",
    "--release",
    "--quiet",
    "--manifest-path",
    "[LOCAL-PATH]/mcp-logger/Cargo.toml"
  ]
}
```

※ 初回実行時にビルドが走り、2回目以降は高速に起動します。

---

## 2. Windows 11 (Remote-Server / Local-Server)

Windows 環境で Antigravity (Claude Desktop 等) を使う場合の手順です。

### 事前準備
1. Rust をインストール (`rustup-init.exe`)
2. ソースコードを `rust-logger` フォルダに配置（Google Drive同期済みであればそのまま利用可）

### `mcp_config.json` の設定
Windows のパス形式に合わせて設定します。

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
        "C:\\[LOCAL-PATH]\\[mcp-logger]\\Cargo.toml"
      ]
    }
  }
}
```
※ パス (`C:\\Users\\...`) は実際の配置場所に合わせて変更してください。

---

## 3. Ubuntu (Dev Server)

Ubuntu 上で Antigravity を動かす場合の手順です。

### 事前準備
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential
```

### `mcp_config.json` の設定

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
        "/home/[USER]/dev/rust-logger/mcp-logger/Cargo.toml"
      ]
    }
  }
}
```

---

## トラブルシューティング

- **ビルドが終わらない**: 初回は依存ライブラリのコンパイルがあるため、数十秒待ちます。
- **JsonRpcエラー**: `cargo run` の出力に余計なログが混じるとエラーになります。`--quiet` オプションが必須です。また、デバッグ用の `println!` は使わず、`eprintln!` (標準エラー出力) を使うように実装されています。
