# Psychohistory

A modular, open-source, Rust-based simulation framework for global economic, political, demographic, and institutional dynamics.

---

## Features & Capabilities

- **Multi-domain simulation**: Economy, governance, and demography as independent plugins
- **Hierarchical, multi-country scenarios**: Load and simulate nested regions (e.g., US, California, Los Angeles)
- **Double-buffered state**: Deterministic, parallel-safe updates
- **Plugin architecture**: Add new domains or swap evaluators without touching the core
- **Automated scenario loading**: Ingests JSON scenario trees from `scenarios/`
- **Per-field, colorized state diffing**: Human-readable, colorized before/after diffs for every entity
- **Deterministic output**: Stable, reproducible results
- **CLI runner**: Run, diff, and report simulations from the command line
- **Ready for extension**: Add new plugins, models, or scenario files easily

---

## Workspace Structure

```
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



## Command-Line Usage

The CLI runner supports flexible simulation configuration:

```
psychohistory-cli [OPTIONS]
```

**Key options:**

- `--years <N>`: Number of years to simulate (default: 10)
- `--runs <N>`: Number of Monte Carlo runs per region (default: 10)
- `--scenario-dir <DIR>`: Path to scenario JSON directory (default: scenarios)
- `--log-dir <DIR>`: Log file output directory (default: logs)
- `-v, --verbose`: Increase logging verbosity (repeat for more detail)

Example:

```
psychohistory-cli --years 20 --runs 100 --scenario-dir scenarios
```

This will run each region for 20 years, performing 100 Monte Carlo runs per region, and print aggregate statistics.

---

## What to Expect as a User

Psychohistory is designed for users who want to simulate and analyze the evolution of complex societies across economic, political, and demographic domains. When using this application, you can expect:

- **Rich Output Metrics**: The simulation tracks and reports a variety of metrics for each region and year, including:
  - Law Quality
  - Corruption Level
  - Public Trust
  - Crisis Response
  - Adaptability
  - Representation Accuracy
  - Legislative Speed
  - Economic Outcome
  - Composite Score (aggregated)
- **Scenario-Driven Simulation**: You define the world and its regions using hierarchical JSON files. Each region can have its own economic, governance, and demographic parameters, and can be nested (e.g., countries, states, cities).
- **Deterministic, Reproducible Results**: Simulations are deterministic by default, ensuring that the same scenario and configuration always produce the same results.
- **Colorized State Diffs**: After each run, the CLI prints human-readable, colorized diffs showing how every entity changed over time.
- **Extensible Plugin System**: You can add new domains (e.g., health, education), swap out models, or extend existing ones by writing new plugins.
- **Cross-Domain Coupling**: Metrics and outcomes in one domain (e.g., governance) can affect others (e.g., economy, demography), allowing for realistic feedback loops.
- **Transition Support**: The system supports mid-simulation transitions (e.g., government or economic system changes) and tracks their effects.
- **Testing and Validation**: The codebase includes tests to ensure metrics, update rules, and cross-domain dependencies behave as specified.

**Typical Workflow:**
1. Edit or create scenario files in `scenarios/` to define your world.
2. Run the simulation using the CLI (`cargo run --bin cli`).
3. Review the output metrics and state diffs to analyze system behavior and outcomes.
4. Extend or modify plugins, models, or scenarios as needed.

For more details on metrics, update rules, and extensibility, see the code comments and module documentation.

## How It Works


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

## Running a Simulation

```bash
cargo run --bin cli
```

- Loads scenario from `scenarios/simulation_config.json`
---

## Extending Psychohistory

- Add new plugins in `plugins/`
- Add new domain state structs in `models/`
- Add new scenario files in `scenarios/`
- Implement new systems and register them in `cli/src/main.rs`
 Loads scenario from `scenarios/simulation_config.json`
 Runs all systems for the specified number of years and Monte Carlo runs per region
 Prints mean and standard deviation for all output metrics per region
 Prints colorized, per-field diffs for all entities (single-run mode)

---

## TODO (Priority‑Ordered)

### 🔥 Priority 1 — Core Architectural Primitives

- [x] Add `GovType` and `EconSystemType` enums to `models` and propagate through `SimulationState`.
- [x] Implement a transition system in `core` to support mid‑simulation switching of government and economic types.
- [x] Add cross-domain coupling hooks in `SimulationState` to enable direct econ ↔ gov ↔ demog interactions.
- [x] Refactor state diffing to track and report system transitions and cross-domain effects.

---

### 🔥 Priority 2 — Domain Plugin Upgrades

- Update `gov` plugin to support multiple government types using the new `GovType` enum.
- Update `econ` plugin to support multiple economic systems using the new `EconSystemType` enum.
- Implement plugin logic for handling mid-simulation transitions (e.g., democracy → autocracy, market → planned).
- Add cross-domain effect handlers in each plugin to respond to changes in other domains.

---

### 🔥 Priority 3 — Scenario Schema Expansion

- Extend scenario JSON schema to accept `gov_type` and `econ_system_type` fields for each region/component.
- Add support for specifying scheduled or conditional system transitions in scenario files.
- Allow cross-domain coupling parameters (e.g., tax policy impact on demography) in scenario JSON.

---

### Priority 4 — CLI / UX Integration

- Update CLI to validate and display new scenario schema fields (`gov_type`, `econ_system_type`, transitions).
- Add CLI options to trigger or schedule system transitions during simulation runs.
- Enhance output to clearly show when and where system transitions and cross-domain effects occur.

---

### Priority 5 — Testing & Determinism Guarantees

- Add unit and integration tests for government and economic system transitions.
- Add tests for scenario parsing with new fields and transition logic.
- Add regression tests for cross-domain coupling to ensure deterministic and reproducible results.

---

### Priority 6 — Labs / Python Bindings

- Add support for Python bindings (pyo3) for new fields, transitions, and cross-domain coupling.

---

## License

PolyForm Noncommercial License 1.0.0 — see LICENSE.md for details.
