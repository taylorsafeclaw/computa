# Computa Repo Scaffold Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Scaffold the `computa` monorepo into a compiling, green-CI skeleton with full repo hygiene, agent/dev-environment setup, and Greptile open-source eligibility — without building the dictation pipeline.

**Architecture:** pnpm workspace (TS) layered over a cargo workspace (Rust). A Tauri v2 desktop app (`apps/desktop`) with a React+Vite+TS webview and a thin Rust core. Latency-critical logic lives in library crates (`crates/audio`, `crates/transcribe`) defined behind traits with placeholder impls. No heavy native deps (cpal/whisper-rs/enigo) are pulled in this scaffold — only trait boundaries — so the skeleton compiles fast and CI is green. Real implementations come in a follow-up plan.

**Tech Stack:** Rust 1.96, Tauri 2.11.2 (`tauri-build` 2.6.2, `tauri-plugin-global-shortcut` 2.3.2), React + Vite + TypeScript, `@tauri-apps/cli` 2.11.2 / `@tauri-apps/api` 2.11.0, `@deepgram/sdk` 5.4.0 (stub only), Vitest, pnpm 11, GitHub Actions CI.

**Reference spec:** `docs/superpowers/specs/2026-06-03-computa-scaffold-design.md`

**Two decisions to confirm before/at execution (spec §11):**
1. `.mcp.json` starts **empty** (no MCP server is required to build computa). Confirm, or name servers to include project-wide.
2. Disabling user-scoped MCP servers per-project may be impossible without managed settings/global removal. The plan attempts a gitignored local override and documents the fallback. Confirm this is acceptable.

---

### Task 1: Root workspace skeleton + hygiene files

**Files:**
- Create: `.gitignore`, `LICENSE`, `README.md`, `CONTRIBUTING.md`, `pnpm-workspace.yaml`, `package.json`, `Cargo.toml`, `rust-toolchain.toml`, `.npmrc`

- [ ] **Step 1: Create `.gitignore`**

```gitignore
# Rust
/target
**/target
Cargo.lock.bak

# Node / pnpm
node_modules
dist
.vite
*.tsbuildinfo

# Tauri
apps/desktop/src-tauri/target
apps/desktop/src-tauri/gen

# Local agent / editor config
.claude/settings.local.json
.DS_Store
*.log

# Secrets / models (never commit)
.env
.env.*
*.bin
models/
```

- [ ] **Step 2: Create `LICENSE` (MIT)**

```
MIT License

Copyright (c) 2026 Taylor Allen

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

- [ ] **Step 3: Create `README.md`**

```markdown
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

\`\`\`bash
pnpm install            # install JS deps
pnpm tauri:dev          # run the app
pnpm test               # JS tests (Vitest)
cargo test              # Rust tests
\`\`\`

See [`docs/development.md`](docs/development.md) for details.

## License

MIT — see [LICENSE](LICENSE).
```

- [ ] **Step 4: Create `CONTRIBUTING.md`**

```markdown
# Contributing to Computa

Thanks for your interest! Computa is MIT-licensed and contributions are welcome.

## Getting started

1. Install Rust (stable), Node 20+, and pnpm 9+.
2. `pnpm install`
3. `pnpm tauri:dev` to run, `cargo test` + `pnpm test` to test.

## Before opening a PR

Run the full check suite locally — CI runs the same:

\`\`\`bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
pnpm lint
pnpm typecheck
pnpm test
\`\`\`

## Conventions

- Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`…).
- Keep `src-tauri` thin; put logic in `crates/*` behind traits so it's testable.
- TS types that mirror the Rust IPC contract live in `packages/shared`.
- Architecture decisions are recorded in `docs/adr/`.
```

- [ ] **Step 5: Create `pnpm-workspace.yaml`**

```yaml
packages:
  - "apps/*"
  - "packages/*"
