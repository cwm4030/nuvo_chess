pub struct Rng {
    state: u64,
}

impl Default for Rng {
    fn default() -> Self {
        Rng::new()
    }
}

impl Rng {
    pub fn new() -> Self {
        Rng { state: 1070372 }
    }

    pub fn seed(&mut self, seed: u64) {
        self.state = seed;
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(2685821657736338717)
    }
}
