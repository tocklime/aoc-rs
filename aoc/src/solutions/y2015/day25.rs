use aoc_harness::aoc_main;

aoc_main!(2015 day 25, part1 [p1]);
use regex::Regex;
use itertools::Itertools;
use utils::nums::mod_pow;


fn grid_ordinal(row: usize, col: usize) -> usize {
    (row + col - 1) * (row + col - 2) / 2 + col
}

fn mod_math(nth: usize) -> usize {
    mod_pow::<usize>(252533, nth - 1, 33554393) * 20151125 % 33554393
}


fn p1(input: &str) -> usize {
    //To continue, please consult the code grid in the manual.  Enter the code at row 3010, column 3019.
    let re = Regex::new(r"\d+").unwrap();
    let data = re.captures_iter(input).map(|x| x[0].parse::<usize>().unwrap()).collect_vec();
    mod_math(grid_ordinal(data[0], data[1]))
}