```

- [ ] **Step 6: Create root `package.json`**

```json
{
  "name": "computa",
  "version": "0.0.0",
  "private": true,
  "packageManager": "pnpm@11.4.0",
  "scripts": {
    "tauri:dev": "pnpm --filter @computa/desktop tauri dev",
    "tauri:build": "pnpm --filter @computa/desktop tauri build",
    "dev": "pnpm --filter @computa/desktop dev",
    "build": "pnpm -r build",
    "test": "pnpm -r --if-present test",
    "lint": "pnpm -r --if-present lint",
    "typecheck": "pnpm -r --if-present typecheck"
  }
}
```

- [ ] **Step 7: Create root `Cargo.toml` (cargo workspace)**

```toml
[workspace]
resolver = "2"
members = [
    "apps/desktop/src-tauri",
    "crates/audio",
    "crates/transcribe",
]

[workspace.package]
edition = "2021"
license = "MIT"
authors = ["Taylor Allen"]
repository = "https://github.com/taylorallen0913/computa"

[profile.release]
codegen-units = 1
lto = true
opt-level = "s"
panic = "abort"
strip = true
```

- [ ] **Step 8: Create `rust-toolchain.toml`**

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

- [ ] **Step 9: Create `.npmrc`**

```
auto-install-peers=true
```

- [ ] **Step 10: Commit**

```bash
git add .gitignore LICENSE README.md CONTRIBUTING.md pnpm-workspace.yaml package.json Cargo.toml rust-toolchain.toml .npmrc
git commit -m "chore: root workspace skeleton and hygiene files"
```

---

### Task 2: Scaffold the Tauri desktop app

**Files:**
- Create (generated): `apps/desktop/**` (via create-tauri-app)
- Modify: `apps/desktop/package.json` (rename), `apps/desktop/src-tauri/Cargo.toml` (workspace integration)

- [ ] **Step 1: Generate the app non-interactively**

Run from the repo root:

```bash
cd apps && pnpm create tauri-app@4 desktop --template react-ts --manager pnpm --yes ; cd ..
```

Expected: creates `apps/desktop/` with `src/` (React+TS+Vite), `src-tauri/` (Rust), `package.json`, `vite.config.ts`, `index.html`, `tsconfig.json`.

- [ ] **Step 2: Rename the JS package to a scoped name**

In `apps/desktop/package.json`, set the name field:

```json
{
  "name": "@computa/desktop"
}
```

Then add `lint` and `typecheck` scripts to the existing `scripts` block (keep generated `dev`/`build`/`preview`/`tauri`):

```json
{
  "scripts": {
    "lint": "tsc --noEmit",
    "typecheck": "tsc --noEmit"
  }
}
```

(If the template already provides a different typecheck, keep theirs and skip duplicates.)

- [ ] **Step 3: Integrate `src-tauri` into the cargo workspace**

Open `apps/desktop/src-tauri/Cargo.toml`. Ensure it does NOT declare its own `[workspace]` (create-tauri-app does not). Remove any `[profile.*]` sections (profiles must live only in the root `Cargo.toml` — leaving them in a workspace member is a hard cargo error). The `[package]`, `[dependencies]`, `[build-dependencies]`, and `[lib]` sections stay.

> **IMPORTANT — frontend must exist before cargo compiles the app crate.** The
> Tauri app crate embeds `apps/desktop/dist` at compile time via
> `tauri::generate_context!`. So `pnpm --filter @computa/desktop build` (which
> produces `dist/`) must run before any `cargo build`/`cargo test`/`cargo clippy`
> that touches the app crate. The pure crates (`computa-audio`, `computa-transcribe`)
> do not need this.

- [ ] **Step 4: Install JS deps and build the frontend (creates `dist/`)**

```bash
pnpm install
pnpm --filter @computa/desktop build
pnpm --filter @computa/desktop typecheck
```

Expected: install succeeds; `apps/desktop/dist/` is created; typecheck PASS (generated template is type-clean).

- [ ] **Step 5: Verify the Rust workspace compiles**

```bash
. "$HOME/.cargo/env"
cargo build
```

Expected: PASS. If cargo errors with `profiles for the non root package will be ignored` → a `[profile]` block is still in a member Cargo.toml; remove it (Step 3). If it errors that `frontendDist` / `../dist` doesn't exist → run Step 4's frontend build first.

- [ ] **Step 6: Commit**

```bash
git add apps pnpm-lock.yaml
git commit -m "feat: scaffold Tauri desktop app (React + Vite + TS)"
```

---

### Task 3: `crates/audio` — capture boundary + tested helper

**Files:**
- Create: `crates/audio/Cargo.toml`, `crates/audio/src/lib.rs`

- [ ] **Step 1: Create `crates/audio/Cargo.toml`**

```toml
[package]
name = "computa-audio"
version = "0.0.0"
edition.workspace = true
license.workspace = true

[dependencies]
```

- [ ] **Step 2: Write the failing test + the trait/impl in `crates/audio/src/lib.rs`**

```rust
//! Audio capture boundary for Computa.
//!
//! Real microphone capture (cpal) lands in a follow-up plan. For now this crate
//! defines the `AudioSource` trait (the seam consumers depend on) and a pure
//! sample-conversion helper that downstream transcription needs.

/// A source of mono f32 PCM audio frames at a known sample rate.
pub trait AudioSource {
    /// Sample rate in Hz of frames returned by [`AudioSource::next_frame`].
    fn sample_rate(&self) -> u32;
    /// Returns the next chunk of mono samples, or `None` when the source ends.
    fn next_frame(&mut self) -> Option<Vec<f32>>;
}

/// Placeholder source that yields no audio. Replaced by a cpal-backed source later.
pub struct SilentSource {
    pub sample_rate: u32,
}

impl AudioSource for SilentSource {
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn next_frame(&mut self) -> Option<Vec<f32>> {
        None
    }
}

/// Converts interleaved i16 PCM samples to normalized f32 in `[-1.0, 1.0]`.
pub fn i16_to_f32(samples: &[i16]) -> Vec<f32> {
    samples
        .iter()
        .map(|&s| s as f32 / i16::MAX as f32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i16_to_f32_normalizes_extremes_and_zero() {
        let out = i16_to_f32(&[i16::MAX, 0, i16::MIN + 1]);
        assert_eq!(out.len(), 3);
        assert!((out[0] - 1.0).abs() < 1e-6);
        assert!(out[1].abs() < 1e-6);
        assert!((out[2] + 1.0).abs() < 1e-3);
    }

    #[test]
    fn silent_source_reports_rate_and_no_frames() {
        let mut s = SilentSource { sample_rate: 16_000 };
        assert_eq!(s.sample_rate(), 16_000);
        assert!(s.next_frame().is_none());
    }
}
```

- [ ] **Step 3: Run the tests**

```bash
cargo test -p computa-audio
```

Expected: PASS (2 tests).

- [ ] **Step 4: Commit**

```bash
git add crates/audio
git commit -m "feat(audio): AudioSource trait + i16_to_f32 helper with tests"
```

---

### Task 4: `crates/transcribe` — transcriber boundary + null impl

**Files:**
- Create: `crates/transcribe/Cargo.toml`, `crates/transcribe/src/lib.rs`

- [ ] **Step 1: Create `crates/transcribe/Cargo.toml`**

```toml
[package]
name = "computa-transcribe"
version = "0.0.0"
edition.workspace = true
license.workspace = true

[dependencies]
```

- [ ] **Step 2: Write the trait, null impl, and tests in `crates/transcribe/src/lib.rs`**

```rust
//! Transcription boundary for Computa.
//!
//! Real local inference (whisper-rs, Metal) lands in a follow-up plan. For now
//! this crate defines the `Transcriber` trait so the rest of the app can be wired
//! and tested against a mock, plus a `NullTranscriber` placeholder.

/// Error returned by a [`Transcriber`].
#[derive(Debug, PartialEq, Eq)]
pub enum TranscribeError {
    /// The model or backend was not available.
    Unavailable,
}

/// Turns mono f32 PCM samples into text.
pub trait Transcriber {
    /// Transcribe the given samples (assumed 16 kHz mono) into text.
    fn transcribe(&self, samples: &[f32]) -> Result<String, TranscribeError>;
}

/// Placeholder transcriber that always returns an empty transcript.
pub struct NullTranscriber;

impl Transcriber for NullTranscriber {
    fn transcribe(&self, _samples: &[f32]) -> Result<String, TranscribeError> {
        Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_transcriber_returns_empty_string() {
        let t = NullTranscriber;
        assert_eq!(t.transcribe(&[0.0, 0.1, -0.1]), Ok(String::new()));
    }
}
```

- [ ] **Step 3: Run the tests**

```bash
cargo test -p computa-transcribe
```

Expected: PASS (1 test).

- [ ] **Step 4: Commit**

```bash
git add crates/transcribe
git commit -m "feat(transcribe): Transcriber trait + NullTranscriber with test"
```

---

### Task 5: `inject_text` Tauri command (stub) + Rust test

**Files:**
- Modify: `apps/desktop/src-tauri/src/lib.rs` (create-tauri-app puts the app entry here; `main.rs` calls `run()`)

- [ ] **Step 1: Inspect the generated entry point**

```bash
sed -n '1,80p' apps/desktop/src-tauri/src/lib.rs
```

Note the existing `run()` function and `tauri::Builder` chain (the template ships a `greet` command — leave it for now or remove in Step 2).

- [ ] **Step 2: Add the `inject_text` command and register it**

Edit `apps/desktop/src-tauri/src/lib.rs`. Add the command above `run()`:

```rust
/// Inject the given text at the current cursor position.
///
/// Stub: real injection (enigo / macOS Accessibility) lands in a follow-up plan.
/// For now it validates input and logs, returning Ok so the IPC path is exercisable.
#[tauri::command]
fn inject_text(text: String) -> Result<(), String> {
    if text.is_empty() {
        return Err("inject_text: empty text".into());
    }
    println!("[inject_text] would inject {} chars", text.chars().count());
    Ok(())
}
```

Then add `inject_text` to the existing `invoke_handler` in `run()`:

```rust
        .invoke_handler(tauri::generate_handler![inject_text])
```

(If the template already lists `greet`, change it to `tauri::generate_handler![greet, inject_text]`.)

- [ ] **Step 3: Add a unit test at the bottom of `lib.rs`**

```rust
#[cfg(test)]
mod tests {
    use super::inject_text;

    #[test]
    fn inject_text_rejects_empty() {
        assert!(inject_text(String::new()).is_err());
    }

    #[test]
    fn inject_text_accepts_nonempty() {
        assert!(inject_text("hello".into()).is_ok());
    }
}
```

- [ ] **Step 4: Run the tests**

The app crate embeds `dist/`, so ensure the frontend is built first (no-op if already built):

```bash
pnpm --filter @computa/desktop build
cargo test -p $(grep -m1 '^name' apps/desktop/src-tauri/Cargo.toml | sed 's/.*"\(.*\)".*/\1/')
```

Expected: PASS (2 tests). (The package name is the `name` in `apps/desktop/src-tauri/Cargo.toml`, typically `desktop` or `computa`.)

- [ ] **Step 5: Commit**

```bash
git add apps/desktop/src-tauri/src/lib.rs
git commit -m "feat(desktop): stub inject_text command with tests"
```

---

### Task 6: `packages/shared` — typed IPC binding

**Files:**
- Create: `packages/shared/package.json`, `packages/shared/tsconfig.json`, `packages/shared/src/ipc.ts`, `packages/shared/src/ipc.test.ts`, `packages/shared/vitest.config.ts`

- [ ] **Step 1: Create `packages/shared/package.json`**

```json
{
  "name": "@computa/shared",
  "version": "0.0.0",
  "private": true,
  "type": "module",
  "main": "src/index.ts",
  "scripts": {
    "test": "vitest run",
    "typecheck": "tsc --noEmit"
  },
  "dependencies": {
    "@tauri-apps/api": "2.11.0"
  },
  "devDependencies": {
    "typescript": "^5.6.0",
    "vitest": "^2.1.0"
  }
}
```

- [ ] **Step 2: Create `packages/shared/tsconfig.json`**

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "strict": true,
    "skipLibCheck": true,
    "noEmit": true,
    "types": ["vitest/globals"]
  },
  "include": ["src"]
}
```

