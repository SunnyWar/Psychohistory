# Psychohistory

A modular, open-source, Rust-based system dynamics framework designed to model macro-level economic, institutional, demographic, and structural trajectories of regional and global systems.

> **Research Objective:** Psychohistory aims to provide a computationally rigorous, non-linear alternative to traditional, static policy-forecasting models. By evaluating institutions as open, dynamic feedback networks rather than static parameters, the engine simulates complex counterfactual scenarios—ranging from micro-policy shocks to foundational transitions in governance systems.

---

## Core Principles & Ethical Mandate

Psychohistory is a public utility for humanity. Its development is guided by the belief that the tools used to simulate and understand global systems must never be weaponized for private financial gain or corporate optimization.

- **Public Good Focus:** This engine is designed to model systemic change for ecological, societal, and institutional resilience—not to maximize profitability or de-risk commercial portfolios.
- **Non-Commercial Exclusivity:** We explicitly reject the integration of corporate, profit-driven research incentives. If an institution cannot utilize this framework because of commercial funding restrictions, this engine is functioning as intended.

## 🛑 Current Status: Pre-Alpha / Not Ready for Prime Time

**Psychohistory is currently in an active, pre-alpha prototyping phase. It is NOT ready for production simulations, academic citations, or institutional policy forecasting.**

The underlying architecture has been refactored away from a hardcoded metrics struct to a fully decoupled, open blackboard pattern. While the repository is public to encourage open-source review, it will not be considered "ready for prime time" or open for public announcement until the following milestones are comprehensively cleared:

### 🟩 Crucial Pre-Announcement Milestones

- [ ] **Mathematical Rigor (State-Space Transition):** Complete the replacement of all legacy, linear metric placeholders with strict, non-linear system dynamics equations and conserved resource-pool vectors.
- [ ] **Robust Python Interface (`lab/`):** Fully expose the core execution loop and state data structures to Python via `pyo3`, enabling a researcher to run a complete simulation and extract Pandas DataFrames from a Jupyter Notebook without reading Rust code.
- [ ] **Cross-Platform Determinism Verification:** Implement automated CI testing to verify that floating-point calculations match identically down to the final decimal place across both `x86_64` (Intel/AMD) and `aarch64` (Apple Silicon/ARM) hardware architectures.
- [ ] **Zero-Warning Compilation & Audit:** Achieve a state of absolute compliance across the entire workspace, passing `cargo clippy -- -D warnings` and strict deterministic regression testing under heavy multi-threaded workloads.
- [ ] **Empirical Validation Blueprint:** Build one verified, end-to-end "Toy Scenario" that replicates a historical macroeconomic or demographic tipping point with documented, mathematically sound calibration data.

*If you are an academic researcher or systems engineer interested in building these foundational pillars, please review our `AGENTS.md` and the roadmap below before opening a Pull Request.*

---

## Features & Capabilities

- **Open Blackboard Architecture**: Eliminates brittle parameter wiring. Models register, write, and query dynamic variables via arbitrary string keys through a thread-safe blackboard layout.
- **Multi-Domain Parallel Simulation**: Economy, governance, and demography run as independent plugins over an isolated snapshot system.
- **Hierarchical, Multi-Country Scenarios**: Load and simulate nested regions (e.g., US, California, Los Angeles).
- **Double-Buffered State Execution**: Double-buffered `SimulationState` provides parallel-safe updates, ensuring zero delayed tracking error or chronological drift across domains during a tick.
- **Plugin Architecture**: Add new domains or swap evaluators by implementing high-level functional traits without touching the simulation kernel.
- **Per-Field, Colorized State Diffing**: Human-readable, colorized before/after structural diffs for every tracked state entity.
- **Deterministic Output**: Stable, reproducible results across identical configurations regardless of multi-threaded thread pool scheduling.
- **CLI Runner**: Execute, aggregate, and report multi-run Monte Carlo simulations directly from the terminal.

---

### Why Psychohistory?

Unlike PySD, Vensim, Stella, or Minsky, Psychohistory offers:

