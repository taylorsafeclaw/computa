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

## Claude Code / MCP

The repo ships a project-scoped MCP config (`.mcp.json`, currently empty — no MCP
server is required to build computa) and project settings (`.claude/settings.json`,
with `enableAllProjectMcpServers: false`). These control **project-scoped** servers.

Disabling **user-scoped or plugin-scoped** MCP servers (e.g. the `claude.ai`
connectors, `plugin:posthog`, `plugin:supabase`) for this project only is **not
achievable** through checked-in or local non-managed settings: `deniedMcpServers`
is a managed-settings (admin-deployed) feature, and `enabled/disabledMcpjsonServers`
gate only `.mcp.json` servers. The only ways to stop a user/plugin-scoped server
are `claude mcp remove <name>` (global) or admin-managed settings. A best-effort
`.claude/settings.local.json` (gitignored, personal) may be present but is
unverified and likely a no-op on current versions.
