use bitintr::Popcnt;

#[derive(Debug)]
pub struct KnotHash {
    pos: usize,
    skip_size: usize,
    data: Vec<u8>
}
impl KnotHash {
    pub fn new() -> Self {
        Self {
            pos: 0,
            skip_size: 0,
            data: (0..=255).collect()
        }
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn step(&mut self, input: &[usize]) {
        let size = self.data.len();
        for &n in input {
            for i in 0..n / 2 {
                self.data.swap((self.pos + i) % size, (self.pos + n - i - 1) % size);
            }
            self.pos = (self.pos + n + self.skip_size) % size;
            self.skip_size += 1;
        }
    }
    pub fn dense_hash(&self) -> String {
        self.dense().map(|x| format!("{:x}",x)).collect()
    }
    pub fn from_str(input: &str) -> Self {
        let suffix: Vec<u8> = vec![17,31,73,47,23];
        let ns : Vec<usize> = input.bytes().chain(suffix).map(|x| x.into()).collect();
        let mut st = KnotHash::new();
        for _ in 0..64 {
            st.step(&ns);
        }
        st
    }
    fn dense(&self) -> impl Iterator<Item=u8> + '_ {
        self.data.chunks(16)
            .map(|ch| ch.iter().fold(0_u8, |a, &b| a ^ b))
    }
    pub fn set_bit_count(&self) -> u8 {
        self.dense().map(|x| x.popcnt()).sum()
    }
    pub fn bit_arr(&self) -> Vec<u8> {
        self.dense().collect()
    }
}
