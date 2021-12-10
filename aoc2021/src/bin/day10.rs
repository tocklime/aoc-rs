use std::str::FromStr;

use aoc_harness::*;

aoc_main!(2021 day 10, part1 [p1], part2 [p2],
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

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let mut stack = Vec::new();
            let mut score = 0;
            for c in l.chars() {
                let expected = stack.last();
                match (c, expected) {
                    ('(', _) => stack.push(')'),
                    ('{', _) => stack.push('}'),
                    ('[', _) => stack.push(']'),
                    ('<', _) => stack.push('>'),
                    (a, Some(&b)) if a == b => {
                        stack.pop().unwrap();
                    }
                    (')', _) => {
                        score += 3;
                        break;
                    }
                    ('}', _) => {
                        score += 1197;
                        break;
                    }
                    (']', _) => {
                        score += 57;
                        break;
                    }
                    ('>', _) => {
                        score += 25137;
                        break;
                    }
                    _ => unreachable!(),
                }
            }
            score
        })
        .sum::<usize>()
}

fn p2(input: &str) -> usize {
    let mut s = input
        .lines()
        .filter_map(|l| {
            let mut stack = Vec::new();
            let mut score = 0;
            for c in l.chars() {
                let expected = stack.last();
                match (c, expected) {
                    ('(', _) => stack.push(')'),
                    ('{', _) => stack.push('}'),
                    ('[', _) => stack.push(']'),
                    ('<', _) => stack.push('>'),
                    (a, Some(&b)) if a == b => {
                        stack.pop().unwrap();
                    }
                    (')', _) => {
                        return None;
                    }
                    ('}', _) => {
                        return None;
                    }
                    (']', _) => {
                        return None;
                    }
                    ('>', _) => {
                        return None;
                    }
                    _ => unreachable!(),
                }
            }
            Some(stack)
        })
        .map(|stack| {
            let score = stack.iter().rev().fold(0, |s, c| {
                let a = match c {
                    ')' => 1,
                    '}' => 3,
                    ']' => 2,
                    '>' => 4,
                    _ => unreachable!(),
                };
                s * 5 + a
            });
            score
        })
        .collect_vec();
    s.sort();
    dbg!(&s);
    s[s.len() / 2]
}

//wrong: 2360429063
