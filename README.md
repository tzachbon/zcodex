# zcodex

`zcodex` is my fork of `ScriptedAlchemy/codex-native`.

This repo is not just the upstream OpenAI CLI. It is a working monorepo for:

- a Rust CLI and TUI
- a native Node SDK
- a TypeScript SDK
- multi-agent review, merge, and CI tooling
- local fork experiments and branding changes

## What lives here

- `codex-rs/`
  Rust workspace for the main CLI, TUI, MCP server, app-server, and shared crates.
- `sdk/native/`
  Native N-API SDK published as `@codex-native/sdk`.
- `sdk/typescript/`
  TypeScript SDK compatibility layer and JS-facing APIs.
- `codex-agents-suite/`
  Multi-agent workflows for diff review, merge solving, and CI fixing.
- `codex-cli/`
  Legacy TypeScript CLI package wiring and packaging assets.

## Repo remotes

This checkout is set up as a normal fork:

- `origin` -> `https://github.com/tzachbon/zcodex.git`
- `upstream` -> `https://github.com/ScriptedAlchemy/codex-native.git`

Typical sync flow:

```bash
git fetch upstream
git rebase upstream/main
git push origin main
```

## Quick start

### Rust CLI and TUI

From the repo root:

```bash
cd codex-rs
cargo run --bin zcodex
```

Run a one-off prompt:

```bash
cd codex-rs
cargo run --bin zcodex -- "explain this repo"
```

Non-interactive mode:

```bash
cd codex-rs
cargo run --bin zcodex -- exec "review the current diff"
```

### Native SDK

Build the workspace packages:

```bash
pnpm install
pnpm build
```

Use the native SDK:

```ts
import { Codex } from "@codex-native/sdk";

const codex = new Codex();
const thread = codex.startThread();
const turn = await thread.run("Summarize this repository");

console.log(turn.finalResponse);
```

Launch the native CLI wrapper:

```bash
pnpm cx
```

### Agents suite

```bash
pnpm --filter codex-agents-suite start
```

Direct entrypoints:

```bash
pnpm --filter codex-agents-suite run run:diff
pnpm --filter codex-agents-suite run run:merge
pnpm --filter codex-agents-suite run run:ci-fix
```

## Build and test

Rust workspace:

```bash
cd codex-rs
just fmt
cargo test -p codex-tui
```

JS workspace:

```bash
pnpm install
pnpm build
```

Full repo checks:

```bash
pnpm test
```

## Notes

- The Rust binary now exposes `zcodex` in addition to upstream naming paths that may still exist in parts of the repo.
- Top-level docs from upstream may still reference OpenAI branding in places. This README is the fork-level entrypoint.
- This repo is meant for local development first, not polished public distribution yet.

## Docs

- [Installing and building](./docs/install.md)
- [Contributing](./docs/contributing.md)
- [Native SDK docs](./sdk/native/README.md)
- [Agents suite docs](./codex-agents-suite/README.md)

Licensed under [Apache-2.0](./LICENSE).
