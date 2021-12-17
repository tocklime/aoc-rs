use std::{collections::VecDeque, convert::Infallible, str::FromStr};

use scan_fmt::scan_fmt;

use aoc_harness::*;

aoc_main!(2021 day 17, generator whole_input_is::<Day17>, part1 [p1] => 8911, part2 [p2] => 4748, example part1 EG => 45, example part2 EG => 112);

const EG: &str = "target area: x=20..30, y=-10..-5";
#[derive(Debug)]
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
) -> impl Iterator<Item =  (usize, usize)>
where
    T: Fn(i64, i64) -> bool + Copy,
{
    range.filter_map(move |x| {
        let mut min_step = None;
        let mut max_step = 0;
        let mut in_range = true;
        let (x, time_offset) = if x.signum() != target.0.signum() {
            //we're firing in the wrong direction! (which we only do in the y direction)
            //first 2n steps will take us back to point zero with opposite velocity, so lets skip those.
            (-x, 2 * usize::try_from(x.abs()).unwrap())
        } else {
            (x, 0)
        };
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
            //it finished the iteration in range,
            //so it will always stay in range.
            max_step = usize::MAX;
        } else {
            //otherwise, we want top-end-exclusive ranges, because they're easier to reason about.
            max_step += 1;
        }
        min_step.map(|min| (min + time_offset, max_step + time_offset))
    })
}

fn p1(i: &Day17) -> i64 {
    //fastest downward speed that hits is downward so it hits the end of target in 1 step.
    //hence, highest we can fire it upwards and hit is -1 * that.
    i.y.0 * (i.y.0 + 1) / 2
}

fn p2(i: &Day17) -> usize {
    let ys = find_speeds(i.y, (i.y.0..-i.y.0).rev(), move |pos, _| pos < i.y.0);
    let mut xs = find_speeds(i.x, 0..i.x.1, move |pos, vel| vel < 0 || pos > i.x.1).peekable();
    let mut matching_xs = VecDeque::new();
    //roughly, both xs and ys are in descending order of time (slowest shots first).
    //we iterate over the ys, and keep a sliding window (the VecDeque) of xs which match
    ys.map(| (y_min, y_max)| {
        //first, pull in all the matching xs we can.
        loop {
            match xs.peek() {
                Some(&(xmin, _)) if xmin >= y_max => {
                    xs.next().unwrap(); //too big for this, the biggest y. drop it.
                }
                Some(&(_, xmax)) if xmax <= y_min => break, //too small for this y, save it for later, but we're done pulling.
                Some(_) => matching_xs.push_back(xs.next().unwrap().0), //otherwise it matches. We only need to remember the xmin, because that's what we need to check against to remove it later.
                None => break, //end of the xs. we're done pulling.
            }
        }
        //now remove xs that no longer fit from matching_xs.
        while matching_xs
            .get(0)
            .map_or(false, |&xmin| xmin >= y_max)
        {
            //too big for this y, drop it.
            matching_xs.pop_front();
        }
        // now matching_xs contains all xs that match this y.
        matching_xs.len()
    })
    .sum()
}
