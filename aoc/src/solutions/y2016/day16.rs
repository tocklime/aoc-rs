use aoc_harness::aoc_main;

aoc_main!(2016 day 16, part1 [p1], part2 [p2]);
fn expand(a: &mut Vec<char>) {
    let len = a.len();
    a.push('0');
    for ix in (0..len).rev() {
        a.push(match a.get(ix) {
            Some('0') => '1',
            _ => '0',
        });
    }
}

fn collapse(a: &[char]) -> Vec<char> {
    (0..a.len())
        .step_by(2)
        .map(|x| if a.get(x) == a.get(x + 1) { '1' } else { '0' })
        .collect()
}

fn p1(input: &str) -> String {
    solve(input.trim(), 272)
}

fn p2(input: &str) -> String {
    solve(input.trim(), 35_651_584)
}

fn solve(input: &str, size: usize) -> String {
    let mut a: Vec<char> = input.chars().collect();
    while a.len() < size {
        expand(&mut a);
    }
    let mut check = collapse(&a[0..size]);
    while check.len() % 2 == 0 {
        check = collapse(&check);
    }
    check.iter().collect()
}

//wrong: 10010001100100111
#[test]
fn test16() {
    use itertools::Itertools;
    assert_eq!(
        collapse(&"110010110100".chars().collect_vec()),
        "110101".chars().collect_vec()
    );
    assert_eq!(solve("10000", 20), "01100");
}
