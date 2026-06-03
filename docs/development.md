# Development

## Prerequisites

- Rust (stable) via rustup
- Node 20+ and pnpm 9+
- macOS (current target). Grant the app **Accessibility** permission for text
  injection once that lands.

## Setup

```bash
pnpm install
```

## Run

```bash
pnpm tauri:dev
```

## Test & checks (CI runs the same)

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
pnpm lint
pnpm typecheck
pnpm test
```

## Layout

See `CLAUDE.md` for the package/crate map and the IPC contract.
