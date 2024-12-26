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
fn hash(k: [i8;4]) -> u32 {
    //values in k are in -9..=9.
    k.into_iter().fold(0, |acc, n| (acc << 5) | (n + 9) as u32)
    //max val is ... complicated, but bounded by 2^20.
}
fn p2(input: &str) -> u64 {
    let mut map = vec![0;1<<20];
    let mut best = 0;
    input.lines().for_each(|l| {
        let sn = SecretNumber::from_str(l);
        let mut seen = vec![false;1<<20];
        for ((a,_), (b,_), (c,_), (d,e)) in sn
            .map(|x| i8::try_from(x % 10).unwrap())
            .tuple_windows()
            .map(|(a,b) | (b-a, b))
            .take(2000)
            .tuple_windows()
        {
            let diffs = [a,b,c,d];
            let h = hash(diffs) as usize;
            if !seen[h] {
                seen[h] = true;
                map[h] += u64::try_from(e).unwrap();
                best = best.max(map[h]);
            }
        }
    }); 
    best
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
