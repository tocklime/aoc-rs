use aoc_harness::aoc_main;

aoc_main!(2020 day 3, generator gen, part1 [p1] => 242, part2 [p2] => 2265549792);

use utils::inputs::input_from_str_sep_by;

fn gen(input: &str) -> Vec<Vec<bool>> {
    input_from_str_sep_by(input, "\n", |x| x.chars().map(|c| c == '#').collect())
}
fn solve(tm: &[Vec<bool>], x: usize, y: usize) -> usize {
    let w = tm[0].len();
    tm.iter()
        .step_by(y)
        .enumerate()
        .filter(|(ix, r)| r[(ix * x) % w])
        .count()
}

fn p1(input: &[Vec<bool>]) -> usize {
    solve(input, 3, 1)
}
fn p2(input: &[Vec<bool>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&s| solve(input, s.0, s.1))
        .product()
}
