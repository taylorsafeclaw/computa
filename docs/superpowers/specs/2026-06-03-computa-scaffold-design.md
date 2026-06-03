# Computa — Repo Scaffold Design

**Date:** 2026-06-03
**Status:** Approved (design); plan pending
**Scope of this spec:** the *initial scaffold* of the repo — structure, tooling, hygiene, a compiling skeleton, and the agent/dev-environment setup (CLAUDE.md, docs, MCP scoping). **Not** the full dictation pipeline; that comes in follow-up specs.

## 1. Product (context)

Computa is a local-first dictation app in the spirit of SuperWhisper / WhisperFlow:

- Global hotkey → capture microphone → transcribe → inject the resulting text at the cursor in whatever app has focus.
- Two transcription providers, chosen at runtime: **local Whisper** (offline, on-device) and **Deepgram** (cloud streaming).
- Tray/menu-bar app with a floating recording indicator and a settings/history window.

The immediate business goal is **Greptile open-source eligibility**, which is satisfied by repo hygiene (MIT license, public repo, real docs/CI) rather than by product code.

## 2. Architecture

### 2.1 Process model (2 processes)

- **Rust core** (`src-tauri`) — the latency-critical, native path: global hotkey, mic capture, **local Whisper inference**, text injection, system tray, secret storage.
- **Webview (React + Vite + TypeScript)** — UI plus **all cloud/TS logic**: the Deepgram streaming client, optional LLM transcript cleanup, and the settings/history screens.

Decision: **webview-only TS layer** (no Node/Bun sidecar). A sidecar would add a bundled JS runtime, a third process, and IPC latency for no benefit — Deepgram's JS SDK runs in the webview over a websocket. The latency-critical path stays in Rust.

### 2.2 IPC contract

- **Audio Rust → webview:** binary PCM frames over Tauri v2's `Channel` API (not JSON-serialized).
- **Transcripts / commands webview → Rust:** `invoke` calls + events.
- Both transcription paths converge on a single `inject_text` command in Rust.
- TS types mirroring this contract live in `packages/shared` as the single source of truth.

### 2.3 Data flow

1. Hotkey (Rust) → start capture (cpal) → PCM frames.
2. **Local path:** frames → `transcribe` crate (whisper-rs, Metal on macOS) → text → `enigo` injects at cursor.
3. **Cloud path:** frames → `Channel` → webview → Deepgram WS → interim/final transcripts → webview returns final text via `invoke` → `enigo` injects.
4. Provider is a runtime setting; both paths end at `inject_text`.

## 3. Monorepo layout

A **pnpm workspace** (TS) layered over a **cargo workspace** (Rust). Start minimal; grow by adding packages/crates, never by pre-creating empty ones (YAGNI).

```
computa/
├── apps/
│   └── desktop/              # the Tauri app
│       ├── src/              # React/TS frontend (UI + Deepgram + cleanup)
│       ├── src-tauri/        # Rust core (Tauri commands, glue) — thin
│       └── package.json
├── packages/
│   └── shared/               # TS types mirroring the Rust IPC contract
├── crates/
│   ├── audio/                # cpal mic capture + resampling (lib crate)
│   └── transcribe/           # local Whisper via whisper-rs (lib crate)
├── docs/                     # see §6
├── Cargo.toml                # cargo workspace root
├── pnpm-workspace.yaml
├── package.json              # root TS workspace + scripts
├── .mcp.json                 # project-scoped MCP servers (see §7)
├── .claude/
│   ├── settings.json         # checked-in project settings
│   └── settings.local.json   # gitignored personal overrides (MCP disable)
├── CLAUDE.md                 # project agent instructions (see §5)
├── LICENSE                   # MIT
├── README.md
├── CONTRIBUTING.md
├── .gitignore
└── .github/workflows/ci.yml
```

Rationale: `audio` and `transcribe` are separate library crates so `src-tauri` stays thin and the latency-critical logic is unit-testable without launching the app. `packages/shared` is one source of truth for the IPC contract.

## 4. Key dependencies

**Rust:**
- `tauri` v2 + plugins: `global-shortcut`, built-in v2 tray. Secret storage for the Deepgram key via OS keychain (`keyring` crate) — chosen over `stronghold` for simplicity.
- `cpal` — mic capture; `enigo` — text injection (keystroke/paste).
- `whisper-rs` — local Whisper (whisper.cpp bindings; **Metal** acceleration on macOS).

**TypeScript:**
- React + Vite + TypeScript.
- `@deepgram/sdk` — Deepgram streaming client (webview).
- Vitest for tests.

## 5. CLAUDE.md (Karpathy best practice)

A lean, high-signal, **project-specific** file. The behavioral guidelines (no silent assumptions, no over-engineering, no orthogonal edits, explicit verification) already live in the user's global `~/.claude/CLAUDE.md`; the project file references rather than duplicates them. Contents:

