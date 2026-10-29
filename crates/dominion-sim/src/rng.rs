use serde::{Deserialize, Serialize};

/// Deterministic, seedable pseudo-random number generator (Xoroshiro128++)
/// used across the entire simulation so runs and save/load resumes are 100% reproducible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeterministicRng {
    state: [u64; 2],
}

impl DeterministicRng {
    pub fn new(seed: u64) -> Self {
        let mut sm = seed;
        let s0 = Self::splitmix64(&mut sm);
        let s1 = Self::splitmix64(&mut sm);
        Self {
            state: if s0 == 0 && s1 == 0 {
                [0x9E37_79B9_7F4A_7C15, 0xBF58_476D_1CE4_E5B9]
            } else {
                [s0, s1]
            },
        }
    }

    fn splitmix64(x: &mut u64) -> u64 {
        *x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *x;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub fn next_u64(&mut self) -> u64 {
        let s0 = self.state[0];
        let mut s1 = self.state[1];
        let result = s0.wrapping_add(s1).rotate_left(17).wrapping_add(s0);

        s1 ^= s0;
        self.state[0] = s0.rotate_left(49) ^ s1 ^ (s1 << 21);
        self.state[1] = s1.rotate_left(28);

        result
    }

    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Generates a float in `[0.0, 1.0)`.
    pub fn next_f32(&mut self) -> f32 {
        let bits = (self.next_u32() >> 8) as f32;
        bits / ((1u32 << 24) as f32)
    }

    /// Generates a float in `[min, max)`.
    pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
        if max <= min {
            return min;
        }
        min + self.next_f32() * (max - min)
    }

    /// Generates an integer in `[min, max_exclusive)`.
    pub fn range_u32(&mut self, min: u32, max_exclusive: u32) -> u32 {
        if max_exclusive <= min {
            return min;
        }
        let span = max_exclusive - min;
        min + (self.next_u32() % span)
    }

    /// Generates an integer in `[min, max_exclusive)`.
    pub fn range_usize(&mut self, min: usize, max_exclusive: usize) -> usize {
        if max_exclusive <= min {
            return min;
        }
        let span = (max_exclusive - min) as u64;
        min + ((self.next_u64() % span) as usize)
    }

    /// Returns true with the given probability in `0.0..=1.0`.
    pub fn chance(&mut self, probability: f32) -> bool {
        if probability <= 0.0 {
            return false;
        }
        if probability >= 1.0 {
            return true;
        }
        self.next_f32() < probability
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        if slice.len() <= 1 {
            return;
        }
        for i in (1..slice.len()).rev() {
            let j = self.range_usize(0, i + 1);
            slice.swap(i, j);
        }
    }
}
