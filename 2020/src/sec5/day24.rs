use std::collections::{HashMap, HashSet};

use crate::utils::cartesian::Point;


//COORDS base are +x is e , +y is ne.

pub fn parse_line(input:&str) -> Point<i32> {
    let mut point = Point::new(0,0);
    let mut pending = None;
    for c in input.chars() {
        match (pending,c) {
            (None,'e') => point = point.right(),
            (None,'w') => point = point.left(),
            (None,'n') => pending = Some('n'),
            (None,'s') => pending = Some('s'),
            (Some('n'),'e') => {
                pending = None;
                point = point.up();
            },
            (Some('n'),'w') => {
                pending = None;
                point = point.up().left();
            },
            (Some('s'),'e') => {
                pending = None;
                point = point.down().right();
            },
            (Some('s'),'w') => {
                pending = None;
                point = point.down();
            },
            _ => panic!("wat")
        }
    }
    point
}
pub fn make_floor(input: &str) -> HashSet<Point<i32>> {
    let mut set = HashSet::new();
    for l in input.lines() {
        let p = parse_line(l);
        if set.contains(&p) {
            set.remove(&p);
        }else {
            set.insert(p);
        }
    }
    set

}

#[aoc(day24,part1)]
pub fn p1(input: &str) -> usize {
    make_floor(input).len()
}
pub fn neighbours(p: &Point<i32>) -> [Point<i32>;6] {
    [p.up(),p.right(),p.down().right(),p.down(),p.left(),p.up().left()]
}
    
pub fn step(a: &HashSet<Point<i32>>) -> HashSet<Point<i32>> {
    let mut counts : HashMap<Point<i32>,usize> = HashMap::new();
    for s in a {
        for n in &neighbours(s) {
            *counts.entry(*n).or_default() += 1;
        }
    }
    counts.iter().filter(|(p,c)| {
        let was_alive = a.contains(p);
        (**c == 2 && !was_alive) || (was_alive && (1..=2).contains(*c))
    }).map(|(a,b)| a).copied().collect()
}
#[aoc(day24,part2)]
pub fn p2(input: &str) -> usize {
    let mut s = make_floor(input);
    for _ in 0..100 {
        s = step(&s);
    }
    s.len()
}