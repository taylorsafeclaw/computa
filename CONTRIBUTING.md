# Contributing to Computa

Thanks for your interest! Computa is MIT-licensed and contributions are welcome.

## Getting started

1. Install Rust (stable), Node 20+, and pnpm 9+.
2. `pnpm install`
3. `pnpm tauri:dev` to run, `cargo test` + `pnpm test` to test.

## Before opening a PR

Run the full check suite locally — CI runs the same:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
pnpm lint
pnpm typecheck
pnpm test
```

## Conventions

- Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`…).
- Keep `src-tauri` thin; put logic in `crates/*` behind traits so it's testable.
- TS types that mirror the Rust IPC contract live in `packages/shared`.
- Architecture decisions are recorded in `docs/adr/`.
