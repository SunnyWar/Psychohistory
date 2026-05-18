# Psychohistory



A modular, open‑source, Rust‑based simulation framework for global economic, political, demographic, and institutional dynamics.



Psychohistory is a workspace‑structured Rust project designed to simulate interacting world systems at scale. It is not a monolithic “world model.” Instead, it is a kernel + domains + models architecture that allows researchers to plug in new systems, swap out evaluators, run experiments, and explore counterfactuals.



The goal is to create a scientifically useful, extensible, and computationally efficient platform for studying macro‑scale human systems.



Project Goals

1\. Modular simulation of world systems

Each domain (economy, governance, demography, climate, etc.) is implemented as an independent crate with its own state, update systems, and models.



2\. Neural‑network‑driven evaluators

Instead of hand‑coding every mechanism, Psychohistory uses NN evaluators trained on real data to approximate system transitions.



3\. Extensible architecture

New systems, new models, and new data sources can be added without modifying the core engine.



4\. Academic‑friendly

Reproducible experiments



Versioned models



Clear APIs



Python bindings for Jupyter/R workflows



5\. Open source \& copyleft

Psychohistory is licensed under AGPL‑3.0 to ensure that improvements remain open.



Workspace Structure

Code

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

├── psychohistory-gov/        # Governance \& political stability domain

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

&#x20;   ├── src/

&#x20;   └── Cargo.toml

Core Concepts

Simulation Kernel (psychohistory-core)

The kernel provides:



SimulationState

A namespaced registry of domain states:



rust

pub struct SimulationState {

&#x20;   pub econ: EconState,

&#x20;   pub gov: GovState,

&#x20;   pub demog: DemogState,

&#x20;   // more domains as added

}

System Trait

Each domain implements one or more systems:



rust

pub trait System {

&#x20;   fn name(\&self) -> \&'static str;

&#x20;   fn dependencies(\&self) -> \&'static \[\&'static str];

&#x20;   fn run(\&mut self, state: \&mut SimulationState, time: SimulationTime, ctx: \&mut SystemContext);

}

Scheduler

Topologically sorts systems



Executes them each tick



Allows enabling/disabling systems per experiment



Experiment Runner

Loads a config file, instantiates systems, runs scenarios, and writes results.



Domain Crates

Each domain crate defines:



Its state struct



Its systems



Its feature extraction for evaluators



Its output metrics



Example (econ):



rust

pub struct EconState {

&#x20;   pub gdp: f64,

&#x20;   pub inflation: f64,

&#x20;   pub unemployment: f64,

&#x20;   pub trade\_balance: f64,

}

Each domain is independent and communicates only through the shared SimulationState.



Model Crate (psychohistory-models)

This crate wraps neural network evaluators behind a common trait:



rust

pub trait Evaluator<I, O> {

&#x20;   fn evaluate(\&self, input: \&I) -> O;

}

Backends supported:



ONNX Runtime



tch-rs (PyTorch)



Burn



Models are versioned and loaded from:



Code

models/

&#x20; econ\_v1.onnx

&#x20; gov\_stability\_v2.onnx

&#x20; demog\_growth\_v1.onnx

Experimentation

Experiments are defined in TOML:



toml

\[meta]

name = "carbon\_tax\_vs\_no\_tax"



\[time]

start\_step = 0

end\_step = 480



\[systems]

enabled = \["econ", "gov", "demog"]



\[scenarios.base]

policies.carbon\_tax = false



\[scenarios.carbon\_tax]

policies.carbon\_tax = true

The CLI runs:



Code

psychohistory run experiments/carbon\_tax.toml

Outputs are written as:



Arrow / Parquet



CSV



JSON



Python Integration (psychohistory-lab)

Provides:



run\_experiment(config\_path)



load\_state\_snapshot(step)



plot\_timeseries(metric)



Ability to plug in Python‑based evaluators for rapid prototyping



Contributing

1\. Fork the repo

2\. Add a new crate for your domain or model

3\. Implement the required traits

4\. Add tests

5\. Submit a PR with:

Description of the domain



Data sources



Model versioning



Experiment examples



All contributions must be licensed under AGPL‑3.0.



License

AGPL‑3.0  

This ensures that all modifications, including network‑served versions, remain open.



Roadmap

Phase 1 — Core Engine

Simulation kernel



Scheduler



Experiment runner



Basic CLI



Phase 2 — Economic Domain

Macro indicators



First NN evaluator



Policy inputs



Phase 3 — Governance Domain

Political stability



Institutional quality



Regime transitions



Phase 4 — Demography

Population dynamics



Migration



Age structure



Phase 5 — Cross‑domain coupling

Econ ↔ Gov



Econ ↔ Demog



Gov ↔ Demog



Phase 6 — Python Lab

Jupyter integration



Visualization tools



Status

Psychohistory is in early development.

This project is licensed under the PolyForm Noncommercial License 1.0.0.
Commercial use is strictly prohibited. See LICENSE for details.

