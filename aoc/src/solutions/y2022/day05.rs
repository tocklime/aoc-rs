use aoc_harness::*;

aoc_main!(2022 day 5, part1 [p1], part2 [p2], example part1 EG => "CMZ");

const EG: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";



fn p1(input: &str) -> String {
    let (map,instrs) = input.split_once("\n\n").unwrap();
    let w = map.lines().map(|x| x.len()).max().unwrap();
    let stack_count = (w+1)/4;
    let mut stacks = vec![vec![]; stack_count];
    for l in map.lines().rev().skip(1) {
        for (ix,c) in l.chars().enumerate() {
            if c.is_alphabetic() {
                stacks[(ix-1)/4].push(c);
            }
        }
    }
    for l in instrs.lines() {
        //move {} from {} to {}
        let ws = l.split_ascii_whitespace().collect_vec();
        let count: usize = ws[1].parse().unwrap();
        let from: usize = ws[3].parse().unwrap();
        let to: usize = ws[5].parse().unwrap();
        for _ in 0..count {
            let x = stacks[from-1].pop().unwrap();
            stacks[to-1].push(x);
        }
    }
    let s : String = stacks.iter().map(|x| x.last().unwrap()).collect();
    s

}

fn p2(input: &str) -> String {
    let (map,instrs) = input.split_once("\n\n").unwrap();
    let w = map.lines().map(|x| x.len()).max().unwrap();
    let stack_count = (w+1)/4;
    let mut stacks = vec![vec![]; stack_count];
    for l in map.lines().rev().skip(1) {
        for (ix,c) in l.chars().enumerate() {
            if c.is_alphabetic() {
                stacks[(ix-1)/4].push(c);
            }
        }
    }
    for l in instrs.lines() {
        //move {} from {} to {}
        let ws = l.split_ascii_whitespace().collect_vec();
        let count: usize = ws[1].parse().unwrap();
        let from: usize = ws[3].parse().unwrap();
        let to: usize = ws[5].parse().unwrap();
        let mut carry = vec![];
        for _ in 0..count {
            let x = stacks[from-1].pop().unwrap();
            carry.push(x);
        }
        while let Some(c) = carry.pop() {
            stacks[to-1].push(c);
        }
    }
    let s : String = stacks.iter().map(|x| x.last().unwrap()).collect();
    s

}