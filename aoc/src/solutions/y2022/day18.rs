use std::collections::HashSet;

use aoc_harness::*;
use nom::{bytes::complete::tag, character::complete, multi::separated_list1, IResult};

aoc_main!(2022 day 18, part1 [p1] => 3432, part2 [p2] => 2042, example both EG0 => (10,10), example both EG => (64,58));

const EG0: &str = "1,1,1\n2,1,1";
const EG: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(tag(","), complete::i32)(input)
}
fn neighbours(c: &[i32]) -> impl Iterator<Item = Vec<i32>> {
    let (x, y, z) = (c[0], c[1], c[2]);
    vec![
        vec![x, y, z - 1],
        vec![x, y, z + 1],
        vec![x, y - 1, z],
        vec![x, y + 1, z],
        vec![x - 1, y, z],
        vec![x + 1, y, z],
    ]
    .into_iter()
}
fn p1(input: &str) -> usize {
    let mut occupied = HashSet::new();
    for l in input.lines() {
        let (_, ns) = parse_line(l).unwrap();
        occupied.insert(ns);
    }
    occupied
        .iter()
        .map(|c| neighbours(c).filter(|n| !occupied.contains(n)).count())
        .sum()
}

fn p2(input: &str) -> usize {
    let mut occupied = HashSet::new();
    for l in input.lines() {
        let (_, ns) = parse_line(l).unwrap();
        occupied.insert(ns);
    }
    let mut ans = 0;
    let mut not_blocked = HashSet::new();
    not_blocked.insert(vec![0, 0, 0]);
    for c in &occupied {
        for n in neighbours(c) {
            //can n escape? lets assume the origin is outside.
            //find a path to it.
            if occupied.contains(&n) {
                continue;
            }
            if not_blocked.contains(&n) {
                ans += 1;
            } else {
                let x = pathfinding::directed::bfs::bfs(
                    &n,
                    |n| neighbours(n).filter(|o| !occupied.contains(o)),
                    |x| not_blocked.contains(x),
                );
                if let Some(p) = x {
                    not_blocked.extend(p);
                    ans += 1;
                } 
            }
        }
    }
    ans
}
