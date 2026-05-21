use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Utility to generate a vector of unique seeds for Monte Carlo runs
pub fn generate_seeds(seed: Option<u64>, count: usize) -> Vec<u64> {
    let mut rng = match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => rand::make_rng(),
    };
    (0..count).map(|_| rng.next_u64()).collect()
}
