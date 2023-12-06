use std::str::FromStr;

use aoc_harness::*;

aoc_harness::aoc_main!(2021 day 10, generator lines::<StackEval>, part1 [p1] => 411_471, part2 [p2] => 3_122_628_974,
          example part1 EG => 26_397, example part2 EG => 288_957);

const EG: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

enum StackEval {
    Corrupt(u8),
    Valid(Vec<u8>),
}
impl FromStr for StackEval {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for c in s.bytes() {
            match (c, stack.last()) {
                (b'(', _) => stack.push(b')'),
                (b'{', _) => stack.push(b'}'),
                (b'[', _) => stack.push(b']'),
                (b'<', _) => stack.push(b'>'),
                (a, Some(&b)) if a == b => {
                    stack.pop().unwrap();
                }
                (x, _) => return Ok(StackEval::Corrupt(x)),
            }
        }
        Ok(StackEval::Valid(stack))
    }
}
impl StackEval {
    fn score_corrupt(&self) -> Option<usize> {
        match self {
            StackEval::Corrupt(b')') => Some(3),
            StackEval::Corrupt(b']') => Some(57),
            StackEval::Corrupt(b'}') => Some(1197),
            StackEval::Corrupt(b'>') => Some(25137),
            _ => None,
        }
    }
    fn score_incomplete(&self) -> Option<usize> {
        match self {
            StackEval::Corrupt(_) => None,
            StackEval::Valid(stack) => Some(stack.iter().rev().fold(0, |s, c| {
                let a = match c {
                    b')' => 1,
                    b']' => 2,
                    b'}' => 3,
                    b'>' => 4,
                    _ => unreachable!(),
                };
                s * 5 + a
            })),
        }
    }
}

fn p1(input: &[StackEval]) -> usize {
    input.iter().filter_map(StackEval::score_corrupt).sum()
}
fn p2(input: &[StackEval]) -> usize {
    let s = input
        .iter()
        .filter_map(StackEval::score_incomplete)
        .collect_vec();
    *s.iter().k_smallest(s.len() / 2 + 1).last().unwrap()
}
