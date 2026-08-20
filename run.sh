#!/usr/bin/env bash
export HTTP_PROXY=http://127.0.0.1:8888
export HTTPS_PROXY=http://127.0.0.1:8888
cargo build
RUST_LOG=codecrafters_claude_code=debug,warn ./target/debug/codecrafters-claude-code -p "Echo 'Hello, world!' to the console." 2> trace.jsonl
