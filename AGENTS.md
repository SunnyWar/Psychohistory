# AGENTS.md

This document provides persistent instructions and behavioral guardrails for AI coding agents and Copilot assistants working in the Psychohistory Rust workspace.

---

## Project Purpose & Identity

Psychohistory is a public-utility, open-science system dynamics framework designed to model macro-level economic, institutional, demographic, and structural trajectories of global systems.

* Target Audience:** Academic researchers, macroeconomists, and domain scientists.

* Licensing Enforcement:** PolyForm Noncommercial License 1.0.0. Do not write or suggest code patterns dependent on proprietary enterprise crates or closed ecosystems. Everything must remain open-source and modular.

---

## Architectural Mandates & Math Style

When generating or refactoring code, you must reject generic web/software engineering paradigms and adopt a scientific-computing mindset:

1. **Reject Naive Linear Logic:** Never use simple incremental modifications (e.g., `metric += adjustment`) for domain traits. Implement non-linear representations, state-space models, and explicit threshold/bifurcation functions (e.g., sigmoids for tipping points).
2. **Conserved Physical Quantities:** Focus models on tracking actual physical or systemic resource pools (capital, demographics, energy throughput) rather than uncalibrated, abstract metrics.
3. **Decoupled Architecture Layers:** Keep a strict boundary between the execution kernel (`core/`) and the research interfaces (`sdk/`, `lab/`). Researchers must be able to write mathematical plugins via the SDK without being exposed to parallel execution, multi-threading, or low-level memory buffers.
4. **Theory-Driven Documentation:** Every state transition formula or plugin update rule you write must include triple-slash (`///`) documentation containing explicit mathematical equations (using LaTeX) and a placeholder referencing the peer-reviewed economic or sociological theory it models.

---

## Exploration Workflow (Critical)

Prefer `ast-outline` for efficient navigation to minimize context bloating:

* Unfamiliar directory — `ast-outline digest .` (or `<dir>`)

* Single file structure — `ast-outline <file.rs>`

* Specific item — `ast-outline show <file.rs> SymbolName`

* Implementors — `ast-outline implements <Trait> .`

Only read full file contents when you need implementation details beyond signatures. Never dump entire large files into context unless requested.

---

## Development & Verification Commands

Always verify changes locally with this precise sequence before considering a task complete:

```bash
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo fmt --all -- --check
---

## Prompt Engineering & Context Layering

When navigating or generating code for complex engineering tasks, always pull context from our localized architectural prompt files:
* **Macro System Architecture:** Refer to `.github/prompts/arch-direction.prompt` for explicit mandates regarding non-linear system dynamics, state-space representations, and cross-domain coupling abstractions.
* **Context Anchoring:** Use the `#file:.github/prompts/arch-direction.prompt` syntax in the chat interface when initiating major refactors of the engine kernel or domain plugins to ensure strict alignment with our mathematical standards.
