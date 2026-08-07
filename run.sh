#!/usr/bin/env bash

cargo build
RUST_LOG=codecrafters_claude_code=debug,warn ./target/debug/codecrafters-claude-code -p "What is the capitol of Washington State?" 2> trace.jsonl
