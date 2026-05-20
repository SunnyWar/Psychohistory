
# TODO: Align Rust program with extracted governance simulation specification

## 1. Output Metrics
- [ ] Implement and track the following metrics:
   - [ ] Law Quality
   - [ ] Corruption Level
   - [ ] Public Trust
   - [ ] Crisis Response
   - [ ] Adaptability
   - [ ] Representation Accuracy
   - [ ] Legislative Speed
   - [ ] Economic Outcome
   - [ ] Composite Score

## 2. Metric Formulas & Update Rules
- [x] Implement formulas for each metric, including:
   - [x] Law Quality (CurrentUsSystem & FederalSensorumSystem)
   - [x] Corruption Level
   - [x] Public Trust
   - [x] Crisis Response
   - [x] Adaptability
   - [x] Representation Accuracy
   - [x] Legislative Speed
   - [x] Economic Outcome
   - [x] Composite Score (weighted average, invert corruption)
- [x] Ensure all formulas use Clamp01 normalization where specified.
- [ ] Implement randomization where required (e.g., cohortQualityShock, deliberationNoise, economicShock).
- [x] Support both CurrentUsSystem and FederalSensorumSystem variants where formulas differ.

## 3. State Variables & Simulation Entities
- [x] Add/verify all required internal state variables:
   - [x] YearOutcomes (list of YearOutcome per run)
   - [x] priorTrust
   - [x] policyStock
   - [x] avgCompetence, avgIntegrity, avgLeadership, avgRepresentation
   - [x] lobbyingPressure, donorPressure, mediaImpact, reelectionPressure, normalizedWealthInfluence, factionFormation, badLawDrag, isGridlocked, externalShock, challengeHappened, legislativeEfficiency, deliberationNoise, deliberationBonus, evidenceBoardEffect, cohortQualityShock, stabilityMultiplier, legislativeCompetence, judicialCompetence, sortition.ExpertSupportEffectiveness
- [x] Define entities and attributes:
   - [x] Legislator (Id, Chamber, Competence, Integrity, Ideology, Wealth, IsExperienced, Representativeness, LeadershipQuality, FactionAffinity)
   - [x] GovernanceSystem (Members, SystemKind, DisplayName)
   - [x] YearOutcome (all output metrics)

## 4. Simulation Flow
- [x] For each year in a run:
   - [x] SimulateYear is called with current members, config, random, prior outcomes.
   - [x] Plugins may modify the outcome (plugin hook implemented).
   - [x] Membership may be rotated (stub present).
   - [x] YearOutcome is recorded.
- [x] At end of run:
   - [x] RunResult is constructed with averages and composite score.

## 5. Cross-Domain Couplings
- [x] Ensure all metric dependencies are respected:
   - [x] Law Quality, Crisis Response, Adaptability, Legislative Speed, Economic Outcome, and Public Trust are interdependent.
   - [x] Corruption Level affects Public Trust and Economic Outcome.
   - [x] Media Impact affects Law Quality and Public Trust.
   - [x] Economic Outcome includes Law Quality, Crisis Response, Adaptability, Corruption Level, and external shocks.
   - [x] Composite Score aggregates all metrics, inverting Corruption.

## 6. Constants, Thresholds, Magic Numbers
- [x] Use the same coefficients and normalization as specified:
   - [x] Clamp01 for all metrics
   - [x] All weights and coefficients (see extracted list)
   - [x] SimulationConfig and CommonDynamicsConfig parameters

## 7. Testing and Validation
- [ ] Add/expand tests to verify that all metrics, update rules, and cross-domain dependencies behave as specified.

## 8. Documentation
- [ ] Document all metric formulas, update rules, and cross-domain dependencies in code comments and/or module docs.
