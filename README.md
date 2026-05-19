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

## License

PolyForm Noncommercial License 1.0.0 — see LICENSE.md for details.
