# Psychohistory Governance Simulator — Focused TODO

**Prioritized for codebase quality: items that find/fix bugs, improve readability, or enhance maintainability are listed first.**

---

## 1. Simulation Correctness & Statistical Output
- [x] **Enable Multi-Run / Timeline Execution**  
  - Complete: The CLI exposes `--years` and `--runs` for timeline and Monte Carlo configuration. These are passed through to the simulation engine and results are aggregated. See README for usage.
- [ ] **Find and Connect Statistical Accumulators**  
  - Inspect: [core/src/run_result.rs], [core/src/experiment.rs], [core/src/app.rs]  
  - Action: Locate where Mean, StdDev, and 95% Confidence Intervals are calculated or defined. Ensure per-run simulation results are fed into these accumulators so aggregates are computed and not left empty. 
  - *Priority: High — ensures output is meaningful and not misleading.*

---

## 2. Output & Usability Improvements
- [ ] **Hook Up CSV Exporting**  
  - Inspect: [core/src/experiment.rs], [core/src/run_result.rs], [core/src/app.rs], [cli/src/main.rs], references to `simulation-summary.csv` and `per-run-results.csv`  
  - Action: Find CSV export logic, verify file paths, and wire export functions to run at the end of each region’s simulation. Ensure cumulative data is written to disk. 
  - *Priority: Medium — improves reproducibility and analysis.*
- [ ] **Polish Console Table Presentation**  
  - Inspect: [cli/src/util.rs], [cli/src/main.rs], Cargo.toml dependencies (look for `comfy-table`, `cli-table`, etc.)  
  - Action: Check for existing table formatting utilities and how they are used. 
  - *Priority: Medium — improves readability for users.*

---

## 3. Performance & Maintainability
- [ ] **Check and Improve Parallelism Support**  
  - Inspect: [Cargo.toml for `rayon`], [core/src/experiment.rs], [core/src/app.rs], [cli/src/main.rs]  
  - Action: Look for existing parallel iterators or async code. If present, ensure region or Monte Carlo loops use them for concurrent execution. 
  - *Priority: Medium — improves performance for large runs.*

---

**Reminder:** For each task, always check the referenced files and modules first—most infrastructure is already present and only needs to be connected or repaired.
