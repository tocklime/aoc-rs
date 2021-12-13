use std::{collections::HashSet, str::FromStr};

use aoc_harness::*;
use utils::{
    cartesian::{render_set_w, Point},
    debug_as_display::DebugAsDisplay,
};

aoc_main!(2021 day 13, generator whole_input_is::<X>, part1 [p1] => 653, part2 [p2], example part1 EG => 17);

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
fn fold(map: &HashSet<Point<usize>>, f: &Fold) -> HashSet<Point<usize>> {
    match f {
        Fold::AlongY(y) => map
            .iter()
            .map(|&p| {
                if p.y > *y {
                    Point::new(p.x, 2 * *y - p.y)
                } else {
                    p
                }
            })
            .collect(),
        Fold::AlongX(x) => map
            .iter()
            .map(|&p| {
                if p.x > *x {
                    Point::new(2 * *x - p.x, p.y)
                } else {
                    p
                }
            })
            .collect(),
    }
}
fn p1(input: &X) -> usize {
    fold(&input.grid, &input.folds[0]).len()
}

fn p2(input: &X) -> DebugAsDisplay<String> {
    let set = input
        .folds
        .iter()
        .fold(input.grid.clone(), |g, f| fold(&g, f));
    render_set_w(&set, '#', ' ', false).into()
}
