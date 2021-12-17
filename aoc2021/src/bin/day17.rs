use std::{
    collections::{BTreeMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

use scan_fmt::scan_fmt;

use aoc_harness::*;
use utils::span::Span;

aoc_main!(2021 day 17, generator whole_input_is::<Day17>, part1 [p1] => 8911, part2 [p2] => 4748, example part1 EG => 45, example part2 EG => 112);

const EG: &str = "target area: x=20..30, y=-10..-5";

struct Day17 {
    x: (i64, i64),
    y: (i64, i64),
}
impl FromStr for Day17 {
    type Err = Infallible;

    #[allow(clippy::similar_names)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (xmin, xmax, ymin, ymax) =
            scan_fmt!(s, "target area: x={}..{}, y={}..{}", i64, i64, i64, i64).unwrap();
        Ok(Self {
            x: (xmin, xmax + 1),
            y: (ymin, ymax + 1),
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
/// the speed and the range of step numbers that hit the target
fn find_speeds<T>(
    target: (i64, i64),
    range: impl Iterator<Item = i64>,
    check: T,
) -> impl Iterator<Item = (i64, (usize, usize))>
where
    T: Fn(i64, i64) -> bool + Copy,
{
    range.filter_map(move |x| {
        let mut min_step = None;
        let mut max_step = 0;
        let mut in_range = true;
        for (step, xp) in posses(x, check).enumerate() {
            in_range = xp >= target.0 && xp < target.1;
            if in_range {
                if min_step.is_none() {
                    min_step = Some(step);
                }
                max_step = step;
            }
        }
        if in_range {
            max_step = usize::MAX;
        } else {
            max_step += 1;
        }
        min_step.map(|min| (x, (min, max_step)))
    })
}

fn p1(i: &Day17) -> i64 {
    let p = find_speeds(i.y, (i.y.0..-i.y.0).rev(), move |pos, _| pos < i.y.0)
        .next()
        .unwrap();
    p.0 * (p.0 + 1) / 2
}

fn p2(i: &Day17) -> usize {
    let ys = find_speeds(i.y, (i.y.0..-i.y.0).rev(), move |pos, _| pos < i.y.0);
    let xs = find_speeds(i.x, 0..i.x.1, move |pos, vel| vel < 0 || pos > i.x.1).collect_vec();
    //ys is sorted in desceding y. -> start times decrease, end times decrease.
    //xs is sorted in ascending x. -> start times decrease, end times decrease.
    ys.map(|(_, t_y)| {
        xs.iter()
            //since xs times descend, we can skip all those at the start where the x start time is greater than the y end...
            .skip_while(|(_, t_x)| t_y.1 <= t_x.0)
            //..and we only need to look at those where the end time is at least the start time of y.
            .take_while(|(_, t_x)| t_x.1 > t_y.0)
            //everything else must match.
            .count()
    })
    .sum::<usize>()
}
