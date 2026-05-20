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

---

## How It Works

- **Scenario loading**: The CLI loads a hierarchical JSON scenario (see `scenarios/simulation_config.json`), recursively inflating all regions and their domain states.
- **Plugin registration**: Each domain (econ, gov, demog) registers its plugin and system with the core engine.
- **Simulation loop**: The scheduler runs all registered systems in parallel for a fixed number of steps.
- **State diffing**: At the end, the engine prints a colorized, per-field diff for every entity, showing exactly what changed.

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
- Runs all systems for 20 yearly steps
- Prints colorized, per-field diffs for all entities

---

## Extending Psychohistory

- Add new plugins in `plugins/`
- Add new domain state structs in `models/`
- Add new scenario files in `scenarios/`
- Implement new systems and register them in `cli/src/main.rs`

---

## TODO (Priority‑Ordered)

### 🔥 Priority 1 — Core Architectural Primitives

- Add `GovType` and `EconSystemType` enums to `models` and propagate through `SimulationState`.
- Implement a transition system in `core` to support mid‑simulation switching of government and economic types.
- Add cross-domain coupling hooks in `SimulationState` to enable direct econ ↔ gov ↔ demog interactions.
- Refactor state diffing to track and report system transitions and cross-domain effects.

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