- [ ] **Step 3: Create `packages/shared/vitest.config.ts`**

```ts
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: { globals: true, environment: "node" },
});
```

- [ ] **Step 4: Create `packages/shared/src/ipc.ts` — the single source of truth for the IPC contract**

```ts
import { invoke } from "@tauri-apps/api/core";

/** Names of every Tauri command the webview may invoke. */
export const Commands = {
  injectText: "inject_text",
} as const;

/** Arguments for the `inject_text` command (mirrors the Rust signature). */
export interface InjectTextArgs {
  text: string;
}

/** Inject text at the cursor via the Rust core. */
export function injectText(text: string): Promise<void> {
  const args: InjectTextArgs = { text };
  return invoke(Commands.injectText, args);
}
```

- [ ] **Step 5: Create `packages/shared/src/index.ts`**

```ts
export * from "./ipc";
```

- [ ] **Step 6: Write `packages/shared/src/ipc.test.ts`**

```ts
import { describe, it, expect, vi } from "vitest";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(() => Promise.resolve()),
}));

import { invoke } from "@tauri-apps/api/core";
import { injectText, Commands } from "./ipc";

describe("injectText", () => {
  it("invokes the inject_text command with a text arg", async () => {
    await injectText("hello");
    expect(invoke).toHaveBeenCalledWith(Commands.injectText, { text: "hello" });
  });
});
```

