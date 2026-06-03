# Computa

Local-first dictation for macOS — global hotkey, capture mic, transcribe with
**local Whisper** (offline) or **Deepgram** (cloud), and inject the text at your
cursor. In the spirit of SuperWhisper / WhisperFlow, built with Tauri + Rust + React.

> Status: early scaffold. The dictation pipeline is not implemented yet.

## Architecture

Two processes: a Rust core (mic capture, global hotkey, local Whisper, text
injection, tray) and a React/TypeScript webview (UI + Deepgram cloud client).
See [`docs/architecture.md`](docs/architecture.md).

## Development

Prerequisites: Rust (stable), Node 20+, pnpm 9+. macOS for now.

```bash
pnpm install            # install JS deps
pnpm tauri:dev          # run the app
pnpm test               # JS tests (Vitest)
cargo test              # Rust tests
```

See [`docs/development.md`](docs/development.md) for details.

## License

MIT — see [LICENSE](LICENSE).
