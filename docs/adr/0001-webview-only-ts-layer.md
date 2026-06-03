# 1. Webview-only TypeScript layer (no Node/Bun sidecar)

Date: 2026-06-03
Status: Accepted

## Context

The TS/cloud logic (Deepgram, future LLM cleanup) needs a home. Options were a
bundled Node/Bun sidecar, the webview itself, or doing it all in Rust.

## Decision

Run all TS/cloud logic in the webview. Deepgram's JS SDK works over a websocket
from the webview; audio reaches it from Rust via Tauri's binary `Channel` API.

## Consequences

- Two processes only; no bundled JS runtime; smaller binary.
- The latency-critical path (mic → local Whisper → injection) stays in Rust.
- If a true hidden background service is ever needed, revisit with a sidecar.
