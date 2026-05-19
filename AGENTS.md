# AGENTS.md

This document provides persistent instructions for AI coding agents working in this Rust project.

## Project Overview
This is a **Rust workspace** (Cargo). Follow Rust idioms, safety, and performance best practices at all times.

## Exploration Workflow (Critical)
Prefer `ast-outline` for efficient navigation:

1. **Unfamiliar directory** — `ast-outline digest .` (or `<dir>`)
2. **Single file structure** — `ast-outline <file.rs>`
3. **Specific item** — `ast-outline show <file.rs> SymbolName` (supports suffix matching)
4. **Implementors** — `ast-outline implements <Trait> .`

Only read full file contents when you need implementation details beyond signatures.

**Never** dump entire large files into context unless necessary.

## Development Commands

**Always verify changes with this sequence:**

```bash
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo fmt --all -- --check
Helpful variants:

cargo test <test_name> -- --exact — run specific test
cargo test --lib / --bins / --examples
cargo nextest run (if using nextest)
cargo build --release for performance validation

Code Style & Rules (MUST Follow)

Write idiomatic, safe Rust. Prefer safe code; use unsafe only with strong justification and clear safety comments.
Use thiserror + anyhow (or equivalent) for error handling.
Prefer async/await with tokio (if async code is present).
Follow Rust API Guidelines (rust-lang.github.io/api-guidelines).
Use rustfmt defaults unless project-specific .rustfmt.toml exists.
Comprehensive documentation: Public items must have /// docs. Internal complex logic should be documented.
Tests: Unit tests near the code (#[cfg(test)] mod tests), integration tests in tests/.
Use #[derive(...)] aggressively (Debug, Clone, PartialEq, etc. where appropriate).
Avoid unwrap()/expect() in production code unless context makes panic impossible. Use proper error propagation.

Architecture & Patterns

Respect existing module structure and crate boundaries.
Prefer small, focused crates in the workspace when it improves compile times or reusability.
Use pub(crate) liberally for internal APIs.
Follow established patterns in the codebase for logging, configuration, error handling, and testing.

Tooling

Primary tools: cargo, rust-analyzer, clippy, rustfmt
Recommended: cargo-nextest, cargo-expand, cargo-udeps, cargo-outdated
Linting is done via Clippy with -D warnings

When Making Changes

Understand the affected area using ast-outline.
Make minimal, focused changes.
Update/add tests.
Run the full verification sequence above.
Ensure no new warnings are introduced.
Keep changes backward-compatible unless explicitly breaking (document in PR).

Agent Behavior Rules

Be concise in responses but thorough in reasoning.
Never suggest todo!(), unimplemented!(), or placeholder code.
Always consider performance, memory usage, and compile times in Rust.
If unsure about project conventions, explore the existing code first.