use std::{convert::Infallible, str::FromStr};

use scan_fmt::scan_fmt;

use aoc_harness::*;

aoc_main!(2021 day 17, generator whole_input_is::<Day17>, part1 [p1] => 8911, part2 [p2] => 4748, example part1 EG => 45, example part2 EG => 112);

const EG: &str = "target area: x=20..30, y=-10..-5";

struct Day17 {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}
impl FromStr for Day17 {
    type Err = Infallible;

    #[allow(clippy::similar_names)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (xmin, xmax, ymin, ymax) =
            scan_fmt!(s, "target area: x={}..{}, y={}..{}", i64, i64, i64, i64).unwrap();
        Ok(Self {
            xmin,
            xmax,
            ymin,
            ymax,
        })
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
fn find_x_speeds(xmin: i64, xmax: i64) -> impl Iterator<Item = ((usize, usize), i64)> {
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
        min_step.map(|min| ((min, max_step.unwrap_or(usize::MAX)), x))
    })
}
fn find_y_speeds(ymin: i64, ymax: i64) -> impl Iterator<Item = ((usize, usize), i64)> {
    (ymin..=-ymin).rev().filter_map(move |y| {
        let mut min_step = None;
        let mut max_step = None;
        for (step, yp) in y_posses_to(y, ymin).enumerate() {
            if yp >= ymin && yp <= ymax {
                if min_step.is_none() {
                    min_step = Some(step + 1);
                }
                max_step = Some(step + 1);
            }
        }
        match (min_step, max_step) {
            (Some(min), Some(max)) => Some(((min, max), y)),
            _ => None,
        }
    })
}
fn p1(input: &Day17) -> i64 {
    let p = find_y_speeds(input.ymin, input.ymax).collect_vec()[0];
    y_posses_to(p.1, input.ymin).max().unwrap()
}

fn p2(input: &Day17) -> usize {
    //now want a yvel that hits target in steps steps.
    let ys = find_y_speeds(input.ymin, input.ymax).collect_vec();
    let xs = find_x_speeds(input.xmin, input.xmax).collect_vec();
    xs.iter()
        .cartesian_product(ys.iter())
        .filter(|&(x, y)| !(x.0 .1 < y.0 .0 || y.0 .1 < x.0 .0))
        .count()
}
