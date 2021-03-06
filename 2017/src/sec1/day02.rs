use regex::Regex;
use itertools::Itertools;

#[aoc(day2, part1)]
fn p1(input: &str) -> u32 {
    let re = Regex::new(r"\s+").unwrap();
    input.lines().map(|l|
        {
            let (a, b) = re.split(l).map(|n| n.parse::<u32>().unwrap()).minmax().into_option().unwrap();
            b - a
        }).sum()
}

#[aoc(day2, part2)]
fn p2(input: &str) -> u32 {
    let re = Regex::new(r"\s+").unwrap();
    input.lines().map(|l|
        {
            re.split(l).map(|n| n.parse::<u32>().unwrap()).permutations(2).filter_map(|v| {
                if v[1] % v[0] == 0 { Some (v[1] / v[0])} else {None}
            }).nth(0).unwrap()

        }).sum()
}