- [ ] **Step 7: Install and run the test**

```bash
pnpm install
pnpm --filter @computa/shared test
```

Expected: PASS (1 test).

- [ ] **Step 8: Commit**

```bash
git add packages/shared pnpm-lock.yaml
git commit -m "feat(shared): typed inject_text IPC binding with test"
```

---

### Task 7: `packages/deepgram` — cloud client stub + test

**Files:**
- Create: `packages/deepgram/package.json`, `packages/deepgram/tsconfig.json`, `packages/deepgram/vitest.config.ts`, `packages/deepgram/src/index.ts`, `packages/deepgram/src/url.ts`, `packages/deepgram/src/url.test.ts`

- [ ] **Step 1: Create `packages/deepgram/package.json`**

```json
{
  "name": "@computa/deepgram",
  "version": "0.0.0",
  "private": true,
  "type": "module",
  "main": "src/index.ts",
  "scripts": {
    "test": "vitest run",
    "typecheck": "tsc --noEmit"
  },
  "dependencies": {
    "@deepgram/sdk": "5.4.0"
  },
  "devDependencies": {
    "typescript": "^5.6.0",
    "vitest": "^2.1.0"
  }
}
```

- [ ] **Step 2: Create `packages/deepgram/tsconfig.json`**

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "strict": true,
    "skipLibCheck": true,
    "noEmit": true,
    "types": ["vitest/globals"]
  },
  "include": ["src"]
}
```

- [ ] **Step 3: Create `packages/deepgram/vitest.config.ts`**

```ts
import { defineConfig } from "vitest/config";

