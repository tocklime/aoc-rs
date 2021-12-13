use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc_harness::*;
use utils::cartesian::{self, Point};

aoc_main!(2021 day 13, generator whole_input_is::<X>, part1 [p1], part2 [p2], example part1 EG => 17);

const EG: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
#[derive(Debug, Clone)]
enum Fold {
    AlongY(usize),
    AlongX(usize),
}
impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split("=");
        match i.next().unwrap() {
            "fold along y" => Ok(Self::AlongY(i.next().unwrap().parse().unwrap())),
            "fold along x" => Ok(Self::AlongX(i.next().unwrap().parse().unwrap())),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
struct X {
    grid: HashSet<Point<usize>>,
    folds: Vec<Fold>,
}

impl FromStr for X {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dbg!(s);
        let mut i = s.split("\n\n");
        let grid = i
            .next()
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let folds = i
            .next()
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        Ok(Self { grid, folds })
    }
}
impl X {
    fn fold(&mut self, f: &Fold) {
        let new_map = match f {
            Fold::AlongY(y) => self
                .grid
                .iter()
                .map(|&p| {
                    if p.y > *y {
                        Point::new(p.x, 2 * *y - p.y)
                    } else {
                        p
                    }
                })
                .collect(),
            Fold::AlongX(x) => self
                .grid
                .iter()
                .map(|&p| {
                    if p.x > *x {
                        Point::new(2 * *x - p.x, p.y)
                    } else {
                        p
                    }
                })
                .collect(),
        };
        self.grid = new_map;
    }
}
fn p1(input: &X) -> usize {
    let mut x = input.clone();
    x.fold(&input.folds[0]);
    x.grid.len()
}

fn p2(input: &X) -> String {
    let mut x = input.clone();
    for f in &input.folds {
        x.fold(&f);
    }
    let map: HashMap<Point<usize>, char> = x.grid.iter().map(|x| (*x, '#')).collect();
    let s = cartesian::render_char_map_w(&map, 1, ".", false);
    println!("{}", s);
    s
}
