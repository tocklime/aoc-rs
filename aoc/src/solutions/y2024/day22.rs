use std::collections::HashMap;

use itertools::Itertools;

aoc_harness::aoc_main!(2024 day 22, part1 [p1] => 19458130434, part2 [p2] => 2130,
    example part1 EG => 37327623,
    example part2 EG2 => 23,
);

struct SecretNumber(u64);

impl Iterator for SecretNumber {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.0;
        self.0 = step(n);
        Some(n)
    }
}
impl SecretNumber {
    fn from_str(input: &str) -> Self {
        Self(input.parse().unwrap())
    }
}

fn step(n: u64) -> u64 {
    let n = ((n << 6) ^ n) & 0xFF_FFFF;
    let n = ((n >> 5) ^ n) & 0xFF_FFFF;
    let n = ((n << 11) ^ n) & 0xFF_FFFF;
    n
}

fn p1(input: &str) -> u64 {
    input
        .lines()
        .into_iter()
        .map(|l| SecretNumber::from_str(l).nth(2000).unwrap())
        .sum()
}
fn p2(input: &str) -> u64 {
    let mut map: HashMap<[i8; 4], u64> = HashMap::new();
    input.lines().for_each(|l| {
        let sn = SecretNumber::from_str(l);
        let mut this_map = HashMap::new();
        for (a, b, c, d, e) in sn
            .map(|x| i8::try_from(x % 10).unwrap())
            .take(2001)
            .tuple_windows()
        {
            let diffs = [b - a, c - b, d - c, e - d];
            this_map.entry(diffs).or_insert(e);
        }
        for (k, v) in this_map {
            let m = map.entry(k).or_default();
            *m += u64::try_from(v).unwrap();
        }
    }); 
    *map.values().max().unwrap()
}

const EG: &str = "1
10
100
2024
";

const EG2: &str = "1
2
3
2024
";
