use std::collections::HashMap;
use utils::cartesian::{Dir, Point};

fn gen(input: &str) -> HashMap<Point<i64>,usize> {
    let mut hm = HashMap::new();
    let mut shortests = HashMap::new();
    let mut stack = Vec::new();
    let mut position = Point::new(0, 0);
    let mut distance = 0;
    hm.insert(position, 'X');
    for c in input.trim().chars() {
        match c {
            '^' => (),
            '$' => (),
            'N' | 'E' | 'W' | 'S' => {
                let dir = Dir::from_x("NSEW", c);
                let door_c = if c == 'N' || c == 'S' { '-' } else { '|' };
                hm.insert(position.step(dir), door_c);
                position += dir.as_point_step() * 2;
                hm.insert(position, '.');
                distance = *shortests.entry(position).or_insert(distance + 1);
            }
            '(' => {
                stack.push((distance, position));
            }
            '|' => {
                let (d, p) = *stack.last().unwrap();
                distance = d;
                position = p;
            }
            ')' => {
                stack.pop();
            }
            _ => { panic!("unknown char: {}", c); }
        }
    }
    shortests
}

fn p1(input: &str) -> usize {
    *gen(input).values().max().unwrap()
}

fn p2(input: &str) -> usize {
    gen(input).values().filter(|x| **x >= 1000).count()
}
