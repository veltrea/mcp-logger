# mcp-logger용 LM Studio 설정 가이드

LM Studio에서 `mcp-logger`를 사용하기 위한 설정 가이드입니다.
이 프로젝트는 LM Studio 특유의 "4가지 규칙"을 완전히 준수하도록 수정되었습니다.

## 특유 규칙 대응 현황

1.  **설치 경로**: 다른 클라이언트(Google Antigravity 등)에서는 임의의 절대 경로에서 작동하지만, LM Studio에서는 `~/.lmstudio/extensions/plugins/mcp/mcp-logger/` (Windows의 경우 `C:\Users\<USERNAME>\.lmstudio\extensions\plugins\mcp\mcp-logger\`)에 배치해야 정상적으로 인식되는 것을 확인했습니다.
2.  **프로토콜 버전**: 클라이언트(LM Studio)가 제시한 버전을 그대로 반환하도록 구현되었습니다(예: `2024-11-05`).
3.  **JSON-RPC 형식**: 필수적이지 않은 필드(`error`가 null인 경우 등)를 제외하고 응답하도록 구현되었습니다.
4.  **도구(Tool) 응답**: `content` 배열과 `isError` 플래그를 포함하는 형식으로 응답합니다.

## 설정 단계

LM Studio의 MCP 설정 파일(보통 `~/.lmstudio/config.json`이나 `mcp.json` 등, 버전에 따라 다르지만 GUI에서 설정 가능한 경우가 많음)에 다음을 추가하십시오.

### macOS / Linux (Ubuntu)

```json
"mcp-logger": {
  "command": "/Users/<USERNAME>/.local/bin/mcp-logger",
  "args": []
}
```
※ 경로 및 사용자 이름은 환경에 맞게 빌드된 바이너리 위치를 지정하십시오.

### Windows (LLM-SVR1 등)

```json
"mcp-logger": {
  "command": "C:\\Users\\<USERNAME>\\AppData\\Local\\Programs\\mcp-logger\\mcp-logger.exe",
  "args": []
}
```

## 동작 확인

1.  LM Studio를 실행하고 서버 연결(Initialize)이 성공하는지 확인하십시오(로그에 `Connected to mcp-logger` 등이 표시됩니다).
2.  채팅창에서 "Syslog 보내줘"라고 지시하여 도구가 인식되는지 확인하십시오.