- **Rust-native performance** with absolute mathematical determinism.
- **A text-keyed Blackboard** allowing cross-domain data sharing without compile-time coupling or cyclic graph dependencies.
- **Native hierarchical regions** (country → state → city) evaluated cleanly.
- **Research-focused tools** (colorized diffs, policy shocks, Python bindings).
- **Strong public-good license**.

Built specifically for modeling complex institutional and civilizational dynamics.

---

## Workspace Structure

```bash
Psychohistory/
├── Cargo.toml         # Workspace manifest
├── AGENTS.md          # Agent instructions
├── LICENSE.md         # License
├── README.md          # Project documentation
├── cli/               # Command-line runner crate
│   └── src/
│       ├── main.rs    # Entrypoint, scenario loader, runner
│       └── util.rs    # Formatting and terminal layout helpers
├── core/              # Simulation kernel crate
│   └── src/
│       ├── app.rs     # Main simulation driver loop, lifecycle execution
│       ├── config.rs  # Context blocks, PRNG management, system configuration parameters
│       ├── entities.rs# Concrete definitions (e.g., GovernanceSystem, Legislator, YearOutcome)
│       ├── experiment.rs# Monte Carlo aggregator and stats engine
│       ├── legal/     # Legal framework processing submodules
│       │   ├── autocracy.rs
│       │   ├── democracy.rs
│       │   ├── mod.rs # LegalSystemModel trait abstraction
│       │   ├── monarchy.rs
│       │   └── other.rs
│       ├── scheduler.rs # Thread-safe orchestration for domain execution
│       ├── simulation.rs# Kernel pipeline driver and core pipeline functions
│       ├── state.rs   # Double-buffered, type-erased state architecture
│       └── system.rs  # Parallel execution definitions and snapshot captures
├── lab/               # Python bindings crate (pyo3)
│   └── src/lib.rs
├── models/            # Core domain enum definitions
│   └── src/lib.rs
├── plugins/           # Custom domain simulation plugins
│   ├── demog/         # Demographic state modeling
│   ├── econ/          # Macroeconomic modeling
│   └── gov/           # Operational regulatory tracking
├── scenarios/         # Ingestion layout configurations
│   ├── simulation_config.json
│   ├── world_state.json
│   └── countries.json
└── sdk/               # Shared plugin SDK crate
    └── src/
        ├── components.rs # Base components
        ├── influence.rs  # Cross-variable influence weights registry
        └── lib.rs        # Global Blackboard definition, traits, and Time types
```

## Developer & Researcher Interfaces

To ensure accessibility for both systems engineers and domain scientists, the workspace enforces a strict separation of concerns across three layers:

- **The Kernel (`core/`):** A data-oriented, double-buffered execution loop mapping type-erased maps (`HashMap<&'static str, Box<dyn Any + Send + Sync>>`) to concurrent workers via isolated `ReadSnapshot` instances.

- **The Research SDK (`sdk/`):** A high-level, declarative Rust interface. Contains the thread-safe, `RwLock`-backed `Blackboard` container. Researchers can query or mutate cross-domain variables inside sub-models via `blackboard.get("key")` and `blackboard.set("key", value)` without managing lock acquisition or compilation ordering.

- **The Simulation Lab (`lab/`):** Native Python bindings powered by `pyo3`. This allows data scientists and macroeconomists to configure scenarios, schedule policy shocks programmatically, run massive Monte Carlo parameter sweeps, and ingest output directly into Jupyter Notebooks as NumPy arrays or Pandas DataFrames.

### Methodological State Tracking (Open Blackboard Layout)

The engine tracks civilizational trajectories dynamically. Instead of static struct values passed through function signatures, systems utilize the shared `Blackboard` matrix during an execution tick.

#### Standard Processing Contract

Systems extract baseline conditions or parameters calculated by preceding plugins, perform non-linear operations, and broadcast outputs instantly back to the environment.

Key metric variables actively utilized by the historical pipeline include:

- `law_quality`: Evaluated from current organizational structures (`lobbying_pressure`, `donor_pressure`).
- `corruption_level`: Tracked using institutional integrity limits (`avg_integrity`, `reelection_pressure`, `normalized_wealth_influence`, `faction_formation`).
- `public_trust`: Tracks social cohesion based on output stability and systemic performance (`prior_trust`, `is_gridlocked`, `external_shock`, `media_impact`).
- `economic_outcome`: Aggregates the drag of structural policy outcomes on production metrics.