export default defineConfig({
  test: { globals: true, environment: "node" },
});
```

- [ ] **Step 4: Create `packages/deepgram/src/url.ts` — pure, testable streaming-URL builder**

```ts
/** Options for a Deepgram live-streaming connection. */
export interface DeepgramStreamOptions {
  model?: string;
  language?: string;
  /** Audio sample rate in Hz (must match the PCM frames sent from Rust). */
  sampleRate: number;
  /** Number of audio channels. */
  channels?: number;
  /** Emit interim (non-final) results. */
  interimResults?: boolean;
}

const LISTEN_ENDPOINT = "wss://api.deepgram.com/v1/listen";

/** Builds the Deepgram live-streaming websocket URL from options (pure function). */
export function buildDeepgramUrl(opts: DeepgramStreamOptions): string {
  const params = new URLSearchParams({
    encoding: "linear16",
    sample_rate: String(opts.sampleRate),
    channels: String(opts.channels ?? 1),
    model: opts.model ?? "nova-2",
    interim_results: String(opts.interimResults ?? true),
  });
  if (opts.language) params.set("language", opts.language);
  return `${LISTEN_ENDPOINT}?${params.toString()}`;
}
```

- [ ] **Step 5: Create `packages/deepgram/src/index.ts` — stub client surface**

```ts
export * from "./url";

/** Configuration for the (not-yet-implemented) streaming client. */
export interface DeepgramClientConfig {
  apiKey: string;
  sampleRate: number;
}

