
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
- [ ] Implement formulas for each metric, including:
   - [ ] Law Quality (CurrentUsSystem & FederalSensorumSystem)
   - [ ] Corruption Level
   - [ ] Public Trust
   - [ ] Crisis Response
   - [ ] Adaptability
   - [ ] Representation Accuracy
   - [ ] Legislative Speed
   - [ ] Economic Outcome
   - [ ] Composite Score (weighted average, invert corruption)
- [ ] Ensure all formulas use Clamp01 normalization where specified.
- [ ] Implement randomization where required (e.g., cohortQualityShock, deliberationNoise, economicShock).
- [ ] Support both CurrentUsSystem and FederalSensorumSystem variants where formulas differ.

## 3. State Variables & Simulation Entities
- [ ] Add/verify all required internal state variables:
   - [ ] YearOutcomes (list of YearOutcome per run)
   - [ ] priorTrust
   - [ ] policyStock
   - [ ] avgCompetence, avgIntegrity, avgLeadership, avgRepresentation
   - [ ] lobbyingPressure, donorPressure, mediaImpact, reelectionPressure, normalizedWealthInfluence, factionFormation, badLawDrag, isGridlocked, externalShock, challengeHappened, legislativeEfficiency, deliberationNoise, deliberationBonus, evidenceBoardEffect, cohortQualityShock, stabilityMultiplier, legislativeCompetence, judicialCompetence, sortition.ExpertSupportEffectiveness
- [ ] Define entities and attributes:
   - [ ] Legislator (Id, Chamber, Competence, Integrity, Ideology, Wealth, IsExperienced, Representativeness, LeadershipQuality, FactionAffinity)
   - [ ] GovernanceSystem (Members, SystemKind, DisplayName)
   - [ ] YearOutcome (all output metrics)

## 4. Simulation Flow
- [ ] For each year in a run:
   - [ ] SimulateYear is called with current members, config, random, prior outcomes.
   - [ ] Plugins may modify the outcome.
   - [ ] Membership may be rotated.
   - [ ] YearOutcome is recorded.
- [ ] At end of run:
   - [ ] RunResult is constructed with averages and composite score.

## 5. Cross-Domain Couplings
- [ ] Ensure all metric dependencies are respected:
   - [ ] Law Quality, Crisis Response, Adaptability, Legislative Speed, Economic Outcome, and Public Trust are interdependent.
   - [ ] Corruption Level affects Public Trust and Economic Outcome.
   - [ ] Media Impact affects Law Quality and Public Trust.
   - [ ] Economic Outcome includes Law Quality, Crisis Response, Adaptability, Corruption Level, and external shocks.
   - [ ] Composite Score aggregates all metrics, inverting Corruption.

## 6. Constants, Thresholds, Magic Numbers
- [ ] Use the same coefficients and normalization as specified:
   - [ ] Clamp01 for all metrics
   - [ ] All weights and coefficients (see extracted list)
   - [ ] SimulationConfig and CommonDynamicsConfig parameters

## 7. Testing and Validation
- [ ] Add/expand tests to verify that all metrics, update rules, and cross-domain dependencies behave as specified.

## 8. Documentation
- [ ] Document all metric formulas, update rules, and cross-domain dependencies in code comments and/or module docs.
