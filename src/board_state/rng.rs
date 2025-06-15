pub struct Rng {
    current: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng {
            current: seed % 2147483648_u64,
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.next_u15() as u64
            | ((self.next_u15() as u64) << 15)
            | ((self.next_u15() as u64) << 30)
            | ((self.next_u15() as u64) << 45)
            | ((self.next_u15() as u64) << 60)
    }

    fn next_u15(&mut self) -> u8 {
        self.current = 1103515245_u64
            .wrapping_mul(self.current)
            .wrapping_add(12345);
        ((self.current / 65536) as u32 % 32768_u32) as u8
    }
}
