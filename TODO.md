
## Phase 7: CLI Refactor, Command-Line, and Logging Improvements

- [ ] **Refactor CLI Entrypoint:**
  - Main logic is still monolithic in [cli/src/main.rs].
  - TODO:
    - [ ] Move scenario file loading and validation into a helper module (e.g., `scenario.rs`).
    - [ ] Move region simulation tree traversal into its own module (e.g., `region_runner.rs`).
    - [ ] Move result printing and formatting into a utility module (e.g., `output.rs`).
    - [ ] Ensure `main.rs` only wires together argument parsing, logging, and high-level orchestration.
    - [ ] Add unit tests for new helper modules.

- [x] **Command-Line Parsing with Crate:**
  - Integrated `clap` for command-line parsing.
  - Options for years, runs, scenario path, log file, verbosity, and help are defined.

- [x] **Command-Line Help Output:**
  - Help message prints all options and usage if no parameters are provided or if `--help` is passed.

- [x] **Scenario File Location Handling:**
  - `.json` scenario files are located relative to the executable by default.
  - Command-line option allows specifying a custom scenario directory.
  - (Build copy to output dir: pending)

- [x] **Logging Support:**
  - Command-line option for log file name and location is present.
  - Log file name is timestamped for each run.
  - Major simulation tasks (region start/end, file loads, errors, etc.) are logged.
  - Verbosity option controls log detail.

- [x] **No-Params Behavior:**
  - If no parameters are provided, help is printed and simulation does not run.
# Psychohistory Governance Simulator — Targeted TODO

This TODO is tailored for the current codebase state. **For each item, first inspect the referenced files/modules to leverage existing logic before writing new code.**

---

## Phase 1: Configuration Audit & The 'us' Region Bug
- [x] **Audit the Config Parser:**  
  - Inspect: [core/src/config.rs], [core/src/app.rs], [cli/src/main.rs]  
  - **Complete:** Config parsing now robust—missing fields are filled with defaults, incomplete configs are accepted, and tests validate this. The `[WARN] Skipping region 'us'` bug was due to missing fields in the top-level config, now handled by serde defaults.
 - [x] **Verify Dual-System Data:**  
  - Inspect: [core/src/config.rs], [scenarios/simulation_config.json], [core/src/state.rs]  
  - **Complete:** Current config/schema only supports a single system per region (no dual-system or alternative block). If dual-system support is needed, schema and code must be extended to allow two sets of parameters per region.

---

## Phase 2: Connecting the Simulation Loop & Engine
 - [x] **Locate the Hidden Engine:**  
  - Inspect: [core/src/app.rs], [core/src/simulation.rs], [core/src/system.rs], [core/src/experiment.rs]  
  - **Complete:** The simulation engine is implemented in [core/src/simulation.rs] as `run_simulation` (multi-year) and `simulate_year` (per-year). It uses `SimulationState`, `SimulationConfig`, and `GovernanceSystem`, supports plugins, and is tested for determinism and metric bounds.
 - [x] **Wire Engine to CLI:**  
  - Inspect: [cli/src/main.rs]  
  - **Complete:** The CLI calls the real simulation engine (`run_simulation`) for each region and prints average metrics from the returned `RunResult`. Timeline/multi-run, CSV export, and richer reporting are future steps.
- [ ] **Enable Multi-Run / Timeline Execution:**  
  - Inspect: [core/src/experiment.rs], [core/src/app.rs], [core/src/scheduler.rs], [cli/src/main.rs]  
  - Action: Make sure the simulation loop uses the configured timeline horizon (e.g., 20 years) and executes all Monte Carlo runs, not just a single pass.

---

## Phase 3: Activating the Statistical Module
- [ ] **Find the Accumulators:**  
  - Inspect: [core/src/run_result.rs], [core/src/experiment.rs], [core/src/app.rs]  
  - Action: Locate where Mean, StdDev, and 95% Confidence Intervals are calculated or defined. Review accumulator structs and their update logic.
- [ ] **Pipe Simulation Output to Stats:**  
  - Inspect: [core/src/experiment.rs], [core/src/run_result.rs]  
  - Action: Ensure per-run simulation results are fed into these accumulators so aggregates are computed and not left empty.

---

## Phase 4: Hooking Up CSV Exporting
- [ ] **Locate Data Writers:**  
  - Inspect: [core/src/experiment.rs], [core/src/run_result.rs], [core/src/app.rs], references to `simulation-summary.csv` and `per-run-results.csv`  
  - Action: Find CSV export logic and verify file paths.
- [ ] **Trigger the Writes:**  
  - Inspect: [cli/src/main.rs], [core/src/app.rs]  
  - Action: Wire CSV export functions to run at the end of each region’s simulation, ensuring cumulative data is written to disk.

---

## Phase 5: Polishing the Console Presentation & Comparison
- [ ] **Review Table Presentation:**  
  - Inspect: [cli/src/util.rs], [cli/src/main.rs], Cargo.toml dependencies (look for `comfy-table`, `cli-table`, etc.)  
  - Action: Check for existing table formatting utilities and how they are used.

---

## Phase 6: Performance Optimization
- [ ] **Check Parallelism Support:**  
  - Inspect: [Cargo.toml for `rayon`], [core/src/experiment.rs], [core/src/app.rs], [cli/src/main.rs]  
  - Action: Look for existing parallel iterators or async code. If present, ensure region or Monte Carlo loops use them for concurrent execution.

---

**Reminder:** For each task, always check the referenced files and modules first—most infrastructure is already present and only needs to be connected or repaired.
