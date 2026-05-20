use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SimulationConfig {
    #[serde(default)]
    pub bias_level: f64,                  // [-1, 1]
    #[serde(default)]
    pub public_trust_decay_rate: f64,     // [0, 0.1]
    #[serde(default)]
    pub lobbying_strength: f64,           // [0, 5]
    #[serde(default)]
    pub wealth_influence_multiplier: f64, // [0, 5]
    #[serde(default)]
    pub crisis_year_probability: f64,     // 0.18
    #[serde(default)]
    pub new_challenge_pressure: f64,      // 0.35
    #[serde(default)]
    pub economic_volatility: f64,         // 0.20
    #[serde(default)]
    pub baseline_public_trust: f64,       // 0.48
    #[serde(default)]
    pub media_influence_strength: f64,    // 0.38
    #[serde(default)]
    pub weights: [f64; 8],                // for composite score
    #[serde(default)]
    pub us_corruption_base: f64,
    #[serde(default)]
    pub us_reelection_bonus: f64,
    #[serde(default)]
    pub partisan_polarization: f64,
    #[serde(default)]
    pub raw_law_quality: f64,
    #[serde(default)]
    pub representative_efficiency: f64,
    #[serde(default)]
    pub raw_speed: f64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            bias_level: 0.0,
            public_trust_decay_rate: 0.015,
            lobbying_strength: 1.0,
            wealth_influence_multiplier: 1.0,
            crisis_year_probability: 0.18,
            new_challenge_pressure: 0.35,
            economic_volatility: 0.20,
            baseline_public_trust: 0.48,
            media_influence_strength: 0.38,
            weights: [1.0; 8],
            us_corruption_base: 0.0,
            us_reelection_bonus: 1.0,
            partisan_polarization: 0.0,
            raw_law_quality: 1.0,
            representative_efficiency: 1.0,
            raw_speed: 1.0,
        }
    }
}
