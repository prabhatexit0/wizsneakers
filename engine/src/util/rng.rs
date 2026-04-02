pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        // Avoid state=0 which would lock xorshift
        let state = if seed == 0 { 1 } else { seed };
        Self { state }
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Inclusive min, exclusive max
    pub fn range(&mut self, min: u32, max: u32) -> u32 {
        let span = max - min;
        if span == 0 {
            return min;
        }
        min + (self.next_u64() % span as u64) as u32
    }

    pub fn chance(&mut self, percent: u8) -> bool {
        if percent == 0 {
            return false;
        }
        if percent >= 100 {
            return true;
        }
        self.range(0, 100) < percent as u32
    }
}