- One-paragraph project overview.
- Monorepo map (the tree above, condensed).
- **Exact** dev commands: build, run (`pnpm tauri dev`), test (`cargo test`, `pnpm test`), lint/format/typecheck.
- The IPC contract summary + where the shared types live.
- Gotchas: macOS Metal build requirements, **Accessibility permission** required for text injection, where the Deepgram key is stored (keychain), local Whisper model download location.
- Pointers into `docs/`.

Kept short and high-signal; no narrative filler.

## 6. Docs structure

```
docs/
├── architecture.md           # the §2 architecture, maintained as it evolves
├── development.md             # setup, prerequisites, commands, troubleshooting
├── adr/                       # Architecture Decision Records
│   ├── 0001-webview-only-ts-layer.md
│   ├── 0002-monorepo-pnpm-plus-cargo.md
│   └── 0003-local-whisper-via-whisper-rs.md
└── superpowers/specs/         # design specs (this file lives here)
```

ADRs capture the decisions already made in brainstorming so they aren't re-litigated.

## 7. MCP scope + disabling unrelated servers

Goal: project-relevant MCP servers configured project-wide (checked in); unrelated user-scoped servers not active in this project.

**Verified mechanics:**
- `.mcp.json` (repo root, checked in) defines **project-scoped** servers shared with anyone who opens the repo.
- `enabledMcpjsonServers` / `disabledMcpjsonServers` (in settings) gate **only `.mcp.json` servers**, not user-scoped ones.
- `deniedMcpServers` / `allowedMcpServers` / `allowManagedMcpServersOnly` are **managed-settings** features (admin-deployed), not available in ordinary project/local settings.

**Hard limitation:** the user's currently-connected servers (gbrain, supabase, posthog, node_repl, claude.ai connectors) are **user/global-scoped**. There is **no checked-in, non-managed setting that disables a user-scoped server for a single project.**

**Approach (decision required during planning — see Open Questions):**
1. **Project-wide MCPs:** create `.mcp.json` containing only servers relevant to building computa. Default proposal: **none are required** for a Rust/Tauri/TS desktop app, so `.mcp.json` starts empty/minimal. (Re-add later if a real need appears, e.g. a GitHub MCP for PR work.)
2. **Disable unrelated:** because they are user-scoped, attempt a gitignored `.claude/settings.local.json` personal override and/or a per-project edit to `~/.claude.json`. **Verify empirically** which mechanism actually stops the servers from loading before declaring it done; if none works without managed settings, document that the only options are global removal (`claude mcp remove`) or managed settings, and stop there rather than shipping a config that silently does nothing.

## 8. Testing

- **Rust:** unit tests in `crates/audio` (resampling, frame chunking — pure functions) and `crates/transcribe` (model I/O behind a trait so it can be mocked). `src-tauri` stays thin enough to need little testing.
- **TS:** Vitest for the Deepgram client (mocked websocket) and for IPC contract types.
- **CI** (`.github/workflows/ci.yml`) runs: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `pnpm install`, `pnpm lint`, `pnpm typecheck`, `pnpm test`. Green CI is part of Greptile-friendly hygiene.

## 9. Scope of this scaffold (what gets built now)

Structure + tooling + hygiene + a **compiling skeleton** — not the dictation pipeline. Concretely:

- pnpm + cargo workspaces wired together.
- Tauri app that launches with a tray icon and an empty React window.
- A stubbed `inject_text` Tauri command (no-op / logs) with its TS binding in `packages/shared`.
- `crates/audio` and `crates/transcribe`: a trait + a placeholder impl + **one passing unit test each**.
- A stub Deepgram client module (TS) + **one passing Vitest test** (mocked).
- Hygiene: MIT `LICENSE`, `README.md`, `CONTRIBUTING.md`, `.gitignore`, CI workflow.
- Agent/dev setup: project `CLAUDE.md`, `docs/` tree with the three ADRs, `.mcp.json`, `.claude/settings*.json`.
- Everything compiles; `cargo test` and `pnpm test` pass green.
- Initial git commit(s) on `main`; ready to push to a public GitHub repo and apply to Greptile.

The real dictation pipeline (actual cpal capture, real whisper-rs inference, real Deepgram streaming, real enigo injection, hotkey wiring, settings UI) is **out of scope** here and gets its own spec/plan.

## 10. Platform assumption

**macOS-first.** whisper-rs uses Metal; text injection uses macOS Accessibility APIs via enigo. Cross-platform crates (cpal, enigo, whisper-rs) are chosen so Windows/Linux stay viable later, but only macOS is targeted/tested in this scaffold.

## 11. Open questions / decisions to confirm at plan review

1. **MCP selection:** confirm that `.mcp.json` should start empty (no MCP server is needed to build computa), or name any server to include project-wide.
2. **MCP disable mechanism:** accept that disabling user-scoped servers per-project may not be achievable without managed settings or global removal; confirm the gitignored-local-override attempt + documentation fallback is acceptable.
