# MCP Logger ([English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md))

## Tổng quan
Dự án này cung cấp một máy chủ MCP (Model Context Protocol) cho phép các trợ lý AI (như Claude, Gemini, v.v.) sử dụng cơ sở hạ tầng Syslog để ghi nhật ký và giám sát.
Bản thân nó không phải là một "Máy chủ Syslog", mà là một "Khả năng của máy khách" dành cho AI để gửi nhật ký đến máy chủ Syslog hiện có (ví dụ: `rsyslog`, `syslog-ng` hoặc `vlt-syslogd`).

Nó chia sẻ logic cốt lõi với `rust-logger` (một bộ ghi nhật ký CLI dành cho con người) đồng thời cung cấp giao diện được tối ưu hóa cho tương tác AI.

## Các tính năng chính
- **Gửi tin nhắn Syslog**: AI có thể chỉ định máy chủ, cổng, facility, mức độ nghiêm trọng (severity), thẻ (tag), bảng mã (encoding) và nội dung tin nhắn để gửi Syslog.
- **Hỗ trợ ký tự 2 byte (UTF-8 với BOM)**: Được thiết kế đặc biệt để xử lý chính xác các ký tự không phải ASCII (tiếng Tiếng Việt, Nhật, Hàn, Trung, v.v.) nhằm đảm bảo việc phân tích AI không bị lỗi font.
- **Thiết kế kép CLI-MCP**: Hoạt động như một công cụ CLI độc lập cho con người và một máy chủ MCP cho AI.

## Cài đặt
Yêu cầu môi trường Rust.

```bash
cargo build --release
```

## Cách sử dụng (Chế độ kép CLI / MCP)

Công cụ này kết hợp "CLI cho con người" và "Máy chủ MCP cho AI".

### 1. Chế độ CLI cho con người
Bạn có thể gửi nhật ký trực tiếp từ terminal của mình.

```bash
# Gửi cơ bản
mcp-logger send 192.168.1.100 "Xin chào từ CLI"

# Gửi với các tùy chọn
mcp-logger send 192.168.1.100 "Đã xảy ra lỗi" --severity error --tag my-app
```

Chạy `mcp-logger` không có đối số trong terminal sẽ hiển thị **Hướng dẫn thiết lập MCP**.

### 2. Chế độ trợ lý AI (MCP)
Thêm nội dung sau vào `claude_desktop_config.json` hoặc tệp cấu hình tương đương của bạn.

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

## Công cụ MCP (Tools)
Các tính năng cung cấp cho AI.

### `send-syslog`
Gửi tin nhắn Syslog đến một máy chủ được chỉ định.

**Đối số (Arguments):**
- `server` (string, bắt buộc): Tên máy chủ hoặc IP của máy chủ Syslog.
- `port` (number): Số cổng (mặc định: `514`).
- `facility` (string): Facility (ví dụ: `user`, `local0`). Mặc định là `user`.
- `severity` (string): Mức độ nghiêm trọng (ví dụ: `info`, `error`). Mặc định là `info`.
- `tag` (string): Tên chương trình (tag). Mặc định là `mcp-syslog`.
- `encoding` (string): Bảng mã (ví dụ: `utf-8`, `shift_jis`). Mặc định là `utf-8`.
- `message` (string, bắt buộc): Nội dung tin nhắn nhật ký.

## Quan trọng: Xử lý ký tự đa byte
Đối với các ngôn ngữ sử dụng ký tự 2 byte (Tiếng Việt có dấu, tiếng Trung, Nhật, Hàn, v.v.), AI được hướng dẫn sử dụng `utf-8`. Máy chủ sẽ tự động đính kèm **UTF-8 BOM** để đảm bảo máy chủ Syslog và các công cụ phân tích sau đó xác định chính xác bảng mã.

## Tài liệu
- [Hướng dẫn chi tiết](docs/MANUAL.vi.md)
- [Hướng dẫn thiết lập đa nền tảng](docs/CROSS_PLATFORM_SETUP.vi.md)
- [Hướng dẫn thiết lập LM Studio](docs/LM_STUDIO_SETUP.vi.md)
- [Ví dụ về giao thức MCP](docs/MCP_PROTOCOL_EXAMPLES.vi.md)
- [Thông số kỹ thuật (SPEC)](docs/SPEC.md)