/**
 * Placeholder for the live streaming client. The real implementation (opening
 * the websocket via buildDeepgramUrl and piping PCM frames from Rust) lands in a
 * follow-up plan. Kept as a typed seam so the UI can be wired against it.
 */
export function createDeepgramClient(config: DeepgramClientConfig) {
  return {
    config,
    connect(): never {
      throw new Error("Deepgram streaming not implemented yet");
    },
  };
}
```

- [ ] **Step 6: Write `packages/deepgram/src/url.test.ts`**

```ts
import { describe, it, expect } from "vitest";
import { buildDeepgramUrl } from "./url";

describe("buildDeepgramUrl", () => {
  it("encodes required params with defaults", () => {
    const url = buildDeepgramUrl({ sampleRate: 16000 });
    expect(url).toContain("wss://api.deepgram.com/v1/listen?");
    expect(url).toContain("encoding=linear16");
    expect(url).toContain("sample_rate=16000");
    expect(url).toContain("channels=1");
    expect(url).toContain("model=nova-2");
    expect(url).toContain("interim_results=true");
  });

  it("includes language when provided", () => {
    const url = buildDeepgramUrl({ sampleRate: 48000, language: "en-US" });
    expect(url).toContain("language=en-US");
    expect(url).toContain("sample_rate=48000");
  });
});
```

- [ ] **Step 7: Install and run the test**

```bash
pnpm install
pnpm --filter @computa/deepgram test
```

Expected: PASS (2 tests).

- [ ] **Step 8: Commit**

```bash
git add packages/deepgram pnpm-lock.yaml
git commit -m "feat(deepgram): streaming URL builder + client stub with tests"
```

---

### Task 8: System tray icon

**Files:**
- Modify: `apps/desktop/src-tauri/src/lib.rs`
- Modify: `apps/desktop/src-tauri/Cargo.toml` (enable tray feature)

- [ ] **Step 1: Enable the tray feature for the `tauri` dependency**

In `apps/desktop/src-tauri/Cargo.toml`, add the `tray-icon` feature to the `tauri` dependency. Find the line like `tauri = { version = "2", features = [...] }` and ensure `"tray-icon"` is in the features list (add the `features` array if absent):

```toml
tauri = { version = "2.11.2", features = ["tray-icon"] }
```

- [ ] **Step 2: Build a tray icon in `run()`'s `setup` closure**

In `apps/desktop/src-tauri/src/lib.rs`, add imports at the top:

```rust
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
```

Then add a `.setup(...)` call to the `tauri::Builder` chain in `run()` (before `.run(...)`):

```rust
        .setup(|app| {
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Computa")
                .build(app)?;
            Ok(())
        })
```

- [ ] **Step 3: Verify it compiles**

```bash
pnpm --filter @computa/desktop build
cargo build -p $(grep -m1 '^name' apps/desktop/src-tauri/Cargo.toml | sed 's/.*"\(.*\)".*/\1/')
```

Expected: PASS. (A tray icon appearing is verified manually in Task 13's `pnpm tauri:dev` smoke.)

- [ ] **Step 4: Commit**

```bash
git add apps/desktop/src-tauri/Cargo.toml apps/desktop/src-tauri/src/lib.rs Cargo.lock
git commit -m "feat(desktop): add system tray icon"
```

---

### Task 9: Project `CLAUDE.md`

**Files:**
- Create: `CLAUDE.md`

- [ ] **Step 1: Create `CLAUDE.md` (lean, high-signal, project-specific)**

```markdown
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

\`\`\`bash
pnpm install            # JS deps
pnpm tauri:dev          # run the app (Rust + webview)
cargo test              # Rust tests
pnpm test               # JS tests (Vitest, all packages)
cargo fmt --check && cargo clippy --all-targets -- -D warnings
pnpm typecheck
\`\`\`

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
```

- [ ] **Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: add project CLAUDE.md"
```

---

### Task 10: `docs/` tree + ADRs

**Files:**
- Create: `docs/architecture.md`, `docs/development.md`, `docs/adr/0001-webview-only-ts-layer.md`, `docs/adr/0002-monorepo-pnpm-plus-cargo.md`, `docs/adr/0003-local-whisper-via-whisper-rs.md`

- [ ] **Step 1: Create `docs/architecture.md`**

```markdown
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
```

- [ ] **Step 2: Create `docs/development.md`**

```markdown
# Development

