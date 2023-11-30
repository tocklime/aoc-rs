use aoc_harness::aoc_main;

aoc_main!(2015 day 25, part1 [p1] => 8_997_277);
use itertools::Itertools;
use num_modular::{ModularInteger, VanillaInt};
use regex::Regex;

fn grid_ordinal(row: u64, col: u64) -> u64 {
    (row + col - 1) * (row + col - 2) / 2 + col
}

fn p1(input: &str) -> u64 {
    //To continue, please consult the code grid in the manual.  Enter the code at row 3010, column 3019.
    let re = Regex::new(r"\d+").unwrap();
    let data = re
        .captures_iter(input)
        .map(|x| x[0].parse::<u64>().unwrap())
        .collect_vec();

    let ord = grid_ordinal(data[0], data[1]) - 1;
    (VanillaInt::new(252_533_u64, &33_554_393).pow(&ord) * 20_151_125)
        .residue()
}
