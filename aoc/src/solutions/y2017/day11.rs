use aoc_harness::aoc_main;

aoc_main!(2017 day 11, part1 [p1], part2 [p2]);
use num::abs;
use std::cmp::max;

#[derive(Debug, Copy, Clone)]
struct Pos {
    n: i32,
    ne: i32,
}

impl Pos {
    fn origin() -> Self {
        Pos { n: 0, ne: 0 }
    }
    fn dist(self) -> i32 {
        if (self.n > 0) != (self.ne > 0) {
            //is nw or se.
            max(abs(self.n), abs(self.ne))
        } else {
            abs(self.n) + abs(self.ne)
        }
    }
    fn step(&mut self, dir: &str) {
        match dir {
            "n" => {
                self.n += 1;
            }
            "ne" => {
                self.ne += 1;
            }
            "se" => {
                self.n -= 1;
                self.ne += 1;
            }
            "s" => {
                self.n -= 1;
            }
            "sw" => {
                self.ne -= 1;
            }
            "nw" => {
                self.n += 1;
                self.ne -= 1;
            }
            _ => panic!("unknown step: {}", dir),
        }
    }
}

fn p1(input: &str) -> i32 {
    let mut pos = Pos::origin();
    input.trim().split(',').for_each(|x| pos.step(x));
    pos.dist()
}

fn p2(input: &str) -> i32 {
    input
        .trim()
        .split(',')
        .scan(Pos::origin(), |state, x| {
            state.step(x);
            Some(state.dist())
        })
        .max()
        .unwrap()
}
