use aoc_harness::*;

aoc_main!(2022 day 2, part1 [p1], part2 [p2], example both EG => (15, 12));

fn p1(input: &str) -> usize {
    let mut score: usize = 0;
    for l in input.lines() {
        let (they, me) = l.split_once(' ').unwrap();
        let they = they.as_bytes()[0] - b'A';
        let me = me.as_bytes()[0] - b'X';
        let shape_score = 1 + me;
        let win_score = match (they, me) {
            (a, b) if a == b => 3,
            (0, 1) | (1, 2) | (2, 0) => 6,
            _ => 0,
        };
        score += usize::from(shape_score) + win_score
    }
    score
}

fn p2(input: &str) -> usize {
    let mut score: usize = 0;
    for l in input.lines() {
        let (they, outcome) = l.split_once(' ').unwrap();
        let they = they.as_bytes()[0] - b'A';
        let outcome = outcome.as_bytes()[0] - b'X'; //0: lose, 1: draw, 2: win
        let me = match (they, outcome) {
            (a, 0) => (a + 2) % 3,
            (a, 1) => a,
            (a, 2) => (a + 1) % 3,
            _ => unreachable!(),
        };
        let shape_score = 1 + me;
        let win_score = outcome * 3;
        score += usize::from(shape_score) + usize::from(win_score);
    }
    score
}
// 6331 wrong.
const EG: &str = "A Y
B X
C Z
";
