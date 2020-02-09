use std::collections::VecDeque;

#[aoc(day17, part1)]
fn p1(input: &str) -> usize {
    let step = input.trim().parse::<usize>().unwrap();
    let mut d = VecDeque::<usize>::new();
    d.push_back(0);
    let mut pos = 0;
    for n in 1..=2017 {
        pos = ((pos + step) % n) + 1;
        d.insert(pos, n);
        println!("{}",pos);
    }
    d[pos+1]
}
//1129 toolow