## Prerequisites

- Rust (stable) via rustup
- Node 20+ and pnpm 9+
- macOS (current target). Grant the app **Accessibility** permission for text
  injection once that lands.

## Setup

\`\`\`bash
pnpm install
\`\`\`

## Run

\`\`\`bash
pnpm tauri:dev
\`\`\`

## Test & checks (CI runs the same)

\`\`\`bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
pnpm lint
pnpm typecheck
pnpm test
\`\`\`

## Layout

See `CLAUDE.md` for the package/crate map and the IPC contract.
```

- [ ] **Step 3: Create `docs/adr/0001-webview-only-ts-layer.md`**

```markdown
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
```

- [ ] **Step 4: Create `docs/adr/0002-monorepo-pnpm-plus-cargo.md`**

```markdown
# 2. Monorepo: pnpm workspace over cargo workspace

Date: 2026-06-03
Status: Accepted

## Context

One repo holds both the Rust core and the TS frontend/packages.

## Decision

A pnpm workspace (`apps/*`, `packages/*`) layered over a cargo workspace
(`apps/desktop/src-tauri`, `crates/*`). Logic lives in library crates behind
traits so `src-tauri` stays thin and testable.

## Consequences

- `src-tauri/Cargo.toml` must not define `[profile.*]` (profiles live at root).
- New packages/crates are added only when needed (YAGNI), never pre-created empty.
```

- [ ] **Step 5: Create `docs/adr/0003-local-whisper-via-whisper-rs.md`**

```markdown
# 3. Local transcription via whisper-rs (Metal)

Date: 2026-06-03
Status: Accepted

## Context

Local, offline transcription is a core feature. Candidates: whisper-rs
(whisper.cpp bindings) vs candle (pure Rust).

## Decision

Use whisper-rs with Metal acceleration on macOS, behind the `Transcriber` trait
in `crates/transcribe`. The scaffold ships only the trait + a `NullTranscriber`;
the whisper-rs dependency and model loading land in a follow-up plan.

## Consequences

- Heavy native build (whisper.cpp) is deferred out of the scaffold to keep CI fast.
- The trait seam lets the UI and pipeline be built/tested before real inference.
```

- [ ] **Step 6: Commit**

```bash
git add docs/architecture.md docs/development.md docs/adr
git commit -m "docs: add architecture, development guide, and ADRs"
```

---

### Task 11: MCP scope (`.mcp.json`) + project Claude settings

> Confirm the two decisions at the top of this plan before doing this task.

**Files:**
- Create: `.mcp.json`, `.claude/settings.json`, `.claude/settings.local.json` (gitignored)

- [ ] **Step 1: Inventory currently-connected MCP servers**

```bash
claude mcp list
```

Record which are user/global-scoped (e.g. gbrain, supabase, posthog, node_repl, claude.ai connectors). These are the "unrelated" servers to disable for this project.

- [ ] **Step 2: Create `.mcp.json` (project-scoped servers, checked in)**

Default: no MCP server is required to build computa, so start with an empty registry. (Add entries here later if a real need appears, e.g. a GitHub MCP.)

```json
{
  "mcpServers": {}
}
```

- [ ] **Step 3: Create `.claude/settings.json` (checked in, project-wide)**

```json
{
  "$schema": "https://json.schemastore.org/claude-code-settings.json",
  "enableAllProjectMcpServers": false,
  "enabledMcpjsonServers": [],
  "disabledMcpjsonServers": [],
  "permissions": {
    "deny": ["Read(./.env)", "Read(./.env.*)"]
  }
}
```

Note: `enabledMcpjsonServers` / `disabledMcpjsonServers` only gate servers
defined in `.mcp.json`. They do NOT disable user-scoped servers.

- [ ] **Step 4: Attempt to disable user-scoped servers for this project (personal, gitignored)**

`.claude/settings.local.json` is already gitignored (Task 1). Create it listing the unrelated user-scoped servers from Step 1:

```json
{
  "deniedMcpServers": [
    { "serverName": "gbrain" },
    { "serverName": "supabase" },
    { "serverName": "posthog" },
    { "serverName": "node_repl" }
  ]
}
```

- [ ] **Step 5: Verify empirically whether the override works**

Restart Claude Code in this directory (or reload), then:

```bash
claude mcp list
```

- If the unrelated servers no longer load here → done; record the working mechanism in `docs/development.md` under a short "Claude Code / MCP" note.
- If they STILL load → `deniedMcpServers` is managed-settings-only on this version. Do NOT ship a config that silently does nothing. Instead: in `docs/development.md`, document that user-scoped MCP servers cannot be disabled per-project without either `claude mcp remove <name>` (global) or admin-managed settings, and leave `.claude/settings.local.json` removed or empty. Report the result to the user for a decision.

- [ ] **Step 6: Commit (checked-in files only; `.claude/settings.local.json` stays gitignored)**

```bash
git add .mcp.json .claude/settings.json
git commit -m "chore: project-scoped MCP config and Claude settings"
```

---

### Task 12: CI workflow

**Files:**
- Create: `.github/workflows/ci.yml`

- [ ] **Step 1: Create `.github/workflows/ci.yml`**

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  rust:
    runs-on: macos-14
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2
      - uses: pnpm/action-setup@v4
        with:
          version: 11
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: pnpm
      # The Tauri app crate embeds apps/desktop/dist at compile time, so the
      # frontend must be built before cargo compiles the app crate.
      - run: pnpm install --frozen-lockfile
      - run: pnpm --filter @computa/desktop build
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features -- -D warnings
      - run: cargo test --all

  js:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 11
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: pnpm
      - run: pnpm install --frozen-lockfile
      - run: pnpm typecheck
      - run: pnpm test
```

Note: the `rust` job runs on macOS because the Tauri app links against macOS
frameworks. The `js` job runs on Linux for speed (no native build).

- [ ] **Step 2: Lint the workflow locally (optional but recommended)**

```bash
[ -x "$(command -v actionlint)" ] && actionlint .github/workflows/ci.yml || echo "actionlint not installed; skipping"
```

Expected: no errors (or skipped).

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add Rust + JS GitHub Actions workflow"
```

---

### Task 13: Full local verification + push + Greptile application

**Files:** none (verification + external steps)

- [ ] **Step 1: Run the entire check suite green**

```bash
. "$HOME/.cargo/env"
pnpm install
pnpm --filter @computa/desktop build   # produce dist/ before cargo touches the app crate
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
pnpm typecheck
pnpm test
```

Expected: every command exits 0. Fix anything red before proceeding.

- [ ] **Step 2: Smoke-test the app launches with a tray icon**

```bash
pnpm tauri:dev
```

Expected: the app window opens and a Computa tray icon appears. Close it (Ctrl-C) once confirmed. (This is the only manual check.)

- [ ] **Step 3: Confirm working tree is clean and review the log**

```bash
git status
git log --oneline
```

Expected: clean tree; commits for each task above.

- [ ] **Step 4: Create the public GitHub repo and push**

This is the user's action (admin/owner). Suggested:

```bash
gh repo create computa --public --source=. --remote=origin --description "Local-first dictation for macOS (Tauri + Rust + Deepgram/Whisper)" --push
```

Expected: repo created at `github.com/<user>/computa`, `main` pushed, CI runs green.

- [ ] **Step 5: Apply to Greptile open source**

Greptile eligibility is now satisfied: public repo, **MIT LICENSE**, real README/CONTRIBUTING, green CI. The user (repo admin) completes the external steps:

1. Go to https://www.greptile.com/open-source and submit the form with name, email, and the repo link, confirming admin/owner status.
2. Install the Greptile GitHub app on the `computa` repo when prompted.

These cannot be automated (admin GitHub-app install) — surface them to the user as the final handoff.

---

## Notes on what is intentionally NOT in this scaffold

Deferred to a follow-up spec/plan (keeps the skeleton compiling fast and CI green):
real cpal mic capture, real whisper-rs + Metal inference and model downloads,
real Deepgram websocket streaming, real enigo/Accessibility text injection,
global-hotkey wiring, the recording-indicator window, and the settings/history UI.
