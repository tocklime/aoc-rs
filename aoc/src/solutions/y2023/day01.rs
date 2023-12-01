use aoc_harness::*;

aoc_main!(2023 day 1, part1 [p1], part2 [p2], example part1 EG => 142, example part2 EG2 => 281); 

fn p1(_input: &str) -> i32 {
    let mut total = 0;
    for l in _input.lines() {
        let first_digit = l.chars().find(|c| c.is_digit(10)).unwrap();
        let last_digit = l.chars().rev().find(|c| c.is_digit(10)).unwrap();
        let str : i32 = (&format!("{first_digit}{last_digit}")).parse().unwrap();
        total += str;
    }
    total
}

const EG : &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";


fn find_from_pos(input: &str, pos: usize) -> Option<u8> {
    println!("Finding from pos {pos} in {input}");
    let as_bytes = &input.as_bytes()[pos..];
    let c = as_bytes[0];
    if c.is_ascii_digit() {
        return Some(c - b'0');
    }
    match c {
        b'z' => if as_bytes.starts_with(b"zero") { Some(0) } else { None }
        b'o' => if as_bytes.starts_with(b"one") {Some(1)} else { None }
        b't' => if as_bytes.starts_with(b"two") { Some(2) } else if as_bytes.starts_with(b"three") {Some(3)} else {None}
        b'f' => if as_bytes.starts_with(b"four") { Some(4) } else if as_bytes.starts_with(b"five") {Some(5)} else {None}
        b's' => if as_bytes.starts_with(b"six") { Some(6) } else if as_bytes.starts_with(b"seven") {Some(7)} else {None}
        b'e' => if as_bytes.starts_with(b"eight") {Some(8)} else { None }
        b'n' => if as_bytes.starts_with(b"nine") {Some(9)} else { None }
        _ => None
    }

}

fn p2(_input: &str) -> i32 {
    let mut total = 0;
    for l in _input.lines() {
        let first = (0..l.len()).find_map(|i| find_from_pos(l, i)).unwrap();
        let last = (0..l.len()).rev().find_map(|i| find_from_pos(l, i)).unwrap();
        let str : i32 = (&format!("{first}{last}")).parse().unwrap();
        total += str;
    }
    total
}

const EG2 : &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";