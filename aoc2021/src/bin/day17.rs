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
/// for a given speed initial speed, return an iterator of all positions hit.
/// stops when check returns true on the old position or the current velocity.
fn posses<F>(mut vel: i64, check: F) -> impl Iterator<Item = i64>
where
    F: Fn(i64, i64) -> bool,
{
    let mut pos = 0;
    std::iter::from_fn(move || {
        let old_pos = pos;
        pos += vel;
        vel -= 1;
        if check(old_pos, vel) {
            None
        } else {
            Some(pos)
        }
    })
}

/// for a given target, find all speeds in `range` which ever hit the target. Return an iterator of
/// the range of step numbers that hit it, and the speed.
fn find_speeds<T>(
    target_min: i64,
    target_max: i64,
    range: impl Iterator<Item = i64>,
    check: T,
) -> impl Iterator<Item = ((usize, usize), i64)>
where
    T: Fn(i64, i64) -> bool + Copy,
{
    range.filter_map(move |x| {
        let mut min_step = None;
        let mut max_step = None;
        let mut in_range = true;
        for (step, xp) in posses(x, check).enumerate() {
            in_range = xp >= target_min && xp <= target_max;
            if in_range {
                if min_step.is_none() {
                    min_step = Some(step + 1);
                }
                max_step = Some(step + 1);
            }
        }
        if in_range {
            max_step = Some(usize::MAX);
        }
        min_step.map(|min| ((min, max_step.unwrap_or(usize::MAX)), x))
    })
}

fn p1(i: &Day17) -> i64 {
    let p = find_speeds(i.ymin, i.ymax, (i.ymin..=-i.ymin).rev(), move |pos, _| {
        pos < i.ymin
    })
    .next()
    .unwrap();
    posses(p.1, move |pos, _| pos < i.ymin).max().unwrap()
}

fn p2(i: &Day17) -> usize {
    let ys = find_speeds(i.ymin, i.ymax, (i.ymin..=-i.ymin).rev(), move |pos, _| {
        pos < i.ymin
    })
    .collect_vec();
    let xs = find_speeds(i.xmin, i.xmax, 0..=i.xmax, move |pos, vel| {
        vel < 0 || pos > i.xmax
    })
    .collect_vec();
    xs.iter()
        .cartesian_product(ys.iter())
        .filter(|&((x, _), (y, _))| !(x.1 < y.0 || y.1 < x.0))
        .count()
}
