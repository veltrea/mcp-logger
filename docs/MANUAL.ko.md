# MCP Logger - 상세 매뉴얼

## 1. 소개
`mcp-logger`는 AI 어시스턴트와 Syslog 인프라를 연결하기 위해 설계된 MCP 서버입니다. AI가 Syslog를 활용하여 로그 기록 및 상태 추적을 수행할 수 있도록 합니다.

## 2. 설정 방법

### Claude Desktop
`%APPDATA%\Claude\claude_desktop_config.json` (Windows) 또는 `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS)에 다음 설정을 추가합니다.

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

## 3. 도구 참조 (Tool Reference)

### `send-syslog`
Syslog 서버로 메시지를 전송합니다.

| 파라미터 | 타입 | 기본값 | 설명 |
| :--- | :--- | :--- | :--- |
| `server` | string | (필수) | 대상 서버의 IP 또는 호스트 이름. |
| `port` | number | 514 | UDP 포트 번호. |
| `facility` | string | user | Syslog 퍼실리티 (kern, user, mail 등) |
| `severity` | string | info | Syslog 심각도 (emerg, alert, crit, err, warning, notice, info, debug) |
| `tag` | string | mcp-syslog | 프로그램 이름/태그. |
| `encoding` | string | utf-8 | 문자 인코딩. |
| `message` | string | (필수) | 로그 내용. |

## 4. 멀티바이트 문자 가이드 (2바이트 문자권 사용자를 위한 지침)
한국어, 일본어, 중국어, 베트남어 등 2바이트 문자를 사용하는 지역에서는 다음 가이드라인을 따르십시오.

### 권장 사항: UTF-8 사용
`mcp-logger`는 UTF-8에 최적화되어 있습니다. 메시지에 비 ASCII 문자가 포함되어 있는지 자동으로 감지하고, 필요한 경우 **UTF-8 BOM (Byte Order Mark)**을 자동으로 부여합니다.

### Syslog 서버와의 연동
많은 Windows 기반 Syslog 서버(`vlt-syslogd` 등)나 최신 Linux `rsyslog` 설정은 인코딩이 올바르게 식별되면 UTF-8을 처리할 수 있습니다. BOM을 부여함으로써 이러한 도구들이 로그를 로컬 레거시 인코딩(CP932, EUC-KR 등)으로 오인하는 것을 방지합니다.

## 5. 문제 해결
- **AI가 도구를 인식하지 못함**: 설정 파일의 경로가 절대 경로인지 확인하십시오.
- **로그가 도착하지 않음**: 대상 서버의 방화벽에서 UDP 포트 514가 허용되어 있는지 확인하십시오.
- **글자 깨짐 발생**: `encoding` 파라미터가 `utf-8`로 설정되어 있는지 확인하십시오. AI 모델은 일반적으로 UTF-8을 선호합니다.

## 6. 보충 가이드
- [플랫폼별 설정 가이드](CROSS_PLATFORM_SETUP.ko.md)
- [LM Studio 설정 가이드](LM_STUDIO_SETUP.ko.md)
- [MCP 프로토콜 예제](MCP_PROTOCOL_EXAMPLES.ko.md)
- [사양서 (SPEC)](SPEC.md)
