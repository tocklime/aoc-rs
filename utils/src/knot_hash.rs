use std::{convert::Infallible, str::FromStr};

#[derive(Debug)]
#[must_use]
pub struct KnotHash {
    pos: usize,
    skip_size: usize,
    data: Vec<u8>,
}
impl KnotHash {
    pub fn new() -> Self {
        Self {
            pos: 0,
            skip_size: 0,
            data: (0..=255).collect(),
        }
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn step(&mut self, input: &[usize]) {
        let size = self.data.len();
        for &n in input {
            for i in 0..n / 2 {
                self.data
                    .swap((self.pos + i) % size, (self.pos + n - i - 1) % size);
            }
            self.pos = (self.pos + n + self.skip_size) % size;
            self.skip_size += 1;
        }
    }
    #[must_use]
    pub fn dense_hash(&self) -> String {
        self.dense().map(|x| format!("{:x}", x)).collect()
    }
    fn dense(&self) -> impl Iterator<Item = u8> + '_ {
        self.data
            .chunks(16)
            .map(|ch| ch.iter().fold(0_u8, |a, &b| a ^ b))
    }
    #[must_use]
    pub fn set_bit_count(&self) -> u8 {
        self.dense().map(bitintr::Popcnt::popcnt).sum()
    }
    #[must_use]
    pub fn bit_arr(&self) -> Vec<u8> {
        self.dense().collect()
    }
}

impl FromStr for KnotHash {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let suffix: Vec<u8> = vec![17, 31, 73, 47, 23];
        let ns: Vec<usize> = s.bytes().chain(suffix).map(Into::into).collect();
        let mut st = KnotHash::new();
        for _ in 0..64 {
            st.step(&ns);
        }
        Ok(st)
    }
}
impl Default for KnotHash {
    fn default() -> Self {
        Self::new()
    }
}
