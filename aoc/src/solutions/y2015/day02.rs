use aoc_harness::aoc_main;

aoc_main!(2015 day 2, part1 [p1], part2 [p2]);
use itertools::Itertools;

fn p1(input: &str) -> u32 {
    input.lines()
        .map(|x|
                 x.split('x')
                     .map(|x| x.parse::<u32>().unwrap())
                     .sorted()
                     .collect_vec()
        ).map(|x| x[0]*x[1]*3+x[0]*x[2]*2+x[1]*x[2]*2)
        .sum()
}

fn p2(input: &str) -> u32 {
    input.lines()
        .map(|x|
            x.split('x')
                .map(|x| x.parse::<u32>().unwrap())
                .sorted()
                .collect_vec()
        ).map(|x| 2*(x[0]+x[1]) + x[0]*x[1]*x[2])
        .sum()
}
