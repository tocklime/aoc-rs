

aoc_harness::aoc_main!(2022 day 2, generator gen_, part1 [p1] => 11150, part2 [p2] => 8295, example both EG => (15, 12));

fn gen_(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|l| {
            let (they, me) = l.split_once(' ').unwrap();
            let they = usize::from(they.as_bytes()[0] - b'A');
            let me = usize::from(me.as_bytes()[0] - b'X');
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
fn p1(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .map(|(they, me)| (1 + me) + (((4 + me - they) % 3) * 3))
        .sum()
}

//in part two we calculate our shape from they and outcome.
fn p2(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .map(|(they, outcome)| (1 + ((they + outcome + 2) % 3)) + (outcome * 3))
        .sum()
}

const EG: &str = "A Y
B X
C Z
";
