# Architecture

Computa runs as **two processes**:

1. **Rust core** (`apps/desktop/src-tauri`, with logic in `crates/*`) — the
   latency-critical native path: global hotkey, mic capture, local Whisper
   inference, text injection, system tray, secret storage.
2. **Webview** (`apps/desktop/src`, React + TS) — UI plus all cloud/TS logic:
   the Deepgram streaming client and optional LLM transcript cleanup.

## IPC contract

- **Audio Rust → webview:** binary PCM frames over Tauri v2's `Channel` API.
- **Commands/transcripts webview → Rust:** `invoke` + events.
- Both transcription paths converge on the `inject_text` command.
- TS types mirroring this contract live in `packages/shared`.

## Data flow

1. Hotkey (Rust) → start capture → PCM frames.
2. Local path: frames → `crates/transcribe` (whisper-rs, Metal) → text → inject.
3. Cloud path: frames → `Channel` → webview → Deepgram WS → transcripts →
   webview returns final text via `invoke` → inject.
4. Provider is a runtime setting; both paths end at `inject_text`.

See `docs/adr/` for the decisions behind this shape.
