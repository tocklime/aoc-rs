use std::{
    cmp::{self, Ordering},
    collections::{hash_map::RandomState, BTreeSet},
};

use itertools::Positions;
use num::iter::RangeInclusive;
use scan_fmt::scan_fmt;

use aoc_harness::*;
use utils::{aabb::Aabb, cartesian::Point};

aoc_main!(2021 day 17, part1 [p1], part2 [p2], example part1 EG => 45, example part2 EG => 112);

const EG: &str = "target area: x=20..30, y=-10..-5";

struct Probe {
    pos: Point<i64>,
    velocity: Point<i64>,
}

impl Probe {
    fn step(&mut self) {
        // self.pos.x += self.velocity.x;
        // self.pos.y += self.velocity.y;
        self.pos += self.velocity;
        self.velocity.x += match self.pos.x.cmp(&0) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1,
        };
        self.velocity.y -= 1;
    }
    fn is_in_range(&self, target: Aabb<i64>) -> bool {
        target.contains(&self.pos)
    }
}
fn x_posses(mut vel: i64) -> impl Iterator<Item = i64> {
    let mut pos = 0;
    std::iter::from_fn(move || {
        pos += vel;
        vel -= 1;
        if vel == -1 {
            None
        } else {
            Some(pos)
        }
    })
}
fn y_posses_to(mut vel: i64, pos_limit: i64) -> impl Iterator<Item = i64> {
    let mut pos = 0;
    std::iter::from_fn(move || {
        pos += vel;
        vel -= 1;
        if pos < pos_limit {
            None
        } else {
            Some(pos)
        }
    })
}
fn find_x_speeds(xmin: i64, xmax: i64) -> impl Iterator<Item = ((usize, Option<usize>), i64)> {
    (0..=xmax).filter_map(move |x| {
        let mut min_step = None;
        let mut max_step = None;
        for (step, xp) in x_posses(x).enumerate() {
            if xp >= xmin && xp <= xmax && min_step.is_none() {
                min_step = Some(step + 1);
            }
            if xp > xmax {
                max_step = Some(step);
                break;
            }
        }
        min_step.map(|min| ((min, max_step), x))

        // x_posses(x)
        //     .enumerate()
        //     .filter(move |&(_, p)| p >= xmin && p <= xmax)
        //     .map(move |(ix, _)| (ix, x))
    })
}
fn find_y_speeds(ymin: i64, ymax: i64) -> impl Iterator<Item = (usize, i64)> {
    (ymin..=-ymin).rev().flat_map(move |y| {
        y_posses_to(y, ymin)
            .enumerate()
            .filter(move |&(_, p)| p >= ymin && p <= ymax)
            .map(move |(ix, _)| (ix + 1, y))
    })
}
fn p1(input: &str) -> i64 {
    let (xmin, xmax, ymin, ymax) =
        scan_fmt!(input, "target area: x={}..{}, y={}..{}", i64, i64, i64, i64).unwrap();
    let c1 = Point::new(xmin, ymin);
    let c2 = Point::new(xmax, ymax);
    // dbg!(x_posses(10).collect_vec());
    let (steps, xvel_min) = dbg!(find_x_speeds(xmin, xmax).next().unwrap());
    //now want a yvel that hits target in steps steps.
    dbg!(y_posses_to(9, ymin).collect_vec());
    let x = find_y_speeds(ymin, ymax).collect_vec();
    // for c in x {
    //     dbg!(c, y_posses_to(c, ymin).collect_vec());
    // }
    let p = dbg!(find_y_speeds(ymin, ymax).collect_vec())[0];
    y_posses_to(p.1, ymin).max().unwrap()
}

fn p2(input: &str) -> usize {
    let (xmin, xmax, ymin, ymax) =
        scan_fmt!(input, "target area: x={}..{}, y={}..{}", i64, i64, i64, i64).unwrap();
    let c1 = Point::new(xmin, ymin);
    let c2 = Point::new(xmax, ymax);
    // dbg!(x_posses(10).collect_vec());
    let (steps, xvel_min) = dbg!(find_x_speeds(xmin, xmax).next().unwrap());
    //now want a yvel that hits target in steps steps.
    dbg!(y_posses_to(9, ymin).collect_vec());
    let ys = find_y_speeds(ymin, ymax).collect_vec();
    let xs = find_x_speeds(xmin, xmax).collect_vec();
    // for c in x {
    //     dbg!(c, y_posses_to(c, ymin).collect_vec());
    // }
    let mut ans = BTreeSet::new();
    dbg!(&xs);
    dbg!(&ys);
    for (&((xmin, xmax), x), &(yix, y)) in xs.iter().cartesian_product(ys.iter()) {
        let max_ok = match xmax {
            Some(xm) => yix <= xm,
            None => true,
        };
        let min_ok = xmin <= yix;
        if max_ok && min_ok {
            ans.insert((x, y));
        }
    }
    let a = "23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
8,-2    27,-8   30,-5   24,-7";
    for x in a.split_whitespace() {
        let (x, y) = scan_fmt!(x, "{},{}", i64, i64).unwrap();
        if !ans.contains(&(x, y)) {
            println!("Missing: {},{}", x, y);
        }
    }

    // dbg!(&ans);
    ans.len()
}
