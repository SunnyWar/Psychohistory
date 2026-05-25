/// Represents a member of the elite cohort in an autocracy.
#[derive(Clone, Debug)]
pub struct Elite {
    pub id: String,
    pub loyalty: f64,    // [0,1] loyalty to the autocrat
    pub influence: f64,  // [0,1] influence over policy
    pub is_active: bool, // false if purged
}
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

/// Update elite loyalty and simulate purges.
fn update_elites(elites: &mut [Elite], context: &mut SimulationContext, _year: usize) -> usize {
    let mut purged = 0;
    for elite in elites.iter_mut() {
        if !elite.is_active {
            continue;
        }
        // Loyalty drifts randomly, but regime actions can increase/decrease
        let drift = 0.05 * (context.rand.random::<f64>() - 0.5);
        elite.loyalty = (elite.loyalty + drift).clamp(0.0, 1.0);
        // Purge if loyalty falls below threshold
        if elite.loyalty < 0.2 && context.rand.random::<f64>() < 0.5 {
            elite.is_active = false;
            purged += 1;
        }
    }
    purged
}

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
    ///
    /// # Math (Extended)
    /// Elite loyalty $L_i$ evolves as:
    /// $$
    /// L_i(t+1) = L_i(t) + \epsilon,\ \epsilon \sim \mathcal{U}(-0.025, 0.025)
    /// $$
    /// Purge if $L_i < 0.2$ with 50% chance. Elite influence $I_i$ affects decree targets:
    /// $$
    /// x^* = x^*_{autocrat} + \sum_i I_i \cdot (\eta_i - 0.5)
    /// $$
    /// where $\eta_i \sim \mathcal{U}(0,1)$ for each active elite.
    ///
    /// # Theory
    /// "Dictatorship, Repression, and Elite Purges" (Svolik, 2012); "The Logic of Political Survival" (Bueno de Mesquita et al., 2003)
    fn simulate_legislative_session(
        &self,
        _system: &GovernanceSystem,
        state: &mut SimulationState,
        year: usize,
        context: &mut SimulationContext,
    ) -> YearOutcome {
        // --- Elite cohort (initialize if needed) ---
        // use std::collections::hash_map::Entry;
        // Initialize elites if needed
        if state.elites.is_empty() {
            state.elites = (0..8)
                .map(|i| Elite {
                    id: format!("E{}", i),
                    loyalty: 0.7 + 0.2 * context.rand.random::<f64>(),
                    influence: 0.08 + 0.12 * context.rand.random::<f64>(),
                    is_active: true,
                })
                .collect();
        }
        let purged = update_elites(&mut state.elites, context, year);
        let active_elites: Vec<_> = state.elites.iter().filter(|e| e.is_active).collect();

        // --- Elite influence on decree targets ---
        let mut target_tax = 0.6;
        let mut target_capital = 0.8;
        let mut target_trade = 0.7;
        let mut target_industry = 0.9;
        let mut target_currency = 0.95;
        let mut target_resource = 0.85;
        for elite in &active_elites {
            let eta = context.rand.random::<f64>();
            target_tax += elite.influence * (eta - 0.5) * 0.1;
            target_capital += elite.influence * (eta - 0.5) * 0.1;
            target_trade += elite.influence * (eta - 0.5) * 0.1;
            target_industry += elite.influence * (eta - 0.5) * 0.1;
            target_currency += elite.influence * (eta - 0.5) * 0.1;
            target_resource += elite.influence * (eta - 0.5) * 0.1;
        }
        target_tax = target_tax.clamp(0.0, 1.0);
        target_capital = target_capital.clamp(0.0, 1.0);
        target_trade = target_trade.clamp(0.0, 1.0);
        target_industry = target_industry.clamp(0.0, 1.0);
        target_currency = target_currency.clamp(0.0, 1.0);
        target_resource = target_resource.clamp(0.0, 1.0);

        // --- Regime stability penalty for purges ---
        let stability_penalty = 0.03 * (purged as f64);
        // Parameters for regime inertia and noise
        let inertia = 0.8;
        let noise_std = 0.02;

        // Leader's targets now include elite influence (see above)

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

        // Output metrics: autocracy prioritizes speed, but purges harm stability, trust, and adaptability
        YearOutcome {
            law_quality: 0.5 + 0.2 * (1.0 - tax_rate) - 0.05 * (purged as f64),
            corruption_level: 0.7 + 0.04 * (purged as f64),
            public_trust: 0.3 - 0.05 * (purged as f64),
            crisis_response: 0.6 + 0.1 * industrial_policy - stability_penalty,
            adaptability: 0.3 + 0.1 * (1.0 - capital_controls) - stability_penalty,
            representation_accuracy: 0.1,
            legislative_speed: 0.95 - 0.02 * (purged as f64),
            economic_outcome: 0.4 + 0.2 * (1.0 - resource_allocation) - stability_penalty,
            composite_score: 0.5 - 0.03 * (purged as f64),
        }
    }
}
