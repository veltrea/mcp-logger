# Google Antigravity (MCP) 설정 가이드

이 문서는 `mcp-logger`를 Google Antigravity(또는 기타 MCP 클라이언트)에서 사용하기 위한 플랫폼별 설정 절차를 설명합니다.

## 개요

`mcp-logger`는 표준 입출력(stdio)을 통해 JSON-RPC로 통신하는 단독 바이너리입니다.
Antigravity와 같은 클라이언트 설정 파일(`mcp_config.json`)에 "이 명령을 실행해 주세요"라고 등록하여 연동합니다.

---

## 1. macOS (현재 설정됨)

자동으로 설정되었습니다. `~/.gemini/antigravity/mcp_config.json`에 다음 설정이 추가되었습니다.

```json
"mcp-logger": {
  "command": "cargo",
  "args": [
    "run",
    "--release",
    "--quiet",
    "--manifest-path",
    "/path/to/your/rust-logger/mcp-logger/Cargo.toml"
  ]
}
```

※ 첫 실행 시 빌드가 진행되며, 두 번째 실행부터는 빠르게 시작됩니다.

---

## 2. Windows 11 (LLM-SVR1 / WORK1)

Windows 환경에서 Antigravity(Claude Desktop 등)를 사용하는 경우의 절차입니다.

### 사전 준비
1. Rust 설치 (`rustup-init.exe`)
2. 소스 코드를 `rust-logger` 폴더에 배치(Google Drive 동기화 중이라면 그대로 사용 가능)

### `mcp_config.json` 설정
Windows 경로 형식에 맞게 설정합니다.

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
        "C:\\path\\to\\your\\rust-logger\\mcp-logger\\Cargo.toml"
      ]
    }
  }
}
```
※ 경로(`C:\\Users\\...`)는 실제 배치 장소에 맞게 변경하십시오.

---

## 3. Ubuntu (Dev Server)

Ubuntu에서 Antigravity를 실행하는 경우의 절차입니다.

### 사전 준비
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential
```

### `mcp_config.json` 설정

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
        "/home/veltrea/DEV/rust-logger/mcp-logger/Cargo.toml"
      ]
    }
  }
}
```

---

## 문제 해결

- **빌드가 끝나지 않음**: 처음에는 의존성 라이브러리 컴파일이 필요하므로 수십 초 정도 걸릴 수 있습니다.
- **JsonRpc 오류**: `cargo run` 출력에 불필요한 로그가 섞이면 오류가 발생합니다. `--quiet` 옵션이 필수입니다. 또한 디버그용 로그는 `println!` 대신 `eprintln!`(표준 에러 출력)을 사용하도록 구현되어 있습니다.
