use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Utility to generate a vector of unique seeds for Monte Carlo runs
pub fn generate_seeds(seed: Option<u64>, count: usize) -> Vec<u64> {
    let mut rng = seed.map_or_else(rand::make_rng, StdRng::seed_from_u64);

    let mut v = Vec::with_capacity(count);
    for _ in 0..count {
        v.push(rng.next_u64());
    }
    v
}
