# **Psychohistory**

A modular, open‑source, Rust‑based simulation framework for global economic, political, demographic, and institutional dynamics.

Psychohistory is a workspace‑structured Rust project designed to simulate interacting world systems at scale. It is not a monolithic “world model.” Instead, it follows a **kernel → domains → models** architecture that allows researchers to plug in new systems, swap evaluators, run experiments, and explore counterfactuals.

The goal is to provide a scientifically useful, extensible, and computationally efficient platform for studying macro‑scale human systems.

---

## **Project Goals**

### **1. Modular simulation of world systems**

Each domain (economy, governance, demography, climate, etc.) is implemented as an independent crate with its own state, update systems, and models.

### **2. Neural‑network‑driven evaluators**

Instead of hand‑coding every mechanism, Psychohistory uses neural network evaluators trained on real data to approximate system transitions.

### **3. Extensible architecture**

New systems, models, and data sources can be added without modifying the core engine.

### **4. Academic‑friendly**

- Reproducible experiments  
- Versioned models  
- Clear APIs  
- Python bindings for Jupyter/R workflows  

### **5. Open source & copyleft**

Licensed under **AGPL‑3.0** to ensure that improvements remain open.

---

## **Workspace Structure**

psychohistory/
│
├── Cargo.toml                # Workspace root
├── README.md                 # This file
│
├── psychohistory-core/       # Simulation kernel
│   ├── src/
│   └── Cargo.toml
│
├── psychohistory-econ/       # Economic domain
│   ├── src/
│   └── Cargo.toml
│
├── psychohistory-gov/        # Governance & political stability domain
│   ├── src/
│   └── Cargo.toml
│
├── psychohistory-demog/      # Demography domain
│   ├── src/
│   └── Cargo.toml
│
├── psychohistory-models/     # NN evaluators (ONNX/Burn/tch)
│   ├── src/
│   └── Cargo.toml
│
├── psychohistory-cli/        # CLI for running experiments
│   ├── src/
│   └── Cargo.toml
│
└── psychohistory-lab/        # Python bindings (pyo3)
    ├── src/
    └── Cargo.toml

---

## **Core Concepts**

### **Simulation Kernel (`psychohistory-core`)**

#### **SimulationState**

A typed registry of domain states:

```rust
pub struct SimulationState {
    pub econ: EconState,
    pub gov: GovState,
    pub demog: DemogState,
    // more domains as added
}
```

#### **System Trait**

Each domain implements one or more systems:

```rust
pub trait System {
    fn name(&self) -> &'static str;
    fn dependencies(&self) -> &'static [&'static str];
    fn run(&mut self, state: &mut SimulationState, time: SimulationTime, ctx: &mut SystemContext);
}
```

#### **Scheduler**

- Topologically sorts systems  
- Executes them each tick  
- Allows enabling/disabling systems per experiment  

#### **Experiment Runner**

Loads a config file, instantiates systems, runs scenarios, and writes results.

---

## **Domain Crates**

Each domain crate defines:

- Its state struct  
- Its systems  
- Feature extraction for evaluators  
- Output metrics  

Example (econ):

```rust
pub struct EconState {
    pub gdp: f64,
    pub inflation: f64,
    pub unemployment: f64,
    pub trade_balance: f64,
}
```

Domains communicate only through the shared `SimulationState`.

---

## **Model Crate (`psychohistory-models`)**

A unified trait for neural network evaluators:

```rust
pub trait Evaluator<I, O> {
    fn evaluate(&self, input: &I) -> O;
}
```

Supported backends:

- ONNX Runtime  
- tch‑rs (PyTorch)  
- Burn  

Models are versioned and loaded from:

```
models/
    econ_v1.onnx
    gov_stability_v2.onnx
    demog_growth_v1.onnx
```

---

## **Experimentation**

Experiments are defined in TOML:

```toml
[meta]
name = "carbon_tax_vs_no_tax"

[time]
start_step = 0
end_step = 480

[systems]
enabled = ["econ", "gov", "demog"]

[scenarios.base]
policies.carbon_tax = false

[scenarios.carbon_tax]
policies.carbon_tax = true
```

Run via CLI:

```bash
psychohistory run experiments/carbon_tax.toml
```

Outputs can be written as:

- Arrow / Parquet  
- CSV  
- JSON  

---

## **Python Integration (`psychohistory-lab`)**

Provides:

- `run_experiment(config_path)`
- `load_state_snapshot(step)`
- `plot_timeseries(metric)`
- Ability to plug in Python‑based evaluators for rapid prototyping

---

## **Contributing**

1. Fork the repository  
2. Add a new crate for your domain or model  
3. Implement the required traits  
4. Add tests  
5. Submit a PR including:
   - Description of the domain  
   - Data sources  
   - Model versioning  
   - Example experiments  

All contributions must be licensed under **AGPL‑3.0**.

---

## **License**

**AGPL‑3.0**

This ensures that all modifications, including network‑served versions, remain open.

---

## **Roadmap**

### **Phase 1 — Core Engine**

- Simulation kernel  
- Scheduler  
- Experiment runner  
- Basic CLI  

### **Phase 2 — Economic Domain**

- Macro indicators  
- First NN evaluator  
- Policy inputs  

### **Phase 3 — Governance Domain**

- Political stability  
- Institutional quality  
- Regime transitions  

### **Phase 4 — Demography**

- Population dynamics  
- Migration  
- Age structure  

### **Phase 5 — Cross‑domain Coupling**

- Econ ↔ Gov  
- Econ ↔ Demog  
- Gov ↔ Demog  

### **Phase 6 — Python Lab**

- Jupyter integration  
- Visualization tools  

---

## **Status**

Psychohistory is in **early development**.

This project is licensed under the **PolyForm Noncommercial License 1.0.0**.  
Commercial use is strictly prohibited. See `LICENSE` for details.

---

If you want, I can also generate:

- a shorter “front‑page” README version  
- a logo/header block  
- badges (build, license, crates.io, docs.rs)  
- a contributor guide  
- a domain‑specific example walkthrough  

Just tell me what direction you want.