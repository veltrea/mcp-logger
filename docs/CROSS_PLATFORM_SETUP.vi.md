# Hướng dẫn thiết lập Google Antigravity (MCP)

Tài liệu này giải thích quy trình thiết lập để sử dụng `mcp-logger` với Google Antigravity (hoặc các ứng dụng MCP khác) theo từng nền tảng.

## Tổng quan

`mcp-logger` là một tệp thực thi độc lập giao tiếp qua JSON-RPC thông qua đầu vào/đầu ra tiêu chuẩn (stdio).
Nó tích hợp với các ứng dụng như Antigravity bằng cách thêm một mục vào tệp cấu hình (`mcp_config.json`) để hướng dẫn ứng dụng "chạy lệnh này".

---

## 1. macOS (Hiện đã được cấu hình)

Đã được cấu hình tự động. Các cài đặt sau đã được thêm vào `~/.gemini/antigravity/mcp_config.json`.

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

*Lưu ý: Lần thực thi đầu tiên sẽ tiến hành biên dịch; các lần khởi chạy sau sẽ rất nhanh.*

---

## 2. Windows 11 (Remote-Server / Local-Server)

Các bước sử dụng Antigravity (hoặc Claude Desktop, v.v.) trong môi trường Windows.

### Chuẩn bị trước
1. Cài đặt Rust (`rustup-init.exe`)
2. Đặt mã nguồn vào thư mục `rust-logger` (nếu đã đồng bộ Google Drive, bạn có thể sử dụng ngay).

### Cài đặt `mcp_config.json`
Thiết lập theo định dạng đường dẫn của Windows.

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
*Lưu ý: Vui lòng thay đổi đường dẫn (`C:\\Users\\...`) cho phù hợp với vị trí triển khai thực tế của bạn.*

---

## 3. Ubuntu (Máy chủ phát triển)

Các bước để chạy Antigravity trên Ubuntu.

### Chuẩn bị trước
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential
```

### Cài đặt `mcp_config.json`

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

## Khắc phục sự cố

- **Biên dịch không kết thúc**: Lần đầu tiên cần biên dịch các thư viện phụ thuộc, có thể mất vài chục giây.
- **Lỗi JsonRpc**: Nếu có các nhật ký thừa lẫn vào đầu ra của `cargo run`, sẽ xảy ra lỗi. Tùy chọn `--quiet` là bắt buộc. Ngoài ra, công cụ này được triển khai để sử dụng `eprintln!` (đầu ra lỗi tiêu chuẩn) cho việc gỡ lỗi thay vì `println!`.
