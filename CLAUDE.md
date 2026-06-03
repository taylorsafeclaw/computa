# Computa — Project Instructions

Local-first dictation app for macOS: global hotkey → mic capture → transcribe
(local Whisper **or** Deepgram cloud) → inject text at the cursor. Tauri v2 +
Rust core + React/TS webview.

> Behavioral coding rules (no silent assumptions, simplicity-first, surgical
> changes, verify before claiming done) live in the global `~/.claude/CLAUDE.md`
> and apply here. This file is project-specific context only.

## Layout

- `apps/desktop/` — the Tauri app. `src/` = React/TS webview (UI + Deepgram +
  cleanup). `src-tauri/` = Rust core (Tauri commands, tray, glue) — keep it THIN.
- `crates/audio/` — mic capture boundary (`AudioSource` trait) + sample helpers.
- `crates/transcribe/` — transcription boundary (`Transcriber` trait).
- `packages/shared/` — TS types mirroring the Rust IPC contract (source of truth).
- `packages/deepgram/` — Deepgram cloud streaming client.
- `docs/` — `architecture.md`, `development.md`, `adr/` (decision records).

## Commands

```bash
pnpm install            # JS deps
pnpm tauri:dev          # run the app (Rust + webview)
cargo test              # Rust tests
pnpm test               # JS tests (Vitest, all packages)
cargo fmt --check && cargo clippy --all-targets -- -D warnings
pnpm typecheck
```

## Architecture contract

- Audio goes Rust → webview as **binary PCM frames over Tauri's `Channel` API**
  (never JSON). Commands/transcripts go webview → Rust via `invoke` + events.
- Both transcription providers converge on the single `inject_text` command.
- Add new commands in `src-tauri` AND their typed binding in `packages/shared`.

## Gotchas

- **macOS-first.** Local Whisper will use Metal; text injection uses the macOS
  Accessibility API — the app must be granted Accessibility permission to type.
- Deepgram API key is stored in the OS keychain (never in the repo / `.env`).
- Local Whisper models download to a user data dir, not the repo (gitignored).
- `src-tauri/Cargo.toml` must NOT define `[profile.*]` — profiles live only in the
  root `Cargo.toml` (it's a cargo workspace).
