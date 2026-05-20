# Psychohistory Governance Simulator — Targeted TODO

This TODO is tailored for the current codebase state. **For each item, first inspect the referenced files/modules to leverage existing logic before writing new code.**

---

## Phase 1: Configuration Audit & The 'us' Region Bug
- [ ] **Audit the Config Parser:**  
  - Inspect: [core/src/config.rs], [core/src/app.rs], [cli/src/main.rs]  
  - Action: Review how scenario/config files are loaded and validated. Find out why `[WARN] Skipping region 'us' due to missing or invalid system/config` appears, while other regions load fine. Check for schema mismatches, missing fields, or validation logic that could cause this.
- [ ] **Verify Dual-System Data:**  
  - Inspect: [core/src/config.rs], [scenarios/simulation_config.json], [core/src/state.rs]  
  - Action: Confirm that configuration structs support loading parameters for *both* a region’s "Status Quo" and "Sortition" alternatives. If fields exist, ensure they are populated from JSON and passed through to simulation state.

---

## Phase 2: Connecting the Simulation Loop & Engine
- [ ] **Locate the Hidden Engine:**  
  - Inspect: [core/src/app.rs], [core/src/simulation.rs], [core/src/system.rs], [core/src/experiment.rs]  
  - Action: Identify simulation structs, state update logic, and timeline processing. Map out how a simulation step is supposed to run.
- [ ] **Wire Engine to CLI:**  
  - Inspect: [cli/src/main.rs]  
  - Action: Replace hardcoded or placeholder metric outputs with actual calls to the simulation engine. Ensure the CLI is invoking the real simulation logic for each region.
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
- [ ] **Implement/Fix Comparative Grid:**  
  - Inspect: [cli/src/main.rs], [cli/src/util.rs]  
  - Action: Wire up the UI to read aggregated stats for both systems, compute Delta ($Sortition - StatusQuo$), determine the "Winner" for each metric, and print a clear comparison table.

---

## Phase 6: Performance Optimization
- [ ] **Check Parallelism Support:**  
  - Inspect: [Cargo.toml for `rayon`], [core/src/experiment.rs], [core/src/app.rs], [cli/src/main.rs]  
  - Action: Look for existing parallel iterators or async code. If present, ensure region or Monte Carlo loops use them for concurrent execution.

---

**Reminder:** For each task, always check the referenced files and modules first—most infrastructure is already present and only needs to be connected or repaired.
