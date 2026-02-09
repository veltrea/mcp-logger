# MCP Logger ([English](README.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [Tiếng Việt](README.vi.md) | [ไทย](README.th.md) | [हिन्दी](README.hi.md) | [العربية](README.ar.md) | [Русский](README.ru.md))

## 개요
이 프로젝트는 AI 어시스턴트(Claude, Gemini 등)가 Syslog 인프라를 활용할 수 있도록 하는 MCP(Model Context Protocol) 서버를 제공합니다.
이 도구 자체는 "Syslog 서버"가 아니라, AI가 기존 Syslog 서버(예: `rsyslog`, `syslog-ng` 또는 `vlt-syslogd`)에 로그를 보낼 수 있는 "클라이언트 기능"을 제공하는 것입니다.

이 프로젝트는 인간을 위한 CLI 로거인 `rust-logger`와 핵심 로직을 공유하면서 AI 상호 작용에 최적화된 인터페이스를 제공합니다.

## 주요 특징
- **Syslog 메시지 전송**: AI가 서버, 포트, 퍼실리티(facility), 심각도(severity), 태그, 인코딩 및 메시지를 지정하여 Syslog를 전송할 수 있습니다.
- **2바이트 문자 지원 (UTF-8 with BOM)**: 특히 한국어, 일본어, 중국어 등 비 ASCII 문자를 사용하는 환경에서 AI 분석 시 문자가 깨지지 않도록 설계되었습니다.
- **CLI-MCP 듀얼 디자인**: 인간을 위한 독립형 CLI 도구로도, AI를 위한 MCP 서버로도 작동합니다.

## 설치 방법
Rust 환경이 필요합니다.

```bash
cargo build --release
```

## 사용법 (CLI / MCP 듀얼 모드)

이 도구는 "인간용 CLI"와 "AI용 MCP 서버"를 통합합니다.

### 1. 인간용 CLI 모드
터미널에서 직접 로그를 보낼 수 있습니다.

```bash
# 기본 전송
mcp-logger send 192.168.1.40 "CLI에서 보낸 메시지"

# 옵션 포함 전송
mcp-logger send 192.168.1.40 "에러 발생" --severity error --tag my-app
```

터미널에서 인자 없이 `mcp-logger`를 실행하면 **MCP 설정 가이드**가 표시됩니다.

### 2. AI 어시스턴트 모드 (MCP)
`claude_desktop_config.json` 또는 해당 설정 파일에 다음 내용을 추가하십시오.

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

## MCP 도구 (Tools)
AI에게 제공되는 기능입니다.

### `send-syslog`
지정된 서버로 Syslog 메시지를 전송합니다.

**인자(Arguments):**
- `server` (string, 필수): Syslog 서버의 호스트 이름 또는 IP 주소.
- `port` (number): 포트 번호 (기본값: `514`).
- `facility` (string): 퍼실리티 (예: `user`, `local0`). 기본값은 `user`.
- `severity` (string): 심각도 (예: `info`, `error`). 기본값은 `info`.
- `tag` (string): 프로그램 이름(태그). 기본값은 `mcp-syslog`.
- `encoding` (string): 인코딩 (예: `utf-8`, `shift_jis`). 기본값은 `utf-8`.
- `message` (string, 필수): 로그 메시지 본문.

## 중요: 멀티바이트 문자 처리
한국어, 중국어, 일본어와 같이 2바이트 문자를 사용하는 언어를 위해 AI는 `utf-8`을 사용하도록 지시받습니다. 서버는 자동으로 **UTF-8 BOM**을 추가하여 Syslog 서버 및 후속 분석 도구가 인코딩을 정확하게 식별할 수 있도록 합니다.

## 문서
- [상세 매뉴얼](docs/MANUAL.ko.md)
- [플랫폼별 설정 가이드](docs/CROSS_PLATFORM_SETUP.ko.md)
- [LM Studio 설정 가이드](docs/LM_STUDIO_SETUP.ko.md)
- [MCP 프로토콜 예제](docs/MCP_PROTOCOL_EXAMPLES.ko.md)
- [사양서 (SPEC)](docs/SPEC.md)
