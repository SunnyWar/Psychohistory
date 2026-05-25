use rand::RngExt;

use crate::config::SimulationContext;
// Removed unused import
// use rand::Rng;

use crate::entities::{GovernanceSystem, YearOutcome};
use crate::legal::LegalSystemModel;
use crate::simulation::SimulationState;

/// Represents a direct economic decree issued by the autocrat.
#[derive(Clone, Debug)]
pub struct EconomicDecree {
    /// Tax rate [0, 1]: 0 = no tax, 1 = 100% taxation
    pub tax_rate: f64,
    /// Capital controls [0, 1]: 0 = free flow, 1 = total control
    pub capital_controls: f64,
    /// Trade policy [0, 1]: 0 = free trade, 1 = autarky
    pub trade_policy: f64,
    /// Industrial policy [0, 1]: 0 = laissez-faire, 1 = total state planning
    pub industrial_policy: f64,
    /// Currency regime [0, 1]: 0 = floating, 1 = fixed/pegged
    pub currency_regime: f64,
    /// Resource allocation [0, 1]: 0 = market, 1 = full state allocation
    pub resource_allocation: f64,
    /// Year decree was issued
    pub year: usize,
}

pub struct AutocracyModel;

impl LegalSystemModel for AutocracyModel {
    /// Simulate an autocratic legislative session: the leader issues direct economic decrees.
    ///
    /// # Math
    /// Each lever is set by the autocrat, possibly with noise or regime inertia:
    /// $$
    /// x_{t+1} = \alpha x_t + (1-\alpha) x^* + \epsilon
    /// $$
    /// where $x^*$ is the leader's target, $\alpha$ is inertia, $\epsilon$ is noise.
    ///
    /// # Theory
    /// "Dictatorship and Economic Policy" (Acemoglu, 2005); "The Political Economy of Autocracy" (Gandhi & Przeworski, 2007)
    fn simulate_legislative_session(
        &self,
        _system: &GovernanceSystem,
        _state: &mut SimulationState,
        year: usize,
        context: &mut SimulationContext,
    ) -> YearOutcome {
        // Parameters for regime inertia and noise
        let inertia = 0.8;
        let noise_std = 0.02;

        // Leader's targets (could be made dynamic or scenario-driven)
        let target_tax = 0.6;
        let target_capital = 0.8;
        let target_trade = 0.7;
        let target_industry = 0.9;
        let target_currency = 0.95;
        let target_resource = 0.85;

        // Previous values (could be tracked in state for realism)
        let prev_tax = 0.5;
        let prev_capital = 0.5;
        let prev_trade = 0.5;
        let prev_industry = 0.5;
        let prev_currency = 0.5;
        let prev_resource = 0.5;

        // Update each lever
        let tax_rate = inertia * prev_tax
            + (1.0 - inertia) * target_tax
            + noise_std * (context.rand.random::<f64>() - 0.5);
        let capital_controls = inertia * prev_capital
            + (1.0 - inertia) * target_capital
            + noise_std * (context.rand.random::<f64>() - 0.5);
        let trade_policy = inertia * prev_trade
            + (1.0 - inertia) * target_trade
            + noise_std * (context.rand.random::<f64>() - 0.5);
        let industrial_policy = inertia * prev_industry
            + (1.0 - inertia) * target_industry
            + noise_std * (context.rand.random::<f64>() - 0.5);
        let currency_regime = inertia * prev_currency
            + (1.0 - inertia) * target_currency
            + noise_std * (context.rand.random::<f64>() - 0.5);
        let resource_allocation = inertia * prev_resource
            + (1.0 - inertia) * target_resource
            + noise_std * (context.rand.random::<f64>() - 0.5);

        // Record the decree (could be pushed to state for history)
        let _decree = EconomicDecree {
            tax_rate,
            capital_controls,
            trade_policy,
            industrial_policy,
            currency_regime,
            resource_allocation,
            year,
        };

        // Output metrics: autocracy prioritizes speed, but may harm adaptability, trust, and representation
        YearOutcome {
            law_quality: 0.5 + 0.2 * (1.0 - tax_rate), // simplistic: high tax may reduce quality
            corruption_level: 0.7,                     // high baseline
            public_trust: 0.3,                         // low trust
            crisis_response: 0.6 + 0.1 * industrial_policy,
            adaptability: 0.3 + 0.1 * (1.0 - capital_controls),
            representation_accuracy: 0.1, // minimal
            legislative_speed: 0.95,      // maximal
            economic_outcome: 0.4 + 0.2 * (1.0 - resource_allocation),
            composite_score: 0.5, // placeholder
        }
    }
}
