# AGENTS.md

This document provides persistent instructions, structural mapping, and behavioral guardrails for AI coding agents and Copilot assistants working in the Psychohistory Rust workspace.

---

## Project Purpose & Identity

Psychohistory is a public-utility, open-science system dynamics framework designed to model macro-level economic, institutional, demographic, and structural trajectories of global systems.

- **Target Audience:** Academic researchers, macroeconomists, and domain scientists.
- **Licensing Enforcement:** PolyForm Noncommercial License 1.0.0[cite: 5]. Do not write or suggest code patterns dependent on proprietary enterprise crates or closed ecosystems[cite: 5]. Everything must remain open-source and modular.

---

## Architectural Mandates & Open Blackboard Architecture

When generating, modifying, or refactoring code, you must discard standard web engineering paradigms and adopt a scientific-computing framework with a strict separation of concerns:

1. **The Shared Blackboard System:** The monolithic, hardcoded state structures are obsolete. All systems and domain plugins and extension modules interoperate dynamically via the thread-safe `Blackboard` container defined in the `sdk` crate.
2. **Data Consumption & Modification Contract:**
   - Systems and models must read metrics globally using string keys: `blackboard.get("metric_name")`[cite: 4].
   - Systems must write or update metrics via: `blackboard.set("metric_name", value)`.
   - **Do not** attempt to pass fields via explicit struct fields or invent concrete parameters on `SimulationState`[cite: 4]. Keep the blackboard dynamic and decoupled.
3. **Reject Naive Linear Logic:** Never use simple incremental additions (e.g., `metric += adjustment`) for domain loops[cite: 5]. Implement non-linear representations, state-space models, and explicit threshold/bifurcation functions (such as sigmoids for system tipping points).
4. **Explicit Floating-Point Typing:** When declaring numerical literals or target bounds for variables inside calculation blocks (e.g., applying bounds via `.clamp(0.0, 1.0)`), you must explicitly type bindings as `f64` (e.g., `let mut target: f64 = 0.6;`) to prevent compiler type ambiguity errors[cite: 4].
5. **Theory-Driven Documentation:** Every state transition formula or blackboard update rule you write must include triple-slash (`///`) documentation containing explicit mathematical equations (using LaTeX) and a placeholder referencing the peer-reviewed economic or sociological theory it models.

---

## Workspace Layout Clues

Before writing code, match your generation against the true structure of the workspace crates:

- **`sdk`**: Host of the shared components, influence weights registry, and the global thread-safe `Blackboard` type.

- **`core`**: Contains the execution engine, the double-buffered `SimulationState`, parallel system runner definitions (`ReadSnapshot`), and the legal models (`core/src/legal/`).

- **`plugins-gov` / `plugins-econ` / `plugins-demog`**: External system plugin executors that feed into the running kernel.

---

## Exploration Workflow (Critical)

Prefer `ast-outline` for efficient navigation to minimize context bloating and prevent stale code references:

- Unfamiliar directory — `ast-outline digest .` (or `<dir>`)
- Single file structure — `ast-outline <file.rs>`
- Specific item — `ast-outline show <file.rs> SymbolName`
- Implementors — `ast-outline implements <Trait> .`

Only read full file contents when you need implementation details beyond signatures. Never dump entire large files into context unless requested.

---

## Development & Verification Commands

Always verify changes locally with this precise sequence before considering a task complete:

```bash
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo fmt --all -- --check
```

## Prompt Engineering & Context Layering

When navigating or generating code for complex engineering tasks, always pull context from our localized architectural prompt files:

### Macro System Architecture

 Refer to .github/prompts/arch-direction.prompt for explicit mandates regarding non-linear system dynamics, state-space representations, and cross-domain coupling abstractions.

### Context Anchoring

Use the #file:.github/prompts/arch-direction.prompt syntax in the chat interface when initiating major refactors of the engine kernel or domain plugins to ensure strict alignment with our mathematical standards.
