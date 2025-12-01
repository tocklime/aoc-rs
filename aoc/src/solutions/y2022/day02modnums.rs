
use num_modular::*;

aoc_harness::aoc_main!(2022 day 2, generator gen_, part1 [p1] => 11150, part2 [p2] => 8295, example both EG => (15, 12));

type T = ReducedInt<u8, Montgomery<u8>>;

fn gen_(input: &str) -> Vec<(T, T)> {
    let zero: T = num_modular::MontgomeryInt::new(0_u8, &3_u8);
    input
        .lines()
        .map(|l| {
            let they = zero.convert(l.as_bytes()[0] - b'A');
            let me = zero.convert(l.as_bytes()[2] - b'X');
            (they, me)
        })
        .collect()
}

// me_shape is 0 = ROCK, 1 = PAPER, 2 = SCISSORS.
// outcome is 0 = LOSS, 1 = DRAW, 2 = WIN.
// score is shape score (1 + me_shape)
// and win score (outcome * 3)
// round score is (1 + me_shape) + (outcome * 3)

// in part one we calculate outcome from me and they.
fn p1(input: &[(T, T)]) -> usize {
    input
        .iter()
        .map(|(they, me)| {
            1 + usize::from(me.residue()) + usize::from((me - they + 1).residue()) * 3
        })
        .sum()
}

//in part two we calculate our shape from they and outcome.
fn p2(input: &[(T, T)]) -> usize {
    input
        .iter()
        .map(|(they, outcome)| {
            1 + usize::from((they + outcome + 2).residue()) + usize::from(outcome.residue()) * 3
        })
        .sum()
}

const EG: &str = "A Y
B X
C Z
";
