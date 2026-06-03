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