---

## Command-Line Usage & Workflow

The CLI runner supports flexible simulation configuration:

```bash
psychohistory-cli [OPTIONS]
```

Key options:

- `--years <N>`: Number of years to simulate (default: 10)
- `--runs <N>`: Number of Monte Carlo runs per region (default: 10)
- `--scenario-dir <DIR>`: Path to scenario JSON directory (default: `scenarios`)
- `--log-dir <DIR>`: Log file output directory (default: `logs`)
- `-v, --verbose`: Increase logging verbosity

### Typical Workflow

1. Define or modify initial parameters in `scenarios/simulation_config.json`.
2. Execute the runner:  
   ```bash
   cargo run --bin cli -- --years 20 --runs 100
   ```
3. Analyze aggregate mean and standard deviation outputs alongside colorized field diffs to examine the trajectory.

### Example Scenario JSON

```json
{
  "regions": {
    "us": {
      "components": {
        "econ": { "gdp": 27360000000000.0, "inflation": 0.024 },
        "gov": { "tax_rate": 0.21, "budget": 0.0, "stability": 0.85 },
        "demog": { "population": 336000000, "birth_rate": 0.012 }
      },
      "sub_regions": {
        "california": {
          "components": {
            "econ": { "gdp": 3860000000000.0, "inflation": 0.026 },
            "gov": { "tax_rate": 0.088, "budget": 0.0, "stability": 0.88 },
            "demog": { "population": 39000000, "birth_rate": 0.011 }
          }
        }
      }
    }
  }
}
```

---

## TODO (Priority-Ordered Roadmap)

### 🔥 Priority 1 — Mathematical & Engine Rigor

- [ ] **Transition from Linear Metrics to State-Space Models:** Refactor existing domain attributes to model conserved physical and institutional resource pools instead of subjective, linear values.
- [ ] **Non-Linear Tipping Points:** Implement sigmoid, step, and bifurcating transition functions to properly capture structural systemic collapses or sudden governmental transitions.
- [ ] **Strict Determinism Audit:** Enforce strict floating-point determinism across target architectures (e.g., addressing cross-platform `f64` rounding differences) to preserve absolute reproducibility in Monte Carlo runs.

### ⚡ Priority 2 — Cross-Domain Coupling & SDK Architecture

- [ ] **Zero-Copy State Ingestion:** Optimize `sdk/` traits to allow plugins instantaneous, read-only access to the comprehensive matrix of the previous time-step's world state.
- [ ] **Feedback Loop Validation:** Implement compile-time or initialization-time verification to prevent deadlocks or ordering bias when plugins depend recursively on cross-domain parameters (e.g., `econ ↔ gov` loops).

### 📊 Priority 3 — The Research Lab (Python & Jupyter Integration)

- [ ] **Complete `pyo3` Exposure:** Map the hierarchical `SimulationState` and `EngineRunner` structs directly to Python classes in the `lab/` crate.
- [ ] **High-Throughput Parameter Sweeps:** Build a native multi-threaded runner interface optimized for execution inside Python to handle sensitivity analyses and automated parameter calibration.

### 📚 Priority 4 — Theoretical Grounding & Documentation

- [ ] **Theory Attribution Standards:** Document every update rule and mathematical formula in the `plugins/` codebase using explicit LaTeX syntax, strictly citing the peer-reviewed economic, demographic, or sociological frameworks they derive from.

---

## License

This project is licensed under the **PolyForm Noncommercial License 1.0.0** — see `LICENSE.md` for details.

> **Notice to Contributors and Institutional Researchers:**  
> While receiving external or corporate research funding does not inherently disqualify you from using this framework, proprietary capture is strictly prohibited.
>
> If your funding comes with "strings attached" that require non-disclosure agreements (NDAs), delayed public release, or the restriction of source code as proprietary intellectual property, you are explicitly barred from using this engine. Any architecture, plugins, models, or scenarios developed using Psychohistory must be made fully, transparently, and publicly available to the global community.  
> *We simulate open systems; we do not tolerate closed science.*
