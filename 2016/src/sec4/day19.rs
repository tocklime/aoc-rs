use nom::lib::std::collections::VecDeque;

#[aoc(day19, part1)]
fn p1(input: &str) -> usize {
    let n = input.trim().parse::<usize>().unwrap();
    let mut arr: VecDeque<usize> = (1..=n).collect();
    while arr.len() > 1 {
        let this_elf = arr.pop_front().unwrap();
        arr.pop_front();
        arr.push_back(this_elf);
    }
    arr[0]
}

#[aoc(day19, part2)]
fn p2(input: &str) -> usize {
    let n = input.trim().parse::<usize>().unwrap();
    solve2(n)
}
fn solve2(n: usize) -> usize {
    let mut left: VecDeque<usize> = (1..=n / 2).collect();
    let mut right: VecDeque<usize> = ((n / 2) + 1..=n).collect();
    while left.len() + right.len() > 1 {
        let this_elf = left.pop_front().expect("no left");
        right.pop_front().expect("no right to eliminate");
        if right.len() > left.len() {
            left.push_back(right.pop_front().expect("no right"));
        }
        right.push_back(this_elf);
    }
    right[0]
}

#[test]
fn test19() {
    assert_eq!(solve2(5),2);
}