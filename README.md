# Psychohistory

A modular, open-source, Rust-based system dynamics framework designed to model macro-level economic, institutional, demographic, and structural trajectories of regional and global systems.

> **Research Objective:** Psychohistory aims to provide a computationally rigorous, non-linear alternative to traditional, static policy-forecasting models. By evaluating institutions as dynamic, multi-domain feedback networks rather than static parameters, the engine simulates complex counterfactual scenarios—ranging from micro-policy shocks to foundational transitions in governance systems.

---

## Core Principles & Ethical Mandate

Psychohistory is a public utility for humanity. Its development is guided by the belief that the tools used to simulate and understand global systems must never be weaponized for private financial gain or corporate optimization.

- **Public Good Focus:** This engine is designed to model systemic change for ecological, societal, and institutional resilience—not to maximize profitability or de-risk commercial portfolios.
- **Non-Commercial Exclusivity:** We explicitly reject the integration of corporate, profit-driven research incentives. If an institution cannot utilize this framework because of commercial funding restrictions, this engine is functioning as intended.

---

## Features & Capabilities

- **Multi-domain simulation**: Economy, governance, and demography as independent plugins.
- **Hierarchical, multi-country scenarios**: Load and simulate nested regions (e.g., US, California, Los Angeles).
- **Double-buffered state**: Deterministic, parallel-safe updates to maintain chronological consistency across domains.
- **Plugin architecture**: Add new domains or swap evaluators without touching the simulation core.
- **Automated scenario loading**: Ingests JSON scenario trees from `scenarios/`.
- **Per-field, colorized state diffing**: Human-readable, colorized before/after diffs for every state entity.
- **Deterministic output**: Stable, reproducible results across identical configurations.
- **CLI runner**: Run, diff, and report multi-run simulations from the command line.

---

### Why Psychohistory?

Unlike PySD, Vensim, Stella, or Minsky, Psychohistory offers:

- **Rust-native performance** with strict determinism
- **Modular plugin architecture** for easy domain extension
- **Native hierarchical regions** (country → state → city)
- **Research-focused tools** (colorized diffs, policy shocks, Python bindings)
- **Strong public-good license**

Built specifically for modeling complex institutional and civilizational dynamics.

---

## Workspace Structure

```bash
Psychohistory/
├── Cargo.toml         # Workspace manifest
├── AGENTS.md          # Agent instructions
├── LICENSE.md         # License
├── README.md          # Project documentation
├── cli/               # Command-line runner
│   ├── src/
│   │   ├── main.rs    # Entrypoint, scenario loader, runner
│   │   └── util.rs    # Formatting helpers
├── core/              # Simulation kernel
│   └── src/
│       ├── app.rs     # App struct, state diffing/reporting
│       ├── scheduler.rs
│       ├── state.rs
│       ├── plugin.rs
│       └── system.rs
├── lab/               # Python bindings (pyo3)
│   └── src/lib.rs
├── models/            # Domain state structs
│   └── src/lib.rs
├── plugins/           # Domain plugins
│   ├── demog/
│   ├── econ/
│   └── gov/
├── scenarios/         # Scenario JSON files
│   ├── simulation_config.json
│   ├── world_state.json
│   └── countries.json
├── sdk/               # Shared SDK for plugins
│   └── src/lib.rs
├── target/            # Build output
└── timebase/          # (Reserved for future time series features)
```

---

## Developer & Researcher Interfaces

To ensure accessibility for both systems engineers and domain scientists, the workspace enforces a strict separation of concerns across three layers:

1. **The Kernel (`core/`):** A data-oriented, double-buffered execution loop optimized for thread-safe, parallel region updates and deterministic state-space evaluation.
2. **The Research SDK (`sdk/`):** A high-level, declarative Rust interface. Researchers can implement custom domain plugins by writing pure mathematical state-transition functions without managing memory allocation, concurrency, or serialization.
3. **The Simulation Lab (`lab/`):** Native Python bindings powered by `pyo3`. This allows data scientists and macroeconomists to configure scenarios, schedule policy shocks programmatically, run massive Monte Carlo parameter sweeps, and ingest output directly into Jupyter Notebooks as NumPy arrays or Pandas DataFrames.

---

## Methodological State Tracking (V1 Placeholders vs. Target State)

The baseline simulation engine tracks a cross-domain array of metrics for each region and time-step.

> ⚠️ **Note to Researchers:** The active V1 implementation utilizes linear, statistical placeholder parameters (e.g., *Law Quality*, *Corruption Level*, *Public Trust*, *Legislative Speed*) to verify the parallel scheduler and diffing engines. Per the roadmap below, these are actively being replaced by conserved, empirical state-space vector representations.

- **Scenario-Driven Ingestion:** The architecture reads structural configurations from nested JSON inputs, defining initial parameters and systemic constraints for nations down to municipalities.
- **Cross-Domain Coupling:** Outcomes in one plugin domain dynamically feed into the state vectors of adjacent domains, preventing delayed tracking error and allowing real feedback loops.
- **Transition Logic:** The core loop supports real-time, scheduled or conditional institutional adjustments (e.g., sudden shifts in a region's regulatory or allocation systems) to track emergent trajectory changes.

---

## Command-Line Usage & Workflow

The CLI runner supports flexible simulation configuration:

```bash
psychohistory-cli [OPTIONS]
```

**Key options:**

- `--years <N>`: Number of years to simulate (default: 10)
- `--runs <N>`: Number of Monte Carlo runs per region (default: 10)
- `--scenario-dir <DIR>`: Path to scenario JSON directory (default: scenarios)
- `--log-dir <DIR>`: Log file output directory (default: logs)
- `-v, --verbose`: Increase logging verbosity

### Typical Workflow:

1. Define or modify initial parameters in `scenarios/simulation_config.json`.
2. Execute the runner command: `cargo run --bin cli -- --years 20 --runs 100`
3. Analyze aggregate mean and standard deviation outputs alongside colorized field diffs to examine the trajectory.

---

## Example Scenario JSON

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
- [ ] **Feedback Loop Validation:** Implement compile-time or initialization-time verification to prevent deadlocks or ordering bias when plugins depend recursively on cross-domain parameters (e.g., econ ↔ gov loops).

### 📊 Priority 3 — The Research Lab (Python & Jupyter Integration)

- [ ] **Complete `pyo3` Exposure:** Map the hierarchical `SimulationState` and `EngineRunner` structs directly to Python classes in the `lab/` crate.
- [ ] **High-Throughput Parameter Sweeps:** Build a native multi-threaded runner interface optimized for execution inside Python to handle sensitivity analyses and automated parameter calibration.

### 📚 Priority 4 — Theoretical Grounding & Documentation

- [ ] **Theory Attribution Standards:** Document every update rule and mathematical formula in the `plugins/` codebase using explicit LaTeX syntax, strictly citing the peer-reviewed economic, demographic, or sociological frameworks they derive from.

---

## License

This project is licensed under the **PolyForm Noncommercial License 1.0.0** — see `LICENSE.md` for details.

> **Notice to Contributors and Institutional Researchers:**  
> While receiving external or corporate research funding does not inherently disqualify you from using this framework, **proprietary capture is strictly prohibited**.  
> If your funding comes with "strings attached" that require non-disclosure agreements (NDAs), delayed public release, or the restriction of source code as proprietary intellectual property, you are explicitly barred from using this engine. Any architecture, plugins, models, or scenarios developed using Psychohistory must be made fully, transparently, and publicly available to the global community. We simulate open systems; we do not tolerate closed science.
