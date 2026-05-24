# Per-Government-Type Simulation TODO

This file tracks the work required to implement detailed, type-specific simulations for each `GovType` (Democracy, Autocracy, Monarchy, Other), with a focus on legal and legislative operations.

---

## 1. Core Architecture

- [x] Refactor `GovernanceSystem` and simulation logic to branch on `GovType` (Democracy, Autocracy, Monarchy, Other).

- [x] Define a trait (e.g., `LegalSystemModel`) with methods for legislative process, law proposal, passage, veto, amendment, enforcement, and judicial review.

- [x] Implement a plugin or strategy pattern to select the correct model for each `GovType`.

## 2. Democracy Simulation

- [ ] Model bicameral/parliamentary structures (e.g., House/Senate, committees).

- [ ] Simulate proposal, debate, amendment, and voting processes.

- [ ] Include party/faction dynamics, lobbying, media, and public opinion.

- [ ] Model veto/override (executive/legislative interaction).

- [ ] Track legislative gridlock, filibuster, and coalition-building.

- [ ] Simulate judicial review and constitutional checks.

## 3. Autocracy Simulation

- [ ] Centralize legislative power (e.g., leader or ruling council).

- [ ] Model decree-based lawmaking, limited debate, and rubber-stamp assemblies.

- [ ] Simulate purges, loyalty checks, and elite influence.

- [ ] Track censorship, repression, and legal unpredictability.

- [ ] Model limited or absent judicial independence.

## 4. Monarchy Simulation

- [ ] Model hereditary succession, regency, and noble councils.

- [ ] Simulate royal decrees, advisory parliaments, and feudal obligations.

- [ ] Track legitimacy, dynastic politics, and noble revolts.

- [ ] Model church/state legal interactions (if relevant).

- [ ] Simulate evolving constitutional monarchy (if present).

## 5. "Other" (Custom) Simulation

- [ ] Allow user-defined or hybrid systems (e.g., theocracy, technocracy).

- [ ] Provide a configuration-driven way to compose legal/legislative rules.

- [ ] Document extension points for new government types.

## 6. Metrics & Output

- [ ] For each `GovType`, output detailed metrics: law quality, legislative speed, corruption, public trust, crisis response, adaptability, representation accuracy, and legal stability.

- [ ] Add per-type explanations to simulation output (e.g., why a law failed in a monarchy vs. democracy).

## 7. Scenarios & Testing

- [ ] Add scenario files for each government type with realistic initial conditions.

- [ ] Write integration tests for each system, ensuring unique behaviors are exercised.

## 8. Documentation

- [ ] Update README and code docs to explain how government type affects simulation.

- [ ] Provide examples for extending or customizing legal/legislative models.
