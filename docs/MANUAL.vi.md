# MCP Logger - Hướng dẫn chi tiết

## 1. Giới thiệu
`mcp-logger` là một máy chủ MCP (Model Context Protocol) được thiết kế để kết nối trợ lý AI và cơ sở hạ tầng Syslog. Nó cho phép AI sử dụng Syslog để ghi nhật ký và theo dõi trạng thái.

## 2. Cách cấu hình

### Claude Desktop
Thêm nội dung sau vào `%APPDATA%\Claude\claude_desktop_config.json` (Windows) hoặc `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS).

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

## 3. Tài liệu tham khảo công cụ (Tool Reference)

### `send-syslog`
Gửi tin nhắn đến máy chủ Syslog.

| Tham số | Kiểu | Mặc định | Mô tả |
| :--- | :--- | :--- | :--- |
| `server` | string | (Bắt buộc) | IP hoặc tên máy chủ đích. |
| `port` | number | 514 | Số cổng UDP. |
| `facility` | string | user | Syslog facility (kern, user, mail, v.v.) |
| `severity` | string | info | Mức độ nghiêm trọng của Syslog (emerg, alert, crit, err, warning, notice, info, debug) |
| `tag` | string | mcp-syslog | Tên chương trình/Thẻ (Tag). |
| `encoding` | string | utf-8 | Bảng mã ký tự. |
| `message` | string | (Bắt buộc) | Nội dung nhật ký. |

## 4. Hướng dẫn ký tự đa byte (Dành cho các quốc gia sử dụng ký tự 2 byte)
Nếu bạn ở quốc gia sử dụng ký tự 2 byte (Việt Nam, Nhật Bản, Hàn Quốc, Trung Quốc, v.v.), vui lòng làm theo các hướng dẫn sau:

### Khuyến nghị: Sử dụng UTF-8
`mcp-logger` được tối ưu hóa cho UTF-8. Nó tự động phát hiện nếu tin nhắn chứa các ký tự không phải ASCII và tự động thêm **UTF-8 BOM (Byte Order Mark)** nếu cần thiết.

### Tương tác với máy chủ Syslog
Nhiều máy chủ Syslog dựa trên Windows (như `vlt-syslogd`) hoặc các thiết lập `rsyslog` Linux hiện đại có thể xử lý UTF-8 nếu được định danh chính xác. BOM đảm bảo rằng các công cụ này không nhầm lẫn nhật ký là bảng mã cũ của địa phương (như CP932 hoặc EUC-KR).

## 5. Xử lý sự cố
- **AI không nhận dạng được công cụ**: Đảm bảo đường dẫn trong tệp cấu hình là đường dẫn tuyệt đối.
- **Nhật ký không đến**: Kiểm tra xem cổng UDP 514 đã được mở trên tường lửa của máy chủ đích chưa.
- **Lỗi font chữ**: Đảm bảo tham số `encoding` được đặt thành `utf-8`. Các mô hình AI thường ưu tiên UTF-8.

## 6. Hướng dẫn bổ sung
- [Hướng dẫn thiết lập đa nền tảng](CROSS_PLATFORM_SETUP.vi.md)
- [Hướng dẫn thiết lập LM Studio](LM_STUDIO_SETUP.vi.md)
- [Ví dụ về giao thức MCP](MCP_PROTOCOL_EXAMPLES.vi.md)
- [Thông số kỹ thuật (SPEC)](SPEC.md)
