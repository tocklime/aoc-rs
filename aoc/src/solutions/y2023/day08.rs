use std::collections::HashMap;

use itertools::Itertools;
use num::Integer;

aoc_harness::aoc_main!(2023 day 8, part1 [p1], part2 [p2], example part1 EG => 2, example part1 EG2 => 6, example part2 EG3 => 6);

fn p1(input: &str) -> usize {
    let mut l = input.lines();
    let rls = l.next().unwrap();
    let _ = l.next().unwrap();
    let map = l.map(|l| {
        let (from, to) = l.split_once(" = ").unwrap();
        let (l,r) = to[1..to.len()-1].split_once(", ").unwrap();
        (from.to_owned(), (l.to_owned(),r.to_owned()))
    }).collect::<HashMap<_,_>>();
    
    let mut pos = "AAA";
    for (count, dir) in rls.chars().cycle().enumerate() {
        if pos == "ZZZ" {
            return count;
        }
        pos = match dir {
            'R' => &map[pos].1,
            'L' => &map[pos].0,
            
            _ => panic!("Unknown dir {dir}")
        };
    }
    unreachable!()
}

const EG: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

const EG2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
const EG3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

fn p2(input: &str) -> usize {
    let mut l = input.lines();
    let rls = l.next().unwrap();
    let _ = l.next().unwrap();
    let map = l.map(|l| {
        let (from, to) = l.split_once(" = ").unwrap();
        let (l,r) = to[1..to.len()-1].split_once(", ").unwrap();
        (from.to_owned(), (l.to_owned(),r.to_owned()))
    }).collect::<HashMap<_,_>>();
    let mut pos = map.keys().filter(|x| x.ends_with('A')).collect_vec();
    let mut anss = Vec::new();
    for (ix, p) in pos.iter().enumerate() {
        let mut pos = *p;
        let mut finish_count = 0;
        for (count, dir) in rls.chars().cycle().enumerate() {
            if pos.ends_with('Z') {
                finish_count += 1;
                println!("pos {ix} finishes {finish_count}th after {count} steps");
                anss.push(count);
                break;
            }
            pos = match dir {
                'R' => &map[pos].1,
                'L' => &map[pos].0,
                
                _ => panic!("Unknown dir {dir}")
            };
        }
    }
    dbg!(&anss);
    return anss.iter().fold(1, |a, b| b.lcm(&a));
    for (count, dir) in rls.chars().cycle().enumerate() {
        if pos.iter().all(|x| x.ends_with('Z')) {
            return count;
        }
        pos = pos.into_iter().enumerate().map(|(ix,p)| {
            // if p.ends_with('Z') {
                // println!("Pos {ix} is at end after {count} steps");
            // }

            match dir {
                'R' => &map[p].1,
                'L' => &map[p].0,
                _ => panic!("Unknown dir {dir}")
            }
        }).collect();
    }
    unreachable!()
}