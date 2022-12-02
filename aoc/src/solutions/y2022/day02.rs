use aoc_harness::*;

aoc_main!(2022 day 2, part1 [p1] => 11150, part2 [p2] => 8295, example both EG => (15, 12));

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (they, me) = l.split_once(' ').unwrap();
            let they = usize::from(they.as_bytes()[0] - b'A');
            let me = usize::from(me.as_bytes()[0] - b'X');
            let outcome = (4 + me - they) % 3;
            (1 + me) + (outcome * 3)
        })
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (they, me) = l.split_once(' ').unwrap();
            let they = usize::from(they.as_bytes()[0] - b'A');
            let outcome = usize::from(me.as_bytes()[0] - b'X'); //0: lose, 1: draw, 2: win
            let me = (they + outcome + 2) % 3;
            (1 + me) + (outcome * 3)
        })
        .sum()
}

const EG: &str = "A Y
B X
C Z
";
