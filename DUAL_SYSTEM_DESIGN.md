# Dual-System Simulation Support: Design Proposal

## Motivation

To enable rigorous, side-by-side comparison of governance or economic systems (e.g., Status Quo vs. Sortition) for any region (country, state, city, etc.), the simulator should support running a full simulation for both a baseline system and one or more alternatives, then produce a comparative diff of all output metrics.

## Core Concepts

- **Primary System (Status Quo):** The default or current system for a region, as defined in the scenario config.
- **Alternative System(s):** One or more alternative configurations (e.g., different governance or economic models) to be compared against the primary system.
- **Paired Simulation Runs:** For each region, the simulation is executed once for the primary system and once for each alternative system, using the same initial conditions and random seeds for fair comparison.
- **Metric Diffing:** After both runs, the simulator computes and displays the difference in all tracked metrics (e.g., Law Quality, Corruption, Public Trust, etc.), highlighting which system performs better and by how much.

## Proposed Scenario Schema Extension

```json
"regions": {
  "us": {
    "archetype": "sovereign_developed",
    "components": {
      "governance_system": { ... },
      "simulation_parameters": { ... }
    },
    "alternatives": [
      {
        "label": "Sortition",
        "governance_system": { ... },
        "simulation_parameters": { ... }
      },
      {
        "label": "Hybrid",
        "governance_system": { ... },
        "simulation_parameters": { ... }
      }
    ]
  }
}
```
- The `alternatives` array allows specifying any number of alternative systems for a region.
- Each alternative can override any or all system/config fields.

## Simulation Engine Changes

- For each region, the engine will:
  1. Run the simulation with the primary system/config.
  2. For each alternative, run the simulation with the alternative system/config, using the same initial state and random seed.
  3. Collect and diff all output metrics, producing a comparative report (table/grid) for each region.

## Output Example

| Metric                | Status Quo | Sortition | Delta (Sortition - SQ) | Winner     |
|-----------------------|------------|-----------|------------------------|------------|
| Law Quality           | 0.72       | 0.81      | +0.09                  | Sortition  |
| Corruption Level      | 0.18       | 0.12      | -0.06                  | Sortition  |
| Public Trust          | 0.55       | 0.67      | +0.12                  | Sortition  |
| ...                   | ...        | ...       | ...                    | ...        |

- The winner is determined by metric-specific logic (e.g., higher is better, lower is better).

## Implementation Notes

- The CLI and core engine must be extended to:
  - Parse and validate the `alternatives` array.
  - Run paired simulations with identical seeds/initial state.
  - Aggregate and diff results for reporting.
- Tests should ensure deterministic, fair comparisons and correct diff logic.

## Future Extensions
- Support for more than two systems (multi-way comparison).
- Scenario-level toggles for which metrics to compare or highlight.
- Visualization of diffs (e.g., charts, colorized tables).

---

**Status:** Design proposal. Not yet implemented.
