# Psychohistory

A high-performance, modular, open-source Rust-based system dynamics framework designed to model macro-level trajectories, non-linear state-space transformations, and emergent system dynamics over complex networks.

> **Research Objective:** Psychohistory provides a computationally rigorous, domain-agnostic execution kernel for running complex, non-linear simulations. By decoupling the execution framework from specific domain logic through an open blackboard architecture and type-erased parallel execution pools, the engine acts as a pure mathematical runtime capable of modeling civilizational transitions, macro-systems, and feedback-driven counterfactual scenarios.

---

## Core Principles & Ethical Mandate

Psychohistory is a public utility for humanity. Its development is guided by the belief that the tools used to simulate and understand global macro-systems must never be weaponized for private financial gain or corporate optimization.

- **Public Good Focus:** This engine is explicitly designed to model systemic change, resource thresholds, and structural resilience—not to maximize profitability or de-risk commercial financial portfolios.
- **Non-Commercial Exclusivity:** We completely reject the integration of corporate, profit-driven research incentives. If an institution cannot utilize this framework because of commercial funding restrictions, this engine is functioning exactly as intended.

## 🛑 Current Status: Pre-Alpha / Not Ready for Prime Time

**Psychohistory is currently in an active, pre-alpha prototyping phase. It is NOT ready for production simulations, academic citations, or institutional forecasting.**

The underlying architecture has completed its structural refactor away from domain-bound parameters to a fully type-erased, double-buffered execution environment. While the repository is public to encourage open-source verification, it will not be considered stable or open for public announcement until the following milestones are cleared:

### 🟩 Crucial Pre-Announcement Milestones

- [ ] **Mathematical Rigor (State-Space Transition):** Fully migrate remaining legacy downstream model footprints to strict state-space systems utilizing differential variations and conserved resource vectors.
- [ ] **Robust Python Interface (`lab/`):** Fully expose the generic engine execution loop and underlying type-erased state blocks to Python via `pyo3`, allowing researchers to execute simulations and ingest outputs directly into Jupyter Notebooks as Pandas DataFrames.
- [ ] **Cross-Platform Determinism Verification:** Enforce automated CI pipelines to guarantee that all internal mathematical operations and non-linear calculations yield bit-identical values across both `x86_64` and `aarch64` architectures.
- [ ] **Zero-Warning Compilation & Audit:** Clean the build path to achieve complete workspace validation, passing `cargo clippy --workspace --all-targets -- -D warnings` under heavy multi-threaded test harnesses.
- [ ] **Empirical Validation Blueprint:** Provide a complete, calibrated baseline scenario in our isolated demonstration modules mapping a historical macro-structural tipping point with verified empirical datasets.

---

## Features & Capabilities

- **Domain-Agnostic Core Engine**: The execution loop remains 100% blind to what is being simulated. It manages generic, type-erased math structures, maximizing architectural purity and reusability.
- **Open Blackboard Architecture**: Eliminates rigid compile-time parameter wiring. Models register, ingest, and mutate dynamic system variables globally via string keys (`blackboard.get()` / `blackboard.set()`) through a thread-safe matrix.
- **Double-Buffered State Execution**: Isolates parallel calculations cleanly within a double-buffered layout. Systems read from an immutable historical snapshot array during a tick, eliminating chronology drift, ordering bias, or delayed tracking errors.
- **Concurrently Scheduled Plugins**: External domains attach seamlessly by implementing the high-level `SimulationPlugin` trait. The execution loop coordinates execution across runtime worker pools without exposing core logic.
- **Rigid Primitive Typing**: Enforces explicit `f64` primitives across all calculations and boundaries, completely eliminating compilation inference bugs and ensuring uniform precision throughout performance-critical paths.
- **Hierarchical Scenarios**: Dynamically instantiates and tracks nested regional state nodes (e.g., Global → Continent → Region) uniformly.
- **CLI Suite**: Supports terminal-driven simulation triggering, scenario loading, and configuration management.

