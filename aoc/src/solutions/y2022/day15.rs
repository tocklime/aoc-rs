use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashSet},
};

use aoc_harness::*;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::all_consuming,
    multi::many1,
    sequence::tuple,
    IResult,
};
use utils::{aabb::Aabb, cartesian::Point, span::Span};

aoc_main!(2022 day 15, generator gen, part1 [p1] => 5_607_466, part2 [scanning_axes, dividing_quadrants, analysing_edges] => 12_543_202_766_584, example both EG => (26, 56_000_011));

const EG: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

#[derive(Debug)]
struct Sensor {
    location: Point<i64>,
    closest_beacon: Point<i64>,
    range: i64,
}
struct X {
    sensors: Vec<Sensor>,
    max_coord: i64,
}

impl Sensor {
    fn can_see(&self, p: Point<i64>) -> bool {
        (self.location - p).manhattan() <= self.range
    }
    fn can_see_all(&self, bb: Aabb<i64>) -> bool {
        let furthest_x = ((bb.bottom_left.x - self.location.x).abs())
            .max((bb.top_right.x - self.location.x).abs());
        let furthest_y = ((bb.bottom_left.y - self.location.y).abs())
            .max((bb.top_right.y - self.location.y).abs());
        furthest_x + furthest_y <= self.range
    }
    fn shadow_y(&self, target: i64) -> Option<Span<i64>> {
        let clear_range = self.range;
        let delta = (self.location.y - target).abs();
        let shadow_size = clear_range * 2 + 1 - 2 * delta;
        if shadow_size > 0 {
            let shadow_start = self.location.x - clear_range + delta;
            let shadow = Span::new_from_range(shadow_start..shadow_start + shadow_size);
            Some(shadow)
        } else {
            None
        }
    }
}
fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, (_, xa, _, ya, _, xb, _, yb, _)) = tuple((
        tag("Sensor at x="),
        complete::i64,
        tag(", y="),
        complete::i64,
        tag(": closest beacon is at x="),
        complete::i64,
        tag(", y="),
        complete::i64,
        newline,
    ))(input)?;
    let location = Point::new(xa, ya);
    let closest_beacon = Point::new(xb, yb);
    Ok((
        input,
        Sensor {
            location,
            closest_beacon,
            range: (location - closest_beacon).manhattan(),
        },
    ))
}
fn gen(input: &str) -> X {
    let sensors = all_consuming(many1(parse_sensor))(input).unwrap().1;
    X {
        sensors,
        max_coord: if input == EG { 20 } else { 4_000_000 },
    }
}
fn tuning_frequency(p: Point<i64>) -> i64 {
    p.x * 4_000_000 + p.y
}

fn p1(input: &X) -> i64 {
    let target_y = input.max_coord / 2;
    let mut shadows: BinaryHeap<_> = input
        .sensors
        .iter()
        .filter_map(|x| x.shadow_y(target_y).map(Reverse))
        .collect();
    let mut count = 0;
    let mut cur_span = shadows.pop().unwrap().0;

    while let Some(Reverse(s)) = shadows.pop() {
        if s.start > cur_span.end {
            //gap!
            cur_span = s;
            count += s.size();
        } else {
            cur_span = s.union(&cur_span);
        }
    }
    let y_beacons: HashSet<Point<i64>> = input
        .sensors
        .iter()
        .map(|l| l.closest_beacon)
        .filter(|l| l.y == target_y)
        .collect();
    count += cur_span.size();
    count - y_beacons.len() as i64
}

fn has_gap<F>(sensors: &[Sensor], full_size: i64, get_shadow: F) -> bool
where
    F: Fn(&Sensor) -> Option<Span<i64>>,
{
    let mut shadows: BinaryHeap<_> = sensors
        .iter()
        .filter_map(|x| get_shadow(x).map(Reverse))
        .collect();

    let mut cur_span = shadows.pop().unwrap().0;
    while let Some(Reverse(s)) = shadows.pop() {
        if s.start > cur_span.end {
            return true;
        }
        cur_span = cur_span.union(&s);
    }
    cur_span.start > 0 || cur_span.end <= full_size
}

fn scanning_axes(input: &X) -> i64 {
    let found_y = (0..=input.max_coord)
        .find(|&y| has_gap(&input.sensors, input.max_coord, |p| p.shadow_y(y)))
        .unwrap();
    //now we know the y coordinate, find the x.
    let found_x = (0..=input.max_coord)
        .find(|&x| !input.sensors.iter().any(|s| s.can_see(Point::new(x, found_y))))
        .unwrap();
    tuning_frequency(Point::new(found_x, found_y))
}

fn dividing_quadrants(input: &X) -> i64 {
    let bb = Aabb::origin_and(Point::new(input.max_coord, input.max_coord));
    let mut to_search = vec![bb];
    while let Some(x) = to_search.pop() {
        if x.area() == 0 || input.sensors.iter().any(|s| s.can_see_all(x)) {
            //zero sized or covered by some sensor.
        } else if x.area() == 1 {
            return tuning_frequency(x.bottom_left);
        } else {
            let new = x.quadrants();
            to_search.extend(new);
        }
    }
    unreachable!()
}

fn analysing_edges(input: &X) -> i64 {
    //each sensor casts 2 / direction lines and 2 \ direction lines from the edges of the
    //diamond of its range.
    //the unique uncovered square must be in the middle of an intersection of these.
    //we assume that the uncovered square is not on the edge of the world, and so only need to
    //consider 1 of the 2 in each pair of parallel lines.
    //let's record the positive and negative lines we see on the top-left and bottom-left slopes.
    let mut pos_lines = BTreeSet::new();
    let mut neg_lines = BTreeSet::new();
    for s in &input.sensors {
        let l = s.location - Point::new(s.range + 1, 0);
        pos_lines.insert(l.y - l.x);
        neg_lines.insert(l.y + l.x);
    }
    //now lets see if any of the top-right and bottom-right edges are coincident.
    let mut double_pos_lines = Vec::new();
    let mut double_neg_lines = Vec::new();
    for s in &input.sensors {
        let r = s.location + Point::new(s.range + 1, 0);
        if pos_lines.contains(&(r.y - r.x)) {
            double_pos_lines.push(r.y - r.x);
        }
        if neg_lines.contains(&(r.y + r.x)) {
            double_neg_lines.push(r.y + r.x);
        }
    }

    //every quadruplet of lines cross /somewhere/. Which ones cross in range?
    let bb = Aabb::origin_and(Point::new(input.max_coord, input.max_coord));
    let p = double_pos_lines
        .into_iter()
        .cartesian_product(double_neg_lines)
        .map(|(p, n)| Point::new((n - p) / 2, (n + p) / 2))
        .find(|x| bb.contains(x) && input.sensors.iter().all(|s| !s.can_see(*x)))
        .unwrap();
    tuning_frequency(p)
}
