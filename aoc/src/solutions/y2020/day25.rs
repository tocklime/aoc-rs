use aoc_harness::aoc_main;

aoc_main!(2020 day 25, part1 [p1] => 11_328_376);

use num_modular::{ModularInteger, MontgomeryInt};
use num_traits::Pow;

const MOD: u64 = 20_201_227;

fn p1(input: &str) -> u64 {
    let keys = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()
        .unwrap();
    let mut value = 1;
    #[allow(clippy::maybe_infinite_iter)]
    let loop_size = (1..)
        .find(|_| {
            value = (value * 7) % MOD;
            value == keys[1]
        })
        .unwrap();
    MontgomeryInt::new(keys[0], &MOD).pow(loop_size).residue()
}
