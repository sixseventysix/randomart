use rand_chacha::ChaCha8Rng;
use rand::{Rng, SeedableRng};

pub struct Rng_ {
    rng: ChaCha8Rng,
}

impl Rng_ {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn next_float(&mut self) -> f32 {
        self.rng.random::<f32>() // guaranteed in [0.0, 1.0)
    }
}
