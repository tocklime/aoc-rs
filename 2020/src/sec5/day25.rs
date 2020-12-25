
use crate::utils::nums::mod_pow;

const MOD : u64 = 20_201_227;

#[aoc(day25,part1)]
pub fn p1(input: &str) -> u64 {
    let keys = input.lines().map(str::parse).collect::<Result<Vec<u64>,_>>().unwrap();
    let mut value = 1;
    #[allow(clippy::maybe_infinite_iter)]
    let loop_size = (1..).find(|_| {
        value = (value * 7) % MOD;
        value == keys[1]
    }).unwrap();
    mod_pow(keys[0],loop_size,MOD)
}