---

## Workspace Structure

```bash
Psychohistory/
├── Cargo.toml         # Workspace manifest
├── AGENTS.md          # AI agent structural constraints & guardrails
├── LICENSE.md         # PolyForm Noncommercial License 1.0.0
├── README.md          # Project documentation
├── cli/               # Command-line interface and configuration parser
│   └── src/
│       ├── main.rs    # CLI entrypoint and execution runner driver
│       └── util.rs    # Terminal formatting and display layout systems
├── core/              # Simulation Kernel (Strict Domain Isolation)
│   └── src/
│       ├── app.rs     # Main engine driver, state allocation, lifecycle hooks
│       ├── config.rs  # Context blocks, tick step delta tracker, PRNG management
│       ├── entities.rs# Pure mathematical base definitions (e.g., YearOutcome)
│       ├── example_social.rs # Isolated baseline plugin demonstrating social dynamics modeling
│       ├── experiment.rs# Multi-run statistics engine and calculation aggregator
│       ├── scheduler.rs # Thread-safe orchestration for multi-threaded system updates
│       ├── simulation.rs# Core engine execution loop; steps abstract plugins blindly
│       ├── state.rs   # Type-erased, double-buffered state space containment arrays
│       └── system.rs  # Parallel runner tasks and safe snapshot allocation mappings
├── lab/               # Python Bindings (pyo3 research laboratory suite)
│   └── src/lib.rs
├── models/            # Shared algebraic primitives and data types
│   └── src/lib.rs
├── plugins/           # Custom external system dynamics extensions
│   ├── demog/         # Demographic vector modeling
│   ├── econ/          # Macroeconomic and capital pool modeling
│   └── gov/           # Operational structural network tracking
├── scenarios/         # Ingestion layout configurations
│   ├── simulation_config.json
│   └── world_state.json
└── sdk/               # Shared developer SDK Crate
    └── src/
        ├── components.rs # Base functional components
        ├── influence.rs  # Cross-variable influence weights mapping
        └── lib.rs        # Thread-safe Blackboard, ReadSnapshot, and plugin trait primitives

```

---

## Developer & Researcher Interfaces

To maintain perfect structural isolation and scalability, the workspace enforces a strict three-tier layout boundary:

1. **The Core Engine (`core/`):** A pure data-oriented, double-buffered state-space executor mapping type-erased structures (`HashMap<&'static str, Box<dyn Any + Send + Sync>>`) to parallel execution pipelines. It has no structural knowledge of governance, finance, or population metrics.
2. **The Research SDK (`sdk/`):** The shared declarative trait interface. Hosts the `RwLock`-backed `Blackboard` container. Extensions fetch tracking indexes via `blackboard.get("key")` and publish updates using `blackboard.set("key", value)` without dealing with lock mechanics or module ordering dependencies.
3. **The Simulation Lab (`lab/`):** High-level Python bindings managed via `pyo3`. This allows macroeconomists, data scientists, and sociologists to design parameters, schedule targeted shocks, run sensitivity sweeps, and pull outputs into Jupyter environments cleanly.

### Extensible System Integration (Example Framework)

Because the engine core is strictly domain-blind, all specific structural relationships are injected via decoupled plugins. To demonstrate how complex social systems are modeled within these strict mathematical boundaries, an isolated reference implementation is provided in `core/src/example_social.rs`.

A plugin exposes a simple, functional boundary, processing tracking matrices dynamically without mutating the underlying scheduling framework:

