use std::str::FromStr;

use aoc_harness::*;

aoc_main!(2021 day 10, generator lines::<StackEval>, part1 [p1] => 411471, part2 [p2] => 3122628974,
          example part1 EG => 26397, example part2 EG => 288957);

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
    Corrupt(char),
    Valid(Vec<char>),
}
impl FromStr for StackEval {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for c in s.chars() {
            let expected = stack.last();
            match (c, expected) {
                ('(', _) => stack.push(')'),
                ('{', _) => stack.push('}'),
                ('[', _) => stack.push(']'),
                ('<', _) => stack.push('>'),
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
            StackEval::Corrupt(')') => Some(3),
            StackEval::Corrupt(']') => Some(57),
            StackEval::Corrupt('}') => Some(1197),
            StackEval::Corrupt('>') => Some(25137),
            _ => None,
        }
    }
    fn score_incomplete(&self) -> Option<usize> {
        match self {
            StackEval::Corrupt(_) => None,
            StackEval::Valid(stack) => Some(stack.iter().rev().fold(0, |s, c| {
                let a = match c {
                    ')' => 1,
                    '}' => 3,
                    ']' => 2,
                    '>' => 4,
                    _ => unreachable!(),
                };
                s * 5 + a
            })),
        }
    }
}

fn p1(input: &[StackEval]) -> usize {
    input.iter().filter_map(|l| l.score_corrupt()).sum()
}
fn p2(input: &[StackEval]) -> usize {
    let mut s = input
        .iter()
        .filter_map(|l| l.score_incomplete())
        .collect_vec();
    s.sort();
    s[s.len() / 2]
}
