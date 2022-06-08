use aoc_harness::aoc_main;

aoc_main!(2015 day 6, generator gen, part1 [p1], part2 [p2]);
use itertools::Itertools;
use utils::cartesian::Point;

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


fn gen(input: &str) -> Vec<Line> {
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


fn p1(input: &[Line]) -> usize {
    let mut grid = vec![vec![false; 1000]; 1000];
    for l in input {
        for row in grid.iter_mut().take(l.b.x + 1).skip(l.a.x) {
            for cell in row.iter_mut().take(l.b.y + 1).skip(l.a.y) {
                *cell = match l.i {
                    Instr::On => true,
                    Instr::Off => false,
                    Instr::Toggle => !*cell
                };
            }
        }
    }

    grid.iter().flat_map(|x| x.iter()).filter(|&x| *x).count()
}


fn p2(input: &[Line]) -> u32 {
    let mut grid = vec![vec![0; 1000]; 1000];
    for l in input {
        for row in grid.iter_mut().take(l.b.x + 1).skip(l.a.x) {
            for cell in row.iter_mut().take(l.b.y + 1).skip(l.a.y) {
                *cell = match l.i {
                    Instr::On => *cell + 1,
                    Instr::Off => if *cell > 0 { *cell - 1 } else { 0 }
                    Instr::Toggle => *cell + 2
                };
            }
        }
    }
    grid.iter().flat_map(|x| x.iter()).sum()
}