```rust
use std::any::Any;
use sdk::{Blackboard, ReadSnapshot, SimulationPlugin, SimulationTime};

pub struct SocialCohesionPlugin;

impl SimulationPlugin for SocialCohesionPlugin {
    fn name(&self) -> &'static str { "social_cohesion_plugin" }

    fn step(
        &self,
        _world: &ReadSnapshot,
        my_state: &mut Box<dyn Any + Send + Sync>,
        time: SimulationTime,
        blackboard: &Blackboard,
    ) {
        // Ingest state indicators globally via text-keyed blackboard indices
        let friction: f64 = blackboard.get("social_friction");
        let resource_allocation: f64 = blackboard.get("resource_allocation");

        let delta_t: f64 = time.delta_years();
        
        // Model non-linear activation shifts utilizing robust sigmoid transformations
        let critical_threshold: f64 = (resource_allocation * 2.0 - 1.0).clamp(-1.0, 1.0);
        let net_activation: f64 = friction - critical_threshold;
        let target_instability: f64 = 1.0 / (1.0 + (-5.0 * net_activation).exp());

        // Compute final precise primitives safely clamped
        let calculated_stability: f64 = (1.0 - target_instability).clamp(0.0, 1.0);

        // Instantly route calculations back out to downstream modules
        blackboard.set("systemic_stability", calculated_stability);
    }
}

```

---

## Command-Line Usage & Workflow

Execute the command-line suite to drive scenarios:

```bash
psychohistory-cli [OPTIONS]

```

Key configuration flags:

- `--years <N>`: Total simulation steps (ticks) to advance (default: 10)
- `--runs <N>`: Iterations to process via Monte Carlo sweeps (default: 10)
- `--scenario-dir <DIR>`: Path to targeting data directory (default: `scenarios`)
- `-v, --verbose`: Toggle internal logging verbosity levels

### Sample Agnostic Scenario JSON

```json
{
  "regions": {
    "global_node_0": {
      "components": {
        "demographic_dynamics": { "input_density": 336000000.0, "rate_alpha": 0.012 },
        "allocation_dynamics": { "resource_throughput": 27360000000000.0, "entropy": 0.024 },
        "cohesion_dynamics": { "social_friction": 0.21, "systemic_stability": 0.85 }
      },
      "sub_regions": {
        "local_node_0a": {
          "components": {
            "demographic_dynamics": { "input_density": 39000000.0, "rate_alpha": 0.011 },
            "allocation_dynamics": { "resource_throughput": 3860000000000.0, "entropy": 0.026 },
            "cohesion_dynamics": { "social_friction": 0.088, "systemic_stability": 0.88 }
          }
        }
      }
    }
  }
}

```

---

## TODO Roadmap

### 🔥 Priority 1 — Core Execution Rigor

- [ ] **Strict Determinism Integration:** Lock down mathematical verification tests across tracking targets to squash sub-decimal cross-platform calculation variations in multi-threaded loops.
- [ ] **Advanced State-Space Topologies:** Provide unified multidimensional matrix utilities in the SDK layer for processing higher-order differential models out of the box.

### ⚡ Priority 2 — State Optimization & Vectorization

- [ ] **Zero-Copy Frame Interrogation:** Optimize snapshot retrieval layers inside `ReadSnapshot` to maximize hardware cache spatial locality during concurrent evaluation runs.
- [ ] **Order Dependency Validation:** Implement structural check passes during setup phases to capture cyclical calculation deadlocks across independent runtime plugins.

### 📊 Priority 3 — Research Lab Bindings

- [ ] **Comprehensive Crate Marshalling:** Wrap type-erased maps cleanly into native `pyo3` reference markers so external scientific work tools can stream internal tracking state directly.

---

## License

This project is licensed under the **PolyForm Noncommercial License 1.0.0** — see `LICENSE.md` for explicit terms.

> **Notice to Contributors and Institutional Researchers:** > While external research grants or academic funding setups do not disqualify you from leveraging this architecture, **proprietary capture is strictly prohibited**.
> If an enterprise or funding entity imposes non-disclosure agreements (NDAs), release delays, or requires any source modifications to be maintained under closed intellectual property barriers, you are explicitly prohibited from building on this engine. All plugins, scenarios, and adaptations must remain transparent, open, and accessible to the global research community. We simulate open systems; we do not tolerate closed science.
