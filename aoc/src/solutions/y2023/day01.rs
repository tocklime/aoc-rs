use aoc_harness::*;

aoc_main!(2023 day 1, part1 [p1] => 54877, part2 [p2] => 54100, example part1 EG => 142, example part2 EG2 => 281);

fn p1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let first = l.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = l.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            first * 10 + last
        })
        .sum()
}

const EG: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

fn find_from_pos(input: &str, pos: usize) -> Option<u32> {
    let as_bytes = &input.as_bytes()[pos..];
    if let Some(d) = char::from(as_bytes[0]).to_digit(10) {
        return Some(d);
    }
    match &as_bytes[0..3.min(as_bytes.len())] {
        b"one" => Some(1),
        b"two" => Some(2),
        b"thr" => as_bytes.starts_with(b"three").then_some(3),
        b"fou" => as_bytes.starts_with(b"four").then_some(4),
        b"fiv" => as_bytes.starts_with(b"five").then_some(5),
        b"six" => Some(6),
        b"sev" => as_bytes.starts_with(b"seven").then_some(7),
        b"eig" => as_bytes.starts_with(b"eight").then_some(8),
        b"nin" => as_bytes.starts_with(b"nine").then_some(9),
        _ => None,
    }
}

fn p2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let first = (0..l.len()).find_map(|i| find_from_pos(l, i)).unwrap();
            let last = (0..l.len())
                .rev()
                .find_map(|i| find_from_pos(l, i))
                .unwrap();
            first * 10 + last
        })
        .sum()
}

const EG2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
