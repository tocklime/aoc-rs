#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<Vec<bool>> {
    input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}
pub fn solve(tm: &[Vec<bool>], x: usize, y: usize) -> usize {
    let w = tm[0].len();
    tm.iter()
        .step_by(y)
        .enumerate()
        .filter(|(ix, r)| r[(ix * x) % w])
        .count()
}

#[aoc(day3, part1)]
pub fn p1(input: &[Vec<bool>]) -> usize {
    solve(input, 3, 1)
}
#[aoc(day3, part2)]
pub fn p2(input: &[Vec<bool>]) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&s| solve(input, s.0, s.1))
        .product()
}
