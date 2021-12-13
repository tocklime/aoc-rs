use std::{collections::HashSet, str::FromStr};

use aoc_harness::*;
use utils::{
    cartesian::{render_set_w, Point},
    ocr::OcrString,
};

aoc_main!(2021 day 13, generator whole_input_is::<X>, part1 [p1] => 653, part2 [p2] => "LKRFBPRK", example part1 EG => 17);

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
        let mut i = s.split('=');
        match i.next().unwrap() {
            "fold along y" => Ok(Self::AlongY(i.next().unwrap().parse().unwrap())),
            "fold along x" => Ok(Self::AlongX(i.next().unwrap().parse().unwrap())),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
struct X {
    grid: Vec<Point<usize>>,
    folds: Vec<Fold>,
}

impl FromStr for X {
    type Err = ();

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
impl Fold {
    fn apply_to_point(&self, p: Point<usize>) -> Point<usize> {
        match self {
            Fold::AlongY(y) => {
                if p.y > *y {
                    Point::new(p.x, 2 * *y - p.y)
                } else {
                    p
                }
            }
            Fold::AlongX(x) => {
                if p.x > *x {
                    Point::new(2 * *x - p.x, p.y)
                } else {
                    p
                }
            }
        }
    }
}
impl X {
    fn after_n_folds(&self, n: usize) -> HashSet<Point<usize>> {
        self.grid
            .iter()
            .map(|&p| self.folds[0..n].iter().fold(p, |p, f| f.apply_to_point(p)))
            .collect()
    }
}
fn p1(input: &X) -> usize {
    input.after_n_folds(1).len()
}

fn p2(input: &X) -> OcrString {
    let s = render_set_w(&input.after_n_folds(input.folds.len()), '#', ' ', false);
    OcrString::new(s, '#')
}
