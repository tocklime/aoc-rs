use aoc_harness::aoc_main;

aoc_main!(2017 day 13, generator gen, part1 [p1], part2 [p2]);
use itertools::Itertools;

fn gen(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            l.split(": ")
                .map(|x| x.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap()
        })
        .collect()
}

fn score(delay: usize, depths: &[(usize, usize)]) -> usize {
    depths
        .iter()
        .map(|&(t, d)| {
            let scanner_pos = (delay + t) % (2 * (d - 1));
            if scanner_pos == 0 {
                t * d
            } else {
                0
            }
        })
        .sum()
}
fn is_safe(delay: usize, depths: &[(usize, usize)]) -> bool {
    for &(depth, range) in depths {
        let s_pos = (delay + depth) % (2 * (range - 1));
        if s_pos == 0 {
            return false;
        }
    }
    true
}

fn p1(input: &[(usize, usize)]) -> usize {
    score(0, input)
}

fn p2(input: &[(usize, usize)]) -> usize {
    #[allow(clippy::maybe_infinite_iter)]
    (0..).find(|&d| is_safe(d, input)).unwrap()
}

//152252 too low
