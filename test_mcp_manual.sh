#!/bin/bash
cd "$(dirname "$0")"

# Compile first to ensure binary exists
cargo build --quiet

# Run interactive test
# We send initialize, then tools/list, then a dummy call, then exit.
(
    echo '{"jsonrpc": "2.0", "method": "initialize", "id": 1}'
    sleep 0.5
    echo '{"jsonrpc": "2.0", "method": "tools/list", "id": 2}'
    sleep 0.5
    echo '{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "send-syslog", "arguments": {"server": "127.0.0.1", "message": "Test from manual verification script"}}, "id": 3}'
) | cargo run --quiet -- run-mcp
