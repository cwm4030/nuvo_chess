use std::time::SystemTime;

pub struct Rng {
    current: u64,
}

impl Rng {
    pub fn new() -> Self {
        let time: u64 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as u64;
        Rng {
            current: time % 2147483648_u64,
        }
    }

    pub fn next_u8(&mut self) -> u8 {
        self.current = 1103515245_u64
            .wrapping_mul(self.current)
            .wrapping_add(12345);
        ((self.current / 65536) as u32 % 32768_u32) as u8
    }

    pub fn next_u64(&mut self) -> u64 {
        self.next_u8() as u64
            | ((self.next_u8() as u64) << 8)
            | ((self.next_u8() as u64) << 16)
            | ((self.next_u8() as u64) << 24)
            | ((self.next_u8() as u64) << 32)
            | ((self.next_u8() as u64) << 40)
            | ((self.next_u8() as u64) << 48)
            | ((self.next_u8() as u64) << 56)
    }
}
