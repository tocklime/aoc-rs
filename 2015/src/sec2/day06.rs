use itertools::Itertools;
use crate::utils::cartesian::Point;

pub enum Instr {
    On,
    Off,
    Toggle,
}

pub struct Line {
    i: Instr,
    a: Point<usize>,
    b: Point<usize>,
}

#[aoc_generator(day6)]
pub fn gen(input: &str) -> Vec<Line> {
    let re = regex::Regex::new("(turn on|turn off|toggle) (\\d+),(\\d+) through (\\d+),(\\d+)").unwrap();
    input.lines().map(|l| {
//turn on 489,959 through 759,964
        let parsed = re.captures(l).unwrap_or_else(|| panic!("Bad line {}", l));
        let mode = match parsed.get(1).unwrap().as_str() {
            "turn on" => Instr::On,
            "turn off" => Instr::Off,
            "toggle" => Instr::Toggle,
            p => panic!("Unknown instruction {} on line {}", p, l)
        };
        let coords = (0..4).map(|i| parsed.get(i + 2).unwrap().as_str().parse::<usize>().unwrap()).collect_vec();
        Line {
            i: mode,
            a: Point::new(coords[0], coords[1]),
            b: Point::new(coords[2], coords[3]),
        }
    }).collect()
}

#[aoc(day6, part1)]
pub fn p1(input: &[Line]) -> usize {
    let mut grid = vec![vec![false; 1000]; 1000];
    for l in input {
        for x in l.a.x..=l.b.x {
            for y in l.a.y..=l.b.y {
                grid[x][y] = match l.i {
                    Instr::On => true,
                    Instr::Off => false,
                    Instr::Toggle => !grid[x][y]
                };
            }
        }
    }

    grid.iter().flat_map(|x| x.iter()).filter(|&x| *x).count()
}

#[aoc(day6, part2)]
pub fn p2(input: &[Line]) -> u32 {
    let mut grid = vec![vec![0; 1000]; 1000];
    for l in input {
        for x in l.a.x..=l.b.x {
            for y in l.a.y..=l.b.y {
                let curr = grid[x][y];
                grid[x][y] = match l.i {
                    Instr::On => curr + 1,
                    Instr::Off => if curr > 0 { curr - 1 } else { 0 }
                    Instr::Toggle => curr + 2
                };
            }
        }
    }
    grid.iter().flat_map(|x| x.iter()).sum()
}
